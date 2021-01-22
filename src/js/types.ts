/**
 * A payment address with its diversifier index.
 *
 * @typedef {Object} SaplingPaymentAddress
 * @property {Buffer} index An 11-byte diversifier index stored as a list of bytes in a little-endian (LE) format
 * @property {Buffer} raw A 43-byte raw address value
 */
export interface SaplingPaymentAddress {
  index: Buffer
  raw: Buffer
}

/**
 * A sapling Spend Description as described in section 4.4 of the protocol specification.
 * 
 * @typedef {Object} SaplingSpendDescription
 * @property {Buffer} cv
 * @property {Buffer} rt
 * @property {Buffer} nf
 * @property {Buffer} rk
 * @property {Buffer} proof
 * @property {Buffer} spendAuthSig
 */
export interface SaplingSpendDescription {
  cv: Buffer
  rt: Buffer
  nf: Buffer
  rk: Buffer
  proof: Buffer
  spendAuthSig: Buffer
}

/**
 * An unsigned sapling Spend Description.
 * 
 * @typedef {Object} SaplingUnsignedSpendDescription
 */
export type SaplingUnsignedSpendDescription = Omit<SaplingSpendDescription, 'spendAuthSig'>

/**
 * A sapling Output Description as described in section 4.5 of the protocol specification.
 * 
 * @typedef {Object} SaplingOutputDescription
 * @property {Buffer} cv
 * @property {Buffer} cm
 * @property {Buffer} epk
 * @property {Buffer} cenc
 * @property {Buffer} cout
 * @property {Buffer} proof
 */
export interface SaplingOutputDescription {
  cv: Buffer
  cm: Buffer
  epk: Buffer
  cenc: Buffer
  cout: Buffer
  proof: Buffer
}

/**
 * A partial sapling Output Description
 * 
 * @typedef {Object} SaplingPartialOutputDescription
 */
export type SaplingPartialOutputDescription = Omit<SaplingOutputDescription, 'epk' | 'cenc' | 'cout'>