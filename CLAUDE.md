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

## Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

### Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
