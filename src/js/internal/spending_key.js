import { bufferFrom } from './utils'

export function getXsk(sapling, seed, derivationPath) {
  const seedBuffer = bufferFrom(seed, 'seed', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.wasm_extended_spending_key(seedBuffer, derivationPath))
}