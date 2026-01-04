#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
zellij plugin \
    --floating \
    --pinned true \
    --width 30% \
    --height 20% \
    --x 70% \
    --y 80% \
    --skip-plugin-cache \
    -- "file:$SCRIPT_DIR/target/wasm32-wasip1/release/zellij-plugin.wasm"

