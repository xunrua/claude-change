// Hook 管理模块
// 负责安装和卸载 Claude Code 的 UserPromptSubmit hook

use crate::error::Result;
use std::path::Path;

/// Hook 脚本文件名
const HOOK_SCRIPT_NAME: &str = "claude-profile-hook.mjs";

/// 安装 hook
/// 将 hook 脚本复制到 Claude Code hooks 目录
pub fn install_hook(claude_dir: &Path) -> Result<()> {
    let hooks_dir = claude_dir.join("hooks");
    std::fs::create_dir_all(&hooks_dir)?;

    let script_path = hooks_dir.join(HOOK_SCRIPT_NAME);

    // 写入 hook 脚本
    std::fs::write(&script_path, HOOK_SCRIPT_CONTENT)?;

    // 设置执行权限（Unix 系统）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
    }

    println!("Hook 脚本已安装: {}", script_path.display());
    println!("请在 Claude Code 中输入 'switch to <profile>' 来测试");

    Ok(())
}

/// 卸载 hook
/// 从 Claude Code hooks 目录移除脚本
pub fn uninstall_hook(claude_dir: &Path) -> Result<()> {
    let script_path = claude_dir.join("hooks").join(HOOK_SCRIPT_NAME);

    if script_path.exists() {
        std::fs::remove_file(&script_path)?;
        println!("Hook 脚本已移除: {}", script_path.display());
    } else {
        println!("Hook 脚本不存在，无需卸载");
    }

    Ok(())
}

/// Hook 脚本内容
/// 这是一个 Node.js 脚本，由 Claude Code 的 UserPromptSubmit hook 调用
/// 功能是检测用户输入中的切换意图，并返回提示信息
const HOOK_SCRIPT_CONTENT: &str = r#"#!/usr/bin/env node
/**
 * Claude Profile Hook
 * 检测用户输入中的 profile 切换意图，并提示用户使用 CLI 命令
 *
 * 支持的触发语：
 *   - "switch to <profile>"
 *   - "use <profile> profile"
 *   - "activate <profile>"
 */

const SWITCH_PATTERNS = [
    /switch\s+(?:to\s+)?(\w+)(?:\s+profile)?/i,
    /use\s+(\w+)\s+(?:profile\s+)?(?:for\s+)?(?:claude\s+)?code/i,
    /activate\s+(\w+)(?:\s+profile)?/i,
];

async function main() {
    // 从 stdin 读取 Claude Code 的 prompt 文本
    let input = '';
    for await (const chunk of process.stdin) {
        input += chunk;
    }

    // 检测切换意图
    for (const pattern of SWITCH_PATTERNS) {
        const match = input.match(pattern);
        if (match) {
            const profile = match[1];

            // 返回提示信息（不修改 settings.json）
            console.error(`\n[claude-profile] 检测到切换意图: "${profile}"`);
            console.error(`\n  请运行: claude-profile switch ${profile}`);
            console.error(`  然后使用 /exit 退出 Claude Code，重新启动以生效。\n`);

            // 在原始 prompt 后附加提示
            process.stdout.write(
                input + `\n\n[提示: 使用 "claude-profile switch ${profile}" 切换配置，然后 /exit 重启]`
            );
            return;
        }
    }

    // 未检测到切换意图，原样输出
    process.stdout.write(input);
}

main().catch(err => {
    console.error('[claude-profile hook error]', err);
    process.exit(1);
});
"#;
