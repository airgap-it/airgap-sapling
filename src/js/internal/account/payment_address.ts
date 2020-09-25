import { SaplingPaymentAddress } from '../../types'
import { WasmSapling } from '../types'

import { bufferFrom, bufferFromOfLength } from '../utils'

export function getPaymentAddressXfvk(
  sapling: WasmSapling, 
  xfvk: Buffer | Int8Array | string, 
  index: Buffer | Int8Array | string | number | undefined
): SaplingPaymentAddress {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const indexBuffer: Buffer | undefined = index !== undefined 
    ? bufferFromOfLength(index, 11, 'index', '`Buffer`, `Int8Array` or hex string').reverse() // LE
    : undefined

  const address = Buffer.from(indexBuffer !== undefined 
    ? sapling.wasm_payment_address_from_xfvk(xfvkBuffer, indexBuffer)
    : sapling.wasm_default_payment_address_from_xfvk(xfvkBuffer)
  )

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function getNextPaymentAddressXfvk(
  sapling: WasmSapling, 
  xfvk: Buffer | Int8Array | string, 
  index: Buffer | Int8Array | string | number
): SaplingPaymentAddress {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const indexBuffer: Buffer = bufferFromOfLength(index, 11, 'index', '`Buffer`, `Int8Array` or hex string').reverse() // LE

  const address: Buffer = Buffer.from(sapling.wasm_next_payment_address_from_xfvk(xfvkBuffer, indexBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function isPaymentAddress(address: unknown): address is SaplingPaymentAddress {
  return address instanceof Object && (address as any).index !== undefined && (address as any).raw !== undefined
}