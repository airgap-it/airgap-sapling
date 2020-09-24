import { bufferFrom, rejectInvalidTypeOrUnknown } from './utils'

export function getSpendDescriptionFromXsk(sapling, context, xsk, address, rcm, ar, value, anchor, merklePath, position, provingKey, verifyingKey) {
  let xskBuffer
  try {
    xskBuffer = bufferFrom(xsk)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('spendingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let addressBuffer
  try {
    addressBuffer = bufferFrom(isPaymentAddress(address) ? address.raw : address)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('address', '`SaplingPaymentAddress`, `Buffer`, `Int8Array` or hex string', error)
  }

  let rcmBuffer
  try {
    rcmBuffer = bufferFrom(rcm)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('rcm', '`Buffer`, `Int8Array` or hex string', error)
  }

  let arBuffer
  try {
    arBuffer = bufferFrom(ar)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('ar', '`Buffer`, `Int8Array` or hex string', error)
  }

  let valueNum
  if (typeof value === 'number') {
    valueNum = value
  } else if (typeof value === 'string') {
    valueNum = parseInt(value, 10)
  } else {
    return rejectInvalidTypeOrUnknown('value', '`number` or `string`')
  }

  let anchorBuffer
  try {
    anchorBuffer = bufferFrom(anchor)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('anchor', '`Buffer`, `Int8Array` or hex string', error)
  }

  let merklePathBuffer
  try {
    merklePathBuffer = bufferFrom(merklePath)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('merklePath', '`Buffer`, `Int8Array` or hex string', error)
  }

  if (typeof position !== 'number') {
    return rejectInvalidTypeOrUnknown('position', '`number`')
  }

  let provingKeyBuffer
  try {
    provingKeyBuffer = bufferFrom(provingKey)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('provingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let verifyingKeyBuffer
  try {
    verifyingKeyBuffer = bufferFrom(verifyingKey)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('verifyingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

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
      position, 
      provingKeyBuffer, 
      verifyingKeyBuffer
    )
  )
}

export function signSpendDescriptionWithXsk(description, xsk, ar, sighash) {
  let descriptionBuffer
  try {
    descriptionBuffer = bufferFrom(description)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('spendDescription', '`Buffer`, `Int8Array` or hex string', error)
  }

  let xskBuffer
  try {
    xskBuffer = bufferFrom(xsk)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('spendingKey', '`Buffer`, `Int8Array` or hex string', error)
  }

  let arBuffer
  try {
    arBuffer = bufferFrom(ar)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('ar', '`Buffer`, `Int8Array` or hex string', error)
  }

  let sighashBuffer
  try {
    sighashBuffer = bufferFrom(sighash)
  } catch (error) {
    return rejectInvalidTypeOrUnknown('sighash', '`Buffer`, `Int8Array` or hex string', error)
  }

  return Buffer.from(sapling.sign_spend_description_with_xsk(descriptionBuffer, xskBuffer, arBuffer, sighashBuffer))
} 