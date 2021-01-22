import { SaplingPaymentAddress, SaplingSpendDescription, SaplingUnsignedSpendDescription } from '../../types'

import { WasmSapling } from '../types'
import { isPaymentAddress } from '../address/payment-address'
import { bigIntFrom, bufferFrom } from '../utils'

export function __wasm__spendDescriptionFromXsk(
  sapling: WasmSapling,
  context: number,
  xsk: Buffer | Uint8Array | string,
  address: SaplingPaymentAddress | Buffer | Uint8Array | string,
  rcm: Buffer | Uint8Array | string,
  ar: Buffer | Uint8Array | string,
  value: string | number | BigInt,
  anchor: Buffer | Uint8Array | string,
  merklePath: Buffer | Uint8Array | string
): SaplingUnsignedSpendDescription {
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Uint8Array` or hex string')
  const addressBuffer: Buffer = bufferFrom(
    isPaymentAddress(address) ? address.raw : address,
    'address',
    '`SaplingPaymentAddress`, `Buffer`, `Uint8Array` or hex string'
  )
  const rcmBuffer: Buffer = bufferFrom(rcm, 'rcm', '`Buffer`, `Uint8Array` or hex string')
  const arBuffer: Buffer = bufferFrom(ar, 'ar', '`Buffer`, `Uint8Array` or hex string')
  const valueNum: BigInt = bigIntFrom(value, 'value', '`number` `BigInt` or `string`')
  const anchorBuffer: Buffer = bufferFrom(anchor, 'anchor', '`Buffer`, `Uint8Array` or hex string')
  const merklePathBuffer: Buffer = bufferFrom(merklePath, 'merklePath', '`Buffer`, `Uint8Array` or hex string')

  const spendDescription = Buffer.from(
    sapling.spendDescriptionFromXsk(
      context,
      xskBuffer,
      addressBuffer,
      rcmBuffer,
      arBuffer,
      valueNum,
      anchorBuffer,
      merklePathBuffer
    )
  )

  return {
    cv: spendDescription.slice(0, 32) /* 32 bytes */,
    rt: spendDescription.slice(32, 64) /* 32 bytes */,
    nf: spendDescription.slice(64, 96) /* 32 bytes */,
    rk: spendDescription.slice(96, 128) /* 32 bytes */,
    proof: spendDescription.slice(128, 320) /* 48 + 96 + 48 bytes */
  }
}

export function __wasm__signSpendDescriptionWithXsk(
  sapling: WasmSapling,
  description: SaplingUnsignedSpendDescription,
  xsk: Buffer | Uint8Array | string,
  ar: Buffer | Uint8Array | string,
  sighash: Buffer | Uint8Array | string
): SaplingSpendDescription {
  const descriptionBuffer: Buffer = Buffer.concat([description.cv, description.rt, description.nf, description.rk, description.proof])
  const xskBuffer: Buffer = bufferFrom(xsk, 'spendingKey', '`Buffer`, `Uint8Array` or hex string')
  const arBuffer: Buffer = bufferFrom(ar, 'ar', '`Buffer`, `Uint8Array` or hex string')
  const sighashBuffer: Buffer = bufferFrom(sighash, 'sighash', '`Buffer`, `Uint8Array` or hex string')

  const signedDescription: Buffer = Buffer.from(sapling.signSpendDescriptionWithXsk(descriptionBuffer, xskBuffer, arBuffer, sighashBuffer))

  return {
    cv: signedDescription.slice(0, 32) /* 32 bytes */,
    rt: signedDescription.slice(32, 64) /* 32 bytes */,
    nf: signedDescription.slice(64, 96) /* 32 bytes */,
    rk: signedDescription.slice(96, 128) /* 32 bytes */,
    proof: signedDescription.slice(128, 320) /* 48 + 96 + 48 bytes */,
    spendAuthSig: signedDescription.slice(320, 384) /* 64 bytes */
  }
}
