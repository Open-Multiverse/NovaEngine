//! 角色状态机 - 描述角色当前在做什么

use bevy::prelude::*;

/// 角色状态
#[derive(Component, Clone, Debug, Default, Reflect, PartialEq)]
pub enum CharacterState {
    /// 待机
    #[default]
    Idle,
    /// 移动中
    Moving { target: Vec3 },
    /// 攻击中
    Attacking { target: Entity },
    /// 眩晕
    Stunned { remaining: f32 },
    /// 死亡
    Dead,
}

impl CharacterState {
    pub fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    pub fn is_moving(&self) -> bool {
        matches!(self, Self::Moving { .. })
    }

    pub fn is_attacking(&self) -> bool {
        matches!(self, Self::Attacking { .. })
    }

    pub fn is_stunned(&self) -> bool {
        matches!(self, Self::Stunned { .. })
    }
}

/// 攻击冷却计时器
#[derive(Component, Clone, Debug, Reflect)]
pub struct AttackCooldown {
    pub timer: f32,
    pub max: f32,
}

impl AttackCooldown {
    pub fn new(interval: f32) -> Self {
        Self { timer: 0.0, max: interval }
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer = (self.timer - delta).max(0.0);
    }

    pub fn can_attack(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn reset(&mut self) {
        self.timer = self.max;
    }
}

/// 眩晕倒计时系统
pub fn stun_tick_system(time: Res<Time>, mut query: Query<&mut CharacterState>) {
    for mut state in query.iter_mut() {
        if let CharacterState::Stunned { remaining } = state.as_mut() {
            *remaining -= time.delta_secs();
            if *remaining <= 0.0 {
                *state = CharacterState::Idle;
            }
        }
    }
}
