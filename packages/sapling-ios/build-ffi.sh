#!/bin/bash

# Currently there are some issues while cross compiling Rust code for iOS.
# (https://github.com/rust-lang/rust/issues/79408)
# As a __workaround__, a custom target with directly specified min iOS version is used.

MIN_IOS_VERSION=11.0
ARM_64=arm64
X86_64=x86_64

CURR_DIR=${BASH_SOURCE[0]%/build-ffi.sh}

function rust_target () {
  case $1 in
    "$ARM_64")
      echo aarch64-apple-ios
      ;;
    "$X86_64")
      echo x86_64-apple-ios
      ;;
    *)
      echo "Error: Unknown Rust target."
      exit 1
      ;;
  esac
}

function ios_target () {
  case $1 in
    "$ARM_64")
      echo ios-arm64
      ;;
    "$X86_64")
      echo ios-x86_64-simulator
      ;;
    *)
      echo "Error: Unknown iOS target."
      exit 1
      ;;
  esac
}

function __workaround__rust_target () {
  echo "$(rust_target "$1")$MIN_IOS_VERSION"
}

function ios_target() {
    case $1 in
    "$ARM_64")
      echo ios-arm64
      ;;
    "$X86_64")
      echo ios-x86_64-simulator
      ;;
    *)
      echo "Error: Unknown iOS target."
      exit 1
      ;;
  esac
}

### CHECK begin ###

echo "Checking the environment..."

echo "  [commands]"
if which xcode-select >/dev/null; then
  echo -e "    \xE2\x9C\x94 xcode-select"
else
  echo -e "    \xE2\x9C\x97 xcode-select"
  ERROR="xcode-select could not been found"
fi

if which xcodebuild >/dev/null; then
  echo -e "    \xE2\x9C\x94 xcodebuild"
else
  echo -e "    \xE2\x9C\x97 xcodebuild"
  ERROR="xcode-select could not been found"
fi

if which rustup >/dev/null; then
  echo -e "    \xE2\x9C\x94 rustup"
else
  echo -e "    \xE2\x9C\x97 rustup"
  ERROR="rustup could not been found"
fi

if which rustc >/dev/null; then
  echo -e "    \xE2\x9C\x94 rustc"
else
  echo -e "    \xE2\x9C\x97 rustc"
  ERROR="rustc could not been found"
fi

if which cargo >/dev/null; then
  echo -e "    \xE2\x9C\x94 cargo-lipo"
else
  echo -e "    \xE2\x9C\x97 cargo-lipo"
  ERROR="cargo-lipo could not been found"
fi

if [[ -n "${ERROR}" ]]; then
  echo "Error: $ERROR."
  exit 1
fi

### CHECK end ###

### SETUP COMPILE begin ###

echo -e "\nSetting up compile targets..."

# __workaround__ begin
CARGO_CONFIG_DIR="$CURR_DIR/.cargo"
if [ -d "$CARGO_CONFIG_DIR" ]; then
  echo "  Cargo config directory already exists. ($CARGO_CONFIG_DIR)"
else
  mkdir "$CARGO_CONFIG_DIR"
  echo "  Cargo config directory created. ($CARGO_CONFIG_DIR)"
fi
# __workaround__ end

function add_target () {
  # __workaround__ begin
  local target custom_target custom_target_file

  target=$(rust_target "$1")
  custom_target=$(__workaround__rust_target "$1")
  custom_target_file=$CARGO_CONFIG_DIR/$custom_target.json
  rustc +nightly -Z unstable-options --target="$target" --print target-spec-json > "$custom_target_file"
  sed -i "" -e "s/\"llvm-target\": \"\(.*\)\"/\"llvm-target\": \"\1$MIN_IOS_VERSION\"/g" "$custom_target_file"
  # __workaround__ end

  # replace __workaround__ start
  # rustup target add "$(rust_target "$1")"
  # replace __workaround__ end
}

echo "  Adding Rust targets..."
add_target $ARM_64
add_target $X86_64

### SETUP COMPILE end ###

### COMPILE begin ###

echo -e "\nCompiling..."
CORE_MANIFEST_PATH="$CURR_DIR/../sapling/Cargo.toml"

function build () {
  # __workaround__ start
  local custom_target custom_target_file

  custom_target=$(__workaround__rust_target "$1")
  custom_target_file="$CARGO_CONFIG_DIR/$custom_target.json"
  echo "  cargo +nightly build -Z build-std --manifest-path $CORE_MANIFEST_PATH --release --features \"c_bindings\" --target $custom_target_file"

  rustup +nightly component add rust-src
  cargo +nightly build -Z build-std --manifest-path "$CORE_MANIFEST_PATH" --release --features "c_bindings" --target "$custom_target_file"
  # __workaround__ end

  # replace __workaround__ start
  # local target

  # target=$(rust_target "$1")
  # echo "  cargo build --manifest-path $CORE_MANIFEST_PATH --release --features \"c_bindings\" --target $target"

  # cargo build --manifest-path "$CORE_MANIFEST_PATH" --release --features "c_bindings" --target ""
  # replace __workaround__ end
}

build $ARM_64
build $X86_64

### COMPILE end ###

### CREATE XCFRAMEWORK begin ###

echo -e "\nCreating XCFramework..."

LIB_NAME=$(grep -o "name\s\+.\+" "$CORE_MANIFEST_PATH" | awk '{ print $3 }' | sed -e 's/^"//' -e 's/"$//')

INCLUDE_DIR="$CURR_DIR/../sapling/include"
TARGET_DIR="$CURR_DIR/../../target"

XCFRAMEWORK_DIR=$CURR_DIR/SaplingFFI.xcframework
rm -rf $XCFRAMEWORK_DIR

# __workaround__ start
xcodebuild -create-xcframework \
  -library "$TARGET_DIR/$(__workaround__rust_target "$ARM_64")/release/lib$LIB_NAME.a" \
  -headers "$INCLUDE_DIR" \
  -library "$TARGET_DIR/$(__workaround__rust_target "$X86_64")/release/lib$LIB_NAME.a" \
  -headers "$INCLUDE_DIR" \
  -output "$XCFRAMEWORK_DIR"
# __workaround__ end

# replace __workaround__ start
#xcodebuild -create-xcframework \
#  -library "$TARGET_DIR/$(rust_target "$ARM_64")/release/lib$LIB_NAME.a" \
#  -headers "$INCLUDE_DIR" \
#  -library "$TARGET_DIR/$(rust_target "$X86_64")/release/lib$LIB_NAME.a" \
#  -headers "$INCLUDE_DIR" \
#  -output "$XCFRAMEWORK_DIR"
# replace __workaround__ end

function create_module () {
  local module_map_path

  module_map_path=$(ios_target "$1")/Headers/module.modulemap
  if printf "%s\n" \
    "module SaplingFFI {" \
    "$(echo -e "\theader \"airgap_sapling.h\"")" \
    "$(echo -e "\texport *")" \
    "}" > "$XCFRAMEWORK_DIR/$module_map_path"; then
    echo -e "  \xE2\x9C\x94 $1 module map ($module_map_path)"
  else
    echo -e "  \xE2\x9C\x97 $1 module map"
    exit 1
  fi
}

create_module $ARM_64
create_module $X86_64

### CREATE XCFRAMEWORK end ###

echo -e "\nDone."
