//! Nova Engine - Web 3D 游戏引擎
//!
//! Nova Engine 是基于 Bevy 的 Web 3D 游戏引擎，
//! 使用 Rust + WebAssembly 构建，以 WebGPU 作为图形后端。
//!
//! # 快速开始
//!
//! ```no_run
//! use nova_engine::prelude::*;
//!
//! fn main() {
//!     NovaApp::new()
//!         .with_title("我的游戏")
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     // 添加相机
//!     commands.spawn(Camera3dBundle::new());
//! }
//! ```

pub mod prelude;

// 重导出子模块
pub use nova_animation;
pub use nova_core;
pub use nova_physics;
pub use nova_render;
pub use nova_ui;

// 重导出核心类型
pub use nova_core::NovaApp;
