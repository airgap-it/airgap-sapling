/**
 * Create a proof authorizing key from an extended spending key
 *
 * Call `npm run build` before running this example.
 */

import * as sapling from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createExtendedSpendingKey(): Promise<Buffer> {
  const mnemonic: string = bip39.generateMnemonic()
  const password: string = ''
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, password)
  const derivationPath = 'm/'

  const spendingKey: Buffer = await sapling.getExtendedSpendingKey(seed, derivationPath)
  // call `sapling#getProofAuthorizingKey` with an extended spending key as an argument to create an proof authorizing key
  const authorizingKey: Buffer = await sapling.getProofAuthorizingKey(spendingKey)

  return authorizingKey
}

createExtendedSpendingKey()
  .then((authorizingKey: Buffer) => {
    console.log('authorizingKey =', authorizingKey.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })
