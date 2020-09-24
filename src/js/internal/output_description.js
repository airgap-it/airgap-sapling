import { isPaymentAddress } from './payment_address'
import { bufferFrom, numberFrom } from './utils'

export function getOutputDescriptionFromXfvk(sapling, context, xfvk, to, rcm, value, provingKey, memo) {
  const xfvkBuffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const toBuffer = bufferFrom(isPaymentAddress(to) ? to.raw : to, 'destination', '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string')
  const rcmBuffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Int8Array` or hex string')
  const valueNum = numberFrom(value, 'value', '`number` or `string`')
  const provingKeyBuffer = bufferFrom(provingKey, 'provingKey', '`Buffer`, `Int8Array` or hex string')
  const memoBuffer = bufferFrom(memo, 'memo', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(memoBuffer !== undefined 
      ? sapling.create_output_description_from_xfvk_with_memo(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer, memoBuffer)
      : sapling.create_output_description_from_xfvk(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer)
    )
}