/**
 * Create an extended full viewing key from a mnemonic
 *
 * Call `npm run build` before running this example.
 */

import * as sapling from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createExtendedFullViewingKey(): Promise<Buffer> {
  const mnemonic: string = bip39.generateMnemonic()
  const password: string = ''
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, password)
  const derivationPath = 'm/'

  // call `sapling#getExtendedFullViewingKey` with a BIP39 seed and derivation path as arguments to create an extended full viewing key
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
