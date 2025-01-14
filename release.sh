#!/usr/bin/env bash

set -euo pipefail

for PKG in base signals-ext css macros task silkenweb tauri-proc-macro tauri parse inline-html test
do
    (cd packages/"$PKG" && cargo publish)
done
