#!/usr/bin/env bash

# 检测坏代码风格

if ! hash cargo-clippy 2>/dev/null; then
  rustup component add clippy
fi

git add -u && git commit -m'.' || true
cargo +nightly clippy --fix -Z unstable-options
cargo fmt
