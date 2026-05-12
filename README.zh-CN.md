# claude-profile (ccp)

中文 | [**English**](README.md)

[![CI](https://github.com/xunrua/claude-change/actions/workflows/ci.yml/badge.svg)](https://github.com/xunrua/claude-change/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/xunrua/claude-change)](https://github.com/xunrua/claude-change/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> 一个用于管理 Claude Code 配置的 CLI/TUI 工具。

一键切换 API 提供商、模型和完整配置，无需手动编辑 `~/.claude/settings.json`。

---

## 功能

- **配置管理** — 创建、导入、验证、切换、删除配置
- **TUI 模式** — 交互式终端界面，方向键导航 + 预览
- **自动备份** — 每次切换自动备份，一键回滚
- **API Key 掩码** — 显示时自动隐藏敏感密钥
- **原子写入** — 临时文件 + 重命名，安全文件操作
- **XDG 规范** — 遵循标准配置/数据目录约定
- **Hook 集成** — 可选的 Claude Code hook，检测切换意图
- **Yolo/Safe 模式** — 切换危险操作确认提示

---

## 快速开始

### 安装

```bash
curl -fsSL https://raw.githubusercontent.com/xunrua/claude-change/main/install.sh | bash
```

### 从源码编译

```bash
git clone https://github.com/xunrua/claude-change.git
cd claude-change
cargo build --release
./install.sh
```

---

## 使用方法

### 命令行

```bash
ccp list                        # 列出所有配置
ccp switch <name>               # 切换到指定配置
ccp switch <name> --dry-run     # 预览切换（不修改）
ccp switch <name> --force       # 跳过脏检查
ccp rollback                    # 回滚到上一个配置
ccp import <file> -n <name>     # 导入现有 settings.json
ccp add <name>                  # 交互式创建配置
ccp show <name>                 # 显示配置（密钥已掩码）
ccp validate <name>             # 验证配置
ccp remove <name>               # 删除配置
ccp tui                         # 启动交互界面
ccp hook install                # 安装 Claude Code Hook
ccp hook uninstall              # 卸载 Hook
ccp yolo                        # 开启 yolo 模式
ccp safe                        # 恢复安全模式
```

### TUI 交互界面

运行 `ccp tui` 启动交互界面。使用方向键或 `j/k` 导航，`Enter` 切换，`q` 退出。

### 配置格式

配置以 TOML 文件格式存储：

```toml
name = "kimi"
description = "Kimi 官方 API"

[settings.env]
ANTHROPIC_BASE_URL = "https://api.kimi.com/coding/"
ANTHROPIC_AUTH_TOKEN = "sk-xxx"

[settings]
model = "opus[1m]"
effortLevel = "xhigh"
language = "zh-CN"
```

---

## 配置

### 文件路径

| 数据 | 路径 |
|------|------|
| 配置文件 | `~/.config/claude-profile/profiles/*.toml` |
| 备份 | `~/.local/share/claude-profile/backups/` |
| 活动标记 | `~/.local/share/claude-profile/active_profile` |
| Claude 设置 | `~/.claude/settings.json` |

### 环境变量

| 变量 | 说明 |
|------|------|
| `CLAUDE_PROFILE_BACKUP_COUNT` | 备份保留数量（默认 50） |
| `XDG_CONFIG_HOME` | 覆盖配置目录 |
| `XDG_DATA_HOME` | 覆盖数据目录 |
| `CLAUDE_CONFIG_DIR` | 覆盖 Claude Code 配置路径 |

---

## 从源码编译

前置条件：[Rust](https://rustup.rs) 1.85+

```bash
cargo build --release                          # 包含 TUI
cargo build --release --no-default-features    # 仅 CLI，更小体积
```

---

## 下载

预编译二进制文件可在 [Releases 页面](https://github.com/xunrua/claude-change/releases/latest) 下载。

| 平台 | 架构 |
|------|------|
| macOS | arm64 (Apple Silicon), x86_64 (Intel) |
| Linux | x86_64, aarch64 |

---

## 许可证

[MIT](LICENSE)
