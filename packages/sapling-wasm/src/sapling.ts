import {
  getAddressFromXfvk,
  getDiversifierFromRawAddress,
  getNextAddressFromXfvk,
  getPkdFromRawAddress,
  getRawAddressFromIvk
} from './internal/address/payment-address'
import { __wasm__pak_from_xsk } from './internal/key/authorizing-key'
import { __wasm__keyAgreement } from './internal/key/key-agreement'
import { __wasm__xsk } from './internal/key/spending-key'
import { __wasm__ivk, __wasm__ovk, __wasm__xfvk, __wasm__xfvkFromXsk } from './internal/key/viewing-key'
import { __wasm__initParameters } from './internal/parameters'
import { __wasm__bindingSignature } from './internal/transaction/binding-signature'
import { __wasm__verifyCmu } from './internal/transaction/commitment'
import { __wasm__merkleHashForDepth } from './internal/transaction/merkle-tree'
import { __wasm__computeNf } from './internal/transaction/nullifier'
import {
  __wasm__deriveEpkFromEsk,
  __wasm__outputDescriptionFromXfvk,
  __wasm__partialOutputDescription
} from './internal/transaction/output-description'
import { __wasm__spendDescriptionFromXsk, __wasm__signSpendDescriptionWithXsk, __wasm__spendDescriptionFromPak } from './internal/transaction/spend-description'
import { WasmSapling } from './internal/types'
import { rejectPromise } from './internal/utils'
import {
  SaplingOutputDescription,
  SaplingPartialOutputDescription,
  SaplingPaymentAddress,
  SaplingSpendDescription,
  SaplingUnsignedSpendDescription
} from './types'

let isInitialized: boolean = false
const saplingPromise: Promise<WasmSapling> = import('../pkg')
  .catch((error) => {
    console.error(error)
    throw new Error(`Could not load sapling-wasm: ${error}`)
  })

/**
 * Initialize the library with specified sapling parameters.
 *
 * @param {Buffer|Uint8Array|string} spendParams The sapling spending parameters.
 * @param {Buffer|Uint8Array|string} outputParams The sapling output parameters.
 */
export async function initParameters(spendParams: Buffer | Uint8Array | string, outputParams: Buffer | Uint8Array | string): Promise<void> {
  try {
    if (!isInitialized) {
      const sapling: WasmSapling = await saplingPromise

      __wasm__initParameters(sapling, spendParams, outputParams)
      isInitialized = true
    }
  } catch (error) {
    return rejectPromise('init', error)
  }
}

/**
 * Create an extended spending key from the given seed.
 *
 * @param {Buffer|Uint8Array|string} seed A seed from which the key will be derived
 * @param {string} derivationPath A valid BIP39 derivation path
 * @returns {Buffer} The generated extended spending key
 */

export async function getExtendedSpendingKey(seed: Buffer | Uint8Array | string, derivationPath: string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__xsk(sapling, seed, derivationPath)
  } catch (error) {
    return rejectPromise('getExtendedSpendingKey', error)
  }
}

/**
 * Create an extended full viewing key from the given seed.
 *
 * @param {Buffer|Uint8Array|string} seed A seed from which the key will be derived
 * @param {string} derivationPath A valid BIP39 derivation path
 * @returns {Buffer} The generated extended full viewing key
 */

export async function getExtendedFullViewingKey(seed: Buffer | Uint8Array | string, derivationPath: string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__xfvk(sapling, seed, derivationPath)
  } catch (error) {
    return rejectPromise('getExtendedFullViewingKey', error)
  }
}

/**
 * Create a proof authorizing key from the given extended spending key.
 * 
 * @param {Buffer|Uint8Array|string} spendingKey An extended spending key
 * @returns {Buffer} The generated proof authorizing key
 */
export async function getProofAuthorizingKey(spendingKey: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__pak_from_xsk(sapling, spendingKey)
  } catch (error) {
    return rejectPromise('getProofAuthorizingKey', error)
  }
}

/**
 * Create an extended full viewing key from the given extended spending key.
 * 
 * @param {Buffer|Uint8Array|string} spendingKey An extended spending key
 * @returns {Buffer} The generated extended full viewing key
 */
export async function getExtendedFullViewingKeyFromSpendingKey(spendingKey: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__xfvkFromXsk(sapling, spendingKey)
  } catch (error) {
    return rejectPromise('getExtendedFullViewingKeyFromSpendingKey', error)
  }
}

/**
 * Derive an outgoing viewing key from extended full viewing key.
 * 
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key
 * @returns {Buffer} The derived outgoing viewing key
 */
