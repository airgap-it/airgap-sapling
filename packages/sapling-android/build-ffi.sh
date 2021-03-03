#!/bin/bash

ARM_64=arm64
ARM=arm
X86_64=x86_64
X86=x86

CURR_DIR=${BASH_SOURCE[0]%/build-ffi.sh}

function rust_target () {
  case $1 in
    "$ARM_64")
      echo aarch64-linux-android
      ;;
    "$ARM")
      echo armv7-linux-androideabi
      ;;
    "$X86_64")
      echo x86_64-linux-android
      ;;
    "$X86")
      echo i686-linux-android
      ;;
    *)
      echo "Error: Unknown Rust target."
      exit 1
      ;;
  esac
}

function android_abi () {
  case $1 in
    "$ARM_64")
      echo arm64-v8a
      ;;
    "$ARM")
      echo armeabi-v7a
      ;;
    "$X86_64")
      echo x86_64
      ;;
    "$X86")
      echo x86
      ;;
    *)
      echo "Error: Unknown Android ABI."
      exit 1
      ;;
  esac
}

### CHECK begin ###

echo "Checking the environment..."

echo "  [variables]"
if [[ -z "${ANDROID_HOME}" ]]; then
  echo -e "    \xE2\x9C\x97 ANDROID_HOME"
  ERROR="ANDROID_HOME has not been set"
else
  echo -e "    \xE2\x9C\x94 ANDROID_HOME"
fi

if [[ -z "${ANDROID_NDK}" ]]; then
  echo -e "    \xE2\x9C\x97 ANDROID_NDK"
  ERROR="ANDROID_NDK has not been set"
else
  echo -e "    \xE2\x9C\x94 ANDROID_NDK"
fi

echo "  [commands]"
if which rustup >/dev/null; then
  echo -e "    \xE2\x9C\x94 rustup"
else
  echo -e "    \xE2\x9C\x97 rustup"
  ERROR="rustup could not been found"
fi

if which cargo >/dev/null; then
  echo -e "    \xE2\x9C\x94 cargo"
else
  echo -e "    \xE2\x9C\x97 cargo"
  ERROR="cargo could not been found"
fi

if [[ -n "${ERROR}" ]]; then
  echo "Error: $ERROR."
  exit 1
fi

### CHECK end ###

### BUILD TOOLCHAINS begin ###

SDK_VERSION=$(grep -o "targetSdkVersion\s\+\d\+" "$CURR_DIR/app/build.gradle" | awk '{ print $2 }')
echo -e "\nBuilding the toolchains for API $SDK_VERSION..."

TOOLCHAINS_DIR="$CURR_DIR/toolchains"
if [ -d "$TOOLCHAINS_DIR" ]; then
  echo "  Toolchains directory already exists, skipping. ($TOOLCHAINS_DIR)"
else
  mkdir "$TOOLCHAINS_DIR"
  echo "  Toolchains directory created. ($TOOLCHAINS_DIR)"
fi

function get_toolchain_dir () {
  echo "$TOOLCHAINS_DIR/api-$SDK_VERSION/$1"
}

function build_toolchain () {
  local dir

  dir=$(get_toolchain_dir "$1")
  if [ -d "$dir" ]; then
    echo "  $1 toolchain already exists, skipping. ($dir)"
  else
    echo "  Making $1 toolchain..."
    "${ANDROID_NDK}"/build/tools/make-standalone-toolchain.sh --platform="android-$SDK_VERSION" --arch="$1" --install-dir="$dir"
    echo "  $1 toolchain installed to $dir"
  fi
}

build_toolchain $ARM_64
build_toolchain $ARM
build_toolchain $X86_64
build_toolchain $X86

### BUILD TOOLCHAINS end ###

### SETUP COMPILE begin ###

echo -e "\nSetting up compile targets..."

CARGO_CONFIG_DIR="$CURR_DIR/.cargo"
if [ -d "$CARGO_CONFIG_DIR" ]; then
  echo "  Cargo config directory already exists. ($CARGO_CONFIG_DIR)"
else
  mkdir "$CARGO_CONFIG_DIR"
  echo "  Cargo config directory created. ($CARGO_CONFIG_DIR)"
fi

CARGO_CONFIG_FILE="$CARGO_CONFIG_DIR/config"
if [ -f "$CARGO_CONFIG_FILE" ]; then
  rm "$CARGO_CONFIG_FILE"
fi

printf "%s\n" \
"[target.aarch64-linux-android]" \
"ar = \"$(get_toolchain_dir $ARM_64)/bin/llvm-ar\"" \
"linker = \"$(get_toolchain_dir $ARM_64)/bin/aarch64-linux-android-clang\"" \
"" \
"[target.armv7-linux-androideabi]" \
"ar = \"$(get_toolchain_dir $ARM)/bin/llvm-ar\"" \
"linker = \"$(get_toolchain_dir $ARM)/bin/arm-linux-androideabi-clang\"" \
"" \
"[target.x86_64-linux-android]" \
"ar = \"$(get_toolchain_dir $X86_64)/bin/llvm-ar\"" \
"linker = \"$(get_toolchain_dir $X86_64)/bin/x86_64-linux-android-clang\"" \
"" \
"[target.i686-linux-android]" \
"ar = \"$(get_toolchain_dir $X86)/bin/llvm-ar\"" \
"linker = \"$(get_toolchain_dir $X86)/bin/i686-linux-android-clang\"" >> "$CARGO_CONFIG_FILE"

echo "  Cargo config saved."

function add_target () {
  rustup target add "$(rust_target "$1")"
}

echo "  Adding Rust targets..."
add_target $ARM_64
add_target $ARM
add_target $X86_64
add_target $X86

### SETUP COMPILE end ###

### COMPILE begin ###

echo -e "\nCompiling..."
CORE_MANIFEST_PATH="$CURR_DIR/../sapling/Cargo.toml"

function build () {
  local target

  target=$(rust_target "$1")
  echo "  cargo build --manifest-path $CORE_MANIFEST_PATH --release --features \"c_bindings\" --target $target"

  cargo build --manifest-path "$CORE_MANIFEST_PATH" --release --features "c_bindings" --target "$target"
}

build $ARM_64
build $ARM
build $X86_64
build $X86

### COMPILE end ###

### COPY begin ###

echo -e "\nCopying the files..."
LIB_NAME=$(grep -o "name\s\+.\+" "$CORE_MANIFEST_PATH" | awk '{ print $3 }' | sed -e 's/^"//' -e 's/"$//')
CPP_DIR="$CURR_DIR/app/src/main/cpp"

INCLUDE_DIR="$CURR_DIR/../sapling/include"
if cp "$INCLUDE_DIR/$LIB_NAME.h" "$CPP_DIR/include/sapling_ffi.h"; then
  echo -e "  \xE2\x9C\x94 header (include/sapling_ffi.h)"
else
  exit 1
fi

TARGET_DIR="$CURR_DIR/../../target"
function cp_lib () {
  local cpp_lib_dir cpp_lib_name

  cpp_lib_dir="libs/$(android_abi "$1")"
  cpp_lib_name="libsapling_ffi.a"

  mkdir -p "$CPP_DIR/$cpp_lib_dir"
  if cp "$TARGET_DIR/$(rust_target "$1")/release/lib$LIB_NAME.a" "$CPP_DIR/$cpp_lib_dir/$cpp_lib_name"; then
    echo -e "  \xE2\x9C\x94 $1 static lib ($cpp_lib_dir/$cpp_lib_name)"
  else
    echo -e "  \xE2\x9C\x97 $1 static lib"
    exit 1
  fi
}

cp_lib $ARM_64
cp_lib $ARM
cp_lib $X86_64
cp_lib $X86

### COPY end ###

echo -e "\nDone."
