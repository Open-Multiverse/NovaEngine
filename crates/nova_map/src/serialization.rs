//! 地图序列化

use serde::{Deserialize, Serialize};

use crate::tile::{TerrainType, Tile};
use crate::tilemap::TileMap;

/// 地图文件版本
pub const MAP_FILE_VERSION: u32 = 1;

/// 地图文件格式
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapFile {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub tiles: Vec<TileData>,
    #[serde(default)]
    pub spawn_points: Vec<SpawnPoint>,
    #[serde(default)]
    pub resource_nodes: Vec<ResourceNodeData>,
}

/// 瓦片数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileData {
    pub terrain: String,
    pub height: f32,
    #[serde(default)]
    pub occupied: bool,
}

/// 出生点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub x: u32,
    pub y: u32,
    pub team: String,
}

/// 资源点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceNodeData {
    pub x: u32,
    pub y: u32,
    pub resource_type: String,
    #[serde(default = "default_resource_amount")]
    pub amount: u32,
}

fn default_resource_amount() -> u32 {
    1000
}

impl TileData {
    pub fn from_tile(tile: &Tile) -> Self {
        Self {
            terrain: terrain_to_string(tile.terrain),
            height: tile.height,
            occupied: tile.occupied,
        }
    }

    pub fn to_tile(&self) -> Tile {
        Tile {
            terrain: string_to_terrain(&self.terrain),
            height: self.height,
            occupied: self.occupied,
        }
    }
}

fn terrain_to_string(terrain: TerrainType) -> String {
    match terrain {
        TerrainType::Grass => "grass".to_string(),
        TerrainType::Desert => "desert".to_string(),
        TerrainType::Water => "water".to_string(),
        TerrainType::Mountain => "mountain".to_string(),
        TerrainType::Forest => "forest".to_string(),
    }
}

fn string_to_terrain(s: &str) -> TerrainType {
    match s.to_lowercase().as_str() {
        "grass" => TerrainType::Grass,
        "desert" => TerrainType::Desert,
        "water" => TerrainType::Water,
        "mountain" => TerrainType::Mountain,
        "forest" => TerrainType::Forest,
        _ => TerrainType::Grass,
    }
}

impl MapFile {
    /// 从 TileMap 创建
    pub fn from_tilemap(tilemap: &TileMap) -> Self {
        let tiles: Vec<TileData> = tilemap
            .iter()
            .map(|(_, _, tile)| TileData::from_tile(tile))
            .collect();

        Self {
            version: MAP_FILE_VERSION,
            width: tilemap.width(),
            height: tilemap.height(),
            tile_size: tilemap.tile_size(),
            tiles,
            spawn_points: vec![],
            resource_nodes: vec![],
        }
    }

    /// 转换为 TileMap
    pub fn to_tilemap(&self) -> TileMap {
        let mut tilemap = TileMap::new(self.width, self.height, self.tile_size);

        for (idx, tile_data) in self.tiles.iter().enumerate() {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            tilemap.set(x, y, tile_data.to_tile());
        }

        tilemap
    }

    /// 序列化为 JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从 JSON 反序列化
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// TileMap 扩展方法
impl TileMap {
    /// 保存到 JSON 字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        MapFile::from_tilemap(self).to_json()
    }

    /// 从 JSON 字符串加载
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        Ok(MapFile::from_json(json)?.to_tilemap())
    }
}
