#!/bin/bash
# claude-profile one-click installer
# 一键安装脚本
#
# Usage / 用法:
#   curl -fsSL https://raw.githubusercontent.com/xunrua/claude-change/main/install.sh | bash
#
# Environment variables / 环境变量:
#   INSTALL_DIR  - Installation directory (default: ~/.local/bin)

set -e

REPO="xunrua/claude-change"
BINARY="claude-profile"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect platform / 检测平台
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
    Darwin-arm64)  ASSET="claude-profile-aarch64-macos" ;;
    Darwin-x86_64) ASSET="claude-profile-x86_64-macos" ;;
    Linux-x86_64)  ASSET="claude-profile-x86_64-linux" ;;
    Linux-aarch64) ASSET="claude-profile-aarch64-linux" ;;
    *)
        echo "Unsupported platform: $OS-$ARCH"
        echo "不支持的平台: $OS-$ARCH"
        echo "Falling back to source build... / 回退到源码编译..."
        build_from_source
        exit $?
        ;;
esac

# Try binary download first / 优先尝试下载预编译二进制
if command -v curl &> /dev/null; then
    echo "=== Installing claude-profile / 安装 claude-profile ==="

    # Get latest release version / 获取最新版本
    LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        2>/dev/null | grep '"tag_name"' | head -1 | sed 's/.*"v\(.*\)".*/\1/')

    if [ -n "$LATEST" ]; then
        echo "Latest version / 最新版本: v$LATEST"
        URL="https://github.com/$REPO/releases/download/v$LATEST/$ASSET.tar.gz"

        # Download and extract / 下载并解压
        TMPDIR=$(mktemp -d)
        trap 'rm -rf "$TMPDIR"' EXIT

        echo "Downloading / 下载中: $URL"
        if curl -fsSL "$URL" | tar xz -C "$TMPDIR" 2>/dev/null; then
            mkdir -p "$INSTALL_DIR"
            cp "$TMPDIR/$BINARY" "$INSTALL_DIR/"
            chmod +x "$INSTALL_DIR/$BINARY"
            ln -sf "$INSTALL_DIR/$BINARY" "$INSTALL_DIR/ccp"

            echo ""
            echo "Installed / 安装完成: $INSTALL_DIR/$BINARY (v$LATEST)"
            echo "Alias / 别名: $INSTALL_DIR/ccp"
            check_path
            exit 0
        else
            echo "Binary download failed, falling back to source build..."
            echo "下载失败，回退到源码编译..."
        fi
    fi
fi

# Fallback: build from source / 回退：从源码编译
build_from_source() {
    if ! command -v cargo &> /dev/null; then
        echo "Error: Rust/Cargo not found / 错误: 未找到 Rust/Cargo"
        echo "Install Rust: https://rustup.rs"
        exit 1
    fi

    echo "Building from source / 从源码编译..."
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

    if [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
        cd "$SCRIPT_DIR"
    else
        # Clone if running standalone / 独立运行时克隆仓库
        CLONE_DIR=$(mktemp -d)
        trap 'rm -rf "$CLONE_DIR"' EXIT
        git clone "https://github.com/$REPO.git" "$CLONE_DIR"
        cd "$CLONE_DIR"
    fi

    cargo build --release --quiet

    mkdir -p "$INSTALL_DIR"
    cp target/release/$BINARY "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY"
    ln -sf "$INSTALL_DIR/$BINARY" "$INSTALL_DIR/ccp"

    echo ""
    echo "Installed / 安装完成: $INSTALL_DIR/$BINARY"
    echo "Alias / 别名: $INSTALL_DIR/ccp"
    check_path
}

check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo "Note: $INSTALL_DIR is not in PATH"
        echo "提示: $INSTALL_DIR 不在 PATH 中"
        echo "Add to shell config / 添加到 shell 配置:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
}

build_from_source
