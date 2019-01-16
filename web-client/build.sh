#!/bin/bash

# Change to the directory that this script is in.
# This allows you to run this script from anywhere and have it still work.
cd $(dirname $0)

# ./build.sh
if [ -z "$RELEASE"  ]; then
  # --------------------------------------------------
  # DEVELOPMENT BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  RUST_BACKTRACE=1 cargo build --target wasm32-unknown-unknown

  # # Process the webgl_water_tutorial.wasm file and generate the necessary
  # # JavaScript glue code to run it in the browser.
  wasm-bindgen ./target/wasm32-unknown-unknown/debug/tacit_web_app.wasm --out-dir . --no-typescript --no-modules

# RELEASE=1 ./build.sh
else

  # --------------------------------------------------
  # RELEASE BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  cargo build --target wasm32-unknown-unknown --release &&
  wasm-bindgen ./target/wasm32-unknown-unknown/release/tacit_web_app.wasm --out-dir . --no-typescript --no-modules &&
  wasm-opt -O3 -o optimized.wasm tacit_web_app_bg.wasm  &&
  mv optimized.wasm tacit_web_app_bg.wasm
fi
