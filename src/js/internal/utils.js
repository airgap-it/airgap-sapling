/* Internal Utils */

// Buffer

export function bufferFrom(value, minLength) {
  let buffer
  if (Buffer.isBuffer(value)) {
    buffer = value
  } else if (isHexString(value)) {
    buffer = Buffer.from(value, 'hex')
  } else if (typeof value === 'number') {
    buffer = numberToBytes(value)
  } else if (typeof value !== 'string' && value !== undefined && value !== null) {
    buffer = Buffer.from(value)
  } else {
    throw new TypeError()
  }

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

// Error

export function reject(methodName, error) {
  return Promise.reject(typeof error === 'string' ? `${methodName}: ${error}` : error)
}

export function ifTypeErrorElseUnknown(error, message) {
  return error instanceof TypeError ? message : 'unknown error'
}