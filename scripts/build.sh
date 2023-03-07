#!/bin/bash

set -euo pipefail

BASE_PATH=$(dirname "$0")
LEVEL=
if [ "$1" = "--debug" ]; then
  LEVEL=" --debug"
fi

export APPLE_SIGNING_IDENTITY="Developer ID Application: Felipe Coury (UXWHD5F6F4)"
export APPLE_CERTIFICATE=$(cat "${BASE_PATH}/../certs/razermon-base64.txt")
export APPLE_CERTIFICATE_PASSWORD="tempra13"
# export APPLE_ID="felipe@coury.com.br"
# export APPLE_PASSWORD="vozu-iagm-hsac-qzds"

if [ -z "${LEVEL}" ]; then
  echo "Building release version"
  ./tauri/tooling/cli/node/tauri.js build --verbose --target aarch64-apple-darwin
else
  echo "Building debug version"
  ./tauri/tooling/cli/node/tauri.js build --debug --verbose --target aarch64-apple-darwin
fi
