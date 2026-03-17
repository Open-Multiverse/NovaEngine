//! Nova Render - 渲染系统
//!
//! 提供 3D 渲染功能：
//! - 相机组件
//! - 灯光系统
//! - 网格组件
//! - 材质管理

pub mod camera;
pub mod light;
pub mod material;
pub mod mesh;
pub mod plugin;
pub mod prelude;

pub use plugin::NovaRenderPlugin;
