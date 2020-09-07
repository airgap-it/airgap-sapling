
import 'regenerator-runtime/runtime'

import { bufferFrom, ifTypeErrorElseUnknown } from './utils'

const saplingPromise = new Promise((resolve, reject) => {
  import('sapling-wasm')
    .then((sapling) => {
      resolve(sapling)
    })
    .catch((error) => {
      reject(`Could not load sapling-wasm: ${error}`)
    })
})

/**
 * Create an extended spending key from the given seed.
 * 
 * @param {Buffer|Int8Array|string} seed A seed from which the key will be derived
 * @param {string} derivationPath A valid BIP39 derivation path
 * @returns {Buffer} The generated extended spending key
 */

export async function getExtendedSpendingKey(seed, derivationPath) {
  const sapling = await saplingPromise

  let seedBuffer
  try {
    seedBuffer = bufferFrom(seed)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`seed` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(`getExtendedSpendingKey: ${details}`)
  }

  return Buffer.from(sapling.get_extended_spending_key(seedBuffer, derivationPath))
}

/**
 * Create an extended full viewing key from the given seed.
 * 
 * @param {Buffer|Int8Array|string} seed A seed from which the key will be derived
 * @param {string} derivationPath A valid BIP39 derivation path
 * @returns {Buffer} The generated extended full viewing key
 */

export async function getExtendedFullViewingKey(seed, derivationPath) {
  const sapling = await saplingPromise

  let seedBuffer
  try {
    seedBuffer = bufferFrom(seed)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`seed` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(`getExtendedFullViewingKey: ${details}`)
  }

  return Buffer.from(sapling.get_extended_full_viewing_key(seedBuffer, derivationPath))
}

/**
 * Derive a payment address from the given extended full viewing key.
 * 
 * @typedef {Object} SaplingPaymentAddress
 * @property {Buffer} index An 11-byte diversifier index stored as a list of bytes in a little-endian (LE) format
 * @property {Buffer} raw A 32-byte raw address value
 * 
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Int8Array|string|number} [index] A 11-byte diversifier index used to determine which diversifier should be used to derive the address. If not present, a new diversifier index is created with a default value of [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getPaymentAddressFromViewingKey(viewingKey, index) {
  const sapling = await saplingPromise

  let viewingKeyBuffer
  try {
    viewingKeyBuffer = bufferFrom(viewingKey)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`viewingKey` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(`getPaymentAddressFromViewingKey: ${details}`)
  }

  let indexBuffer
  try {
    indexBuffer = index !== undefined
      ? bufferFrom(index, 11).reverse() // LE
      : undefined
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`index` is of an invalid type, expected `Buffer`, `Int8Array`, hex string or number')

    return Promise.reject(`getPaymentAddressFromViewingKey: ${details}`)
  }

  const address = Buffer.from(indexBuffer !== undefined 
    ? sapling.get_payment_address_from_viewing_key(viewingKeyBuffer, indexBuffer)
    : sapling.get_default_payment_address_from_viewing_key(viewingKeyBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

/**
 * Derive next valid payment address from the given extended full viewing key and current diversifier index.
 * 
 * @typedef {Object} SaplingPaymentAddress
 * @property {Buffer} index An 11-byte diversifier index stored as a list of bytes in a little-endian (LE) format
 * @property {Buffer} raw A 32-byte raw address value
 * 
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Int8Array|string|number} index The last used 11-byte diversifier index. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getNextPaymentAddressFromViewingKey(viewingKey, index) {
  const sapling = await saplingPromise

  let viewingKeyBuffer
  try {
    viewingKeyBuffer = bufferFrom(viewingKey)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`viewingKey` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(`getNextPaymentAddressFromViewingKey: ${details}`)
  }

  let indexBuffer
  try {
    indexBuffer = bufferFrom(index, 11).reverse() // LE
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`index` is of an invalid type, expected `Buffer`, `Int8Array`, hex string or number')

    return Promise.reject(`getNextPaymentAddressFromViewingKey: ${details}`)
  }

  const address = Buffer.from(sapling.get_next_payment_address_from_viewing_key(viewingKeyBuffer, indexBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}