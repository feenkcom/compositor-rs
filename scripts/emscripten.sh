#!/bin/bash

set -e

emsdk_executable="$(which emsdk)"
export EMSDK="$(dirname "$emsdk_executable")"
export RUST_BACKTRACE=1
export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true
export EMCC_CFLAGS="-mbulk-memory -matomics -sSHARED_MEMORY -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=GL -sMAX_WEBGL_VERSION=2 -sFULL_ES2=1 -sASSERTIONS -sALLOW_MEMORY_GROWTH=1 -sPTHREAD_POOL_SIZE=navigator.hardwareConcurrency -sUSE_PTHREADS=1 -sASYNCIFY"
#export PATH="/opt/homebrew/opt/binutils/bin:$PATH"
# --emrun

# prevents rust-skia from using prebuilt binaries
export SKIA_BINARIES_URL='file://path/to/skia-binaries.tar.gz'

echo "Emscripten SDK: ${EMSDK}"
echo "Installed git: $(git --version)"
cargo +nightly build -Z build-std=panic_abort,std --release --package compositor-skia-platform --target wasm32-unknown-emscripten