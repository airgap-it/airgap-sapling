import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function __wasm__keyAgreement(sapling: WasmSapling, p: Buffer | Uint8Array | string, sk: Buffer | Uint8Array | string): Buffer {
  const pBuffer = bufferFrom(p, 'p', '`Buffer`, `Uint8Array` or hex string')
  const skBuffer = bufferFrom(sk, 'sk', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.keyAgreement(pBuffer, skBuffer))
}