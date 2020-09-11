
import 'regenerator-runtime/runtime'

import { getPaymentAddressXfvk, getNextPaymentAddressXfvk } from './internal/payment_address'
import { getXsk } from './internal/spending_key'
import { getOutputDescriptionFromXfvk } from './internal/transaction'
import { getXfvk } from './internal/viewing_key'
import { reject } from './internal/utils'

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
  try {
    const sapling = await saplingPromise

    return getXsk(sapling, seed, derivationPath)
  } catch (error) {
    reject('getExtendedSpendingKey', error)
  }
}

/**
 * Create an extended full viewing key from the given seed.
 * 
 * @param {Buffer|Int8Array|string} seed A seed from which the key will be derived
 * @param {string} derivationPath A valid BIP39 derivation path
 * @returns {Buffer} The generated extended full viewing key
 */

export async function getExtendedFullViewingKey(seed, derivationPath) {
  try {
    const sapling = await saplingPromise

    return getXfvk(sapling, seed, derivationPath)
  } catch (error) {
    reject('getExtendedFullViewingKey', error)
  }
}

/**
 * A payment address with its diversifier index.
 * 
 * @typedef {Object} SaplingPaymentAddress
 * @property {Buffer} index An 11-byte diversifier index stored as a list of bytes in a little-endian (LE) format
 * @property {Buffer} raw A 32-byte raw address value
 */

/**
 * Derive a payment address from the given extended full viewing key.
 *
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Int8Array|string|number|undefined} [index] A 11-byte diversifier index used to determine which diversifier should be used to derive the address. If not present, a new diversifier index is created with a default value of [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getPaymentAddressFromViewingKey(viewingKey, index) {
  try {
    const sapling = await saplingPromise

    return getPaymentAddressXfvk(sapling, viewingKey, index)
  } catch (error) {
    reject('getPaymentAddressFromViewingKey', error)
  }
}

/** Derive next valid payment address from the given extended full viewing key and current diversifier index.
 * 
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Int8Array|string|number} index The last used 11-byte diversifier index. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getNextPaymentAddressFromViewingKey(viewingKey, index) {
  try {
    const sapling = await saplingPromise

    return getNextPaymentAddressXfvk(sapling, viewingKey, index)
  } catch (error) {
    reject('getNextPaymentAddressFromViewingKey', error)
  }
}

/**
 * Execute action within a new sapling proving context.
 * 
 * @function
 * @template T
 * @param {function(Object): T} action An action to be executed
 * @returns {T} Result returned by the action
 */
export async function withProvingContext(action) {
  try {
    const sapling = await saplingPromise

    const context = sapling.init_proving_context()
    const result = action(context)
    sapling.drop_proving_context(context)

    return result
  } catch (error) {
    reject('withSaplingProvingContext', error)
  }
}

/**
 * Prepare a sapling output description.
 * 
 * @param {Object} context A sapling proving context
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} destination The destination address
 * @param {string|number} value The value to transfer
 * @param {Buffer|Int8Array|string} provingKey A proving key which should be used to create a proof
 * @param {Buffer|Int8Array|string|undefined} [memo] An optional message
 * @returns {Buffer} The created output description
 */
export async function prepareOutputDescription(context, viewingKey, destination, value, provingKey, memo) {
  try {
    const sapling = await saplingPromise

    return getOutputDescriptionFromXfvk(sapling, context, viewingKey, destination, value, provingKey, memo)
  } catch (error) {
    reject('prepareOutputDescription', error)
  }
}