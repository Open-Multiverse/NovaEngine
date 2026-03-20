//! 程序化待机动画 - 让静止单位看起来"活着"

use bevy::prelude::*;

/// 程序化待机动画组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct ProceduralIdle {
    pub enabled: bool,
    /// 摇摆幅度（弧度）
    pub sway_amplitude: f32,
    /// 摇摆速度
    pub sway_speed: f32,
    /// 呼吸缩放幅度
    pub breathe_scale: f32,
    /// 相位偏移（避免所有单位同步）
    pub phase: f32,
}

impl ProceduralIdle {
    pub fn new_with_phase(phase: f32) -> Self {
        Self {
            enabled: true,
            sway_amplitude: 0.05,
            sway_speed: 1.2,
            breathe_scale: 0.02,
            phase,
        }
    }
}

impl Default for ProceduralIdle {
    fn default() -> Self {
        Self::new_with_phase(0.0)
    }
}

/// 程序化待机动画系统
pub fn procedural_idle_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ProceduralIdle)>,
) {
    let t = time.elapsed_secs();

    for (mut transform, idle) in query.iter_mut() {
        if !idle.enabled {
            continue;
        }

        let phase = idle.phase;
        let sway = (t * idle.sway_speed + phase).sin() * idle.sway_amplitude;
        let breathe = 1.0 + (t * idle.sway_speed * 0.7 + phase).sin() * idle.breathe_scale;

        transform.rotation = Quat::from_rotation_z(sway);
        transform.scale = Vec3::splat(breathe);
    }
}
