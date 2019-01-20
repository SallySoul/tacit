#!/bin/bash

#
# build.sh
#
# This script coordinates cargo, wasm-bindgen, and other utilities to create a 
# web deployable set of artifacts. This file was started as a copy of build.sh
# from Chinedu Francis Nwafili's excellent webgl-water-tutorial

# Change to the directory that this script is in.
# This allows you to run this script from anywhere and have it still work.
cd $(dirname $0)

# Install wasm-bindgen if its not there already
# This is done here mostly becuase cargo install errors
# if tool is installed / cache, which is rough on CI
if ! (test -f $HOME/.cargo/bin/wasm-bindgen && test "`wasm-bindgen --version`" = "wasm-bindgen 0.2.33" ); then
  echo "wasm-bindgen not installed or out of date: installing"
  cargo install -f wasm-bindgen-cli --version 0.2.33
else
  echo "wasm-bindgen installed and up-to-date"
fi

# ./build.sh
if [ -z "$RELEASE"  ]; then
  # --------------------------------------------------
  # DEVELOPMENT BUILD
  # --------------------------------------------------

  # Build the tacit_web_app.wasm file
  RUST_BACKTRACE=1 cargo build --target wasm32-unknown-unknown

  # Process the tacit_web_app.wasm file and generate the necessary
  # JavaScript glue code to run it in the browser.
  wasm-bindgen ../target/wasm32-unknown-unknown/debug/tacit_web_app.wasm --out-dir . --no-typescript --no-modules

# RELEASE=1 ./build.sh
else

  # --------------------------------------------------
  # RELEASE BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  cargo build --target wasm32-unknown-unknown --release &&
  wasm-bindgen ../target/wasm32-unknown-unknown/release/tacit_web_app.wasm --out-dir . --no-typescript --no-modules &&
  wasm-opt -O4 -o optimized.wasm tacit_web_app_bg.wasm  &&
  mv optimized.wasm tacit_web_app_bg.wasm
fi
