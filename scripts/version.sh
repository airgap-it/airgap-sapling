#!/bin/bash

function help () {
  echo -e "Usage: \t ./version.sh OPTION"
  echo -e "\t-c, --check\tCheck if versions match"
  echo -e "\t-f, --fix\tFix mismatched versions"
  echo -e "\t-h, --help\tPrint help"
}

### Utils ###

ANDROID_PATH=${BASH_SOURCE[0]/%version.sh/../packages/sapling-android}
CORE_PATH=${BASH_SOURCE[0]/%version.sh/../packages/sapling}
LERNA_PATH=${BASH_SOURCE[0]/%version.sh/..}

function load_versions () {
  ANDROID_VERSION=$(grep -o "versionName\s\+.\+" "$ANDROID_PATH/app/build.gradle" | awk '{ print $2 }' | sed -e 's/^"//' -e 's/"$//')
  echo "  android $ANDROID_VERSION"

  CORE_VERSION=$(grep -o "version\s\+.\+" "$CORE_PATH/Cargo.toml" | head -1 | awk '{ print $3 }' | sed -e 's/^"//' -e 's/"$//')
  echo "  core $CORE_VERSION"

  LERNA_VERSION=$(grep -o "\"version\":\s\+.\+" "$LERNA_PATH/lerna.json" | awk '{ print $2 }' | sed -e 's/^"//' -e 's/"$//')
  echo "  js $LERNA_VERSION"
}

function compare_versions() {
  if [[ "$ANDROID_VERSION" == "$CORE_VERSION" && "$CORE_VERSION" == "$LERNA_VERSION" ]]; then
    return 0
  fi

  local android android_beta core core_beta lerna lerna_beta

  android="$(echo "$ANDROID_VERSION" | awk -F'-' '{print $1}')"
  android_beta=$(echo "$ANDROID_VERSION" | awk -F'-beta' '{print $2 + 0}')

  core=$(echo "$CORE_VERSION" | awk -F'-' '{print $1}')
  core_beta=$(echo "$CORE_VERSION" | awk -F'-beta.' '{print $2}')

  lerna=$(echo "$LERNA_VERSION" | awk -F'-' '{print $1}')
  lerna_beta=$(echo "$LERNA_VERSION" | awk -F'-beta.' '{print $2}')

  if [[ "$android" == "$core" && "$core" == "$lerna" && "$android_beta" == "$core_beta" && "$core_beta" == "$lerna_beta" ]]; then
    return 0
  fi

  return 1
}

function format_android_beta() {
  printf "%02d\n" "$1"
}

function format_core_beta() {
  echo ".$1"
}

function format_js_beta() {
  echo ".$1"
}

function find_latest() {
  local latest latest_beta

  latest=$(printf '%s\n' "$ANDROID_VERSION" "$CORE_VERSION" "$LERNA_VERSION" | sort --version-sort | grep -v "beta" | tail -1)
  latest_beta=$(printf '%s\n' "$ANDROID_VERSION" "$CORE_VERSION" "$LERNA_VERSION" | sed -e 's/beta\([0-9]*$\)/beta.\1/g' | sort --version-sort | grep "beta" | tail -1)

  local beta_target

  beta_target=$(echo "$latest_beta" | awk -F'-' '{print $1}')
  if [[ $(printf '%s\n' "$latest" "$beta_target" | sort --version-sort | tail -1) == "$latest" ]]; then
    echo "$latest"
  else
    local beta_version beta_formatted

    beta_version=$(echo "$latest_beta" | awk -F'-beta.' '{print $2+0}')
    beta_formatted=$($1 "$beta_version")
    echo "$beta_target-beta$beta_formatted"
  fi
}

### Commands ###

function check_versions () {
  echo "Checking packages versions..."

  load_versions

  if compare_versions; then
    echo "Versions match."
    return 0
  else
    echo -e "\n  Versions mismatch."
    return 1
  fi
}

function fix_versions () {
  echo "Fixing packages to the latest version..."
  local android_latest core_latest js_latest

  android_latest=$(find_latest format_android_beta)
  if [[ "$ANDROID_VERSION" != "$android_latest" ]]; then
    echo "  android $ANDROID_VERSION -> $android_latest"
    sed -i "" -e "s/versionName \"$ANDROID_VERSION\"/versionName \"$android_latest\"/g" "$ANDROID_PATH/app/build.gradle"
  fi

  core_latest=$(find_latest format_core_beta)
  if [[ "$CORE_VERSION" != "$core_latest" ]]; then
    echo "  core $CORE_VERSION -> $core_latest"
    sed -i "" -e "s/version = \"$CORE_VERSION\"/version = \"$core_latest\"/1" "$CORE_PATH/Cargo.toml"
  fi

  js_latest=$(find_latest format_js_beta)
  if [[ "$LERNA_VERSION" != "$js_latest" ]]; then
    echo "  js $LERNA_VERSION -> $js_latest"
    npm --prefix "$LERNA_PATH" run bump:version -- "$js_latest" --yes
  fi

  if check_versions; then
    echo "Done."
  else
    exit 1
  fi
}

### Execution ###

if [[ $# -eq 0 ]]; then
  help
else
  while [[ $# -gt 0 ]]
    do
        key="$1"

        case $key in
            -c|--check)
              if check_versions; then
                exit 0
              else
                exit 1
              fi
              ;;
            -f|--fix)
              if check_versions; then
                exit 0
              else
                fix_versions
                exit 0
              fi
              ;;
            -h|--help|*)
              help
              exit 0
              ;;
        esac
    done
fi
