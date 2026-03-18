//! 战争迷雾系统

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 迷雾状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
pub enum FogState {
    /// 未探索 - 完全黑暗
    #[default]
    Unexplored,
    /// 已探索 - 显示地形，不显示敌人
    Explored,
    /// 可见 - 在当前视野内
    Visible,
}

/// 战争迷雾资源
#[derive(Resource, Clone, Debug)]
pub struct FogOfWar {
    /// 地图宽度
    width: u32,
    /// 地图高度
    height: u32,
    /// 每个瓦片的迷雾状态
    states: Vec<FogState>,
    /// 视野计数（多少单位能看到该瓦片）
    vision_count: Vec<u32>,
    /// 是否启用
    pub enabled: bool,
}

impl FogOfWar {
    /// 创建新的迷雾系统
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            states: vec![FogState::Unexplored; size],
            vision_count: vec![0; size],
            enabled: true,
        }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 检查坐标是否在范围内
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// 坐标转索引
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// 获取迷雾状态
    pub fn get_state(&self, x: u32, y: u32) -> FogState {
        if !self.enabled {
            return FogState::Visible;
        }
        if self.in_bounds(x, y) {
            self.states[self.index(x, y)]
        } else {
            FogState::Unexplored
        }
    }

    /// 获取视野计数
    pub fn get_vision_count(&self, x: u32, y: u32) -> u32 {
        if self.in_bounds(x, y) {
            self.vision_count[self.index(x, y)]
        } else {
            0
        }
    }

    /// 添加视野
    pub fn add_vision(&mut self, center_x: u32, center_y: u32, range: u32) {
        let range_sq = (range * range) as i32;

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                if dx * dx + dy * dy > range_sq {
                    continue;
                }

                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;

                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    let x = x as u32;
                    let y = y as u32;
                    let idx = self.index(x, y);
                    self.vision_count[idx] += 1;
                }
            }
        }
    }

    /// 移除视野
    pub fn remove_vision(&mut self, center_x: u32, center_y: u32, range: u32) {
        let range_sq = (range * range) as i32;

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                if dx * dx + dy * dy > range_sq {
                    continue;
                }

                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;

                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    let x = x as u32;
                    let y = y as u32;
                    let idx = self.index(x, y);
                    self.vision_count[idx] = self.vision_count[idx].saturating_sub(1);
                }
            }
        }
    }

    /// 更新迷雾状态
    pub fn update_states(&mut self) {
        for idx in 0..self.states.len() {
            if self.vision_count[idx] > 0 {
                self.states[idx] = FogState::Visible;
            } else if self.states[idx] == FogState::Visible {
                // 从可见变为已探索
                self.states[idx] = FogState::Explored;
            }
            // Unexplored 保持不变
        }
    }

    /// 重置所有迷雾
    pub fn reset(&mut self) {
        self.states.fill(FogState::Unexplored);
        self.vision_count.fill(0);
    }

    /// 揭示全部地图
    pub fn reveal_all(&mut self) {
        self.states.fill(FogState::Visible);
    }
}

/// 视野组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Vision {
    /// 视野半径（瓦片数）
    pub range: u32,
    /// 上一帧的瓦片位置
    pub last_tile: Option<(u32, u32)>,
}

impl Vision {
    pub fn new(range: u32) -> Self {
        Self {
            range,
            last_tile: None,
        }
    }
}
