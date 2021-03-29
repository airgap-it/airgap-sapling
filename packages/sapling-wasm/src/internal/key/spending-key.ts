import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function __wasm__xsk(sapling: WasmSapling, seed: Buffer | Uint8Array | string, derivationPath: string): Buffer {
  const seedBuffer: Buffer = bufferFrom(seed, 'seed', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.xsk(seedBuffer, derivationPath))
}
