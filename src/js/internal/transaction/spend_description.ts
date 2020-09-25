import { SaplingPaymentAddress } from '../../types'

import { WasmSapling } from '../types'
import { isPaymentAddress } from '../account/payment_address'
import { bigIntFrom, bufferFrom } from '../utils'

export function getSpendDescriptionFromXsk(
  sapling: WasmSapling, 
  context: number, 
  xsk: Buffer | Int8Array | string, 
  address: SaplingPaymentAddress | Buffer | Int8Array | string, 
  rcm: Buffer | Int8Array | string, 
  ar: Buffer | Int8Array | string, 
  value: string | number | BigInt, 
  anchor: Buffer | Int8Array | string, 
  merklePath: Buffer | Int8Array | string, 
  position: number | BigInt, 
  provingKey: Buffer | Int8Array | string, 
  verifyingKey: Buffer | Int8Array | string
): Buffer {
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Int8Array` or hex string')
  const addressBuffer: Buffer = bufferFrom(
    isPaymentAddress(address) ? address.raw : address, 
    'address', 
    '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string'
  )
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Int8Array` or hex string')
  const arBuffer: Buffer = bufferFrom(ar, 'ar', '`Buffer`, `Int8Array` or hex string')
  const valueNum: BigInt = bigIntFrom(value, 'value', '`number` `BigInt` or `string`')
  const anchorBuffer: Buffer = bufferFrom(anchor, 'anchor', '`Buffer`, `Int8Array` or hex string')
  const merklePathBuffer: Buffer = bufferFrom(merklePath, 'merklePath', '`Buffer`, `Int8Array` or hex string')
  const positionNum: BigInt = bigIntFrom(position, 'position', '`number`')
  const provingKeyBuffer: Buffer = bufferFrom(provingKey, 'provingKey', '`Buffer`, `Int8Array` or hex string')
  const verifyingKeyBuffer: Buffer = bufferFrom(verifyingKey, 'verifyingKey', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(
    sapling.wasm_spend_description_from_xsk(
      context, 
      xskBuffer, 
      addressBuffer, 
      rcmBuffer, 
      arBuffer, 
      valueNum, 
      anchorBuffer, 
      merklePathBuffer, 
      positionNum, 
      provingKeyBuffer, 
      verifyingKeyBuffer
    )
  )
}

export function signSpendDescriptionWithXsk(
  sapling: WasmSapling, 
  description: Buffer | Int8Array | string, 
  xsk: Buffer | Int8Array | string, 
  ar: Buffer | Int8Array | string,
  sighash: Buffer | Int8Array | string,
): Buffer {
  const descriptionBuffer: Buffer = bufferFrom(description, 'spendDescription', '`Buffer`, `Int8Array` or hex string')
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Int8Array` or hex string')
  const arBuffer: Buffer = bufferFrom(ar, 'ar', '`Buffer`, `Int8Array` or hex string')
  const sighashBuffer: Buffer = bufferFrom(sighash, 'sighash', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.wasm_sign_spend_description_with_xsk(descriptionBuffer, xskBuffer, arBuffer, sighashBuffer))
} 