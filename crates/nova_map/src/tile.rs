//! 瓦片和地形类型定义

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 地形类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Reflect)]
pub enum TerrainType {
    /// 草地 - 正常移动速度
    #[default]
    Grass,
    /// 沙漠 - 移动速度 x0.8
    Desert,
    /// 水域 - 不可通行（除非水上单位）
    Water,
    /// 山地 - 不可通行
    Mountain,
    /// 森林 - 移动速度 x0.6，提供视野遮蔽
    Forest,
}

impl TerrainType {
    /// 获取移动代价（用于寻路权重）
    /// 返回 None 表示不可通行
    pub fn move_cost(&self) -> Option<f32> {
        match self {
            Self::Grass => Some(1.0),
            Self::Desert => Some(1.25),
            Self::Forest => Some(1.67),
            Self::Water | Self::Mountain => None,
        }
    }

    /// 是否可建造
    pub fn buildable(&self) -> bool {
        matches!(self, Self::Grass | Self::Desert)
    }

    /// 是否阻挡视野
    pub fn blocks_vision(&self) -> bool {
        matches!(self, Self::Mountain | Self::Forest)
    }

    /// 获取地形颜色（用于调试渲染）
    pub fn color(&self) -> Color {
        match self {
            Self::Grass => Color::srgb(0.3, 0.7, 0.3),
            Self::Desert => Color::srgb(0.9, 0.8, 0.5),
            Self::Water => Color::srgb(0.2, 0.4, 0.8),
            Self::Mountain => Color::srgb(0.5, 0.5, 0.5),
            Self::Forest => Color::srgb(0.1, 0.5, 0.2),
        }
    }
}

/// 单个瓦片
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Tile {
    /// 地形类型
    pub terrain: TerrainType,
    /// 高度值（0.0 ~ 1.0）
    pub height: f32,
    /// 是否被占用（有建筑/资源）
    pub occupied: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Grass,
            height: 0.5,
            occupied: false,
        }
    }
}

impl Tile {
    /// 创建新瓦片
    pub fn new(terrain: TerrainType, height: f32) -> Self {
        Self {
            terrain,
            height,
            occupied: false,
        }
    }

    /// 是否可通行
    pub fn walkable(&self) -> bool {
        !self.occupied && self.terrain.move_cost().is_some()
    }

    /// 获取移动代价
    pub fn move_cost(&self) -> Option<f32> {
        if self.occupied {
            None
        } else {
            self.terrain.move_cost()
        }
    }
}
