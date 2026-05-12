use ratatui::prelude::*;
use ratatui::widgets::*;

use super::app::App;

/// 主绘制函数
/// 根据当前应用状态渲染整个终端界面
pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // 如果终端太小，显示提示信息
    if area.height < 10 || area.width < 40 {
        draw_too_small(f, area);
        return;
    }

    // 创建主布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题栏
            Constraint::Min(5),    // 主内容区
            Constraint::Length(1), // 状态栏
        ])
        .split(area);

    // 绘制标题栏
    draw_header(f, chunks[0]);

    // 绘制主内容区（列表 + 预览）
    draw_main_content(f, app, chunks[1]);

    // 绘制状态栏
    draw_status_bar(f, app, chunks[2]);
}

/// 当终端太小时显示的提示
fn draw_too_small(f: &mut Frame, area: Rect) {
    let text = Paragraph::new("终端窗口太小\n请调整为至少 40x10")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    f.render_widget(text, area);
}

/// 绘制标题栏
fn draw_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(" Claude Profile Manager ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, area);
}

/// 绘制主内容区
/// 根据是否显示预览面板决定布局
fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    if app.show_preview && !app.profiles.is_empty() {
        // 左右分栏：列表 + 预览
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        draw_profile_list(f, app, chunks[0]);
        draw_preview(f, app, chunks[1]);
    } else {
        // 全宽列表
        draw_profile_list(f, app, area);
    }
}

/// 绘制 profile 列表
fn draw_profile_list(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Profile 列表 ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    if app.profiles.is_empty() {
        // 没有 profile 时显示提示
        let text = Paragraph::new("暂无 profile\n按 'a' 创建新 profile")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .block(block);
        f.render_widget(text, area);
        return;
    }

    // 创建列表项
    let items: Vec<ListItem> = app
        .profiles
        .iter()
        .enumerate()
        .map(|(i, (profile, is_active))| {
            // 激活标记
            let marker = if *is_active { "● " } else { "○ " };

            // 选中项高亮
            let style = if i == app.selected {
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            // 激活的 profile 使用绿色
            let marker_style = if *is_active {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            };

            let content = Line::from(vec![
                Span::styled(marker, marker_style),
                Span::styled(&profile.name, style),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("→ ");

    // 创建有状态的列表并设置选中位置
    let mut state = ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(list, area, &mut state);
}

/// 绘制预览面板
/// 显示当前选中 profile 的详细信息
fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" 预览 ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let content = if let Some((profile, is_active)) = app.profiles.get(app.selected) {
        let mut lines = vec![];

        // Profile 名称和状态
        let status = if *is_active {
            Span::styled(" [已激活]", Style::default().fg(Color::Green))
        } else {
            Span::styled(" [未激活]", Style::default().fg(Color::Gray))
        };
        lines.push(Line::from(vec![
            Span::styled("名称: ", Style::default().fg(Color::Cyan)),
            Span::styled(&profile.name, Style::default().add_modifier(Modifier::BOLD)),
            status,
        ]));

        // 描述
        if let Some(desc) = &profile.description {
            lines.push(Line::from(vec![
                Span::styled("描述: ", Style::default().fg(Color::Cyan)),
                Span::raw(desc),
            ]));
        }

        lines.push(Line::from(""));

        // 模型
        if let Some(model) = &profile.settings.model {
            lines.push(Line::from(vec![
                Span::styled("模型: ", Style::default().fg(Color::Cyan)),
                Span::raw(model),
            ]));
        }

        // Effort Level
        if let Some(level) = &profile.settings.effort_level {
            lines.push(Line::from(vec![
                Span::styled("Effort: ", Style::default().fg(Color::Cyan)),
                Span::raw(level),
            ]));
        }

        // 语言
        if let Some(lang) = &profile.settings.language {
            lines.push(Line::from(vec![
                Span::styled("语言: ", Style::default().fg(Color::Cyan)),
                Span::raw(lang),
            ]));
        }

        lines.push(Line::from(""));

        // 环境变量
        if let Some(env) = &profile.settings.env {
            lines.push(Line::styled("环境变量:", Style::default().fg(Color::Cyan)));

            for (key, value) in env {
                // API key 掩码显示
                let display_value = if key.contains("TOKEN") || key.contains("KEY") {
                    crate::profile::mask_api_key(value)
                } else {
                    value.clone()
                };

                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(key, Style::default().fg(Color::Yellow)),
                    Span::raw(" = "),
                    Span::raw(display_value),
                ]));
            }
        }

        lines
    } else {
        vec![Line::styled("未选择 profile", Style::default().fg(Color::Gray))]
    };

    let paragraph = Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .block(block);

    f.render_widget(paragraph, area);
}

/// 绘制底部状态栏
fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = Paragraph::new(app.status_message.as_str())
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray),
        );
    f.render_widget(status, area);
}
