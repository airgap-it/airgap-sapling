/**
 * For the provided extended full viewing key get the default payment address
 *
 * Call `npm run build` before running this example.
 */

import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createAddress(): Promise<SaplingPaymentAddress> {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  // call `sapling#getPaymentAddressFromViewingKey` with an extended full viewing key as the only argument
  // to get its default payment address (first valid)
  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey)

  return address
}

createAddress()
  .then((address: SaplingPaymentAddress) => {
    console.log('address.index =', address.index.toString('hex'))
    console.log('address.raw =', address.raw.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })
