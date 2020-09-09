import { bufferFrom, ifTypeErrorElseUnknown } from './utils'

export function getXfvk(sapling, seed, derivationPath) {
  let seedBuffer
  try {
    seedBuffer = bufferFrom(seed)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`seed` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  return Buffer.from(sapling.get_extended_full_viewing_key(seedBuffer, derivationPath))
}