import { bufferFrom } from './utils'

export function getXfvk(sapling, seed, derivationPath) {
  const seedBuffer = bufferFrom(seed, 'seed', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.wasm_extended_full_viewing_key(seedBuffer, derivationPath))
}