import { getPaymentAddressXfvk, getNextPaymentAddressXfvk } from './internal/account/payment_address'
import { getXsk } from './internal/account/spending_key'
import { getXfvk } from './internal/account/viewing_key'
import { createBindingSignatureForTx } from './internal/transaction/binding_signature'
import { getOutputDescriptionFromXfvk } from './internal/transaction/output_description'
import { getSpendDescriptionFromXsk, signSpendDescriptionWithXsk } from './internal/transaction/spend_description'
import { WasmSapling } from './internal/types'
import { rejectPromise } from './internal/utils'
import { SaplingPaymentAddress } from './types'

const saplingPromise = new Promise<WasmSapling>((resolve, reject) => {
  import('../../pkg')
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

export async function getExtendedSpendingKey(
  seed: Buffer | Int8Array | string, 
  derivationPath: string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

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

export async function getExtendedFullViewingKey(
  seed: Buffer | Int8Array | string, 
  derivationPath: string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getXfvk(sapling, seed, derivationPath)
  } catch (error) {
    return rejectPromise('getExtendedFullViewingKey', error)
  }
}

/**
 * Derive a payment address from the given extended full viewing key.
 *
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Int8Array|string|number|undefined} [index] A 11-byte diversifier index used to determine which diversifier should be used to derive the address. If not present, a new diversifier index is created with a default value of [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getPaymentAddressFromViewingKey(
  viewingKey: Buffer | Int8Array | string, 
  index?: Buffer | Int8Array | string | number | undefined
): Promise<SaplingPaymentAddress> {
  try {
    const sapling: WasmSapling = await saplingPromise

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

export async function getNextPaymentAddressFromViewingKey(
  viewingKey: Buffer | Int8Array | string,
  index: Buffer | Int8Array | string | number
): Promise<SaplingPaymentAddress> {
  try {
    const sapling: WasmSapling = await saplingPromise

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
export async function withProvingContext<T>(action: (context: number) => T): Promise<T> {
  try {
    const sapling: WasmSapling = await saplingPromise

    const context: number = sapling.wasm_init_proving_context()
    const result: T = action(context)
    sapling.wasm_drop_proving_context(context)

    return result
  } catch (error) {
    return rejectPromise('withSaplingProvingContext', error)
  }
}

/**
 * Create a random scalar
 * 
 * @returns {Buffer} The generated scalar
 */
export async function randR(): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return Buffer.from(sapling.wasm_rand_r())
  } catch (error) {
    return rejectPromise('randR', error)
  }
}

/**
 * Create a binding signature
 * 
 * Must be called after all spend and output description has been created
 * 
 * @param {number} context A pointer to sapling proving context
 * @param {string|number|BigInt} valueBalance 
 * @param {Buffer|Int8Array|string} sighash The data to be signed
 * @returns {Buffer} The created binding signature
 */
export async function createBindingSignature(
  context: number, 
  valueBalance: string | number | BigInt, 
  sighash: Buffer | Int8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return createBindingSignatureForTx(sapling, context, valueBalance, sighash)
  } catch (error) {
    return rejectPromise('createBindingSignature', error)
  }
}

/**
 * Prepare an unsigned sapling spend description
 * 
 * @param {number} context A pointer to sapling proving context
 * @param {Buffer|Int8Array|string} spendingKey An extended spending key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} address The address to which the input has been linked
 * @param {Buffer|Int8Array|string} rcm The randomness of the commitment
 * @param {Buffer|Int8Array|string} ar Re-randomization of the public key
 * @param {string|number|BigInt} value The value of the input
 * @param {Buffer|Int8Array|string} anchor The root of the merkle tree
 * @param {Buffer|Int8Array|string} merklePath The path of the commitment in the tree
 * @param {number|BigInt} position The note position
 * @param {Buffer|Int8Array|string} provingKey A proving key which should be used to create a proof
 * @param {Buffer|Int8Array|string} verifyingKey A key used to verify the proof
 * @returns {Buffer} The created unsinged spend description
 */
export async function prepareSpendDescription(
  context: number,
  spendingKey: Buffer | Int8Array | string,
  address: SaplingPaymentAddress | Buffer | Int8Array | string,
  rcm: Buffer | Int8Array | string,
  ar: Buffer | Int8Array | string,
  value: string | number | BigInt,
  anchor: Buffer | Int8Array | string,
  merklePath: Buffer | Int8Array | string,
  position: number | BigInt,
  provingKey: Buffer | Int8Array | string,
  verifyingKey: Buffer | Int8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getSpendDescriptionFromXsk(sapling, context, spendingKey, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey)
  } catch (error) {
    return rejectPromise('prepareSpendDescription', error)
  }
}

/**
 * Sign an unsigned sapling spend description
 * 
 * @param {Buffer|Int8Array|string} spendDescription An unsigned spend description
 * @param {Buffer|Int8Array|string} spendingKey An extended spending key
 * @param {Buffer|Int8Array|string} ar Re-randomization of the public key
 * @param {Buffer|Int8Array|string} sighash The data to be signed
 * @return {Buffer} The signed spend description
 */
export async function signSpendDescription(
  spendDescription: Buffer | Int8Array | string,
  spendingKey: Buffer | Int8Array | string,
  ar: Buffer | Int8Array | string,
  sighash: Buffer | Int8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return signSpendDescriptionWithXsk(sapling, spendDescription, spendingKey, ar, sighash)
  } catch (error) {
    return rejectPromise('signSpendDescription', error)
  }
}

/**
 * Prepare a sapling output description.
 * 
 * @param {number} context A pointer to sapling proving context
 * @param {Buffer|Int8Array|string} viewingKey An extended full viewing key
 * @param {SaplingPaymentAddress|Buffer|Int8Array|string} destination The destination address
 * @param {Buffer|Int8Array|string} rcm The randomness of the commitment
 * @param {string|number|BigInt} value The value to transfer
 * @param {Buffer|Int8Array|string} provingKey A proving key which should be used to create a proof
 * @param {Buffer|Int8Array|string|undefined} [memo] An optional message
 * @returns {Buffer} The created output description
 */
export async function prepareOutputDescription(
  context: number,
  viewingKey: Buffer | Int8Array | string, 
  destination: SaplingPaymentAddress | Buffer | Int8Array | string, 
  rcm: Buffer | Int8Array | string,
  value: string | number | BigInt,
  provingKey: Buffer | Int8Array | string,
  memo?: Buffer | Int8Array | string | undefined
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getOutputDescriptionFromXfvk(sapling, context, viewingKey, destination, rcm, value, provingKey, memo)
  } catch (error) {
    return rejectPromise('prepareOutputDescription', error)
  }
}