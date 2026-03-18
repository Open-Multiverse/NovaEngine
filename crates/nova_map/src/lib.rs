//! Nova Map - 地图系统
//!
//! 提供瓦片地图、程序化生成、寻路、战争迷雾等功能。
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_map::prelude::*;
//!
//! // 生成地图
//! let config = MapGeneratorConfig::default();
//! let tilemap = MapGenerator::generate(&config);
//!
//! // 寻路
//! if let Some(result) = Pathfinder::find_path(&tilemap, (0, 0), (10, 10)) {
//!     println!("路径: {:?}", result.path);
//! }
//! ```

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

pub use camera::RtsCameraController;
pub use fog::{FogOfWar, FogState, Vision};
pub use generator::{MapGenerator, MapGeneratorConfig, TerrainWeights};
pub use heightmap::HeightMap;
pub use pathfinding::{PathFollow, PathResult, Pathfinder};
pub use serialization::MapFile;
pub use tile::{Tile, TerrainType};
pub use tilemap::TileMap;

/// Nova Map 插件
pub struct NovaMapPlugin;

impl Plugin for NovaMapPlugin {
    fn build(&self, app: &mut App) {
        app
            // 注册类型
            .register_type::<TerrainType>()
            .register_type::<Tile>()
            .register_type::<TileMap>()
            .register_type::<FogState>()
            .register_type::<Vision>()
            .register_type::<PathFollow>()
            .register_type::<RtsCameraController>()
            // 添加系统
            .add_systems(
                Update,
                (
                    camera::rts_camera_system,
                    camera::rts_camera_zoom_system,
                ),
            );
    }
}

/// 带迷雾系统的地图插件
pub struct NovaMapWithFogPlugin;

impl Plugin for NovaMapWithFogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NovaMapPlugin)
            .add_systems(Update, fog_vision_system);
    }
}

/// 视野更新系统
fn fog_vision_system(
    tilemap: Option<Res<TileMap>>,
    mut fog: Option<ResMut<FogOfWar>>,
    mut units: Query<(&Transform, &mut Vision), Changed<Transform>>,
) {
    let (Some(tilemap), Some(ref mut fog)) = (tilemap, fog.as_mut()) else {
        return;
    };

    for (transform, mut vision) in units.iter_mut() {
        let Some(current_tile) = tilemap.world_to_tile(transform.translation) else {
            continue;
        };

        // 如果位置改变
        if vision.last_tile != Some(current_tile) {
            // 移除旧视野
            if let Some(old_tile) = vision.last_tile {
                fog.remove_vision(old_tile.0, old_tile.1, vision.range);
            }

            // 添加新视野
            fog.add_vision(current_tile.0, current_tile.1, vision.range);
            vision.last_tile = Some(current_tile);
        }
    }

    fog.update_states();
}
