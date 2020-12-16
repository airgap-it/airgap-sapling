import { SaplingPaymentAddress } from '../../types'

import { WasmSapling } from '../types'
import { isPaymentAddress } from '../account/payment-address'
import { bigIntFrom, bufferFrom } from '../utils'

export function getOutputDescriptionFromXfvk(
  sapling: WasmSapling,
  context: number,
  xfvk: Buffer | Int8Array | string,
  to: SaplingPaymentAddress | Buffer | Int8Array | string,
  rcm: Buffer | Int8Array | string,
  value: string | number | BigInt,
  memo?: Buffer | Int8Array | string
): Buffer {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const toBuffer: Buffer = bufferFrom(
    isPaymentAddress(to) ? to.raw : to,
    'destination',
    '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string'
  )
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Int8Array` or hex string')
  const valueNum: BigInt = bigIntFrom(value, 'value', '`number`, `BigInt` or `string`')
  const memoBuffer: Buffer | undefined = memo !== undefined ? bufferFrom(memo, 'memo', '`Buffer`, `Int8Array` or hex string') : undefined

  return Buffer.from(
    memoBuffer !== undefined
      ? sapling.outputDescriptionFromXfvkWithMemo(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, memoBuffer)
      : sapling.outputDescriptionFromXfvk(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum)
  )
}
