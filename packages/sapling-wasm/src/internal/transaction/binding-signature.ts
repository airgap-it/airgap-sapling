import { WasmSapling } from '../types'
import { bufferFrom, stringFrom } from '../utils'

export function __wasm__bindingSignature(
  sapling: WasmSapling,
  context: number,
  valueBalance: string | number | BigInt,
  sighash: Buffer | Uint8Array | string
): Buffer {
  const valueBalanceString: string = stringFrom(valueBalance, 'valueBalance', '`number`, `BigInt` or `string`')
  const sighashBuffer: Buffer = bufferFrom(sighash, 'sighash', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.bindingSignature(context, valueBalanceString, sighashBuffer))
}
