/**
 * For the provided extended full viewing key get a default payment address
 */

import * as bip39 from 'bip39'
import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'

async function createAddress() {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey)

  return address
}

createAddress()
  .then((address: SaplingPaymentAddress) => {
    console.log(
      'address.index =', address.index.toString('hex'),
      'address.raw =', address.raw.toString('hex')
    )
  })
  .catch((error) => {
    console.warn(error)
  })