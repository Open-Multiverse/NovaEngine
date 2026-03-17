//! 相机组件

use bevy::prelude::*;

/// 主相机标记组件
#[derive(Component, Debug, Default)]
pub struct MainCamera;

/// Nova 3D 相机配置
pub struct NovaCamera3d {
    pub transform: Transform,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for NovaCamera3d {
    fn default() -> Self {
        Self {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl NovaCamera3d {
    /// 创建新的相机配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置相机位置
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.transform.translation = position;
        self
    }

    /// 设置相机朝向
    pub fn looking_at(mut self, target: Vec3, up: Vec3) -> Self {
        self.transform = self.transform.looking_at(target, up);
        self
    }

    /// 生成相机组件元组
    pub fn bundle(self) -> impl Bundle {
        (
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                fov: self.fov,
                near: self.near,
                far: self.far,
                aspect_ratio: 16.0 / 9.0,
            }),
            self.transform,
            MainCamera,
        )
    }
}
