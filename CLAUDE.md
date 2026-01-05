# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# CLI (default build)
cargo build -p maestro

# Zellij plugin (requires WASM target)
cargo build -p zellij-plugin --target wasm32-wasip1

# Release builds
cargo build --release -p maestro
cargo build --release -p zellij-plugin --target wasm32-wasip1
```

## Testing

```bash
cargo test                    # all tests
cargo test -p maestro         # CLI tests only
cargo test -p zellij-plugin   # plugin tests only
```

## Architecture

Maestro manages Claude Code instances running in Zellij terminal multiplexer. Two crates share instance data via `/tmp/maestro-ai/instances.json`.

### CLI (`crates/cli`) → binary: `maestro`

Command-line tool for instance lifecycle management:
- `register` - Register instance (reads JSON from stdin, uses `ZELLIJ_PANE_ID`)
- `update <status>` - Set status to `running` or `waiting`
- `unregister` - Remove instance
- `list` - Display all instances

`InstanceStore` handles JSON persistence. `Instance` contains `pane_id`, `folder`, `status`.

### Zellij Plugin (`crates/zellij-plugin`)

WASM plugin providing interactive UI. Built with `zellij-tile` + `ratatui`.

**Modules:**
- `state.rs` - UI state, keyboard handling, instance refresh via `cat` command
- `instance.rs` - `ClaudeInstance` model with JSON parsing
- `ui.rs` - Ratatui widgets for list rendering
- `ansi.rs` - Converts ratatui buffer to ANSI escape codes

**Key bindings:** `j/k` or arrows to navigate, `Enter` to focus pane, `r` to refresh, `q/Esc` to hide.

**Event loop:** Subscribes to `Key`, `Timer`, `RunCommandResult`, `PermissionRequestResult`. Auto-refreshes every 1 second.

### Data Flow

```
CLI register/update/unregister
         ↓
/tmp/maestro-ai/instances.json
         ↓
Plugin reads via run_command(cat)
         ↓
Renders instance list with ratatui
```

## Loading the Plugin

```bash
./load-plugin.sh
```

Or manually in Zellij with the WASM path: `target/wasm32-wasip1/release/zellij_plugin.wasm`
