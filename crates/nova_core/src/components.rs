//! 核心组件定义

use bevy::prelude::*;

/// 实体名称组件
#[derive(Component, Debug, Clone)]
pub struct EntityName(pub String);

impl EntityName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// 标记为可见的组件
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Visible(pub bool);

impl Visible {
    pub fn new(visible: bool) -> Self {
        Self(visible)
    }
}

/// 标记实体为静态的（不会移动）
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Static;

/// 游戏时间资源
#[derive(Resource, Debug, Default)]
pub struct GameTime {
    /// 游戏总运行时间（秒）
    pub elapsed: f32,
    /// 上一帧到当前帧的时间间隔（秒）
    pub delta: f32,
    /// 时间缩放因子
    pub scale: f32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            elapsed: 0.0,
            delta: 0.0,
            scale: 1.0,
        }
    }

    /// 获取缩放后的 delta 时间
    pub fn scaled_delta(&self) -> f32 {
        self.delta * self.scale
    }
}
