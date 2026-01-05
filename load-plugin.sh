#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
zellij plugin \
    --floating \
    --pinned true \
    --skip-plugin-cache \
    -- "file:$SCRIPT_DIR/target/wasm32-wasip1/release/zellij-plugin.wasm"

