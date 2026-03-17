//! Nova Assets - 资源管理系统
//!
//! 提供资源管理功能：
//! - 资源注册和追踪
//! - 资源句柄管理
//! - 预加载支持
//! - 资源组管理
//! - 加载状态和进度
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_assets::prelude::*;
//!
//! // 注册资源
//! fn setup(mut registry: ResMut<AssetRegistry>) {
//!     registry.register("textures/player.png");
//!     registry.register_many(["sounds/jump.ogg", "sounds/land.ogg"]);
//! }
//!
//! // 创建资源组
//! fn setup_groups(mut registry: ResMut<AssetRegistry>) {
//!     registry.create_group("level1")
//!         .add("levels/level1.json")
//!         .add("textures/tileset.png")
//!         .with_preload(true);
//! }
//!
//! // 检查加载状态
//! fn check_loading(state: Res<AssetLoadState>) {
//!     if state.all_loaded() {
//!         println!("所有资源加载完成！");
//!     } else {
//!         println!("加载进度: {:.0}%", state.progress() * 100.0);
//!     }
//! }
//! ```
//!
//! # 模块说明
//!
//! - [`loader`] - 资源注册表和加载状态
//! - [`handle`] - 类型安全的资源句柄

pub mod handle;
pub mod loader;
pub mod plugin;
pub mod prelude;

pub use plugin::NovaAssetsPlugin;
