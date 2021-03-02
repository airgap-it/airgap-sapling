import { SaplingPaymentAddress } from '../../types'
import { WasmSapling } from '../types'

import { bufferFrom, bufferFromOfLength } from '../utils'

export function getAddressFromXfvk(
  sapling: WasmSapling,
  xfvk: Buffer | Uint8Array | string,
  index: Buffer | Uint8Array | string | number | undefined
): SaplingPaymentAddress {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')
  const indexBuffer: Buffer | undefined =
    index !== undefined
      ? bufferFromOfLength(index, 11, 'index', '`Buffer`, `Uint8Array` or hex string').reverse() // LE
      : undefined

  const address = Buffer.from(
    indexBuffer !== undefined
      ? sapling.paymentAddressFromXfvk(xfvkBuffer, indexBuffer)
      : sapling.defaultPaymentAddressFromXfvk(xfvkBuffer)
  )

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function getNextAddressFromXfvk(
  sapling: WasmSapling,
  xfvk: Buffer | Uint8Array | string,
  index: Buffer | Uint8Array | string | number
): SaplingPaymentAddress {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')
  const indexBuffer: Buffer = bufferFromOfLength(index, 11, 'index', '`Buffer`, `Uint8Array` or hex string').reverse() // LE

  const address: Buffer = Buffer.from(sapling.nextPaymentAddressFromXfvk(xfvkBuffer, indexBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function isPaymentAddress(address: unknown): address is SaplingPaymentAddress {
  return address instanceof Object && (address as any).index !== undefined && (address as any).raw !== undefined
}

export function getRawAddressFromIvk(
  sapling: WasmSapling, 
  ivk: Buffer | Uint8Array | string, 
  diversifier: Buffer | Uint8Array | string
): Buffer {
  const ivkBuffer: Buffer = bufferFrom(ivk, 'ivk', '`Buffer`, `Uint8Array`, or hex string')
  const diversifierBuffer: Buffer = bufferFrom(diversifier, 'ivk', '`Buffer`, `Uint8Array`, or hex string')

  return Buffer.from(sapling.paymentAddressFromIvk(ivkBuffer, diversifierBuffer))
}

export function getDiversifierFromRawAddress(sapling: WasmSapling, address: Buffer | Uint8Array | string): Buffer {
  const addressBuffer: Buffer = bufferFrom(address, 'address', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.diversifierFromPaymentAddress(addressBuffer))
}

export function getPkdFromRawAddress(sapling: WasmSapling, address: Buffer | Uint8Array | string): Buffer {
  const addressBuffer: Buffer = bufferFrom(address, 'address', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.pkdFromPaymentAddress(addressBuffer))
}
