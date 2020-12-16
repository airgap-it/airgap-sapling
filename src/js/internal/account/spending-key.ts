import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function getXsk(sapling: WasmSapling, seed: Buffer | Int8Array | string, derivationPath: string): Buffer {
  const seedBuffer: Buffer = bufferFrom(seed, 'seed', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.xsk(seedBuffer, derivationPath))
}
