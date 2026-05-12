# Contributing to claude-profile

## Development Setup / 开发环境

1. Install Rust 1.85+ via [rustup](https://rustup.rs)
2. Clone the repo / 克隆仓库
3. Run tests / 运行测试: `cargo test --all-features`
4. Run lints / 运行检查: `cargo fmt && cargo clippy --all-targets --all-features -- -D warnings`

## Architecture / 架构

| File | Description / 描述 |
|------|------|
| `src/cli.rs` | CLI argument parsing and command dispatch / 命令行参数解析和命令分发 |
| `src/switcher.rs` | Core profile switching logic / 核心切换逻辑 |
| `src/profile.rs` | Profile TOML serialization and management / 配置文件序列化管理 |
| `src/config.rs` | Path resolution (XDG) and atomic file writes / 路径解析和原子写入 |
| `src/settings.rs` | Claude settings.json schema (serde) / Claude 设置文件模型 |
| `src/backup.rs` | Backup creation, rollback, pruning / 备份创建、回滚、清理 |
| `src/validation.rs` | URL and API key validation / URL 和 API Key 验证 |
| `src/hook.rs` | Claude Code hook management / Hook 管理 |
| `src/tui/` | Terminal UI (feature-gated) / 终端交互界面 |
| `tests/integration.rs` | End-to-end tests / 端到端测试 |

## Making Changes / 提交修改

1. Create a feature branch / 创建功能分支
2. Make changes with tests / 编写代码和测试
3. Ensure all lints pass / 确保所有检查通过: `cargo fmt --check && cargo clippy -- -D warnings`
4. Update `CHANGELOG.md` under `[Unreleased]` / 在 `[Unreleased]` 下更新变更日志
5. Open a pull request / 提交 Pull Request

## Release Process / 发布流程

1. Update `version` in `Cargo.toml` / 更新 `Cargo.toml` 中的版本号
2. Move `[Unreleased]` entries to `[X.Y.Z] - YYYY-MM-DD` in `CHANGELOG.md`
3. Commit: `git commit -m "chore: release vX.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push origin main --tags`
6. CI builds binaries and creates GitHub Release automatically / CI 自动构建并创建 GitHub Release

## Code Style / 代码风格

- Follow `rustfmt.toml` configuration / 遵循 `rustfmt.toml` 配置
- No unnecessary comments / 不添加不必要的注释
- Error handling via `thiserror` for library errors, `anyhow` for CLI errors
- Chinese comments for user-facing code, English for internal utilities
