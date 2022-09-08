#!/bin/bash

cd projectm

git checkout emscripten

cd ..

export EMCC_CFLAGS="-s USE_SDL=2 -s NO_DISABLE_EXCEPTION_CATCHING -s ERROR_ON_UNDEFINED_SYMBOLS=0"

cargo clean && cargo build --target wasm32-unknown-emscripten