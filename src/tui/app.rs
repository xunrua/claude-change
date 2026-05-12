use crate::error::Result;
use crate::profile::Profile;
use crate::switcher::Switcher;

/// TUI 应用状态
/// 管理 profile 列表、当前选中项、滚动位置等状态
pub struct App {
    /// 配置切换器，用于操作 profile
    pub switcher: Switcher,
    /// 所有 profile 列表，每个元素包含 profile 和是否激活的标记
    pub profiles: Vec<(Profile, bool)>,
    /// 当前选中项的索引
    pub selected: usize,
    /// 列表滚动偏移量（处理列表比屏幕高的情况）
    pub scroll_offset: usize,
    /// 当前显示的状态信息
    pub status_message: String,
    /// 是否显示确认对话框
    pub show_confirm: bool,
    /// 是否显示预览面板
    pub show_preview: bool,
}

impl App {
    /// 创建新的 TUI 应用状态
    /// 加载所有 profile 并初始化界面状态
    pub fn new() -> Result<Self> {
        let switcher = Switcher::new()?;
        let profiles = switcher.list_profiles()?;

        // 如果没有 profile，显示提示信息
        let status_message = if profiles.is_empty() {
            "按 'a' 添加 profile | 按 'q' 退出".to_string()
        } else {
            "↑↓ 导航 | Enter 切换 | p 预览 | q 退出".to_string()
        };

        Ok(Self {
            switcher,
            profiles,
            selected: 0,
            scroll_offset: 0,
            status_message,
            show_confirm: false,
            show_preview: true,
        })
    }

    /// 获取当前选中的 profile
    pub fn selected_profile(&self) -> Option<&Profile> {
        self.profiles.get(self.selected).map(|(p, _)| p)
    }

    /// 获取当前选中 profile 是否已激活
    pub fn is_selected_active(&self) -> bool {
        self.profiles.get(self.selected).map(|(_, active)| *active).unwrap_or(false)
    }

    /// 向下移动选中项
    pub fn next(&mut self) {
        if self.profiles.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.profiles.len();
        self.status_message.clear();
    }

    /// 向上移动选中项
    pub fn prev(&mut self) {
        if self.profiles.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.profiles.len() - 1;
        } else {
            self.selected -= 1;
        }
        self.status_message.clear();
    }

    /// 切换到当前选中的 profile
    /// 执行实际的配置切换操作
    pub fn switch_selected(&mut self) -> Result<()> {
        if let Some((profile, _)) = self.profiles.get(self.selected) {
            let name = profile.name.clone();
            self.switcher.switch_to(&name, false, false)?;

            // 刷新列表以更新激活标记
            self.profiles = self.switcher.list_profiles()?;
            self.status_message = format!("已切换到: {}", name);
            self.show_confirm = false;
        }
        Ok(())
    }

    /// 显示确认对话框，请求用户确认切换
    pub fn request_confirm(&mut self) {
        if !self.profiles.is_empty() && !self.is_selected_active() {
            self.show_confirm = true;
            if let Some(profile) = self.selected_profile() {
                self.status_message = format!("确认切换到 '{}'? (y/n)", profile.name);
            }
        }
    }

    /// 取消确认对话框
    pub fn cancel_confirm(&mut self) {
        self.show_confirm = false;
        self.status_message = "↑↓ 导航 | Enter 切换 | p 预览 | q 退出".to_string();
    }

    /// 切换预览面板的显示/隐藏
    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    /// 添加一个新的 profile（交互式）
    pub fn add_profile(&mut self) -> Result<()> {
        // 生成默认名称
        let name = format!("profile{}", self.profiles.len() + 1);

        let profile = Profile {
            name: name.clone(),
            description: Some("新 profile".to_string()),
            settings: Default::default(),
        };

        let profile_path = Profile::profile_path(&self.switcher.paths.profiles_dir, &name);
        profile.save(&profile_path)?;

        // 刷新列表
        self.profiles = self.switcher.list_profiles()?;
        // 选中新创建的 profile
        self.selected = self.profiles.len().saturating_sub(1);
        self.status_message = format!("已创建 profile: {}", name);

        Ok(())
    }
}
