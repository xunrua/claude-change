# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2025-05-12

### Added
- Core profile management: list, switch, rollback, import, validate, show, add, remove
- TUI mode with profile list and preview panel (feature-gated, ratatui)
- Claude Code hook integration (install/uninstall)
- Yolo/safe mode toggle for dangerous operation prompts
- Atomic file writes with automatic backup and retention (50 backups)
- API key masking in display output
- URL and API key format validation
- XDG-compliant configuration paths
- `--dry-run` and `--force` flags for switch command
- Interactive profile creation with guided prompts
- Short alias `ccp`
- One-click install script with binary download and source build fallback
- Cross-platform CI (macOS arm64/x86_64, Linux x86_64/aarch64)
- Automated GitHub Release workflow with binary uploads
- Chinese/English bilingual README
