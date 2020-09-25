/**
 * Create an extended full viewing key from a mnemonic
 */

import * as bip39 from 'bip39'
import * as sapling from '@airgap/sapling-wasm'

async function createExtendedFullViewingKey() {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  return viewingKey
}

createExtendedFullViewingKey()
  .then((viewingKey: Buffer) => {
    console.log('viewingKey =', viewingKey.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })