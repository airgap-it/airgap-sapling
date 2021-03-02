# AirGap Sapling Android

[![release](https://img.shields.io/jitpack/v/github/airgap-it/airgap-sapling)](https://jitpack.io/#airgap-it/airgap-sapling)

An Android wrapper around [Zcash Rust crates](https://github.com/zcash/librustzcash).

## Install

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
  def saplingVersion = "0.0.4"

  implementation "com.github.airgap-it:airgap-sapling:$saplingVersion"
  ```

## Development

### Update C Bindings
To update the C bindings from the core `sapling` package run:
```bash
# $ pwd
# <project-dir>/packages/sapling-android

$ ./build-ffi.sh
```
