import { numberFrom } from '../utils';

export function createBindingSignatureForTx(sapling, context, valueBalance, sighash) {
  const valueBalanceNum = numberFrom(valueBalance)
  const sighashBuffer = bufferFrom(sighash)

  return Buffer.from(sapling.wasm_binding_sig(context, valueBalanceNum, sighashBuffer))
}