//! Nova UI 插件

use bevy::prelude::*;
use bevy_egui::egui::{FontData, FontDefinitions, FontFamily};
use bevy_egui::{EguiContexts, EguiPlugin};

/// Nova UI 插件
///
/// 封装 egui 即时模式 UI 系统，支持中文显示
pub struct NovaUiPlugin;

impl Plugin for NovaUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, setup_chinese_fonts);
    }
}

/// 配置中文字体
fn setup_chinese_fonts(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();

    // 使用思源黑体简体中文子集
    fonts.font_data.insert(
        "source_han_sans_cn".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/SourceHanSansCN-Regular.otf")),
    );

    // 将中文字体添加到默认字体族的最前面
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "source_han_sans_cn".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "source_han_sans_cn".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}
