//! Nova 插件系统
//!
//! 提供 Nova 引擎的插件接口和默认插件

use bevy::prelude::*;

/// Nova 默认插件组
///
/// 包含运行 Nova 应用所需的所有基础插件
pub struct NovaDefaultPlugins;

impl Plugin for NovaDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Nova Engine".into(),
                        canvas: Some("#nova-canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        );
    }
}

/// Nova 最小插件组
///
/// 仅包含核心功能，不包含渲染相关插件
pub struct NovaMinimalPlugins;

impl Plugin for NovaMinimalPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
    }
}

/// Nova 核心插件
///
/// 提供 Nova 引擎核心功能
pub struct NovaCorePlugin;

impl Plugin for NovaCorePlugin {
    fn build(&self, app: &mut App) {
        // 注册核心资源
        app.init_resource::<crate::components::GameTime>();

        // 添加核心系统
        app.add_systems(Update, update_game_time);
    }
}

/// 更新游戏时间资源
fn update_game_time(time: Res<Time>, mut game_time: ResMut<crate::components::GameTime>) {
    game_time.delta = time.delta_secs();
    game_time.elapsed += game_time.delta * game_time.scale;
}
