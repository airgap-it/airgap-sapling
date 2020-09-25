/**
 * A payment address with its diversifier index.
 * 
 * @typedef {Object} SaplingPaymentAddress
 * @property {Buffer} index An 11-byte diversifier index stored as a list of bytes in a little-endian (LE) format
 * @property {Buffer} raw A 32-byte raw address value
 */

export interface SaplingPaymentAddress {
  index: Buffer
  raw: Buffer
}