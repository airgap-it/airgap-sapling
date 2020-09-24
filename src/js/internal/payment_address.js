import { bufferFrom, bufferFromOfLength } from './utils'

export async function getPaymentAddressXfvk(sapling, xfvk, index) {
  const xfvkBuffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const indexBuffer = index !== undefined 
    ? bufferFromOfLength(index, 11, 'index', '`Buffer`, `Int8Array` or hex string').reverse() // LE
    : undefined

  const address = Buffer.from(indexBuffer !== undefined 
    ? sapling.get_payment_address_from_viewing_key(xfvkBuffer, indexBuffer)
    : sapling.get_default_payment_address_from_viewing_key(xfvkBuffer)
  )

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export async function getNextPaymentAddressXfvk(sapling, xfvk, index) {
  const xfvkBuffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const indexBuffer = bufferFromOfLength(index, 11, 'index', '`Buffer`, `Int8Array` or hex string').reverse() // LE

  const address = Buffer.from(sapling.get_next_payment_address_from_viewing_key(xfvkBuffer, indexBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function isPaymentAddress(address) {
  return address instanceof Object && address.index !== undefined && address.raw !== undefined
}