export async function getOutgoingViewingKey(viewingKey: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__ovk(sapling, viewingKey)
  } catch (error) {
    return rejectPromise('getOutgoingViewingKey', error)
  }
}

/**
 * Derive an incoming viewing key from extended full viewing key.
 * 
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key
 * @returns {Buffer} The derived incoming viewing key
 */
export async function getIncomingViewingKey(viewingKey: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__ivk(sapling, viewingKey)
  } catch (error) {
    return rejectPromise('getIncomingViewingKey', error)
  }
}

/**
 * Derive a payment address from the given extended full viewing key.
 *
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Uint8Array|string|number|undefined} [index] A 11-byte diversifier index used to determine which diversifier should be used to derive the address. If not present, a new diversifier index is created with a default value of [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getPaymentAddressFromViewingKey(
  viewingKey: Buffer | Uint8Array | string,
  index?: Buffer | Uint8Array | string | number | undefined
): Promise<SaplingPaymentAddress> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getAddressFromXfvk(sapling, viewingKey, index)
  } catch (error) {
    return rejectPromise('getPaymentAddressFromViewingKey', error)
  }
}

/** 
 * Derive next valid payment address from the given extended full viewing key and current diversifier index.
 *
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key
 * @param {Buffer|Uint8Array|string|number} index The last used 11-byte diversifier index. If provided as bytes, it is expected to be in the little-endian (LE) format.
 * @returns {SaplingPaymentAddress} The derived payment address
 */

export async function getNextPaymentAddressFromViewingKey(
  viewingKey: Buffer | Uint8Array | string,
  index: Buffer | Uint8Array | string | number
): Promise<SaplingPaymentAddress> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getNextAddressFromXfvk(sapling, viewingKey, index)
  } catch (error) {
    return rejectPromise('getNextPaymentAddressFromViewingKey', error)
  }
}

/**
 * Create a raw payment address value from the specified incoming viewing key and diversifier.
 * 
 * @param {Buffer|Uint8Array|string} incomingViewingKey An incoming viewing key from which an address should be created
 * @param {Buffer|Uint8Array|string} diversifier The address diversifier
 * @returns {Buffer} The raw payment address
 */
export async function getRawPaymentAddressFromIncomingViewingKey(
  incomingViewingKey: Buffer | Uint8Array | string,
  diversifier: Buffer | Uint8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getRawAddressFromIvk(sapling, incomingViewingKey, diversifier)
  } catch (error) {
    return rejectPromise('getRawPaymentAddressFromIncomingViewingKey', error)
  }
}

/**
 * Get a diversifier from a raw payment address.
 * 
 * @param {Buffer | Uint8Array | string} address A raw payment address
 * @returns {Buffer} The diversifier
 */
export async function getDiversifiedFromRawPaymentAddress(address: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getDiversifierFromRawAddress(sapling, address)
  } catch (error) {
    return rejectPromise('getDiversifierFromRawPaymentAddress', error)
  }
}

/**
 * Get a pkd from a raw payment address.
 * 
 * @param {Buffer | Uint8Array | string} address A raw payment address
 * @returns {Buffer} The diversifier
 */
export async function getPkdFromRawPaymentAddress(address: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return getPkdFromRawAddress(sapling, address)
  } catch (error) {
    return rejectPromise('getPkdFromRawPaymentAddress', error)
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
export async function withProvingContext<T>(action: (context: number) => Promise<T>): Promise<T> {
  try {
    const sapling: WasmSapling = await saplingPromise

    const context: number = sapling.initProvingContext()
    const result: T = await action(context)
    sapling.dropProvingContext(context)

    return result
  } catch (error) {
    return rejectPromise('withSaplingProvingContext', error)
  }
}

/**
 * Create a random scalar.
 *
 * @returns {Buffer} The generated scalar
 */
export async function randR(): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return Buffer.from(sapling.randR())
  } catch (error) {
    return rejectPromise('randR', error)
  }
}

/**
 * Create a binding signature.
 *
 * Must be called after all spend and output description has been created
 *
 * @param {number} context A pointer to sapling proving context
 * @param {string|number|BigInt} valueBalance
 * @param {Buffer|Uint8Array|string} sighash The data to be signed
 * @returns {Buffer} The created binding signature
 */
export async function createBindingSignature(
  context: number,
  valueBalance: string | number | BigInt,
  sighash: Buffer | Uint8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__bindingSignature(sapling, context, valueBalance, sighash)
  } catch (error) {
    return rejectPromise('createBindingSignature', error)
  }
}

