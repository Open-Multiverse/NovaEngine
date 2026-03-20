//! RTS 游戏原型
//!
//! Nova Engine 示例 - 展示地图系统、单位控制、战斗系统

use nova_engine::prelude::*;
use nova_map::prelude::*;

use nova_ai::NovaAiPlugin;
use nova_animation::NovaAnimationPlugin;
use nova_character::NovaCharacterPlugin;
use nova_formation::NovaFormationPlugin;

mod character_setup;
mod combat;
mod components;
mod movement;
mod selection;
mod setup;
mod ui;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(NovaCharacterPlugin)
        .add_plugin(NovaAiPlugin)
        .add_plugin(NovaFormationPlugin)
        .add_plugin(NovaAnimationPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(setup::setup_game)
        .run();
}
