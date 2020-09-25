/**
 * Create an extendend spending key from a mnemonic
 */

import * as bip39 from 'bip39'
import * as sapling from '@airgap/sapling-wasm'

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