/**
 * Prepare an unsigned sapling spend description using an extended spending key.
 *
 * @param {number} context A pointer to sapling proving context
 * @param {Buffer|Uint8Array|string} spendingKey An extended spending key
 * @param {SaplingPaymentAddress|Buffer|Uint8Array|string} address The address to which the input has been linked
 * @param {Buffer|Uint8Array|string} rcm The randomness of the commitment
 * @param {Buffer|Uint8Array|string} ar Re-randomization of the public key
 * @param {string|number|BigInt} value The value of the input
 * @param {Buffer|Uint8Array|string} anchor The root of the merkle tree
 * @param {Buffer|Uint8Array|string} merklePath The path of the commitment in the tree
 * @returns {SaplingUnsignedSpendDescription} The created unsinged spend description
 */
export async function prepareSpendDescriptionWithSpendingKey(
  context: number,
  spendingKey: Buffer | Uint8Array | string,
  address: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  ar: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  anchor: Buffer | Uint8Array | string,
  merklePath: Buffer | Uint8Array | string
): Promise<SaplingUnsignedSpendDescription> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__spendDescriptionFromXsk(sapling, context, spendingKey, address, rcm, ar, value, anchor, merklePath)
  } catch (error) {
    return rejectPromise('prepareSpendDescription', error)
  }
}

/**
 * Prepare an unsigned sapling spend description using a proof authorizing key.
 *
 * @param {number} context A pointer to sapling proving context
 * @param {Buffer|Uint8Array|string} authorizingKey A proof authorizing key
 * @param {SaplingPaymentAddress|Buffer|Uint8Array|string} address The address to which the input has been linked
 * @param {Buffer|Uint8Array|string} rcm The randomness of the commitment
 * @param {Buffer|Uint8Array|string} ar Re-randomization of the public key
 * @param {string|number|BigInt} value The value of the input
 * @param {Buffer|Uint8Array|string} anchor The root of the merkle tree
 * @param {Buffer|Uint8Array|string} merklePath The path of the commitment in the tree
 * @returns {SaplingUnsignedSpendDescription} The created unsinged spend description
 */
 export async function prepareSpendDescriptionWithAuthorizingKey(
  context: number,
  authorizingKey: Buffer | Uint8Array | string,
  address: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  ar: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  anchor: Buffer | Uint8Array | string,
  merklePath: Buffer | Uint8Array | string
): Promise<SaplingUnsignedSpendDescription> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__spendDescriptionFromPak(sapling, context, authorizingKey, address, rcm, ar, value, anchor, merklePath)
  } catch (error) {
    return rejectPromise('prepareSpendDescription', error)
  }
}

/**
 * Sign an unsigned sapling spend description.
 *
 * @param {SaplingUnsignedSpendDescription} spendDescription An unsigned spend description
 * @param {Buffer|Uint8Array|string} spendingKey An extended spending key
 * @param {Buffer|Uint8Array|string} ar Re-randomization of the public key
 * @param {Buffer|Uint8Array|string} sighash The data to be signed
 * @return {SaplingSpendDescription} The signed spend description
 */
export async function signSpendDescription(
  spendDescription: SaplingUnsignedSpendDescription,
  spendingKey: Buffer | Uint8Array | string,
  ar: Buffer | Uint8Array | string,
  sighash: Buffer | Uint8Array | string
): Promise<SaplingSpendDescription> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__signSpendDescriptionWithXsk(sapling, spendDescription, spendingKey, ar, sighash)
  } catch (error) {
    return rejectPromise('signSpendDescription', error)
  }
}

/**
 * Prepare a sapling output description.
 *
 * @param {number} context A pointer to sapling proving context
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key
 * @param {SaplingPaymentAddress|Buffer|Uint8Array|string} destination The destination address
 * @param {Buffer|Uint8Array|string} rcm The randomness of the commitment
 * @param {string|number|BigInt} value The value to transfer
 * @param {Buffer|Uint8Array|string|undefined} [memo] An optional message
 * @returns {SaplingOutputDescription} The created output description
 */
export async function prepareOutputDescription(
  context: number,
  viewingKey: Buffer | Uint8Array | string,
  destination: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  memo?: Buffer | Uint8Array | string | undefined
): Promise<SaplingOutputDescription> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__outputDescriptionFromXfvk(sapling, context, viewingKey, destination, rcm, value, memo)
  } catch (error) {
    return rejectPromise('prepareOutputDescription', error)
  }
}

