//! Nova Character - 人物建模系统
//!
//! 负责角色"是什么"：数据、属性、状态、视觉反馈。
//! 不负责决策（nova_ai）和编队（nova_formation）。

pub mod attributes;
pub mod character;
pub mod feedback;
pub mod loader;
pub mod prelude;
pub mod state;

use bevy::prelude::*;

/// 角色系统插件
pub struct NovaCharacterPlugin;

impl Plugin for NovaCharacterPlugin {
    fn build(&self, app: &mut App) {
        use crate::{attributes::Attributes, state::*};
        app
            .register_type::<CharacterState>()
            .register_type::<AttackCooldown>()
            .register_type::<Attributes>()
            .add_systems(Update, state::stun_tick_system);
    }
}
