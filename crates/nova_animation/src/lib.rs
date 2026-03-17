//! Nova Animation - 动画系统
//!
//! 提供动画功能：
//! - 动画片段定义
//! - 动画播放器
//! - 补间动画

pub mod clip;
pub mod player;
pub mod plugin;
pub mod prelude;
pub mod tween;

pub use plugin::NovaAnimationPlugin;
