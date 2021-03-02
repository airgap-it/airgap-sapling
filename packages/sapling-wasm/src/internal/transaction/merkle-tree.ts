import { WasmSapling } from '../types'
import { bigIntFrom, bufferFrom } from '../utils'

export function __wasm__merkleHashForDepth(
  sapling: WasmSapling,
  depth: number | BigInt,
  lhs: Buffer | Uint8Array | string,
  rhs: Buffer | Uint8Array | string
): Buffer {
  const depthNum: BigInt = bigIntFrom(depth)
  const lhsBuffer: Buffer = bufferFrom(lhs)
  const rhsBuffer: Buffer = bufferFrom(rhs)

  return Buffer.from(sapling.merkleHash(depthNum, lhsBuffer, rhsBuffer))
}
