
import 'regenerator-runtime/runtime'

import { getOutputDescriptionFromXfvk } from './internal/output_description'
import { getPaymentAddressXfvk, getNextPaymentAddressXfvk } from './internal/payment_address'
import { getSpendDescriptionFromXfvk, signSpendDescriptionWithXsk } from './internal/spend_description'
import { getXsk } from './internal/spending_key'
import { getXfvk } from './internal/viewing_key'
import { rejectPromise } from './internal/utils'

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
    return rejectPromise('getExtendedSpendingKey', error)
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
    return rejectPromise('getExtendedFullViewingKey', error)
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
    return rejectPromise('getPaymentAddressFromViewingKey', error)
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
    return rejectPromise('getNextPaymentAddressFromViewingKey', error)
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
    return rejectPromise('withSaplingProvingContext', error)
  }
}

/**
 * Prepare an unsigned sapling spend description
 * 
 * @param {Object} context A sapling proving context
 * @param {Buffer|Int8Array|string} spendingKey An extended spending key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} address The address to which the input has been linked
 * @param {Buffer|Int8Array|string} rcm The randomness of the commitment
 * @param {Buffer|Int8Array|string} ar Re-randomization of the public key
 * @param {string|number} value The value of the input
 * @param {Buffer|Int8Array|string} anchor The root of the merkle tree
 * @param {Buffer|Int8Array|string} merklePath The path of the commitment in the tree
 * @param {number} position The note position
 * @param {Buffer|Int8Array|string} provingKey A proving key which should be used to create a proof
 * @param {Buffer|Int8Array|string} verifyingKey A key used to verify the proof
 * @returns {Buffer} The created unsinged spend description
 */
export async function prepareSpendDescription(context, spendingKey, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey) {
  try {
    const sapling = await saplingPromise

    return getSpendDescriptionFromXfvk(sapling, context, spendingKey, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey)
  } catch (error) {
    return rejectPromise('prepareSpendDescription', error)
  }
}

/**
 * Sign an unsinged sapling spend description
 * 
 * @param {Buffer|Int8Array|string} spendDescription An unsigned spend description
 * @param {Buffer|Int8Array|string} spendingKey An extended spending key
 * @param {Buffer|Int8Array|string} ar Re-randomization of the public key
 * @param {Buffer|Int8Array|string} sighash The data to be signed
 * @return {Buffer} The signed spend description
 */
export async function signSpendDescription(spendDescription, spendingKey, ar, sighash) {
  try {
    const sapling = await saplingPromise

    return signSpendDescriptionWithXsk(spendDescription, spendingKey, ar, sighash)
  } catch (error) {
    return rejectPromise('signSpendDescription', error)
  }
}

/**
 * Prepare a sapling output description.
 * 
 * @param {Object} context A sapling proving context
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} destination The destination address
 * @param {Buffer|Int8Array|string} rcm The randomness of the commitment
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
    return rejectPromise('prepareOutputDescription', error)
  }
}