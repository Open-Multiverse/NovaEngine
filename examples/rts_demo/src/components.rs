//! 游戏组件定义

use bevy::prelude::*;

/// 单位标记
#[derive(Component)]
pub struct Unit;

/// 阵营
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Player,
    Enemy,
}

/// 生命值
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// 攻击能力
#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    pub timer: f32,
}

impl Attack {
    pub fn new(damage: f32, range: f32, cooldown: f32) -> Self {
        Self {
            damage,
            range,
            cooldown,
            timer: 0.0,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn reset_cooldown(&mut self) {
        self.timer = self.cooldown;
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer = (self.timer - delta).max(0.0);
    }
}

/// 移动能力
#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

/// 可选中标记
#[derive(Component)]
pub struct Selectable;

/// 当前被选中
#[derive(Component)]
pub struct Selected;

/// 攻击目标
#[derive(Component)]
pub struct AttackTarget(pub Entity);

/// 资源点（预留扩展）
#[allow(dead_code)]
#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: u32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Crystal,
    Gas,
}

/// 游戏模式
#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameMode {
    #[default]
    Playing,
    #[allow(dead_code)]
    Editor,
}
