#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd patch patches/gd32vf103.yaml
svd2rust --target riscv -i GD32VF103.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt
