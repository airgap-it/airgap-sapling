import { isPaymentAddress } from './payment_address'
import { bufferFrom, ifTypeErrorElseUnknown } from './utils'

export function getOutputDescriptionFromXfvk(sapling, xfvk, to, value, memo) {
  let xfvkBuffer
  try {
    xfvkBuffer = bufferFrom(xfvk)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`viewingKey` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  let toBuffer
  try {
    const toAddress = isPaymentAddress(to) ? to.raw : to
    toBuffer = bufferFrom(toAddress)
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`destination` is of an invalid type, expected `SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  let valueNum
  if (typeof value === 'number') {
    valueNum = value
  } else if (typeof value === 'string') {
    valueNum = parseInt(value, 10)
  } else {
    return Promise.reject('`value` is of an invalid type, expected `number` or `string`')
  }

  let memoBuffer
  try {
    memoBuffer = memo !== undefined
      ? bufferFrom(memo)
      : undefined
  } catch (error) {
    const details = ifTypeErrorElseUnknown(error, '`memo` is of an invalid type, expected `Buffer`, `Int8Array` or hex string')

    return Promise.reject(details)
  }

  const rcm = sapling.generate_r()
  const ovk = sapling.get_outgoing_viewing_key_from_xfvk(xfvkBuffer, valueNum, rcm)
  
}