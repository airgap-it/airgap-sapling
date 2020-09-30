/**
 * For the provided viewing key get a payment address with the specified index if valid, or the first valid one otherwise
 */

import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createIndexedAddress(): Promise<SaplingPaymentAddress> {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)

  const index: Buffer | Int8Array | string | number = 1

  // call `sapling#getPaymentAddressFromViewingKey` with an extended full viewing key and a specified diversifier index
  // to get a payment address with this or the next valid index
  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey, index)

  return address
}

createIndexedAddress()
  .then((address: SaplingPaymentAddress) => {
    console.log('address.index =', address.index.toString('hex'))
    console.log('address.raw =', address.raw.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })

