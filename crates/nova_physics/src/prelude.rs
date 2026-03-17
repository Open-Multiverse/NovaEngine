//! Nova Physics Prelude

pub use crate::plugin::{disable_physics_debug, enable_physics_debug, NovaPhysicsPlugin};
pub use crate::rigidbody::{
    spawn_physics_cube, spawn_physics_sphere, PhysicsMaterial, RigidBodyType,
};

// 重导出常用的 Rapier 类型
pub use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
