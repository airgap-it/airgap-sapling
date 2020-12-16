import { WasmSapling } from './types'
import { bufferFrom } from './utils'

export function initSaplingParameters(
  sapling: WasmSapling,
  spendParams: Buffer | Int8Array | string,
  outputParams: Buffer | Int8Array | string
) {
  const spendParamsBuffer: Buffer = bufferFrom(spendParams, 'spendParams', '`Buffer`, `Int8Array` or hex string')
  const outputParamsBuffer: Buffer = bufferFrom(outputParams, 'outputParams', '`Buffer`, `Int8Array` or hex string')

  sapling.initParams(spendParamsBuffer, outputParamsBuffer)
}
