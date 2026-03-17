//! UI 上下文管理
//!
//! 提供 egui 上下文的便捷访问和管理

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// UI 主题配置
#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    /// 主色调
    pub primary_color: egui::Color32,
    /// 背景色
    pub background_color: egui::Color32,
    /// 文本颜色
    pub text_color: egui::Color32,
    /// 字体大小
    pub font_size: f32,
    /// 窗口圆角
    pub window_rounding: f32,
    /// 按钮圆角
    pub button_rounding: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            primary_color: egui::Color32::from_rgb(66, 135, 245),
            background_color: egui::Color32::from_rgba_unmultiplied(30, 30, 40, 240),
            text_color: egui::Color32::from_rgb(220, 220, 220),
            font_size: 14.0,
            window_rounding: 8.0,
            button_rounding: 4.0,
        }
    }
}

impl UiTheme {
    /// 深色主题
    pub fn dark() -> Self {
        Self::default()
    }

    /// 浅色主题
    pub fn light() -> Self {
        Self {
            primary_color: egui::Color32::from_rgb(25, 118, 210),
            background_color: egui::Color32::from_rgba_unmultiplied(245, 245, 245, 250),
            text_color: egui::Color32::from_rgb(33, 33, 33),
            ..default()
        }
    }

    /// 应用主题到 egui 上下文
    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // 设置颜色
        style.visuals.widgets.noninteractive.bg_fill = self.background_color;
        style.visuals.widgets.inactive.bg_fill = self.background_color;
        style.visuals.widgets.hovered.bg_fill = self.primary_color;
        style.visuals.widgets.active.bg_fill = self.primary_color;

        // 设置圆角
        style.visuals.window_rounding = egui::Rounding::same(self.window_rounding);
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(self.button_rounding);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(self.button_rounding);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(self.button_rounding);
        style.visuals.widgets.active.rounding = egui::Rounding::same(self.button_rounding);

        ctx.set_style(style);
    }
}

/// UI 状态资源
#[derive(Resource, Debug, Default)]
pub struct UiState {
    /// 是否显示调试面板
    pub show_debug_panel: bool,
    /// 是否显示性能统计
    pub show_fps: bool,
    /// 当前悬停的 UI 元素
    pub hovered_element: Option<String>,
}

/// 应用 UI 主题系统
pub fn apply_ui_theme(mut contexts: EguiContexts, theme: Res<UiTheme>) {
    if theme.is_changed() {
        theme.apply(contexts.ctx_mut());
    }
}

/// UI 上下文扩展 trait
pub trait UiContextExt {
    /// 快速创建窗口
    fn nova_window(&self, title: &str) -> egui::Window<'_>;

    /// 快速创建面板
    fn nova_panel(&self, id: impl Into<egui::Id>) -> egui::SidePanel;
}

impl UiContextExt for egui::Context {
    fn nova_window(&self, title: &str) -> egui::Window<'_> {
        egui::Window::new(title).resizable(true).collapsible(true)
    }

    fn nova_panel(&self, id: impl Into<egui::Id>) -> egui::SidePanel {
        egui::SidePanel::left(id).resizable(true)
    }
}
