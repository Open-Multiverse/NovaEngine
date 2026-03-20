//! Nova Render - 渲染系统
//!
//! 提供 3D 渲染功能：
//! - 相机组件
//! - 相机控制器（轨道相机、FPS 相机）
//! - 灯光系统（方向光、点光源）
//! - 网格组件
//! - 材质管理
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_render::prelude::*;
//!
//! // 创建轨道相机
//! commands.spawn((
//!     Camera3d::default(),
//!     OrbitCameraController::new()
//!         .with_distance(10.0)
//!         .with_target(Vec3::ZERO),
//! ));
//!
//! // 添加方向光
//! commands.spawn(DirectionalLight {
//!     illuminance: 10000.0,
//!     ..default()
//! });
//!
//! // 创建网格
//! let mesh = NovaMeshBuilder::cube(1.0);
//! ```
//!
//! # 模块说明
//!
//! - [`camera`] - 相机组件和配置
//! - [`camera_controller`] - 轨道相机和 FPS 相机控制器
//! - [`light`] - 光照组件
//! - [`mesh`] - 网格构建器
//! - [`material`] - 材质构建器和预设

pub mod camera;
pub mod camera_controller;
pub mod light;
pub mod material;
pub mod mesh;
pub mod performance;
pub mod plugin;
pub mod prelude;

pub use camera_controller::{FpsCameraPlugin, OrbitCameraPlugin};
pub use performance::NovaPerformancePlugin;
pub use plugin::NovaRenderPlugin;
