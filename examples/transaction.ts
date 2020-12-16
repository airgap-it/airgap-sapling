/**
 * Prepare a sapling transaction
 *
 * Call `npm run build:for:examples` before running this example.
 */

import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'
import axios, { AxiosResponse } from 'axios'
import * as bip39 from 'bip39'
import * as BN from 'bn.js'
import * as fs from 'fs'
import * as path from 'path'

const TO_TRANSFER: number = 180

const SAPLING_PARAMS_DIR = path.resolve(__dirname, 'sapling-params')
const SPEND_PARAMS_FILE_NAME = 'sapling-spend.params'
const OUTPUT_PARAMS_FILE_NAME = 'sapling-output.params'
const ZCASH_DOWNLOAD_URL = 'https://download.z.cash/downloads'

// a sapling transaction consists of a list of spend descriptions, list of output descriptions and a binding signature
interface Transaction {
  spendDescriptions: Buffer[]
  outputDescriptions: Buffer[]
  bindingSignature: Buffer
}

async function prepareAndSignTransaction(): Promise<Transaction> {
  // initialize sapling parameters
  await initParameters()

  const [account, inputs, outputs]: [Account, Input[], Output[]] = await setupAccounts()

  // generate a public key re-randomizer used to pepare and sign spending descriptions
  const ar: Buffer = await sapling.randR()

  const unsigned: Transaction = await prepareTransaction(account, inputs, outputs, ar)
  const signed: Transaction = await signTransactionInputs(account, unsigned, ar)

  return signed
}

async function initParameters(): Promise<void> {
  const [spendParams, outputParams] = await Promise.all([prepareParams(SPEND_PARAMS_FILE_NAME), prepareParams(OUTPUT_PARAMS_FILE_NAME)])

  return sapling.initParameters(spendParams, outputParams)
}

async function setupAccounts(): Promise<[Account, Input[], Output[]]> {
  const alice: Account = await createAccount('alice')
  const bob: Account = await createAccount('bob')
  const value: number = TO_TRANSFER

  const aliceInputs: Input[] = await getInputs(alice, value)
  const aliceOutputs: Output[] = await createOuputs(alice, bob, value, aliceInputs)

  return [alice, aliceInputs, aliceOutputs]
}

async function prepareTransaction(account: Account, inputs: Input[], outputs: Output[], ar: Buffer): Promise<Transaction> {
  // all calls to `sapling#prepareSpendDescription`, `sapling#prepareOutputDescription` and `sapling#createBindingSignature`
  // must be performed with the same proving context, use `withProvingContext` to safely execute a block of code with a proving context instance
  const transaction: Transaction = await sapling.withProvingContext(async (context: number) => {
    const spendDescriptions: Buffer[] = await Promise.all(
      inputs.map(async (input: Input) => {
        const description = await sapling.prepareSpendDescription(
          context,
          account.spendingKey,
          input.address,
          input.rcm,
          ar,
          input.value,
          STATE.root,
          await getWitness(input.position)
        )
        return description
      })
    )

    // generate a commitment randomness
    const rcm = await sapling.randR()
    const outputDescriptions: Buffer[] = await Promise.all(
      outputs.map(async (output: Output) => {
        return sapling.prepareOutputDescription(context, account.viewingKey, output.destination, rcm, output.value)
      })
    )

    const inputSum: number = inputs.reduce((sum: number, next: Input) => sum + next.value, 0)
    const outputSum: number = outputs.reduce((sum: number, next: Output) => sum + next.value, 0)
    const valueBalance: number = inputSum - outputSum

    // create data to be signed
    const sighash = sighashDescriptions(spendDescriptions, outputDescriptions)
    const bindingSignature = await sapling.createBindingSignature(context, valueBalance, sighash)

    return {
      spendDescriptions,
      outputDescriptions,
      bindingSignature
    }
  })

  return transaction
}

