#!/bin/sh

set -ex

# Originally from https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.4/clang+llvm-16.0.4-x86_64-linux-gnu-ubuntu-22.04.tar.xz
curl https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.4/clang+llvm-16.0.4-x86_64-linux-gnu-ubuntu-22.04.tar.xz | \
  tar xJf -
bin="$PWD/clang+llvm-16.0.4-x86_64-linux-gnu-ubuntu-22.04/bin"

git clone https://github.com/WebAssembly/wasi-libc

cd wasi-libc
git reset --hard 1dfe5c302d1c5ab621f7abf04620fae92700fd22
make -j$(nproc) \
    CC="$bin/clang" \
    NM="$bin/llvm-nm" \
    AR="$bin/llvm-ar" \
    THREAD_MODEL=posix \
    INSTALL_DIR=/wasm32-wasi-threads \
    install

cd ..
rm -rf wasi-libc
rm -rf clang+llvm*
