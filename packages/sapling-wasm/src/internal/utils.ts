/* Internal Utils */

// Buffer

export function bufferFrom(value: Buffer | Uint8Array | string | number, name?: string, expectedType?: string): Buffer {
  if (Buffer.isBuffer(value)) {
    return value
  } else if (isHexString(value)) {
    return Buffer.from(value, 'hex')
  } else if (typeof value === 'number') {
    return numberToBytes(value)
  } else if (typeof value !== 'string' && value !== undefined && value !== null) {
    return Buffer.from(value)
  } else {
    throw invalidTypeError(name, expectedType)
  }
}

export function bufferFromOfLength(
  value: Buffer | Uint8Array | string | number,
  minLength: number,
  name?: string,
  expectedType?: string
): Buffer {
  let buffer = bufferFrom(value, name, expectedType)

  if (minLength === undefined || minLength <= buffer.byteLength) {
    return buffer
  }

  const leadingBuffer = Buffer.alloc(minLength - buffer.byteLength)
  leadingBuffer.fill(0)

  return Buffer.concat([leadingBuffer, buffer])
}

export function numberToBytes(number: number): Buffer {
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

  return firstNonZero > 0 ? buffer.slice(firstNonZero) : buffer
}

// String

const hexRe = /^(0x)?[0-9a-fA-F]*$/
export function isHexString(string: any): string is string {
  return typeof string === 'string' && hexRe.test(string)
}

// Number

export function bigIntFrom(value: number | string | BigInt, name?: string, expectedType?: string): BigInt {
  if (typeof value === 'bigint') {
    return value
  } else if (typeof value === 'number' || typeof value === 'string') {
    return BigInt(value)
  } else {
    throw invalidTypeError(name, expectedType)
  }
}

// Error

export function uninitialized(): void {
  throw new Error('sapling-wasm uninitialized')
}

export async function rejectPromise<T>(methodName: string, error: any): Promise<T> {
  return Promise.reject(typeof error === 'string' ? `${methodName}: ${error}` : error)
}

export function invalidTypeError(name?: string, expectedType?: string): Error {
  return name !== undefined && expectedType !== undefined
    ? new Error(`\`${name}\` is of invalid type, expected ${expectedType}`)
    : new TypeError()
}
