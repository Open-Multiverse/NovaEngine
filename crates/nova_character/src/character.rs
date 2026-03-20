//! 角色标识与类型定义

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::attributes::{Attributes, Health};
use crate::state::CharacterState;

/// 角色唯一标识
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct CharacterId(pub u64);

/// 角色类型
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum CharacterType {
    Infantry,    // 步兵
    Archer,      // 弓箭手
    Mage,        // 法师
    Knight,      // 骑士
    Custom(u32), // 自定义
}

impl Default for CharacterType {
    fn default() -> Self {
        Self::Infantry
    }
}

/// 角色标识组件
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub character_type: CharacterType,
}

impl Character {
    pub fn new(id: u64, name: impl Into<String>, character_type: CharacterType) -> Self {
        Self {
            id: CharacterId(id),
            name: name.into(),
            character_type,
        }
    }
}

/// 阵营
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum Faction {
    Player,
    Enemy,
    Neutral,
}

/// 角色数值统计（Attributes 的 builder 包装）
#[derive(Component, Clone, Debug, Default, Reflect, Serialize, Deserialize)]
pub struct CharacterStats {
    pub name: String,
    pub attributes: Attributes,
}

impl CharacterStats {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_health(mut self, hp: f32) -> Self {
        self.attributes.health = Health::new(hp);
        self
    }

    pub fn with_attack(mut self, val: f32) -> Self {
        self.attributes.attack = val;
        self
    }

    pub fn with_defense(mut self, val: f32) -> Self {
        self.attributes.defense = val;
        self
    }

    /// 委托访问器（满足集成测试断言）
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn max_health(&self) -> f32 {
        self.attributes.health.max
    }

    pub fn attack(&self) -> f32 {
        self.attributes.attack
    }

    pub fn defense(&self) -> f32 {
        self.attributes.defense
    }
}

/// 角色完整 ECS Bundle（用于 world.spawn）
#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub stats: CharacterStats,
    pub state: CharacterState,
    pub transform: Transform,
    pub visibility: Visibility,
}
