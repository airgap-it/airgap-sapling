import { WasmSapling } from './types'
import { bufferFrom } from './utils'

export function __wasm__initParameters(
  sapling: WasmSapling,
  spendParams: Buffer | Uint8Array | string,
  outputParams: Buffer | Uint8Array | string
) {
  const spendParamsBuffer: Buffer = bufferFrom(spendParams, 'spendParams', '`Buffer`, `Uint8Array` or hex string')
  const outputParamsBuffer: Buffer = bufferFrom(outputParams, 'outputParams', '`Buffer`, `Uint8Array` or hex string')

  sapling.initParams(spendParamsBuffer, outputParamsBuffer)
}
