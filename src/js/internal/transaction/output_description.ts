import { SaplingPaymentAddress } from '../../types'

import { WasmSapling } from '../types'
import { isPaymentAddress } from '../account/payment_address'
import { bigIntFrom, bufferFrom } from '../utils'

export function getOutputDescriptionFromXfvk(
  sapling: WasmSapling, 
  context: number, 
  xfvk: Buffer | Int8Array | string, 
  to: SaplingPaymentAddress | Buffer | Int8Array | string, 
  rcm: Buffer | Int8Array | string,
  value: string | number | BigInt,
  provingKey: Buffer | Int8Array | string,
  memo?: Buffer | Int8Array | string,
): Buffer {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Int8Array` or hex string')
  const toBuffer: Buffer = bufferFrom(isPaymentAddress(to) ? to.raw : to, 'destination', '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string')
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Int8Array` or hex string')
  const valueNum: BigInt = bigIntFrom(value, 'value', '`number`, `BigInt` or `string`')
  const provingKeyBuffer: Buffer = bufferFrom(provingKey, 'provingKey', '`Buffer`, `Int8Array` or hex string')
  const memoBuffer: Buffer | undefined = memo !== undefined ? bufferFrom(memo, 'memo', '`Buffer`, `Int8Array` or hex string') : undefined

  return Buffer.from(memoBuffer !== undefined 
      ? sapling.wasm_output_description_from_xfvk_with_memo(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer, memoBuffer)
      : sapling.wasm_output_description_from_xfvk(context, xfvkBuffer, toBuffer, rcmBuffer, valueNum, provingKeyBuffer)
    )
}