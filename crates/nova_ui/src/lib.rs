//! Nova UI - 用户界面系统
//!
//! 基于 egui 的即时模式 UI：
//! - UI 上下文管理
//! - 主题系统（深色/浅色）
//! - 预制组件（调试面板、FPS 显示、属性编辑器）
//! - 调试工具
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_ui::prelude::*;
//! use bevy_egui::EguiContexts;
//!
//! fn ui_system(mut contexts: EguiContexts) {
//!     egui::Window::new("设置").show(contexts.ctx_mut(), |ui| {
//!         ui.label("游戏设置");
//!         if ui.button("保存").clicked() {
//!             // 保存设置
//!         }
//!     });
//! }
//! ```
//!
//! # 预制组件
//!
//! ```ignore
//! use nova_ui::prelude::*;
//!
//! // 调试面板
//! DebugPanel::show(&mut egui_ctx, |ui| {
//!     ui.label(format!("FPS: {:.1}", fps));
//! });
//!
//! // FPS 显示
//! FpsDisplay::show(&mut egui_ctx, fps);
//! ```
//!
//! # 模块说明
//!
//! - [`context`] - UI 状态和主题管理
//! - [`widgets`] - 预制 UI 组件

pub mod context;
pub mod plugin;
pub mod prelude;
pub mod widgets;

pub use plugin::NovaUiPlugin;
