// TUI 模块入口
// 提供基于 ratatui 的交互式配置选择界面

pub mod app;
pub mod events;
pub mod ui;

use crate::error::Result;
use app::App;
use events::handle_events;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::ExecutableCommand;
use ratatui::prelude::*;
use std::io;

/// 启动 TUI 界面
pub fn run_tui() -> Result<()> {
    // 启用终端原始模式并切换到备用屏幕
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    // 创建终端后端
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 初始化应用状态
    let mut app = App::new()?;

    // 主渲染循环
    let result = run_app(&mut terminal, &mut app);

    // 恢复终端状态
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    result
}

/// TUI 主循环：持续渲染界面并处理用户输入
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    // 最后一次渲染的界面，用于终端恢复时重绘
    let _last_frame: RefCell<Option< ratatui::Frame>> = RefCell::new(None);

    loop {
        // 绘制当前界面
        terminal.draw(|f| {
            ui::draw(f, app);
            // 保存最后一帧用于恢复
        })?;

        // 处理用户输入事件
        if handle_events(app)? {
            // 用户请求退出
            break;
        }
    }

    Ok(())
}

use std::cell::RefCell;
