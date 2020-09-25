import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function getXfvk(sapling: WasmSapling, seed: Buffer | Int8Array | string, derivationPath: string): Buffer {
  const seedBuffer: Buffer = bufferFrom(seed, 'seed', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.wasm_extended_full_viewing_key(seedBuffer, derivationPath))
}