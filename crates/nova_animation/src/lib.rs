//! Nova Animation - 动画系统
//!
//! 提供动画功能：
//! - 关键帧动画（位置、旋转、缩放）
//! - 动画播放器（播放、暂停、循环）
//! - 补间动画（多种缓动函数）
//!
//! # 快速开始
//!
//! ## 补间动画
//!
//! ```ignore
//! use nova_animation::prelude::*;
//!
//! // 创建位置补间动画
//! commands.spawn((
//!     Mesh3d(mesh),
//!     Transform::from_xyz(0.0, 0.0, 0.0),
//!     PositionTween::new(
//!         Vec3::ZERO,
//!         Vec3::new(5.0, 0.0, 0.0),
//!         2.0, // 持续时间
//!     )
//!     .with_ease(NovaEaseFunction::QuadInOut)
//!     .with_loop(LoopMode::PingPong),
//! ));
//! ```
//!
//! ## 关键帧动画
//!
//! ```ignore
//! use nova_animation::prelude::*;
//!
//! // 创建关键帧动画片段
//! let clip = SimpleAnimationBuilder::new("bounce")
//!     .position_at(0.0, Vec3::ZERO)
//!     .position_at(0.5, Vec3::new(0.0, 2.0, 0.0))
//!     .position_at(1.0, Vec3::ZERO)
//!     .looping()
//!     .build();
//! ```
//!
//! # 缓动函数
//!
//! 支持的缓动函数：
//! - `Linear` - 线性
//! - `QuadIn` / `QuadOut` / `QuadInOut` - 二次
//! - `CubicIn` / `CubicOut` / `CubicInOut` - 三次
//!
//! # 模块说明
//!
//! - [`tween`] - 补间动画组件和系统
//! - [`clip`] - 关键帧动画片段
//! - [`player`] - 动画播放器

pub mod clip;
pub mod player;
pub mod plugin;
pub mod prelude;
pub mod procedural;
pub mod state_machine;
pub mod tween;

pub use plugin::NovaAnimationPlugin;
pub use procedural::{procedural_idle_system, ProceduralIdle};
pub use state_machine::{
    AnimationState, AnimationStateConfig, AnimationStateMachine, AnimationTransition,
    TransitionCondition,
};
