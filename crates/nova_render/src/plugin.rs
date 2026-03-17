//! Nova 渲染插件

use bevy::prelude::*;

use crate::light::AmbientLightConfig;

/// Nova 渲染插件
///
/// 提供默认的渲染配置和环境光设置
pub struct NovaRenderPlugin;

impl Plugin for NovaRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AmbientLightConfig>()
            .add_systems(Startup, setup_ambient_light);
    }
}

fn setup_ambient_light(mut commands: Commands, config: Res<AmbientLightConfig>) {
    commands.insert_resource(AmbientLight {
        color: config.color,
        brightness: config.brightness,
    });
}
