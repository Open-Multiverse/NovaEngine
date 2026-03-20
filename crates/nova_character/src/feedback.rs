//! 视觉反馈系统 - 伤害数字、受击闪烁、生命条、状态图标

use bevy::prelude::*;

/// 伤害数字组件
#[derive(Component, Clone, Debug)]
pub struct DamageNumber {
    pub value: f32,
    pub is_crit: bool,
    pub lifetime: f32,       // 剩余显示时间（秒）
    pub velocity: Vec3,      // 漂浮速度
}

/// 生成伤害数字事件
#[derive(Event, Clone, Debug)]
pub struct SpawnDamageNumber {
    pub position: Vec3,
    pub damage: f32,
    pub is_crit: bool,
}

/// 受击闪烁组件
#[derive(Component, Clone, Debug)]
pub struct HitFlash {
    pub timer: f32,           // 剩余闪烁时间
    pub total: f32,           // 总闪烁时间
}

impl HitFlash {
    pub fn new(duration: f32) -> Self {
        Self { timer: duration, total: duration }
    }
}

/// 触发受击闪烁事件
#[derive(Event, Clone, Debug)]
pub struct TriggerHitFlash {
    pub entity: Entity,
    pub duration: f32,
}

/// 头顶生命条组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct HealthBar {
    pub width: f32,
    pub height: f32,
    pub offset: Vec3,        // 相对实体的偏移
    pub show_when_full: bool,
    pub ally_color: Color,
    pub enemy_color: Color,
}

impl Default for HealthBar {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 0.1,
            offset: Vec3::new(0.0, 1.2, 0.0),
            show_when_full: false,
            ally_color: Color::srgb(0.2, 0.8, 0.2),
            enemy_color: Color::srgb(0.9, 0.2, 0.2),
        }
    }
}

/// 状态图标类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusIconType {
    Moving,
    Attacking,
    Stunned,
    Slowed,
    Enraged,
    Fearful,
}

/// 头顶状态图标组件
#[derive(Component, Clone, Debug)]
pub struct StatusIndicator {
    pub icons: Vec<StatusIconType>,
    pub offset: Vec3,
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self {
            icons: vec![],
            offset: Vec3::new(0.0, 1.5, 0.0),
        }
    }
}

/// 单位死亡事件
#[derive(Event, Clone, Debug)]
pub struct UnitDiedEvent {
    pub entity: Entity,
    pub position: Vec3,
}

/// 伤害数字漂浮和销毁系统
pub fn damage_number_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut num, mut transform) in query.iter_mut() {
        num.lifetime -= time.delta_secs();
        transform.translation += num.velocity * time.delta_secs();

        if num.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// 受击闪烁系统
pub fn hit_flash_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash)>,
    mut commands: Commands,
) {
    for (entity, mut flash) in query.iter_mut() {
        flash.timer -= time.delta_secs();
        if flash.timer <= 0.0 {
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}
