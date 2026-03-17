//! Nova Physics - 物理系统
//!
//! 基于 Rapier 3D 的物理模拟：
//! - 刚体组件
//! - 碰撞检测
//! - 物理材质

pub mod plugin;
pub mod prelude;
pub mod rigidbody;

pub use plugin::NovaPhysicsPlugin;
