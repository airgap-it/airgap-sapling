import { bufferFrom, rejectInvalidTypeOrUnknown } from './utils'

export function getXfvk(sapling, seed, derivationPath) {
  let seedBuffer
  try {
    seedBuffer = bufferFrom(seed)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('seed', '`Buffer`, `Int8Array` or hex string', error)
  }

  return Buffer.from(sapling.get_extended_full_viewing_key(seedBuffer, derivationPath))
}