
import 'regenerator-runtime/runtime'

import { getOutputDescriptionFromXfvk } from './internal/output_description'
import { getPaymentAddressXfvk, getNextPaymentAddressXfvk } from './internal/payment_address'
import { getXsk } from './internal/spending_key'
import { getXfvk } from './internal/viewing_key'
import { rejectWithError } from './internal/utils'
import { getSpendDescriptionFromXfvk } from './internal/spend_description'

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
    return rejectWithError('getExtendedSpendingKey', error)
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
    return rejectWithError('getExtendedFullViewingKey', error)
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
    return rejectWithError('getPaymentAddressFromViewingKey', error)
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
    return rejectWithError('getNextPaymentAddressFromViewingKey', error)
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
    return rejectWithError('withSaplingProvingContext', error)
  }
}

/**
 * Prepare an unsigned sapling spend description
 * 
 * @param {Object} context 
 * @param {Buffer|Int8Array|string} spendingKey 
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} address 
 * @param {Buffer|Int8Array|string} rcm 
 * @param {Buffer|Int8Array|string} ar 
 * @param {string|number} value 
 * @param {Buffer|Int8Array|string} anchor 
 * @param {Buffer|Int8Array|string} merklePath 
 * @param {number} position 
 * @param {Buffer|Int8Array|string} provingKey 
 * @param {Buffer|Int8Array|string} verifyingKey 
 */
export async function prepareSpendDescription(context, spendingKey, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey) {
  try {
    const sapling = await saplingPromise

    return getSpendDescriptionFromXfvk(sapling, context, spendingKey, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey)
  } catch (error) {
    return rejectWithError('prepareSpendDescription', error)
  }
}

/**
 * Prepare a sapling output description.
 * 
 * @param {Object} context A sapling proving context
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} destination The destination address
 * @param {Buffer|Int8Array|string} rcm The chosen random commitment trapdoor
 * @param {string|number} value The value to transfer
 * @param {Buffer|Int8Array|string} provingKey A proving key which should be used to create a proof
 * @param {Buffer|Int8Array|string|undefined} [memo] An optional message
 * @returns {Buffer} The created output description
 */
export async function prepareOutputDescription(context, viewingKey, destination, rcm, value, provingKey, memo) {
  try {
    const sapling = await saplingPromise

    return getOutputDescriptionFromXfvk(sapling, context, viewingKey, destination, rcm, value, provingKey, memo)
  } catch (error) {
    return rejectWithError('prepareOutputDescription', error)
  }
}