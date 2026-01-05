#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
zellij plugin \
    --floating \
    --pinned true \
    --width 20% \
    --height 20% \
    --x 80% \
    --y 80% \
    --skip-plugin-cache \
    -- "file:$SCRIPT_DIR/target/wasm32-wasip1/release/zellij-plugin.wasm"