/**
 * Prepare a partial sapling output description.
 *
 * @param {number} context A pointer to sapling proving context
 * @param {SaplingPaymentAddress|Buffer|Uint8Array|string} destination The destination address
 * @param {Buffer|Uint8Array|string} rcm The randomness of the commitment
 * @param {Buffer|Uint8Array|string} ephemeralKey An ephemeral private key that will be used to create an output proof
 * @param {string|number|BigInt} value The value to transfer
 * @returns {SaplingPartialOutputDescription} The created partial output description
 */
export async function preparePartialOutputDescription(
  context: number,
  destination: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  ephemeralKey: Buffer | Uint8Array | string,
  value: string | number | BigInt,
): Promise<SaplingPartialOutputDescription> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__partialOutputDescription(sapling, context, destination, rcm, ephemeralKey, value)
  } catch (error) {
    return rejectPromise('preparePartialOutputDescription', error)
  }
}

/**
 * Derive an ephemeral public key from the specified address diversifier and ephemeral private key.
 * 
 * @param {Buffer|Uint8Array|string} diversifier A payment address diversifier
 * @param {Buffer|Uint8Array|string} privateKey An ephemeral private key
 * @returns {Buffer} The derived ephemeral public key
 */
export async function deriveEphemeralPublicKey(
  diversifier: Buffer | Uint8Array | string, 
  privateKey: Buffer | Uint8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__deriveEpkFromEsk(sapling, diversifier, privateKey)
  } catch (error) {
    return rejectPromise('deriveEphemeralPublicKey', error)
  }
}

/**
 * Verify if the specified commitment is valid in the context of provided address, transfer value and rcm.
 * 
 * @param {Buffer|Uint8Array|string} commitment The commitment to verify
 * @param {Buffer|Uint8Array|string} address The expected raw address
 * @param {string|number|BigInt} value The expected transfer value
 * @param {Buffer|Uint8Array|string} rcm The expected randomness of the commitment
 * @returns {boolean} `true` if commitment is valid, `false` otherwise
 */
export async function verifyCommitment(
  commitment: Buffer | Uint8Array | string,
  address: Buffer | Uint8Array | string, 
  value: string | number | BigInt, 
  rcm: Buffer | Uint8Array | string
): Promise<boolean> {
  try {
    const sapling: WasmSapling = await saplingPromise
    
    return __wasm__verifyCmu(sapling, commitment, address, value, rcm)
  } catch (error) {
    return rejectPromise('verifyCommitment', error)
  }
}

/**
 * Compute a nullifier for the commitment.
 * 
 * @param {Buffer|Uint8Array|string} viewingKey An extended full viewing key that is the owner of the commitment
 * @param {Buffer|Uint8Array|string} address The destination address of the commitment
 * @param {string|number|BigInt} value The transfer value of the commitment
 * @param {Buffer|Uint8Array|string} rcm The randomness of the commitment
 * @param {string|number|BigInt} position The position of the commitment
 * @returns {Buffer} The computed nullifier
 */
export async function computeNullifier(
  viewingKey: Buffer | Uint8Array | string,
  address: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  rcm: Buffer | Uint8Array | string,
  position: string | number | BigInt
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__computeNf(sapling, viewingKey, address, value, rcm, position)
  } catch (error) {
    return rejectPromise('computeNullifier', error)
  }
}

/**
 * Compute a Merkle Tree parent hash for the specified depth and its children.
 *
 * @param {string|number} depth The depth of the tree, cannot be larger than 62
 * @param {Buffer|Uint8Array|string} lhs A 32-byte child hash
 * @param {Buffer|Uint8Array|string} rhs A 32-byte child hash
 * @returns {Buffer} The computed parent hash
 */
export async function merkleHash(
  depth: string | number,
  lhs: Buffer | Uint8Array | string,
  rhs: Buffer | Uint8Array | string
): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__merkleHashForDepth(sapling, depth, lhs, rhs)
  } catch (error) {
    return rejectPromise('merkleHash', error)
  }
}

/**
 * Compute a key agreement.
 * 
 * @param {Buffer|Uint8Array|string} p A 32-byte point
 * @param {Buffer|Uint8Array|string} sk A 32-byte scalar
 * @returns {Buffer} The computed key agreement
 */
export async function keyAgreement(p: Buffer | Uint8Array | string, sk: Buffer | Uint8Array | string): Promise<Buffer> {
  try {
    const sapling: WasmSapling = await saplingPromise

    return __wasm__keyAgreement(sapling, p, sk)
  } catch (error) {
    return rejectPromise('keyAgreement', error)
  }
}
