/**
 * For the provided extended full viewing key and index get the next valid address
 */

import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'
import * as bip39 from 'bip39'

async function createNextPaymentAddress(): Promise<SaplingPaymentAddress> {
  const mnemonic: string = bip39.generateMnemonic()
  const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
  const derivationPath: string = 'm/'

  const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)
  const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey)

  // call `sapling#getNextPaymentAddressFromViewingKey` with an extended full viewing key and a specified index
  // to get a payment address with the next valid index
  const nextAddress: SaplingPaymentAddress = await sapling.getNextPaymentAddressFromViewingKey(viewingKey, address.index)

  return nextAddress
}

createNextPaymentAddress()
  .then((nextAddress: SaplingPaymentAddress) => {
    console.log('nextAddress.index =', nextAddress.index.toString('hex'))
    console.log('nextAddress.raw =', nextAddress.raw.toString('hex'))
  })
  .catch((error) => {
    console.warn(error)
  })
