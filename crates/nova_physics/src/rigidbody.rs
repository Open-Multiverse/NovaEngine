//! 刚体组件封装

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// 刚体类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidBodyType {
    /// 动态刚体 - 受物理模拟影响
    #[default]
    Dynamic,
    /// 静态刚体 - 不移动
    Static,
    /// 运动学刚体 - 由代码控制移动
    Kinematic,
}

impl From<RigidBodyType> for RigidBody {
    fn from(value: RigidBodyType) -> Self {
        match value {
            RigidBodyType::Dynamic => RigidBody::Dynamic,
            RigidBodyType::Static => RigidBody::Fixed,
            RigidBodyType::Kinematic => RigidBody::KinematicPositionBased,
        }
    }
}

/// 物理材质配置
#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    /// 摩擦系数 (0.0 - 1.0)
    pub friction: f32,
    /// 弹性系数 (0.0 - 1.0)
    pub restitution: f32,
}

impl Default for PhysicsMaterial {
    fn default() -> Self {
        Self {
            friction: 0.5,
            restitution: 0.3,
        }
    }
}

/// 创建带碰撞体的刚体实体
pub fn spawn_physics_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    size: Vec3,
    body_type: RigidBodyType,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(position),
            RigidBody::from(body_type),
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        ))
        .id()
}

/// 创建带碰撞体的球体实体
pub fn spawn_physics_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    radius: f32,
    body_type: RigidBodyType,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(radius))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(position),
            RigidBody::from(body_type),
            Collider::ball(radius),
        ))
        .id()
}
