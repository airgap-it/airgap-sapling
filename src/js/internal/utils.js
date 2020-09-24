/* Internal Utils */

// Buffer

export function bufferFrom(value, name, expectedType) {
  if (Buffer.isBuffer(value)) {
    return value
  } else if (isHexString(value)) {
    return  Buffer.from(value, 'hex')
  } else if (typeof value === 'number') {
    return numberToBytes(value)
  } else if (typeof value !== 'string' && value !== undefined && value !== null) {
    return Buffer.from(value)
  } else {
    throwInvalidType(name, expectedType)
  }
}

export function bufferFromOfLength(value, minLength, name, expectedType) {
  let buffer = bufferFrom(value, name, expectedType)

  if (minLength === undefined || minLength <= buffer.byteLength) {
    return buffer
  }

  const leadingBuffer = Buffer.alloc(minLength - buffer.byteLength)
  leadingBuffer.fill(0)

  return Buffer.concat([leadingBuffer, buffer])
}

export function numberToBytes(number) {
  if (typeof number !== 'number') {
    throw new Error(`numberToBytes: expected a number, got ${typeof number}`)
  }

  const buffer = Buffer.alloc(4)
  buffer.fill(0)
  buffer.writeInt32BE(number)

  let firstNonZero = 0
  for (let i = 0; i < buffer.byteLength; i++) {
    if (buffer[0] !== 0x00) {
      firstNonZero = i
      break
    }
  }

  return firstNonZero > 0
    ? buffer.slice(firstNonZero)
    : buffer
}

// String

const hexRe = /^(0x)?[0-9a-fA-F]*$/
export function isHexString(string) {
  return typeof string === 'string' && hexRe.test(string)
}

// Number

export function numberFrom(value, name, expectedType) {
  if (typeof value === 'number') {
    return value
  } else if (typeof value === 'string') {
    return parseInt(value, 10)
  } else {
    throwInvalidType(name, expectedType)
  }
}

// Error

export function rejectPromise(methodName, error) {
  return Promise.reject(typeof error === 'string' ? `${methodName}: ${error}` : error)
}

export function throwInvalidType(name, expectedType) {
  const error = name !== undefined && expected !== undefined
      ? new Error(`\`${paramName}\` is of invalid type, expected ${expectedType}`)
      : new TypeError()

    throw error
}