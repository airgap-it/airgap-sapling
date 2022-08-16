import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function __wasm__pak_from_xsk(sapling: WasmSapling, xsk: Buffer | Uint8Array | string): Buffer {
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.pakFromXsk(xskBuffer))
}