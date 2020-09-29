/**
 * Create an extendend spending key from a mnemonic
 */

import * as sapling from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createExtendedSpendingKey(): Promise<Buffer> {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

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