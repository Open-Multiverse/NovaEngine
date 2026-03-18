//! 瓦片地图数据结构

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tile::{Tile, TerrainType};

/// 瓦片地图资源
#[derive(Resource, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct TileMap {
    /// 地图宽度（瓦片数）
    width: u32,
    /// 地图高度（瓦片数）
    height: u32,
    /// 瓦片数据（row-major 存储）
    tiles: Vec<Tile>,
    /// 瓦片世界尺寸
    tile_size: f32,
}

impl TileMap {
    /// 创建空地图（全部为草地）
    pub fn new(width: u32, height: u32, tile_size: f32) -> Self {
        let tiles = vec![Tile::default(); (width * height) as usize];
        Self {
            width,
            height,
            tiles,
            tile_size,
        }
    }

    /// 获取地图宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取地图高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 获取瓦片尺寸
    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }

    /// 检查坐标是否在地图范围内
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// 坐标转索引
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// 获取指定位置的瓦片
    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        if self.in_bounds(x, y) {
            Some(&self.tiles[self.index(x, y)])
        } else {
            None
        }
    }

    /// 获取指定位置的瓦片（可变）
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            Some(&mut self.tiles[idx])
        } else {
            None
        }
    }

    /// 设置瓦片
    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            self.tiles[idx] = tile;
        }
    }

    /// 设置地形类型
    pub fn set_terrain(&mut self, x: u32, y: u32, terrain: TerrainType) {
        if let Some(tile) = self.get_mut(x, y) {
            tile.terrain = terrain;
        }
    }

    /// 设置高度
    pub fn set_height(&mut self, x: u32, y: u32, height: f32) {
        if let Some(tile) = self.get_mut(x, y) {
            tile.height = height.clamp(0.0, 1.0);
        }
    }

    /// 世界坐标转瓦片坐标
    pub fn world_to_tile(&self, world_pos: Vec3) -> Option<(u32, u32)> {
        // 地图中心在世界原点
        let half_width = (self.width as f32 * self.tile_size) / 2.0;
        let half_height = (self.height as f32 * self.tile_size) / 2.0;

        let local_x = world_pos.x + half_width;
        let local_z = world_pos.z + half_height;

        if local_x < 0.0 || local_z < 0.0 {
            return None;
        }

        let tile_x = (local_x / self.tile_size) as u32;
        let tile_y = (local_z / self.tile_size) as u32;

        if self.in_bounds(tile_x, tile_y) {
            Some((tile_x, tile_y))
        } else {
            None
        }
    }

    /// 瓦片坐标转世界坐标（瓦片中心）
    pub fn tile_to_world(&self, x: u32, y: u32) -> Vec3 {
        let half_width = (self.width as f32 * self.tile_size) / 2.0;
        let half_height = (self.height as f32 * self.tile_size) / 2.0;

        let world_x = (x as f32 + 0.5) * self.tile_size - half_width;
        let world_z = (y as f32 + 0.5) * self.tile_size - half_height;
        let world_y = self.get_world_height(x, y);

        Vec3::new(world_x, world_y, world_z)
    }

    /// 获取瓦片的世界高度
    pub fn get_world_height(&self, x: u32, y: u32) -> f32 {
        self.get(x, y)
            .map(|t| t.height * 5.0) // 高度缩放因子
            .unwrap_or(0.0)
    }

    /// 获取相邻瓦片（4 方向）
    pub fn neighbors4(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result
    }

    /// 获取相邻瓦片（8 方向，包括对角线）
    pub fn neighbors8(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut result = Vec::with_capacity(8);
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let nx = nx as u32;
                    let ny = ny as u32;
                    if self.in_bounds(nx, ny) {
                        result.push((nx, ny));
                    }
                }
            }
        }
        result
    }

    /// 迭代所有瓦片
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, &Tile)> {
        self.tiles.iter().enumerate().map(move |(idx, tile)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            (x, y, tile)
        })
    }
}
