//! Nova AI - AI 决策系统
//!
//! 负责角色"怎么想"：感知、行为树决策、性格情绪。

pub mod behavior;
pub mod decision;
pub mod emotion;
pub mod perception;
pub mod personality;
pub mod prelude;
pub mod tactics;

use bevy::prelude::*;

/// AI 系统执行顺序
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiSet {
    Perception,
    Decision,
}

/// AI 系统插件
pub struct NovaAiPlugin;

impl Plugin for NovaAiPlugin {
    fn build(&self, _app: &mut App) {
        // 后续任务填充
    }
}
