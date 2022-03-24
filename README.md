# AirGap Sapling

[![npm](https://img.shields.io/npm/v/@airgap/sapling-wasm.svg?colorB=brightgreen)](https://www.npmjs.com/package/@airgap/sapling-wasm)
[![jitpack](https://img.shields.io/jitpack/v/github/airgap-it/airgap-sapling)](https://jitpack.io/#airgap-it/airgap-sapling)
[![spm](https://img.shields.io/github/v/tag/airgap-it/airgap-sapling?include_prereleases&label=spm)](https://github.com/airgap-it/airgap-sapling/releases)

Wasm, Android and iOS wrappers around [Zcash Rust crates](https://github.com/zcash/librustzcash).

## Project Overview

The project is divided into the following packages:
- `sapling` - common sources in Rust, provides C and Wasm bindings for [Zcash crates](https://github.com/zcash/librustzcash)
- `sapling-wasm` - a JavaScript library using Wasm bindings from the `sapling` package
- `sapling-android` - a native Android library using C bindings from the `sapling` package
- `sapling-ios` - a native iOS library using C bindings from the `sapling` package

## Sapling Wasm

### Install

To add JS AirGap Sapling library into your project run:

```bash
$ npm install --save @airgap/sapling-wasm
```

### Examples

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

More advanced examples can be found in `js/examples`.

## Sapling Android

### Install

To add Android AirGap Sapling library into your project:

1. Ensure [Android NDK](https://developer.android.com/ndk) is supported in your project. 

2. Add the [JitPack](https://jitpack.io/) repository to your root `build.gradle` file:
  ```groovy
  allprojects {
    repositories {
      ...
      maven { url 'https://jitpack.io' }
    }
  }
  ```

1. Add the dependency:
  ```groovy
  def saplingVersion = "x.y.z"

  implementation "com.github.airgap-it:airgap-sapling:$saplingVersion"
  ```

## Sapling iOS

### Install

To add iOS AirGap Sapling into your project, add the package dependency:

#### Xcode

Open the `Add Package Dependency` window (as described in [the official guide](https://developer.apple.com/documentation/xcode/adding_package_dependencies_to_your_app)) and enter the AirGap Sapling GitHub repository URL:
```
https://github.com/airgap-it/airgap-sapling
```

#### Package.swift file

Add the following dependency in your `Package.swift` file:

```swift
.package(url: "https://github.com/airgap-it/airgap-sapling", from: "x.y.z")
```

