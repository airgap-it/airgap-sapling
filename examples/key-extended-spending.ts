/**
 * Create an extendend spending key from a mnemonic
 */

import * as sapling from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createExtendedSpendingKey(): Promise<Buffer> {
  const mnemonic: string = bip39.generateMnemonic()
  const password: string = ''
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, password)
  const derivationPath = 'm/'

  // call `sapling#getExtendedSpendingKey` with a BIP39 seed and derivation path as arguments to create an extended spending key
  const spendingKey: Buffer = await sapling.getExtendedSpendingKey(seed, derivationPath)

  return spendingKey
}

createExtendedSpendingKey()
  .then((spendingKey: Buffer) => {
    console.log('spendingKey =', spendingKey.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })