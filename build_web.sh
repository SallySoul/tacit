#!/usr/bin/env sh

# Ensure wasm-bindgen in installed
if ! (test -f $HOME/.cargo/bin/wasm-bindgen && test "`wasm-bindgen --version`" = "wasm-bindgen 0.2.67" ); then
    echo "wasm-bindgen not installed or out of date: installing"
    cargo install -f wasm-bindgen-cli --version 0.2.67
else
    echo "wasm-bindgen installed and up-to-date"
fi

# Ensure target dir exists
#rm -rf target/web
mkdir -p target/web

# Install index.html
cp web-client/src_web/index.html target/web

# Build file
cargo build --release --target wasm32-unknown-unknown --manifest-path web-client/Cargo.toml

# Build and Install web bindings
wasm-bindgen target/wasm32-unknown-unknown/release/tacit_web_app.wasm --out-dir target/web --no-typescript --target web

