//! Nova Core - 核心类型与 ECS 封装
//!
//! 提供 Nova Engine 的核心功能：
//! - App 生命周期管理
//! - 核心组件和资源
//! - Bevy 重导出

pub mod app;
pub mod components;
pub mod prelude;

pub use app::NovaApp;

// 重导出 Bevy 核心类型
pub use bevy;
