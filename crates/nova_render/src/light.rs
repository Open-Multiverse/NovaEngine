//! 灯光系统

use bevy::prelude::*;

/// 环境光配置
#[derive(Resource, Debug, Clone)]
pub struct AmbientLightConfig {
    pub color: Color,
    pub brightness: f32,
}

impl Default for AmbientLightConfig {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            brightness: 0.2,
        }
    }
}

/// 创建方向光的辅助函数
pub fn spawn_directional_light(
    commands: &mut Commands,
    direction: Vec3,
    color: Color,
    intensity: f32,
) {
    commands.spawn((
        DirectionalLight {
            color,
            illuminance: intensity,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize())),
    ));
}

/// 创建点光源的辅助函数
pub fn spawn_point_light(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    intensity: f32,
    range: f32,
) {
    commands.spawn((
        PointLight {
            color,
            intensity,
            range,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(position),
    ));
}
