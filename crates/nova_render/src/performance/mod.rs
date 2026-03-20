//! 性能优化系统
//!
//! 提供渲染和 ECS 性能优化：
//! - 视锥剔除
//! - 实例化渲染
//! - 空间哈希加速
//! - LOD 系统

pub mod frustum_culling;
pub mod instancing;
pub mod lod;
pub mod spatial_grid;

pub use frustum_culling::*;
pub use instancing::*;
pub use lod::*;
pub use spatial_grid::*;

use bevy::prelude::*;

/// 性能优化插件
pub struct NovaPerformancePlugin;

impl Plugin for NovaPerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerformanceSettings>()
            .add_systems(Startup, setup_performance_systems)
            .add_systems(
                PostUpdate,
                (
                    frustum_culling::update_frustum_culling,
                    instancing::update_instance_batches,
                    spatial_grid::update_spatial_grid,
                )
                    .chain(),
            );
    }
}

/// 性能设置
#[derive(Resource)]
pub struct PerformanceSettings {
    /// 启用视锥剔除
    pub enable_frustum_culling: bool,
    /// 启用实例化渲染
    pub enable_instancing: bool,
    /// 实例化阈值（同批次实体数）
    pub instancing_threshold: usize,
    /// 启用空间哈希
    pub enable_spatial_grid: bool,
    /// 空间哈希格子大小
    pub spatial_cell_size: f32,
    /// 启用 LOD
    pub enable_lod: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            enable_frustum_culling: true,
            enable_instancing: true,
            instancing_threshold: 50,
            enable_spatial_grid: true,
            spatial_cell_size: 50.0,
            enable_lod: true,
        }
    }
}

fn setup_performance_systems(settings: Res<PerformanceSettings>) {
    info!("Performance systems initialized:");
    info!("  - Frustum culling: {}", settings.enable_frustum_culling);
    info!("  - Instancing: {}", settings.enable_instancing);
    info!("  - Spatial grid: {}", settings.enable_spatial_grid);
    info!("  - LOD: {}", settings.enable_lod);
}
