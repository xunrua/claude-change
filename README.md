# claude-profile (ccp)

[**中文**](README.zh-CN.md) | English

[![CI](https://github.com/xunrua/claude-change/actions/workflows/ci.yml/badge.svg)](https://github.com/xunrua/claude-change/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/xunrua/claude-change)](https://github.com/xunrua/claude-change/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> CLI/TUI tool for managing Claude Code configuration profiles.

Switch API providers, models, and full configurations with a single command — no more manually editing `~/.claude/settings.json`.

---

## Features

- **Profile management** — create, import, validate, switch, remove profiles
- **TUI mode** — interactive terminal UI with arrow-key navigation and preview
- **Automatic backup** — every switch creates a backup, rollback with one command
- **API key masking** — sensitive keys are masked in display output
- **Atomic writes** — safe file operations with temp file + rename
- **XDG-compliant** — follows standard config/data directory conventions
- **Hook integration** — optional Claude Code hook for switch intent detection
- **Yolo/Safe mode** — toggle dangerous operation confirmation prompts

---

## Quick Start

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/xunrua/claude-change/main/install.sh | bash
```

### Install from source

```bash
git clone https://github.com/xunrua/claude-change.git
cd claude-change
cargo build --release
./install.sh
```

---

## Usage

### CLI Commands

```bash
ccp list                        # List all profiles
ccp switch <name>               # Switch to a profile
ccp switch <name> --dry-run     # Preview without changes
ccp switch <name> --force       # Skip dirty check
ccp rollback                    # Undo last switch
ccp import <file> -n <name>     # Import existing settings.json
ccp add <name>                  # Create profile interactively
ccp show <name>                 # Display profile (keys masked)
ccp validate <name>             # Validate profile
ccp remove <name>               # Delete a profile
ccp tui                         # Launch interactive TUI
ccp hook install                # Install Claude Code hook
ccp hook uninstall              # Uninstall hook
ccp yolo                        # Enable yolo mode
ccp safe                        # Re-enable safe mode
```

### TUI Mode

Run `ccp tui` to launch the interactive terminal UI. Use arrow keys or `j/k` to navigate, `Enter` to switch, `q` to quit.

### Profile Format

Profiles are stored as TOML files:

```toml
name = "kimi"
description = "Kimi Official API"

[settings.env]
ANTHROPIC_BASE_URL = "https://api.kimi.com/coding/"
ANTHROPIC_AUTH_TOKEN = "sk-xxx"

[settings]
model = "opus[1m]"
effortLevel = "xhigh"
language = "en"
```

---

## Configuration

### File Locations

| Data | Path |
|------|------|
| Profiles | `~/.config/claude-profile/profiles/*.toml` |
| Backups | `~/.local/share/claude-profile/backups/` |
| Active marker | `~/.local/share/claude-profile/active_profile` |
| Claude settings | `~/.claude/settings.json` |

### Environment Variables

| Variable | Description |
|----------|------|
| `CLAUDE_PROFILE_BACKUP_COUNT` | Number of backups to retain (default: 50) |
| `XDG_CONFIG_HOME` | Override config directory |
| `XDG_DATA_HOME` | Override data directory |
| `CLAUDE_CONFIG_DIR` | Override Claude Code config location |

---

## Building from Source

Prerequisites: [Rust](https://rustup.rs) 1.85+

```bash
cargo build --release                          # with TUI
cargo build --release --no-default-features    # CLI only, smaller binary
```

---

## Download

Pre-built binaries are available on the [Releases page](https://github.com/xunrua/claude-change/releases/latest).

| Platform | Architecture |
|----------|------|
| macOS | arm64 (Apple Silicon), x86_64 (Intel) |
| Linux | x86_64, aarch64 |

---

## License

[MIT](LICENSE)
