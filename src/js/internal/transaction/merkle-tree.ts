import { WasmSapling } from '../types'
import { bigIntFrom, bufferFrom } from '../utils'

export function getMerkleHashForDepth(
  sapling: WasmSapling,
  depth: number | BigInt,
  lhs: Buffer | Int8Array | string,
  rhs: Buffer | Int8Array | string
): Buffer {
  const depthNum: BigInt = bigIntFrom(depth)
  const lhsBuffer: Buffer = bufferFrom(lhs)
  const rhsBuffer: Buffer = bufferFrom(rhs)

  return Buffer.from(sapling.merkleHash(depthNum, lhsBuffer, rhsBuffer))
}
