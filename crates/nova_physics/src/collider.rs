//! 碰撞器组件封装
//!
//! 提供简化的碰撞器创建接口

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// 碰撞器形状类型
#[derive(Debug, Clone)]
pub enum ColliderShape {
    /// 球体
    Ball { radius: f32 },
    /// 长方体
    Cuboid { half_extents: Vec3 },
    /// 胶囊体
    Capsule { half_height: f32, radius: f32 },
    /// 圆柱体
    Cylinder { half_height: f32, radius: f32 },
    /// 圆锥体
    Cone { half_height: f32, radius: f32 },
}

impl ColliderShape {
    /// 创建球体碰撞器
    pub fn ball(radius: f32) -> Self {
        Self::Ball { radius }
    }

    /// 创建长方体碰撞器
    pub fn cuboid(half_x: f32, half_y: f32, half_z: f32) -> Self {
        Self::Cuboid {
            half_extents: Vec3::new(half_x, half_y, half_z),
        }
    }

    /// 创建立方体碰撞器
    pub fn cube(half_size: f32) -> Self {
        Self::cuboid(half_size, half_size, half_size)
    }

    /// 创建胶囊体碰撞器
    pub fn capsule(half_height: f32, radius: f32) -> Self {
        Self::Capsule {
            half_height,
            radius,
        }
    }

    /// 创建圆柱体碰撞器
    pub fn cylinder(half_height: f32, radius: f32) -> Self {
        Self::Cylinder {
            half_height,
            radius,
        }
    }

    /// 转换为 Rapier Collider
    pub fn to_collider(&self) -> Collider {
        match self {
            ColliderShape::Ball { radius } => Collider::ball(*radius),
            ColliderShape::Cuboid { half_extents } => {
                Collider::cuboid(half_extents.x, half_extents.y, half_extents.z)
            }
            ColliderShape::Capsule {
                half_height,
                radius,
            } => Collider::capsule_y(*half_height, *radius),
            ColliderShape::Cylinder {
                half_height,
                radius,
            } => Collider::cylinder(*half_height, *radius),
            ColliderShape::Cone {
                half_height,
                radius,
            } => Collider::cone(*half_height, *radius),
        }
    }
}

/// 碰撞层定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollisionLayer(pub u32);

impl CollisionLayer {
    /// 默认层
    pub const DEFAULT: Self = Self(0);
    /// 玩家层
    pub const PLAYER: Self = Self(1);
    /// 敌人层
    pub const ENEMY: Self = Self(2);
    /// 地形层
    pub const TERRAIN: Self = Self(3);
    /// 触发器层
    pub const TRIGGER: Self = Self(4);

    /// 创建自定义层
    pub const fn custom(id: u32) -> Self {
        Self(id)
    }
}

/// 碰撞器配置
#[derive(Debug, Clone)]
pub struct ColliderConfig {
    /// 碰撞器形状
    pub shape: ColliderShape,
    /// 是否为传感器（触发器）
    pub sensor: bool,
    /// 摩擦系数
    pub friction: f32,
    /// 弹性系数
    pub restitution: f32,
    /// 密度
    pub density: f32,
}

impl Default for ColliderConfig {
    fn default() -> Self {
        Self {
            shape: ColliderShape::cube(0.5),
            sensor: false,
            friction: 0.5,
            restitution: 0.3,
            density: 1.0,
        }
    }
}

impl ColliderConfig {
    /// 创建新的碰撞器配置
    pub fn new(shape: ColliderShape) -> Self {
        Self { shape, ..default() }
    }

    /// 设置为传感器
    pub fn as_sensor(mut self) -> Self {
        self.sensor = true;
        self
    }

    /// 设置摩擦系数
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// 设置弹性系数
    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution;
        self
    }

    /// 设置密度
    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density;
        self
    }

    /// 构建碰撞器组件元组
    pub fn build(self) -> impl Bundle {
        // 注意：传感器模式需要单独添加 Sensor 组件
        // 使用 .as_sensor() 配置后，需要在 spawn 时额外添加 Sensor 组件
        (
            self.shape.to_collider(),
            Friction::coefficient(self.friction),
            Restitution::coefficient(self.restitution),
            ColliderMassProperties::Density(self.density),
        )
    }
}
