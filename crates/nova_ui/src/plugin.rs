//! Nova UI 插件

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// Nova UI 插件
///
/// 封装 egui 即时模式 UI 系统
pub struct NovaUiPlugin;

impl Plugin for NovaUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
    }
}
