import { isPaymentAddress } from './payment_address'
import { bufferFrom, rejectInvalidTypeOrUnknown } from './utils'

export function getOutputDescriptionFromXfvk(sapling, context, xfvk, to, rcm, value, provingKey, memo) {
  let xfvkBuffer
  try {
    xfvkBuffer = bufferFrom(xfvk)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('viewingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let toBuffer
  try {
    toBuffer = bufferFrom(isPaymentAddress(to) ? to.raw : to)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('destination', '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string', error)
  }

  let rcmBuffer
  try {
    rcmBuffer = bufferFrom(rcm)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('rcm', '`Buffer`, `Int8Array` or hex string', error)
  }

  let valueNum
  if (typeof value === 'number') {
    valueNum = value
  } else if (typeof value === 'string') {
    valueNum = parseInt(value, 10)
  } else {
    return rejectInvalidTypeOrUnknown('value', '`number` or `string`')
  }

  let provingKeyBuffer
  try {
    provingKeyBuffer = bufferFrom(provingKey)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('provingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let memoBuffer
  try {
    memoBuffer = memo !== undefined
      ? bufferFrom(memo)
      : undefined
  } catch (error) {
    return rejectInvalidTypeOrUnknown('memo', '`Buffer`, `Int8Array` or hex string', error)
  }

  return Buffer.from(memoBuffer !== undefined 
      ? sapling.create_output_description_from_xfvk_with_memo(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer, memoBuffer)
      : sapling.create_output_description_from_xfvk(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer)
    )
}