import { WasmSapling } from '../types'
import { bufferFrom } from '../utils'

export function __wasm__xfvk(sapling: WasmSapling, seed: Buffer | Uint8Array | string, derivationPath: string): Buffer {
  const seedBuffer: Buffer = bufferFrom(seed, 'seed', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.xfvk(seedBuffer, derivationPath))
}

export function __wasm__xfvkFromXsk(sapling: WasmSapling, xsk: Buffer | Uint8Array | string): Buffer {
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.xfvkFromXsk(xskBuffer))
}

export function __wasm__ovk(sapling: WasmSapling, xfvk: Buffer | Uint8Array | string): Buffer {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.ovkFromXfvk(xfvkBuffer))
}

export function __wasm__ivk(sapling: WasmSapling, xfvk: Buffer | Uint8Array | string): Buffer {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.xfvkToIvk(xfvkBuffer))
}