async function signTransactionInputs(account: Account, transaction: Transaction, ar: Buffer): Promise<Transaction> {
  const signedSpendDescrptions: Buffer[] = await Promise.all(
    transaction.spendDescriptions.map((spendDescription: Buffer) => {
      // create data to be signed
      const sighash = sighashSpendDescription(spendDescription)

      return sapling.signSpendDescription(spendDescription, account.spendingKey, ar, sighash)
    })
  )

  return {
    spendDescriptions: signedSpendDescrptions,
    outputDescriptions: transaction.outputDescriptions,
    bindingSignature: transaction.bindingSignature
  }
}

prepareAndSignTransaction()
  .then((transaction: Transaction) => {
    console.log('transaction.spendDescriptions')
    transaction.spendDescriptions.forEach((desc: Buffer) => {
      console.log('\t', desc.toString('hex'))
    })
    console.log('transaction.outputDescriptions')
    transaction.outputDescriptions.forEach((desc: Buffer) => {
      console.log('\t', desc.toString('hex'))
    })
    console.log('transaction.bindingSignature')
    console.log('\t', transaction.bindingSignature.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })

/**
 * Utils
 */

type AccountName = 'alice' | 'bob'

interface State {
  root: string
  tree: StateTree
  accounts: Record<AccountName, AccountState>
}

interface StateTree {
  height: number
  size: number
  tree: Tree
}

type Tree = undefined | string | [string, Tree, Tree]

interface AccountState {
  inputs: [number, number][]
}

interface Account {
  name: AccountName
  spendingKey: Buffer
  viewingKey: Buffer
}

interface Input {
  rcm: Buffer
  position: number
  address: SaplingPaymentAddress
  value: number
}

interface Output {
  destination: SaplingPaymentAddress
  value: number
}

async function createAccount(name: AccountName): Promise<Account> {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const spendingKey: Buffer = await sapling.getExtendedSpendingKey(seed, derivationPath)
  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  return {
    name,
    spendingKey,
    viewingKey
  }
}

async function getInputs(account: Account, value: number): Promise<Input[]> {
  const rcm: Buffer = await sapling.randR()
  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(account.viewingKey)
  const inputs: [number, number][] = []

  let sum: number = 0
  for (const input of STATE.accounts[account.name].inputs) {
    if (sum >= value) {
      break
    }
    inputs.push(input)
    sum += input[1]
  }

  return inputs.map(([position, value]: [number, number]) => {
    return {
      rcm,
      position,
      address,
      value
    }
  })
}

async function createOuputs(account: Account, to: Account, value: number, inputs: Input[]): Promise<Output[]> {
  const paybackAddress: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(account.viewingKey)
  const destinationAddress: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(to.viewingKey)

  const inputSum: number = inputs.reduce((sum: number, next: Input) => sum + next.value, 0)
  const diff = inputSum - value

  return [
    {
      destination: paybackAddress,
      value: diff
    },
    {
      destination: destinationAddress,
      value
    }
  ]
}

async function getWitness(position: number): Promise<string> {
  const heightBuffer: Buffer = new BN(STATE.tree.height).toBuffer('le')
  const positionBuffer: Buffer = new BN(position).toBuffer('le', 8)

  const neighboringHashes: Buffer[] = await getNeighboringHashes([], STATE.tree.height, new BN(position), STATE.tree.tree)
  const witness: Buffer = neighboringHashes
    .map((hash: Buffer) => Buffer.concat([new BN(hash.length).toBuffer('le'), hash]))
    .reverse()
    .reduce((acc: Buffer, next: Buffer) => Buffer.concat([acc, next]))

  return Buffer.concat([heightBuffer, witness, positionBuffer]).toString('hex')
}

async function getNeighboringHashes(acc: Buffer[], height: number, position: BN, tree: Tree): Promise<Buffer[]> {
  if (typeof tree === 'undefined') {
    return Promise.reject('Invalid tree')
  } else if (typeof tree === 'string') {
    return acc
  } else {
    const full: BN = new BN(2).pow(new BN(height - 1))
    const [nextPosition, nextTree, otherTree]: [BN, Tree, Tree] = position.lt(full)
      ? [position, tree[1], tree[2]]
      : [position.sub(full), tree[2], tree[1]]

    return getNeighboringHashes([await getRootHeight(otherTree, height - 1), ...acc], height - 1, nextPosition, nextTree)
  }
}

async function getRootHeight(tree: Tree, height: number): Promise<Buffer> {
  if (typeof tree === 'undefined') {
    const uncommitedHashes = await createUncommitedHashes()
    return uncommitedHashes[height]
  } else if (typeof tree === 'string') {
    return Buffer.from(tree, 'hex')
  } else {
    return Buffer.from(tree[0], 'hex')
  }
}

async function createUncommitedHashes(): Promise<Buffer[]> {
  const height: number = STATE.tree.height
  const res: Buffer[] = [Buffer.from(uncomittedHash, 'hex')]
  for (var i = 0; i < height; i++) {
    const hash: Buffer = res[i]
    res[i + 1] = await sapling.merkleHash(i, hash, hash)
  }

  return res
}

function sighashDescriptions(spendDescriptions: Buffer[], outputDescriptions: Buffer[]): Buffer {
  const descriptions = spendDescriptions.concat(outputDescriptions)
  return Buffer.concat(descriptions)
}

function sighashSpendDescription(spendDescription: Buffer): Buffer {
  return spendDescription
}

async function prepareParams(name: string): Promise<Buffer> {
  const paramsFilePath: string = path.resolve(SAPLING_PARAMS_DIR, name)

  if (!fs.existsSync(paramsFilePath)) {
    await fetchSaplingParams(name)
  }

  return fs.readFileSync(paramsFilePath)
}

async function fetchSaplingParams(name: string): Promise<void> {
  const response: AxiosResponse = await axios.get(`${ZCASH_DOWNLOAD_URL}/${name}`, { responseType: 'stream' })

  fs.mkdirSync(SAPLING_PARAMS_DIR, { recursive: true })
  const writer: fs.WriteStream = fs.createWriteStream(path.resolve(SAPLING_PARAMS_DIR, name))

  return new Promise((resolve, reject) => {
    response.data.pipe(writer)
    let error: Error | undefined = undefined
    writer.on('error', (err: Error) => {
      error = err
      writer.close()
    })

    writer.on('close', () => {
      if (error !== undefined) {
        reject(error)
      } else {
        resolve()
      }
    })
  })
}

/**
 * State
 */

const uncomittedHash: string = '0100000000000000000000000000000000000000000000000000000000000000'

const tree: Tree = [
  '9d9407eb5b8098dbefd35c38816c8770eb7d3f39adee18ccb3a1731cca642003',
  [
    '1e250772944354be9d58bcf8c23dba7a122b60794f7b973aa63e904074940634',
    [
      'e2376ba51515954e02e9d97ed1aa3251ed84a2eb2f4e701eb515b51497b09d38',
      [
        'b35650990247382b14fb96abc47c50c5ac8c442f80b947bf9192e99e857c7d54',
        [
          'b27ef907ba65df8bfa6cd084d3c7c9d5995d513c45e111b34839997676cdd457',
          [
            'e27c106b4b68eed3c564df3fa186ccab242ead052015ce81f423298dbf84e81e',
            [
              '529c908f2b720c0a871621e727034679485c96b86006796524171398666ee215',
              [
                'fa80843729b2e44bec97c04b17d292a0c07038c203d7964d13668093cdb2236a',
                [
                  '783a4cade9587a1dd0875cc9e35011e542e4e204ef4c5f66387155fc983e9547',
                  [
                    '5c31b7dafed3ab76b3068af5082851cf2f9024bd26f138423578968e10c0ef01',
                    [
                      '9f9d35e4abe11adb0f644f7c19de6b4fb10aa06bdd497312cb7f83cda1279b13',
                      [
                        '358d02fd1ffd2de70a5e42b39bf971c447361b2bb96b7553b35f885db8d57b02',
                        [
                          'b9ad6617e5fddd68fde387fbc19692d21ac4d4193bb5d4dd59284dc8aeafa95d',
                          [
                            '50c8ffa8ffe12c908fefe62cc7d4716a1088d3514dee96c06de3fa0ac30c3155',
                            [
                              'eb9c12ddeba5d9de82b90b53eb12589bd731885059e240c9d1084bfe0d650531',
                              [
                                '24d096793cfd206fbb025d90be324dec2095b5c04bc529cdce6f90e3a002ae09',
                                [
                                  'd819a259b34e6fc967349ad1c93c9396ad8fe18f92bd18d48321509cb3d74d10',
                                  [
                                    '17323396d39c01da7dd34490ff2a06ff5bd151fe7f82215ed10de7343bc08329',
                                    [
                                      '96d16b1d5da7e8a1ff5d6b30eebaf4d0ed70bd4bd19ea52acc117c0979534d31',
                                      [
                                        'cbaa6754a2bf5106ab30f7f2dcc79a0089cf2771986287d165aaa78299a4a24d',
                                        [
                                          '8433538fd058e8d6a9d7176e11773c5a40987fe616e44ff8403a5b7f1e70b55f',
                                          [
                                            'dcefadb8fd7d50b06ddb7a2afb32bb87fec3653e16f423197830e605cb33663f',
                                            [
                                              'd55d1f7f3881e0536cfbeb5b8994a1271fb70204918e5620c85de1e7e0418428',
                                              [
                                                'aa1211fdb293d729bf1000b918fa12471e7d23d7f22981a9e6e18aa2fc78ca16',
                                                [
                                                  'ba35e3b2ddd6f1c4581ce37a14e74ab016cc7d010899e50f34cd78c55ad0f334',
                                                  [
                                                    '1d0de8bbc6f7e2076fd3014e13c47d0e583a279baa9ae58a88561c1db10bdd18',
                                                    [
                                                      '806a935479a88f3c3d5aa92cae2537feae78c5ca5f839db68819206bd732b03c',
                                                      [
                                                        '53438973e8186f4ed173662888e825843f0a02d8eac21a29cc871f91aa791965',
                                                        [
                                                          '14337411e4e27dacfbeb86ff0262e34aee4c72cb0479e2fec3b81020fc1b900b',
                                                          [
                                                            '1620f771d9091f2075b3764956c8f91d9c0b9b78d122e4eb1acbde4a3f17341e',
                                                            [
                                                              'a421b09c0faf5eff6f6278c812b888906f97bfaea278f216eb4bf6aea3119e32',
                                                              [
                                                                '5288dfead5ddf5d6250febcca879207288ff2428677c28c35d379d6fe5428d17',
                                                                '39e907c0104bf8b17db44e8d8c08f67083ee3334032e97fb34c87f9cf147da29',
                                                                '0b01009b6f40e0a175f61985c17acb442e4a249814ce1dd943335d8089294d01'
                                                              ],
                                                              [
                                                                '61c2e3fb9d960c05916eaf18c5ed849be8122505352e34e7d24ab29f657fbb3c',
                                                                '2ca1346c1c85a1e92a72b5242254ebe3262a99e8ae73a5bed0b6c89f3f79ef5e',
                                                                undefined
                                                              ]
                                                            ],
                                                            undefined
                                                          ],
                                                          undefined
                                                        ],
                                                        undefined
                                                      ],
                                                      undefined
                                                    ],
                                                    undefined
                                                  ],
                                                  undefined
                                                ],
                                                undefined
                                              ],
                                              undefined
                                            ],
                                            undefined
                                          ],
                                          undefined
                                        ],
                                        undefined
                                      ],
                                      undefined
                                    ],
                                    undefined
                                  ],
                                  undefined
                                ],
                                undefined
                              ],
                              undefined
                            ],
                            undefined
                          ],
                          undefined
                        ],
                        undefined
                      ],
                      undefined
                    ],
                    undefined
                  ],
                  undefined
                ],
                undefined
              ],
              undefined
            ],
            undefined
          ],
          undefined
        ],
        undefined
      ],
      undefined
    ],
    undefined
  ],
  undefined
]

const STATE: State = {
  root: '9d9407eb5b8098dbefd35c38816c8770eb7d3f39adee18ccb3a1731cca642003',
  tree: {
    height: 32,
    size: 3,
    tree
  },
  accounts: {
    alice: {
      inputs: [
        [0, 100],
        [1, 50],
        [2, 50]
      ]
    },
    bob: {
      inputs: []
    }
  }
}
