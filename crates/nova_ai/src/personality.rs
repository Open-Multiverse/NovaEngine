//! 性格特质 - 影响 AI 决策权重

use bevy::prelude::*;

/// 性格组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Personality {
    /// 0-1，高=主动进攻，低=防守
    pub aggression: f32,
    /// 0-1，高=不畏死亡，低=容易逃跑
    pub courage: f32,
    /// 0-1，高=严格执行命令，低=自由行动
    pub discipline: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            courage: 0.5,
            discipline: 0.5,
        }
    }
}

impl Personality {
    pub fn soldier() -> Self {
        Self {
            aggression: 0.6,
            courage: 0.7,
            discipline: 0.8,
        }
    }

    pub fn coward() -> Self {
        Self {
            aggression: 0.2,
            courage: 0.2,
            discipline: 0.4,
        }
    }

    pub fn berserker() -> Self {
        Self {
            aggression: 0.95,
            courage: 0.9,
            discipline: 0.2,
        }
    }
}
