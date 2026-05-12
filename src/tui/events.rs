use crate::error::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use super::app::App;

/// 处理用户输入事件
/// 返回 true 表示用户请求退出应用程序
pub fn handle_events(app: &mut App) -> Result<bool> {
    // 使用 100ms 超时，使界面保持响应同时不阻塞
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
    {
        // 只处理按键按下事件，忽略释放事件
        if key.kind != KeyEventKind::Press {
            return Ok(false);
        }

        // 如果有确认对话框显示，优先处理确认相关按键
        if app.show_confirm {
            return handle_confirm_dialog(app, key.code);
        }

        // 处理常规导航按键
        return handle_navigation(app, key.code);
    }

    Ok(false)
}

/// 处理确认对话框的按键
fn handle_confirm_dialog(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        // 确认切换
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Err(e) = app.switch_selected() {
                app.status_message = format!("切换失败: {}", e);
                app.show_confirm = false;
            }
        }
        // 取消切换
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel_confirm();
        }
        _ => {}
    }
    Ok(false)
}

/// 处理常规导航按键
fn handle_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        // 退出应用程序
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),

        // 向下导航
        KeyCode::Down | KeyCode::Char('j') => app.next(),

        // 向上导航
        KeyCode::Up | KeyCode::Char('k') => app.prev(),

        // 确认切换（打开确认对话框）
        KeyCode::Enter => {
            if app.is_selected_active() {
                app.status_message = "当前 profile 已激活".to_string();
            } else {
                app.request_confirm();
            }
        }

        // 切换预览面板
        KeyCode::Char('p') => app.toggle_preview(),

        // 添加新 profile
        KeyCode::Char('a') => {
            if let Err(e) = app.add_profile() {
                app.status_message = format!("创建失败: {}", e);
            }
        }

        // 首页
        KeyCode::Home => app.selected = 0,

        // 末页
        KeyCode::End => {
            if !app.profiles.is_empty() {
                app.selected = app.profiles.len() - 1;
            }
        }

        _ => {}
    }

    Ok(false)
}
