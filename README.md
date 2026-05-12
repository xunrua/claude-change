# claude-profile (ccp)

[![CI](https://github.com/xunrua/claude-change/actions/workflows/ci.yml/badge.svg)](https://github.com/xunrua/claude-change/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/xunrua/claude-change)](https://github.com/xunrua/claude-change/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> CLI/TUI tool for managing Claude Code configuration profiles.
> 一个用于管理 Claude Code 配置的 CLI/TUI 工具。

Switch API providers, models, and full configurations with a single command.

一键切换 API 提供商、模型和完整配置，无需手动编辑 `~/.claude/settings.json`。

---

## Why / 为什么需要它

If you use Claude Code with multiple API providers (Anthropic, Kimi, OpenRouter, etc.), switching between them requires manually editing `~/.claude/settings.json`. `ccp` makes this a single command or keystroke.

如果你同时使用多个 API 提供商（Anthropic、Kimi、OpenRouter 等），每次切换都需要手动编辑 `~/.claude/settings.json`。`ccp` 将其简化为一条命令。

---

## Features / 功能

- **Profile management** — create, import, validate, switch, remove profiles
- **TUI mode** — interactive terminal UI with arrow-key navigation and preview
- **Automatic backup** — every switch creates a backup, rollback with one command
- **API key masking** — sensitive keys are masked in display output
- **Atomic writes** — safe file operations with temp file + rename
- **XDG-compliant** — follows standard config/data directory conventions
- **Hook integration** — optional Claude Code hook for switch intent detection
- **Yolo/Safe mode** — toggle dangerous operation confirmation prompts

---

## Quick Start / 快速开始

### Install / 安装

```bash
curl -fsSL https://raw.githubusercontent.com/xunrua/claude-change/main/install.sh | bash
```

### Install from source / 从源码编译

```bash
git clone https://github.com/xunrua/claude-change.git
cd claude-change
cargo build --release
./install.sh
```

---

## Usage / 使用方法

### CLI Commands / 命令行

```bash
ccp list                        # List all profiles / 列出所有配置
ccp switch <name>               # Switch to a profile / 切换到指定配置
ccp switch <name> --dry-run     # Preview without changes / 预览切换（不修改）
ccp switch <name> --force       # Skip dirty check / 跳过脏检查
ccp rollback                    # Undo last switch / 回滚到上一个配置
ccp import <file> -n <name>     # Import existing settings.json / 导入现有配置
ccp add <name>                  # Create profile interactively / 交互式创建配置
ccp show <name>                 # Display profile (keys masked) / 显示配置（密钥已掩码）
ccp validate <name>             # Validate profile / 验证配置
ccp remove <name>               # Delete a profile / 删除配置
ccp tui                         # Launch interactive TUI / 启动交互界面
ccp hook install                # Install Claude Code hook / 安装 Hook
ccp hook uninstall              # Uninstall hook / 卸载 Hook
ccp yolo                        # Enable yolo mode / 开启 yolo 模式
ccp safe                        # Re-enable safe mode / 恢复安全模式
```

### TUI Mode / 交互界面

Run `ccp tui` to launch the interactive terminal UI. Use arrow keys or `j/k` to navigate, `Enter` to switch, `q` to quit.

运行 `ccp tui` 启动交互界面。使用方向键或 `j/k` 导航，`Enter` 切换，`q` 退出。

### Profile Format / 配置格式

Profiles are stored as TOML files:

配置以 TOML 文件格式存储：

```toml
name = "kimi"
description = "Kimi Official API"

[settings.env]
ANTHROPIC_BASE_URL = "https://api.kimi.com/coding/"
ANTHROPIC_AUTH_TOKEN = "sk-xxx"

[settings]
model = "opus[1m]"
effortLevel = "xhigh"
language = "zh-CN"
```

---

## Configuration / 配置

### File Locations / 文件路径

| Data / 数据 | Path / 路径 |
|------|------|
| Profiles / 配置文件 | `~/.config/claude-profile/profiles/*.toml` |
| Backups / 备份 | `~/.local/share/claude-profile/backups/` |
| Active marker / 活动标记 | `~/.local/share/claude-profile/active_profile` |
| Claude settings / Claude 设置 | `~/.claude/settings.json` |

### Environment Variables / 环境变量

| Variable | Description / 描述 |
|----------|------|
| `CLAUDE_PROFILE_BACKUP_COUNT` | Number of backups to retain (default: 50) / 备份保留数量（默认 50） |
| `XDG_CONFIG_HOME` | Override config directory / 覆盖配置目录 |
| `XDG_DATA_HOME` | Override data directory / 覆盖数据目录 |
| `CLAUDE_CONFIG_DIR` | Override Claude Code config location / 覆盖 Claude Code 配置路径 |

---

## Building from Source / 从源码编译

Prerequisites / 前置条件: [Rust](https://rustup.rs) 1.85+

```bash
cargo build --release                          # with TUI / 包含 TUI
cargo build --release --no-default-features    # CLI only, smaller binary / 仅 CLI，更小体积
```

---

## Download / 下载

Pre-built binaries are available on the [Releases page](https://github.com/xunrua/claude-change/releases/latest).

预编译二进制文件可在 [Releases 页面](https://github.com/xunrua/claude-change/releases/latest) 下载。

| Platform / 平台 | Architecture |
|---------|------|
| macOS | arm64 (Apple Silicon), x86_64 (Intel) |
| Linux | x86_64, aarch64 |

---

## License

[MIT](LICENSE)
