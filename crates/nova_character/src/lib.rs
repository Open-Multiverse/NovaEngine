//! Nova Character - 人物建模系统
//!
//! 负责角色"是什么"：数据、属性、状态、视觉反馈。
//! 不负责决策（nova_ai）和编队（nova_formation）。

pub mod attributes;
pub mod character;
pub mod feedback;
pub mod loader;
pub mod movement;
pub mod prelude;
pub mod state;

// 顶层重导出，方便集成测试直接 use nova_character::{...}
pub use character::{CharacterBundle, CharacterStats};
pub use movement::PreviousCharacterState;
pub use state::CharacterState;

use bevy::prelude::*;

/// 角色系统插件
pub struct NovaCharacterPlugin;

impl Plugin for NovaCharacterPlugin {
    fn build(&self, app: &mut App) {
        use crate::{attributes::Attributes, state::*};
        app.register_type::<CharacterState>()
            .register_type::<AttackCooldown>()
            .register_type::<Attributes>()
            .register_type::<movement::PreviousCharacterState>()
            .register_type::<feedback::HealthBar>()
            .add_event::<feedback::SpawnDamageNumber>()
            .add_event::<feedback::TriggerHitFlash>()
            .add_event::<feedback::UnitDiedEvent>()
            .add_systems(
                Update,
                (
                    movement::update_previous_state_system,
                    movement::movement_system
                        .after(movement::update_previous_state_system),
                    state::stun_tick_system,
                    feedback::damage_number_system,
                    feedback::hit_flash_system,
                ),
            );
    }
}
