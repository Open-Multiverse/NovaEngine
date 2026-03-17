//! Nova Physics - 物理系统
//!
//! 基于 Rapier 3D 的物理模拟：
//! - 刚体组件（动态、静态、运动学）
//! - 碰撞器组件（盒子、球体、胶囊）
//! - 碰撞检测和事件
//! - 物理材质（弹性、摩擦）
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_physics::prelude::*;
//!
//! // 创建动态刚体
//! commands.spawn((
//!     RigidBodyConfig::dynamic(),
//!     ColliderConfig::box_shape(Vec3::ONE),
//!     Transform::from_xyz(0.0, 5.0, 0.0),
//! ));
//!
//! // 创建静态地面
//! commands.spawn((
//!     RigidBodyConfig::fixed(),
//!     ColliderConfig::box_shape(Vec3::new(10.0, 0.1, 10.0)),
//! ));
//!
//! // 监听碰撞事件
//! fn on_collision(mut events: EventReader<NovaCollisionEvent>) {
//!     for event in events.read() {
//!         println!("碰撞: {:?} <-> {:?}", event.entity_a, event.entity_b);
//!     }
//! }
//! ```
//!
//! # 模块说明
//!
//! - [`rigidbody`] - 刚体配置和类型
//! - [`collider`] - 碰撞器形状和配置
//! - [`events`] - 碰撞事件系统

pub mod collider;
pub mod events;
pub mod plugin;
pub mod prelude;
pub mod rigidbody;

pub use events::NovaCollisionEventsPlugin;
pub use plugin::NovaPhysicsPlugin;
