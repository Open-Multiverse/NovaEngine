//! 属性系统 - 角色数值数据

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 生命值
#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
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

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 { self.current / self.max } else { 0.0 }
    }
}

/// 角色属性组件
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Attributes {
    pub health: Health,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,   // 攻击间隔（秒）
    pub vision_range: f32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            health: Health::new(100.0),
            attack: 10.0,
            defense: 5.0,
            move_speed: 5.0,
            attack_range: 2.0,
            attack_speed: 1.0,
            vision_range: 10.0,
        }
    }
}

impl Attributes {
    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
    }
}
