# 更新日志

中文 | [**English**](CHANGELOG.md)

所有重要变更都会记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/)，版本遵循[语义化版本](https://semver.org/)。

## [0.1.0] - 2025-05-12

### 新增
- 核心配置管理：list、switch、rollback、import、validate、show、add、remove
- TUI 模式，支持配置列表和预览面板（feature gate，基于 ratatui）
- Claude Code Hook 集成（安装/卸载）
- Yolo/Safe 模式切换，控制危险操作确认提示
- 原子文件写入，自动备份，保留最近 50 个备份
- API Key 显示时自动掩码
- URL 和 API Key 格式验证
- XDG 规范的配置路径
- switch 命令支持 `--dry-run` 和 `--force` 参数
- 交互式引导创建配置
- 短别名 `ccp`
- 一键安装脚本，支持二进制下载 + 源码编译回退
- 跨平台 CI（macOS arm64/x86_64、Linux x86_64/aarch64）
- 自动化 GitHub Release 工作流，上传编译产物
