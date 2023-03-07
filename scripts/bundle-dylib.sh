#!/bin/bash

set -e

LEVEL=release
if [ "$1" = "--debug" ]; then
  LEVEL=debug
fi

BASE_PATH=$(dirname "$0")
ROOT="$BASE_PATH/../src-tauri/target/aarch64-apple-darwin/${LEVEL}/bundle/macos"
APP_NAME="RazerMon"
APP_PATH="${ROOT}/${APP_NAME}.app"

test -d "$APP_PATH/Contents/Frameworks" || mkdir -p "$APP_PATH/Contents/Frameworks"
cp scripts/librazermacos.so "$APP_PATH/Contents/Frameworks"
chmod 644 "$APP_PATH/Contents/Frameworks/librazermacos.so"

cd "$APP_PATH/Contents/MacOS"
install_name_tool -id "@rpath/librazermacos.so" "../Frameworks/librazermacos.so"
cd -
install_name_tool -change "librazermacos.so" "@rpath/librazermacos.so" "$APP_PATH/Contents/MacOS/$APP_NAME"
install_name_tool -add_rpath "@executable_path/../Frameworks" "$APP_PATH/Contents/MacOS/$APP_NAME"

codesign --force --sign "Developer ID Application: Felipe Coury (UXWHD5F6F4)" --deep --options runtime "$APP_PATH"
