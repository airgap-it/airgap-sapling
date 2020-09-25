/**
 * For the provided viewing key get a payment address with the specified index if valid, or the first valid one otherwise
 */

import * as bip39 from 'bip39'
import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'

async function createAddressOfIndex(index: Buffer | Int8Array | string | number) {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey, index)

  return address
}

createAddressOfIndex(1)
  .then((address: SaplingPaymentAddress) => {
    console.log(
      'address.index =', address.index.toString('hex'),
      'address.raw =', address.raw.toString('hex')
    )
  })
  .catch((error) => {
    console.warn(error)
  })

