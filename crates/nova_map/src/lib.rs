//! Nova Map - 地图系统
//!
//! 提供瓦片地图、程序化生成、寻路、战争迷雾等功能。

pub mod camera;
pub mod fog;
pub mod generator;
pub mod heightmap;
pub mod pathfinding;
pub mod prelude;
pub mod serialization;
pub mod tile;
pub mod tilemap;

use bevy::prelude::*;

/// Nova Map 插件
pub struct NovaMapPlugin;

impl Plugin for NovaMapPlugin {
    fn build(&self, _app: &mut App) {
        // 后续添加系统
    }
}
