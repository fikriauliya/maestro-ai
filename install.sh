#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Build release binaries
make -C "$SCRIPT_DIR" release

# Install CLI
CLI_DEST="${HOME}/.local/bin"
mkdir -p "$CLI_DEST"
cp "$SCRIPT_DIR/target/release/maestro" "$CLI_DEST/"
echo "Installed CLI to $CLI_DEST/maestro"

# Install plugin
PLUGIN_DEST="${HOME}/.config/zellij/plugins"
mkdir -p "$PLUGIN_DEST"
cp "$SCRIPT_DIR/target/wasm32-wasip1/release/zellij-plugin.wasm" "$PLUGIN_DEST/maestro.wasm"
echo "Installed plugin to $PLUGIN_DEST/maestro.wasm"

echo ""
echo "Installation complete!"
echo ""
echo "Make sure ~/.local/bin is in your PATH:"
echo "  export PATH=\"\$PATH:\$HOME/.local/bin\""
echo ""
echo "Add this keybinding to ~/.config/zellij/config.kdl:"
cat << 'EOF'
keybinds {
    shared {
        bind "Alt m" {
            LaunchPlugin "file:~/.config/zellij/plugins/maestro.wasm" {
                floating true
                pinned true
                width "20%"
                height "20%"
                x "80%"
                y "80%"
            }
        }
    }
}
EOF
