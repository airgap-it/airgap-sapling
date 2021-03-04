import { WasmSapling } from '../types'
import { bufferFrom, numberFrom } from '../utils'

export function __wasm__merkleHashForDepth(
  sapling: WasmSapling,
  depth: string | number,
  lhs: Buffer | Uint8Array | string,
  rhs: Buffer | Uint8Array | string
): Buffer {
  const depthNum: number = numberFrom(depth)
  const lhsBuffer: Buffer = bufferFrom(lhs)
  const rhsBuffer: Buffer = bufferFrom(rhs)

  return Buffer.from(sapling.merkleHash(depthNum, lhsBuffer, rhsBuffer))
}
