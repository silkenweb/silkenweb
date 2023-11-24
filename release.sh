#!/usr/bin/env bash

set -euo pipefail

for PKG in base signals-ext css macros task silkenweb tauri-proc-macro tauri parse inline-svgs test
do
    echo '(cd packages/'"$PKG"' && cargo publish)'
done
