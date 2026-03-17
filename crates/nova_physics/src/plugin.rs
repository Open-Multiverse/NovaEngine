//! Nova 物理插件

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Nova 物理插件
///
/// 封装 Rapier 3D 物理引擎
pub struct NovaPhysicsPlugin;

impl Plugin for NovaPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<()>::default())
            .add_plugins(RapierDebugRenderPlugin::default().disabled());
    }
}

/// 启用物理调试渲染
pub fn enable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = true;
}

/// 禁用物理调试渲染
pub fn disable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = false;
}
