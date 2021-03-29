import { SaplingOutputDescription, SaplingPartialOutputDescription, SaplingPaymentAddress } from '../../types'

import { WasmSapling } from '../types'
import { isPaymentAddress } from '../address/payment-address'
import { stringFrom, bufferFrom } from '../utils'

export function __wasm__outputDescriptionFromXfvk(
  sapling: WasmSapling,
  context: number,
  xfvk: Buffer | Uint8Array | string,
  to: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  memo?: Buffer | Uint8Array | string
): SaplingOutputDescription {
  const xfvkBuffer: Buffer = bufferFrom(xfvk, 'viewingKey', '`Buffer`, `Uint8Array` or hex string')
  const toBuffer: Buffer = bufferFrom(
    isPaymentAddress(to) ? to.raw : to,
    'destination',
    '`SaplingPaymentAddress`, `Buffer`, `Uint8Array` or hex string'
  )
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Uint8Array` or hex string')
  const valueString: string = stringFrom(value, 'value', '`number`, `BigInt` or `string`')
  const memoBuffer: Buffer | undefined = memo !== undefined ? bufferFrom(memo, 'memo', '`Buffer`, `Uint8Array` or hex string') : undefined

  const outputDescription: Buffer = Buffer.from(
    memoBuffer !== undefined
      ? sapling.outputDescriptionFromXfvkWithMemo(context, xfvkBuffer, toBuffer, rcmBuffer, valueString, memoBuffer)
      : sapling.outputDescriptionFromXfvk(context, xfvkBuffer, toBuffer, rcmBuffer, valueString)
  )

  return {
    cv: outputDescription.slice(0, 32) /* 32 bytes */,
    cm: outputDescription.slice(32, 64) /* 32 bytes */,
    epk: outputDescription.slice(64, 96) /* 32 bytes */,
    cenc: outputDescription.slice(96, 676) /* 580 bytes */,
    cout: outputDescription.slice(676, 756) /* 80 bytes */,
    proof: outputDescription.slice(756, 948) /* 48 + 96 + 48 bytes */
  }
}

export function __wasm__partialOutputDescription(
  sapling: WasmSapling,
  context: number,
  to: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  esk: Buffer | Uint8Array | string,
  value: string | number | BigInt
): SaplingPartialOutputDescription {
  const toBuffer: Buffer = bufferFrom(
    isPaymentAddress(to) ? to.raw : to,
    'destination',
    '`SaplingPaymentAddress`, `Buffer`, `Uint8Array` or hex string'
  )
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Uint8Array` or hex string')
  const eskBuffer: Buffer = bufferFrom(esk, 'esk', '`Buffer`, `Uint8Array` or hex string')
  const valueString: string = stringFrom(value, 'value', '`number`, `BigInt` or `string`')

  const outputDescription: Buffer = Buffer.from(sapling.partialOutputDescription(context, toBuffer, rcmBuffer, eskBuffer, valueString))

  return {
    cv: outputDescription.slice(0, 32) /* 32 bytes */,
    cm: outputDescription.slice(32, 64) /* 32 bytes */,
    proof: outputDescription.slice(64, 256) /* 48 + 96 + 48 bytes */
  }
}

export function __wasm__deriveEpkFromEsk(
  sapling: WasmSapling, 
  diversifier: Buffer | Uint8Array | string, 
  esk: Buffer | Uint8Array | string
): Buffer {
  const diversifierBuffer: Buffer = bufferFrom(diversifier, 'diversifier', '`Buffer`, `Uint8Array` or hex string')
  const eskBuffer: Buffer = bufferFrom(esk, 'esk', '`Buffer`, `Uint8Array` or hex string')

  return Buffer.from(sapling.deriveEpkFromEsk(diversifierBuffer, eskBuffer))
}