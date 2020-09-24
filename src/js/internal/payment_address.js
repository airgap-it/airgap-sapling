import { bufferFrom, rejectInvalidTypeOrUnknown } from './utils'

export async function getPaymentAddressXfvk(sapling, xfvk, index) {
  let xfvkBuffer
  try {
    xfvkBuffer = bufferFrom(xfvk)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('viewingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let indexBuffer
  try {
    indexBuffer = index !== undefined
      ? bufferFrom(index, 11).reverse() // LE
      : undefined
  } catch (error) {
    return rejectInvalidTypeOrUnknown( 'index', '`Buffer`, `Int8Array` or hex string', error)
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
    return rejectInvalidTypeOrUnknown('viewingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let indexBuffer
  try {
    indexBuffer = bufferFrom(index, 11).reverse() // LE
  } catch (error) {
    return rejectInvalidTypeOrUnknown('index', '`Buffer`, `Int8Array` or hex string', error)
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