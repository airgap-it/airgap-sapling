# AirGap Sapling Wasm

[![npm](https://img.shields.io/npm/v/@airgap/sapling-wasm.svg?colorB=brightgreen)](https://www.npmjs.com/package/@airgap/sapling-wasm)

A Wasm wrapper around [Zcash Rust crates](https://github.com/zcash/librustzcash).

## Install

To add JS AirGap Sapling library into your project run:

```bash
$ npm install --save @airgap/sapling-wasm
```

## Examples

```ts
import * as bip39 from 'bip39'
import * as sapling from '@airgap/sapling-wasm'
import { SaplingPaymentAddress } from '@airgap/sapling-wasm'

const mnemonic: string = bip39.generateMnemonic()
const seed: Buffer = await bip39.mnemonicToSeed(mnemonic, '')
const derivationPath: string = 'm/'

// create an extended spending key
const spendingKey: Buffer = await sapling.getExtendedSpendingKey(seed, derivationPath)
console.log('spendingKey =', spendingKey.toString('hex'))

// create an extended full viewing key
const viewingKey: Buffer = await sapling.getExtendedFullViewingKey(seed, derivationPath)
console.log('viewingKey =', viewingKey.toString('hex'))

// get default address
const address: SaplingPaymentAddress = await sapling.getPaymentAddressFromViewingKey(viewingKey)
console.log(
  'address.index =', address.index.toString('hex'),
  'address.raw =', address.raw.toString('hex')
)
```

More advanced examples can be found in `./examples`.

## Development

### Build

To build a development version that links the library to examples run:
```bash
$ npm run build
```

To build a production version run:
```bash
$ npm run build:prod
```
