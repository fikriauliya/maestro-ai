# Maestro

Manage Claude Code instances in Zellij. Track running instances across panes and quickly switch between them.

## Installation

```bash
git clone https://github.com/user/maestro-ai
cd maestro-ai
./install.sh
```

This builds and installs:
- CLI to `~/.local/bin/maestro`
- Plugin to `~/.config/zellij/plugins/maestro.wasm`

Make sure `~/.local/bin` is in your PATH:
```bash
export PATH="$PATH:$HOME/.local/bin"
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

## Zellij Keybinding

Add a keyboard shortcut in `~/.config/zellij/config.kdl`:

```kdl
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
```

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
