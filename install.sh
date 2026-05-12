#!/bin/bash
# claude-profile 一键安装脚本
# 用法: ./install.sh

set -e

echo "=== claude-profile 安装 ==="

# 检查 Rust 环境
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 Rust/Cargo，请先安装 Rust: https://rustup.rs"
    exit 1
fi

# 获取安装目标目录
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# 构建 Release 版本
echo "正在编译..."
cargo build --release --quiet

# 确保目标目录存在
mkdir -p "$INSTALL_DIR"

# 复制二进制文件
cp target/release/claude-profile "$INSTALL_DIR/"

# 设置执行权限
chmod +x "$INSTALL_DIR/claude-profile"

# 创建短别名 ccp
ln -sf "$INSTALL_DIR/claude-profile" "$INSTALL_DIR/ccp"

echo ""
echo "安装完成:"
echo "  $INSTALL_DIR/claude-profile"
echo "  $INSTALL_DIR/ccp (短别名)"
echo ""

# 检查是否在 PATH 中
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "提示: $INSTALL_DIR 不在 PATH 中"
    echo "请添加以下行到 ~/.zshrc 或 ~/.bashrc:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
    echo "或者临时生效:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
else
    echo "ccp 已可用，运行 'ccp --help' 查看用法"
fi
