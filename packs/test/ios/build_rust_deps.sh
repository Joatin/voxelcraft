#!/bin/sh

set -e

PATH=$PATH:$HOME/.cargo/bin
echo "$PATH"

cargo build -p packs_test --release --target x86_64-apple-ios
cd ..
cbindgen --config cbindgen.toml --output ios/src/bindings.h