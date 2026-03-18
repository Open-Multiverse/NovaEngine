//! RTS 游戏原型
//!
//! Nova Engine 示例 - 展示地图系统、单位控制、战斗系统

use nova_engine::prelude::*;
use nova_map::prelude::*;

mod combat;
mod components;
mod movement;
mod selection;
mod setup;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_startup_system(setup::setup_game)
        .run();
}
