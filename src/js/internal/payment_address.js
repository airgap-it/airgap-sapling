import { bufferFrom, ifTypeErrorElseUnknown } from './utils'

export async function getPaymentAddressXfvk(sapling, xfvk, index) {
  let xfvkBuffer
  try {
    xfvkBuffer = bufferFrom(xfvk)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`viewingKey` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  let indexBuffer
  try {
    indexBuffer = index !== undefined
      ? bufferFrom(index, 11).reverse() // LE
      : undefined
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`index` is of an invalid type, expected `Buffer`, `Int8Array`, hex string or number')

    return Promise.reject(details)
  }

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
  let xfvkBuffer
  try {
    xfvkBuffer = bufferFrom(xfvk)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`viewingKey` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  let indexBuffer
  try {
    indexBuffer = bufferFrom(index, 11).reverse() // LE
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`index` is of an invalid type, expected `Buffer`, `Int8Array`, hex string or number')

    return Promise.reject(details)
  }

  const address = Buffer.from(sapling.get_next_payment_address_from_viewing_key(xfvkBuffer, indexBuffer))

  return {
    index: address.slice(0, 11),
    raw: address.slice(11)
  }
}

export function isPaymentAddress(address) {
  return address instanceof Object && address.index !== undefined && address.raw !== undefined
}