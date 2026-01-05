# Maestro

Manage Claude Code instances in Zellij. Track running instances across panes and quickly switch between them.

## Installation

```bash
# Build CLI
cargo build --release -p maestro

# Build Zellij plugin
cargo build --release -p zellij-plugin --target wasm32-wasip1

# Add CLI to PATH
export PATH="$PATH:/path/to/maestro-ai/target/release"
```

## CLI Usage

```bash
# Register current pane as Claude Code instance (requires ZELLIJ_PANE_ID)
echo '{"cwd":"/path/to/project"}' | maestro register

# Update instance status
maestro update running
maestro update waiting

# List all instances
maestro list

# Unregister instance
maestro unregister
```

## Plugin

Load the floating plugin in Zellij:

```bash
./load-plugin.sh
```

Or manually:

```bash
zellij plugin --floating --pinned true \
    -- "file:target/wasm32-wasip1/release/zellij-plugin.wasm"
```

**Keybindings:**
- `j/k` or arrows: Navigate instances
- `Enter`: Focus selected pane
- `r`: Refresh list
- `q/Esc`: Hide plugin

## Claude Code Hooks Configuration

Add hooks to `.claude/settings.json` (user) or `.claude/settings.local.json` (project):

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": ".*",
        "hooks": ["maestro update running"]
      }
    ],
    "PostToolUse": [
      {
        "matcher": ".*",
        "hooks": ["maestro update waiting"]
      }
    ],
    "SessionStart": [
      {
        "hooks": ["maestro register"]
      }
    ],
    "Stop": [
      {
        "hooks": ["maestro unregister"]
      }
    ]
  }
}
```

This automatically registers/unregisters Claude Code instances and tracks their running/waiting status.
