//! Nova UI - 用户界面系统
//!
//! 基于 egui 的即时模式 UI：
//! - UI 上下文管理
//! - 主题系统
//! - 基础组件
//! - 调试工具

pub mod context;
pub mod plugin;
pub mod prelude;
pub mod widgets;

pub use plugin::NovaUiPlugin;
