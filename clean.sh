#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

cd quickjs
make clean

cargo_clean(){
cd $DIR/$1
cargo clean
}

cargo_clean quickjs_rust
cargo_clean quickjs_ffi
cargo_clean rust
cargo_clean rust_macro
