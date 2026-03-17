//! Nova Core - 核心类型与 ECS 封装
//!
//! 提供 Nova Engine 的核心功能：
//! - App 生命周期管理
//! - 核心组件和资源
//! - 插件系统
//! - 调度阶段定义
//! - 输入处理
//! - 场景序列化
//! - Bevy 重导出
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_core::prelude::*;
//!
//! fn main() {
//!     NovaApp::new()
//!         .with_title("我的游戏")
//!         .with_window_size(1280.0, 720.0)
//!         .add_startup_system(setup)
//!         .add_system(update)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera3d::default());
//! }
//!
//! fn update(time: Res<Time>) {
//!     // 游戏逻辑
//! }
//! ```
//!
//! # 输入处理
//!
//! ```ignore
//! use nova_core::prelude::*;
//!
//! // 使用输入辅助函数
//! fn movement(keyboard: Res<ButtonInput<KeyCode>>) {
//!     let dir = input_helpers::wasd_movement(&keyboard);
//!     // 移动角色...
//! }
//!
//! // 使用输入轴
//! fn setup_input(mut axes: ResMut<InputAxes>) {
//!     axes.register("horizontal", InputAxis::horizontal_wasd());
//!     axes.register("vertical", InputAxis::vertical_wasd());
//! }
//! ```
//!
//! # 模块说明
//!
//! - [`app`] - NovaApp 应用构建器
//! - [`input`] - 输入处理系统
//! - [`scene`] - 场景序列化
//! - [`plugin`] - 插件集合
//! - [`schedule`] - 调度阶段定义

pub mod app;
pub mod components;
pub mod input;
pub mod plugin;
pub mod prelude;
pub mod scene;
pub mod schedule;

pub use app::NovaApp;
pub use input::NovaInputPlugin;
pub use plugin::{NovaCorePlugin, NovaDefaultPlugins, NovaMinimalPlugins};

// 重导出 Bevy 核心类型
pub use bevy;
