//! Nova Core - 核心类型与 ECS 封装
//!
//! 提供 Nova Engine 的核心功能：
//! - App 生命周期管理
//! - 核心组件和资源
//! - 插件系统
//! - 调度阶段定义
//! - Bevy 重导出

pub mod app;
pub mod components;
pub mod plugin;
pub mod prelude;
pub mod schedule;

pub use app::NovaApp;
pub use plugin::{NovaCorePlugin, NovaDefaultPlugins, NovaMinimalPlugins};

// 重导出 Bevy 核心类型
pub use bevy;
