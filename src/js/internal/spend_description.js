import { bufferFrom, numberFrom } from './utils'

export function getSpendDescriptionFromXsk(sapling, context, xsk, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey) {
  const xskBuffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Int8Array` or hex string')
  const addressBuffer = bufferFrom(address, 'address', '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string')
  const rcmBuffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Int8Array` or hex string')
  const arBuffer = bufferFrom(ar, 'ar', '`Buffer`, `Int8Array` or hex string')
  const valueNum = numberFrom(value, 'value', '`number` or `string`')
  const anchorBuffer = bufferFrom(anchor, 'anchor', '`Buffer`, `Int8Array` or hex string')
  const merklePathBuffer = bufferFrom(merklePath, 'merklePath', '`Buffer`, `Int8Array` or hex string')
  const positionNum = numberFrom(position, 'position', '`number`')
  const provingKeyBuffer = bufferFrom(provingKey, 'provingKey', '`Buffer`, `Int8Array` or hex string')
  const verifyingKeyBuffer = bufferFrom(verifyingKey, 'verifyingKey', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(
    sapling.prepare_spend_transaction_from_xsk(
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

export function signSpendDescriptionWithXsk(description, xsk, ar, sighash) {
  const descriptionBuffer = bufferFrom(description, 'spendDescription', '`Buffer`, `Int8Array` or hex string')
  const xskBuffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Int8Array` or hex string')
  const arBuffer = bufferFrom(ar, 'ar', '`Buffer`, `Int8Array` or hex string')
  const sighashBuffer = bufferFrom(sighash, 'sighash', '`Buffer`, `Int8Array` or hex string')

  return Buffer.from(sapling.sign_spend_description_with_xsk(descriptionBuffer, xskBuffer, arBuffer, sighashBuffer))
} 