import { WasmSapling } from '../types'
import { bufferFrom, stringFrom } from '../utils'

export function __wasm__computeNf(
  sapling: WasmSapling,
  xfvk: Buffer | Uint8Array | string, 
  address: Buffer | Uint8Array | string, 
  value: string | number | BigInt,
  rcm: Buffer | Uint8Array | string,
  position: string | number | BigInt
): Buffer {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')
  const addressBuffer: Buffer = bufferFrom(address, 'address', '`Buffer`, `Uint8Array` or hex string')
  const valueString: string = stringFrom(value, 'value', '`number`, `BigInt` or `string`')
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Uint8Array` or hex string')
  const positionString: string = stringFrom(position, 'position', '`number`, `BigInt` or `string`')

  return Buffer.from(sapling.computeNullifier(xfvkBuffer, addressBuffer, valueString, rcmBuffer, positionString))
}