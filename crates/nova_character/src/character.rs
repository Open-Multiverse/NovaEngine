//! 角色标识与类型定义

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 角色唯一标识
#[derive(
    Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize,
)]
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
