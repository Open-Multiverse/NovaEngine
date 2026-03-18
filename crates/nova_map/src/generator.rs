//! 程序化地图生成器

use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};

use crate::heightmap::HeightMap;
use crate::tile::{TerrainType, Tile};
use crate::tilemap::TileMap;

/// 地形权重配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainWeights {
    pub grass: f32,
    pub desert: f32,
    pub forest: f32,
}

impl Default for TerrainWeights {
    fn default() -> Self {
        Self {
            grass: 0.5,
            desert: 0.3,
            forest: 0.2,
        }
    }
}

/// 地图生成器配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapGeneratorConfig {
    /// 随机种子
    pub seed: u64,
    /// 地图尺寸（宽, 高）
    pub size: (u32, u32),
    /// 瓦片世界尺寸
    pub tile_size: f32,
    /// 水位线（低于此高度为水域）
    pub water_level: f32,
    /// 山地线（高于此高度为山地）
    pub mountain_level: f32,
    /// 地形权重
    pub terrain_weights: TerrainWeights,
    /// 噪声八度数
    pub noise_octaves: u32,
    /// 噪声频率
    pub noise_frequency: f64,
    /// 噪声振幅
    pub noise_amplitude: f64,
}

impl Default for MapGeneratorConfig {
    fn default() -> Self {
        Self {
            seed: 12345,
            size: (128, 128),
            tile_size: 1.0,
            water_level: 0.3,
            mountain_level: 0.75,
            terrain_weights: TerrainWeights::default(),
            noise_octaves: 4,
            noise_frequency: 0.02,
            noise_amplitude: 1.0,
        }
    }
}

/// 地图生成器
pub struct MapGenerator;

impl MapGenerator {
    /// 生成完整地图
    pub fn generate(config: &MapGeneratorConfig) -> TileMap {
        let heightmap = Self::generate_heightmap(config);
        Self::heightmap_to_tilemap(&heightmap, config)
    }

    /// 生成高度图
    pub fn generate_heightmap(config: &MapGeneratorConfig) -> HeightMap {
        let (width, height) = config.size;
        let mut data = Vec::with_capacity((width * height) as usize);

        let perlin = Perlin::new(config.seed as u32);

        for y in 0..height {
            for x in 0..width {
                let mut value = 0.0;
                let mut amplitude = config.noise_amplitude;
                let mut frequency = config.noise_frequency;

                // 叠加多个八度
                for _ in 0..config.noise_octaves {
                    let nx = x as f64 * frequency;
                    let ny = y as f64 * frequency;
                    value += perlin.get([nx, ny]) * amplitude;
                    amplitude *= 0.5;
                    frequency *= 2.0;
                }

                // 归一化到 0.0 ~ 1.0
                let normalized = (value + 1.0) / 2.0;
                data.push(normalized as f32);
            }
        }

        let mut heightmap = HeightMap::from_data(width, height, data);
        heightmap.normalize();
        heightmap
    }

    /// 高度图转瓦片地图
    pub fn heightmap_to_tilemap(heightmap: &HeightMap, config: &MapGeneratorConfig) -> TileMap {
        let (width, height) = config.size;
        let mut tilemap = TileMap::new(width, height, config.tile_size);

        // 简单的伪随机数生成（用于地形分配）
        let mut rng_state = config.seed;
        let mut next_rand = || -> f32 {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            ((rng_state >> 16) & 0x7FFF) as f32 / 32767.0
        };

        for (x, y, h) in heightmap.iter() {
            let terrain = if h < config.water_level {
                TerrainType::Water
            } else if h > config.mountain_level {
                TerrainType::Mountain
            } else {
                // 根据权重随机分配地形
                let weights = &config.terrain_weights;
                let total = weights.grass + weights.desert + weights.forest;
                let r = next_rand() * total;

                if r < weights.grass {
                    TerrainType::Grass
                } else if r < weights.grass + weights.desert {
                    TerrainType::Desert
                } else {
                    TerrainType::Forest
                }
            };

            tilemap.set(x, y, Tile::new(terrain, h));
        }

        tilemap
    }
}
