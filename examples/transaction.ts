/**
 * Prepare a sapling transaction
 */

import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

const STATE: State = {
  root: 'efa23f82682391842c1fdaa56b2eef75da71e4034569ec50f389aa80d3fb392d',
  accounts: {
    'alice': {
    inputs: [20, 50, 25, 15, 70]
    },
    'bob': {
      inputs: []
    }
  }
}
const TO_TRANSFER: number = 100

// TODO: set valid keys
const PROVING_KEY: string = ''
const VERIFYING_KEY: string = ''

// a sapling transaction consists of a list of spend descriptions, list of output descriptions and a binding signature
interface Transaction {
  spendDescriptions: Buffer[]
  outputDescriptions: Buffer[]
  bindingSignature: Buffer
}

async function prepareAndSignTransaction(): Promise<Transaction> {
  const [account, inputs, outputs]: [Account, Input[], Output[]] = await setupAccounts()
  
  // generate a public key re-randomizer used to pepare and sign spending descriptions
  const ar: Buffer = await sapling.randR()

  const unsigned: Transaction = await prepareTransaction(account, inputs, outputs, ar)
  const signed: Transaction = await signTransactionInputs(account, unsigned, ar)

  return signed
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
  const transaction: Transaction = await sapling.withProvingContext<Promise<Transaction>>(async (context: number) => {
    const spendDescriptions: Buffer[] = await Promise.all(inputs.map((input: Input) => {
      return sapling.prepareSpendDescription(
        context,
        account.spendingKey,
        input.address,
        input.rcm,
        ar,
        input.value,
        STATE.root,
        getWitness(input.position),
        PROVING_KEY,
        VERIFYING_KEY
      )
    }))

    // generate a commitment randomness
    const rcm = await sapling.randR()
    const outputDescriptions: Buffer[] = await Promise.all(outputs.map((output: Output) => {
      return sapling.prepareOutputDescription(
        context,
        account.viewingKey,
        output.destination,
        rcm,
        output.value,
        PROVING_KEY
      )
    }))

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
  const signedSpendDescrptions: Buffer[] = await Promise.all(transaction.spendDescriptions.map((spendDescription: Buffer) => {
    // create data to be signed
    const sighash = sighashSpendDescription(spendDescription)

    return sapling.signSpendDescription(spendDescription, account.spendingKey, ar, sighash)
  }))
  
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
  accounts: Record<AccountName, AccountState>
}

interface AccountState {
  inputs: number[]
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
  const inputs: number[] = []

  let sum: number = 0
  for (const input of STATE.accounts[account.name].inputs) {
    if (sum >= value) {
      break
    }
    inputs.push(input)
    sum += input
  }

  return inputs.map((input: number, index: number) => {
    return {
      rcm,
      position: index,
      address,
      value: input
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

function getWitness(position: number): string {
  // TODO: return valid witness
  return ''
}

function sighashDescriptions(spendDescriptions: Buffer[], outputDescriptions: Buffer[]): Buffer {
  const descriptions = spendDescriptions.concat(outputDescriptions)
  return Buffer.concat(descriptions)
}

function sighashSpendDescription(spendDescription: Buffer): Buffer {
  return spendDescription
}