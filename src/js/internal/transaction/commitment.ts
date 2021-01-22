import { WasmSapling } from '../types'
import { bigIntFrom, bufferFrom } from '../utils'

export function __wasm__verifyCmu(
  sapling: WasmSapling, 
  cmu: Buffer | Uint8Array | string,
  address: Buffer | Uint8Array | string,
  value: string | number | BigInt, 
  rcm: Buffer | Uint8Array | string
): boolean {
  const cmuBuffer: Buffer = bufferFrom(cmu, 'commitment', '`Buffer`, `Uint8Array` or hex string')
  const addresBuffer: Buffer = bufferFrom(address, 'address', '`Buffer`, `Uint8Array` or hex string')
  const valueBigInt: BigInt = bigIntFrom(value, 'value', '`number`, `BigInt` or `string`')
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Uint8Array` or hex string')

  const computedCmu: Buffer = Buffer.from(sapling.computeCommitment(addresBuffer, valueBigInt, rcmBuffer))

  return cmuBuffer.equals(computedCmu)
}