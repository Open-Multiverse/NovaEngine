# Nova Map 与 RTS Demo 实施计划

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 nova_map 地图系统模块并构建一个完整的 RTS 游戏原型 demo。

**Architecture:** 新建 nova_map crate 提供瓦片地图、程序化生成、寻路、战争迷雾等能力。在 examples 下创建 rts_demo 示例，演示完整 RTS 玩法：单位选中、移动、战斗、迷雾。

**Tech Stack:** Rust, Bevy 0.15, noise (Simplex Noise), serde/serde_json (序列化), bevy_egui (UI)

---

## File Structure

```
crates/nova_map/
├── Cargo.toml
└── src/
    ├── lib.rs              # 模块入口、NovaMapPlugin
    ├── tile.rs             # Tile, TerrainType
    ├── tilemap.rs          # TileMap 数据结构
    ├── heightmap.rs        # HeightMap
    ├── generator.rs        # MapGenerator, MapGeneratorConfig
    ├── fog.rs              # FogOfWar, FogState, Vision
    ├── pathfinding.rs      # Pathfinder, PathFollow
    ├── camera.rs           # RtsCameraController
    ├── serialization.rs    # MapFile, load/save
    └── prelude.rs          # 公共导出

examples/rts_demo/
├── Cargo.toml
├── Trunk.toml
├── index.html
└── src/
    ├── main.rs             # 入口
    ├── components.rs       # Unit, Health, Attack, etc.
    ├── setup.rs            # 场景初始化
    ├── selection.rs        # 选中系统
    ├── movement.rs         # 移动系统
    ├── combat.rs           # 战斗系统
    ├── ai.rs               # 敌方 AI
    ├── fog_render.rs       # 迷雾渲染
    ├── ui.rs               # HUD
    └── editor.rs           # 编辑模式
```

---

## Phase 1: nova_map 核心模块

### Task 1: 项目脚手架

**Files:**
- Create: `crates/nova_map/Cargo.toml`
- Create: `crates/nova_map/src/lib.rs`
- Create: `crates/nova_map/src/prelude.rs`
- Modify: `Cargo.toml` (workspace members)
- Modify: `crates/nova_engine/Cargo.toml` (add nova_map dependency)
- Modify: `crates/nova_engine/src/lib.rs` (re-export nova_map)

- [ ] **Step 1: 创建 nova_map/Cargo.toml**

```toml
[package]
name = "nova_map"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Nova Engine 地图系统 - 瓦片地图、程序化生成、寻路、战争迷雾"

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
noise = "0.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
criterion = { workspace = true }
```

- [ ] **Step 2: 创建 nova_map/src/lib.rs**

```rust
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
```

- [ ] **Step 3: 创建 nova_map/src/prelude.rs**

```rust
//! 公共导出

pub use crate::camera::*;
pub use crate::fog::*;
pub use crate::generator::*;
pub use crate::heightmap::*;
pub use crate::pathfinding::*;
pub use crate::serialization::*;
pub use crate::tile::*;
pub use crate::tilemap::*;
pub use crate::NovaMapPlugin;
```

- [ ] **Step 4: 创建空的模块文件**

为每个模块创建空文件（后续任务填充）:
- `crates/nova_map/src/tile.rs`
- `crates/nova_map/src/tilemap.rs`
- `crates/nova_map/src/heightmap.rs`
- `crates/nova_map/src/generator.rs`
- `crates/nova_map/src/fog.rs`
- `crates/nova_map/src/pathfinding.rs`
- `crates/nova_map/src/camera.rs`
- `crates/nova_map/src/serialization.rs`

每个文件初始内容:
```rust
//! [模块名]
```

- [ ] **Step 5: 更新 workspace Cargo.toml**

在 `Cargo.toml` 的 `[workspace]` members 中添加:
```toml
members = [
    # ... existing members ...
    "crates/nova_map",  # 新增
]
```

在 `[workspace.dependencies]` 中添加:
```toml
nova_map = { path = "crates/nova_map" }
noise = "0.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

- [ ] **Step 6: 更新 nova_engine/Cargo.toml**

添加依赖:
```toml
nova_map = { workspace = true }
```

- [ ] **Step 7: 更新 nova_engine/src/lib.rs**

添加重导出:
```rust
pub use nova_map;
```

- [ ] **Step 8: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功，无错误

- [ ] **Step 9: 提交**

```bash
git add -A
git commit -m "feat(nova_map): 初始化地图系统模块脚手架"
```

---

### Task 2: Tile 和 TerrainType

**Files:**
- Create: `crates/nova_map/src/tile.rs`

- [ ] **Step 1: 实现 TerrainType**

```rust
//! 瓦片和地形类型定义

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 地形类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Reflect)]
pub enum TerrainType {
    /// 草地 - 正常移动速度
    #[default]
    Grass,
    /// 沙漠 - 移动速度 x0.8
    Desert,
    /// 水域 - 不可通行（除非水上单位）
    Water,
    /// 山地 - 不可通行
    Mountain,
    /// 森林 - 移动速度 x0.6，提供视野遮蔽
    Forest,
}

impl TerrainType {
    /// 获取移动代价（用于寻路权重）
    /// 返回 None 表示不可通行
    pub fn move_cost(&self) -> Option<f32> {
        match self {
            Self::Grass => Some(1.0),
            Self::Desert => Some(1.25),
            Self::Forest => Some(1.67),
            Self::Water | Self::Mountain => None,
        }
    }

    /// 是否可建造
    pub fn buildable(&self) -> bool {
        matches!(self, Self::Grass | Self::Desert)
    }

    /// 是否阻挡视野
    pub fn blocks_vision(&self) -> bool {
        matches!(self, Self::Mountain | Self::Forest)
    }

    /// 获取地形颜色（用于调试渲染）
    pub fn color(&self) -> Color {
        match self {
            Self::Grass => Color::srgb(0.3, 0.7, 0.3),
            Self::Desert => Color::srgb(0.9, 0.8, 0.5),
            Self::Water => Color::srgb(0.2, 0.4, 0.8),
            Self::Mountain => Color::srgb(0.5, 0.5, 0.5),
            Self::Forest => Color::srgb(0.1, 0.5, 0.2),
        }
    }
}
```

- [ ] **Step 2: 实现 Tile**

在同一文件中添加:

```rust
/// 单个瓦片
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Tile {
    /// 地形类型
    pub terrain: TerrainType,
    /// 高度值（0.0 ~ 1.0）
    pub height: f32,
    /// 是否被占用（有建筑/资源）
    pub occupied: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Grass,
            height: 0.5,
            occupied: false,
        }
    }
}

impl Tile {
    /// 创建新瓦片
    pub fn new(terrain: TerrainType, height: f32) -> Self {
        Self {
            terrain,
            height,
            occupied: false,
        }
    }

    /// 是否可通行
    pub fn walkable(&self) -> bool {
        !self.occupied && self.terrain.move_cost().is_some()
    }

    /// 获取移动代价
    pub fn move_cost(&self) -> Option<f32> {
        if self.occupied {
            None
        } else {
            self.terrain.move_cost()
        }
    }
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add crates/nova_map/src/tile.rs
git commit -m "feat(nova_map): 实现 Tile 和 TerrainType"
```

---

### Task 3: TileMap 数据结构

**Files:**
- Create: `crates/nova_map/src/tilemap.rs`

- [ ] **Step 1: 实现 TileMap 基础结构**

```rust
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
}
```

- [ ] **Step 2: 添加坐标转换方法**

```rust
impl TileMap {
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
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add crates/nova_map/src/tilemap.rs
git commit -m "feat(nova_map): 实现 TileMap 数据结构"
```

---

### Task 4: HeightMap

**Files:**
- Create: `crates/nova_map/src/heightmap.rs`

- [ ] **Step 1: 实现 HeightMap**

```rust
//! 高度图数据结构

use serde::{Deserialize, Serialize};

/// 高度图数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeightMap {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
    /// 高度数据（0.0 ~ 1.0）
    data: Vec<f32>,
}

impl HeightMap {
    /// 创建空高度图
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0.5; (width * height) as usize],
        }
    }

    /// 从数据创建
    pub fn from_data(width: u32, height: u32, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Self {
            width,
            height,
            data,
        }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 检查坐标是否在范围内
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// 获取指定位置的高度值
    pub fn get(&self, x: u32, y: u32) -> f32 {
        if self.in_bounds(x, y) {
            self.data[(y * self.width + x) as usize]
        } else {
            0.0
        }
    }

    /// 设置指定位置的高度值
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        if self.in_bounds(x, y) {
            self.data[(y * self.width + x) as usize] = value.clamp(0.0, 1.0);
        }
    }

    /// 获取插值高度（双线性插值）
    pub fn get_interpolated(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x.fract();
        let fy = y.fract();

        let v00 = self.get(x0, y0);
        let v10 = self.get(x1, y0);
        let v01 = self.get(x0, y1);
        let v11 = self.get(x1, y1);

        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;

        v0 * (1.0 - fy) + v1 * fy
    }

    /// 归一化高度值到 0.0 ~ 1.0
    pub fn normalize(&mut self) {
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;

        if range > 0.0 {
            for v in &mut self.data {
                *v = (*v - min) / range;
            }
        }
    }

    /// 迭代所有高度值
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, f32)> + '_ {
        self.data.iter().enumerate().map(move |(idx, &h)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            (x, y, h)
        })
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_map/src/heightmap.rs
git commit -m "feat(nova_map): 实现 HeightMap 数据结构"
```

---

### Task 5: MapGenerator 程序化生成

**Files:**
- Create: `crates/nova_map/src/generator.rs`

- [ ] **Step 1: 实现 MapGeneratorConfig**

```rust
//! 程序化地图生成器

use noise::{NoiseFn, Perlin, Seedable};
use serde::{Deserialize, Serialize};

use crate::heightmap::HeightMap;
use crate::tile::{Tile, TerrainType};
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
```

- [ ] **Step 2: 实现 MapGenerator**

```rust
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
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add crates/nova_map/src/generator.rs
git commit -m "feat(nova_map): 实现 MapGenerator 程序化地图生成"
```

---

### Task 6: 战争迷雾系统

**Files:**
- Create: `crates/nova_map/src/fog.rs`

- [ ] **Step 1: 实现 FogState 和 FogOfWar**

```rust
//! 战争迷雾系统

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 迷雾状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
pub enum FogState {
    /// 未探索 - 完全黑暗
    #[default]
    Unexplored,
    /// 已探索 - 显示地形，不显示敌人
    Explored,
    /// 可见 - 在当前视野内
    Visible,
}

/// 战争迷雾资源
#[derive(Resource, Clone, Debug)]
pub struct FogOfWar {
    /// 地图宽度
    width: u32,
    /// 地图高度
    height: u32,
    /// 每个瓦片的迷雾状态
    states: Vec<FogState>,
    /// 视野计数（多少单位能看到该瓦片）
    vision_count: Vec<u32>,
    /// 是否启用
    pub enabled: bool,
}

impl FogOfWar {
    /// 创建新的迷雾系统
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            states: vec![FogState::Unexplored; size],
            vision_count: vec![0; size],
            enabled: true,
        }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 检查坐标是否在范围内
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// 坐标转索引
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// 获取迷雾状态
    pub fn get_state(&self, x: u32, y: u32) -> FogState {
        if !self.enabled {
            return FogState::Visible;
        }
        if self.in_bounds(x, y) {
            self.states[self.index(x, y)]
        } else {
            FogState::Unexplored
        }
    }

    /// 获取视野计数
    pub fn get_vision_count(&self, x: u32, y: u32) -> u32 {
        if self.in_bounds(x, y) {
            self.vision_count[self.index(x, y)]
        } else {
            0
        }
    }

    /// 添加视野
    pub fn add_vision(&mut self, center_x: u32, center_y: u32, range: u32) {
        let range_sq = (range * range) as i32;

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                if dx * dx + dy * dy > range_sq {
                    continue;
                }

                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;

                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    let x = x as u32;
                    let y = y as u32;
                    let idx = self.index(x, y);
                    self.vision_count[idx] += 1;
                }
            }
        }
    }

    /// 移除视野
    pub fn remove_vision(&mut self, center_x: u32, center_y: u32, range: u32) {
        let range_sq = (range * range) as i32;

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                if dx * dx + dy * dy > range_sq {
                    continue;
                }

                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;

                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    let x = x as u32;
                    let y = y as u32;
                    let idx = self.index(x, y);
                    self.vision_count[idx] = self.vision_count[idx].saturating_sub(1);
                }
            }
        }
    }

    /// 更新迷雾状态
    pub fn update_states(&mut self) {
        for idx in 0..self.states.len() {
            if self.vision_count[idx] > 0 {
                self.states[idx] = FogState::Visible;
            } else if self.states[idx] == FogState::Visible {
                // 从可见变为已探索
                self.states[idx] = FogState::Explored;
            }
            // Unexplored 保持不变
        }
    }

    /// 重置所有迷雾
    pub fn reset(&mut self) {
        self.states.fill(FogState::Unexplored);
        self.vision_count.fill(0);
    }

    /// 揭示全部地图
    pub fn reveal_all(&mut self) {
        self.states.fill(FogState::Visible);
    }
}

/// 视野组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Vision {
    /// 视野半径（瓦片数）
    pub range: u32,
    /// 上一帧的瓦片位置
    pub last_tile: Option<(u32, u32)>,
}

impl Vision {
    pub fn new(range: u32) -> Self {
        Self {
            range,
            last_tile: None,
        }
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_map/src/fog.rs
git commit -m "feat(nova_map): 实现战争迷雾系统"
```

---

### Task 7: A* 寻路

**Files:**
- Create: `crates/nova_map/src/pathfinding.rs`

- [ ] **Step 1: 实现 Pathfinder**

```rust
//! A* 寻路算法

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use bevy::prelude::*;

use crate::tilemap::TileMap;

/// 寻路结果
#[derive(Clone, Debug)]
pub struct PathResult {
    /// 路径点序列（不包含起点）
    pub path: Vec<(u32, u32)>,
    /// 总代价
    pub cost: f32,
}

/// 寻路节点（用于优先队列）
#[derive(Clone, Debug)]
struct PathNode {
    pos: (u32, u32),
    f_score: f32, // g + h
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反向排序（最小堆）
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 寻路器
pub struct Pathfinder;

impl Pathfinder {
    /// A* 寻路
    pub fn find_path(
        tilemap: &TileMap,
        start: (u32, u32),
        goal: (u32, u32),
    ) -> Option<PathResult> {
        if start == goal {
            return Some(PathResult {
                path: vec![],
                cost: 0.0,
            });
        }

        // 检查目标是否可达
        if !tilemap.get(goal.0, goal.1).map(|t| t.walkable()).unwrap_or(false) {
            return None;
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
        let mut g_score: HashMap<(u32, u32), f32> = HashMap::new();

        g_score.insert(start, 0.0);
        open_set.push(PathNode {
            pos: start,
            f_score: Self::heuristic(start, goal),
        });

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                // 重建路径
                let path = Self::reconstruct_path(&came_from, goal);
                let cost = g_score[&goal];
                return Some(PathResult { path, cost });
            }

            let current_g = g_score[&current.pos];

            // 遍历邻居（8方向）
            for neighbor in tilemap.neighbors8(current.pos.0, current.pos.1) {
                // 检查是否可通行
                let Some(tile) = tilemap.get(neighbor.0, neighbor.1) else {
                    continue;
                };
                let Some(move_cost) = tile.move_cost() else {
                    continue;
                };

                // 对角线移动代价更高
                let dx = (neighbor.0 as i32 - current.pos.0 as i32).abs();
                let dy = (neighbor.1 as i32 - current.pos.1 as i32).abs();
                let distance = if dx + dy == 2 { 1.414 } else { 1.0 };

                let tentative_g = current_g + move_cost * distance;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                    came_from.insert(neighbor, current.pos);
                    g_score.insert(neighbor, tentative_g);

                    let f_score = tentative_g + Self::heuristic(neighbor, goal);
                    open_set.push(PathNode {
                        pos: neighbor,
                        f_score,
                    });
                }
            }
        }

        None // 无法到达
    }

    /// 启发函数（欧几里得距离）
    fn heuristic(a: (u32, u32), b: (u32, u32)) -> f32 {
        let dx = (a.0 as f32 - b.0 as f32).abs();
        let dy = (a.1 as f32 - b.1 as f32).abs();
        (dx * dx + dy * dy).sqrt()
    }

    /// 重建路径
    fn reconstruct_path(
        came_from: &HashMap<(u32, u32), (u32, u32)>,
        goal: (u32, u32),
    ) -> Vec<(u32, u32)> {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.pop(); // 移除起点
        path.reverse();
        path
    }
}

/// 路径跟随组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct PathFollow {
    /// 路径点序列
    pub path: Vec<(u32, u32)>,
    /// 当前目标点索引
    pub current_index: usize,
    /// 是否到达终点
    pub finished: bool,
}

impl PathFollow {
    pub fn new(path: Vec<(u32, u32)>) -> Self {
        Self {
            path,
            current_index: 0,
            finished: false,
        }
    }

    /// 获取当前目标瓦片
    pub fn current_target(&self) -> Option<(u32, u32)> {
        self.path.get(self.current_index).copied()
    }

    /// 前进到下一个目标点
    pub fn advance(&mut self) {
        self.current_index += 1;
        if self.current_index >= self.path.len() {
            self.finished = true;
        }
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_map/src/pathfinding.rs
git commit -m "feat(nova_map): 实现 A* 寻路算法"
```

---

### Task 8: RTS 相机控制器

**Files:**
- Create: `crates/nova_map/src/camera.rs`

- [ ] **Step 1: 实现 RtsCameraController**

```rust
//! RTS 相机控制器

use bevy::prelude::*;

/// RTS 相机控制器组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct RtsCameraController {
    /// 移动速度（世界单位/秒）
    pub move_speed: f32,
    /// 缩放速度
    pub zoom_speed: f32,
    /// 缩放范围（最小高度, 最大高度）
    pub zoom_min: f32,
    pub zoom_max: f32,
    /// 边缘滚动触发区域（像素）
    pub edge_margin: f32,
    /// 是否启用边缘滚动
    pub edge_scroll_enabled: bool,
    /// 相机移动边界
    pub bounds_min: Option<Vec2>,
    pub bounds_max: Option<Vec2>,
}

impl Default for RtsCameraController {
    fn default() -> Self {
        Self {
            move_speed: 20.0,
            zoom_speed: 10.0,
            zoom_min: 10.0,
            zoom_max: 50.0,
            edge_margin: 20.0,
            edge_scroll_enabled: true,
            bounds_min: None,
            bounds_max: None,
        }
    }
}

impl RtsCameraController {
    /// 设置边界
    pub fn with_bounds(mut self, min: Vec2, max: Vec2) -> Self {
        self.bounds_min = Some(min);
        self.bounds_max = Some(max);
        self
    }

    /// 根据地图尺寸设置边界
    pub fn with_map_bounds(self, width: f32, height: f32, margin: f32) -> Self {
        let half_w = width / 2.0 + margin;
        let half_h = height / 2.0 + margin;
        self.with_bounds(Vec2::new(-half_w, -half_h), Vec2::new(half_w, half_h))
    }
}

/// RTS 相机控制系统
pub fn rts_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &RtsCameraController)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    for (mut transform, controller) in cameras.iter_mut() {
        let mut move_dir = Vec3::ZERO;

        // 键盘移动
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            move_dir.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            move_dir.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            move_dir.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            move_dir.x += 1.0;
        }

        // 边缘滚动
        if controller.edge_scroll_enabled {
            if let Some(cursor_pos) = window.cursor_position() {
                let margin = controller.edge_margin;
                let width = window.width();
                let height = window.height();

                if cursor_pos.x < margin {
                    move_dir.x -= 1.0;
                }
                if cursor_pos.x > width - margin {
                    move_dir.x += 1.0;
                }
                if cursor_pos.y < margin {
                    move_dir.z -= 1.0;
                }
                if cursor_pos.y > height - margin {
                    move_dir.z += 1.0;
                }
            }
        }

        // 应用移动
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            transform.translation += move_dir * controller.move_speed * time.delta_secs();
        }

        // 应用边界限制
        if let (Some(min), Some(max)) = (controller.bounds_min, controller.bounds_max) {
            transform.translation.x = transform.translation.x.clamp(min.x, max.x);
            transform.translation.z = transform.translation.z.clamp(min.y, max.y);
        }
    }
}

/// RTS 相机缩放系统（需要 bevy_egui 的 mouse wheel 事件）
pub fn rts_camera_zoom_system(
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    mut cameras: Query<(&mut Transform, &RtsCameraController)>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut scroll_delta = 0.0;
    for ev in scroll_evr.read() {
        scroll_delta += match ev.unit {
            MouseScrollUnit::Line => ev.y * 3.0,
            MouseScrollUnit::Pixel => ev.y * 0.1,
        };
    }

    if scroll_delta.abs() < 0.001 {
        return;
    }

    for (mut transform, controller) in cameras.iter_mut() {
        let current_y = transform.translation.y;
        let new_y = (current_y - scroll_delta * controller.zoom_speed)
            .clamp(controller.zoom_min, controller.zoom_max);
        transform.translation.y = new_y;
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_map/src/camera.rs
git commit -m "feat(nova_map): 实现 RTS 相机控制器"
```

---

### Task 9: 地图序列化

**Files:**
- Create: `crates/nova_map/src/serialization.rs`

- [ ] **Step 1: 实现 MapFile 格式**

```rust
//! 地图序列化

use serde::{Deserialize, Serialize};

use crate::tile::{Tile, TerrainType};
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
```

- [ ] **Step 2: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_map/src/serialization.rs
git commit -m "feat(nova_map): 实现地图序列化"
```

---

### Task 10: NovaMapPlugin 完善

**Files:**
- Modify: `crates/nova_map/src/lib.rs`

- [ ] **Step 1: 更新 lib.rs 添加系统**

```rust
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
```

- [ ] **Step 2: 更新 prelude.rs**

```rust
//! 公共导出

pub use crate::camera::*;
pub use crate::fog::*;
pub use crate::generator::*;
pub use crate::heightmap::*;
pub use crate::pathfinding::*;
pub use crate::serialization::*;
pub use crate::tile::*;
pub use crate::tilemap::*;
pub use crate::{NovaMapPlugin, NovaMapWithFogPlugin};
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_map`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add crates/nova_map/src/lib.rs crates/nova_map/src/prelude.rs
git commit -m "feat(nova_map): 完善 NovaMapPlugin 系统注册"
```

---

## Phase 2: RTS Demo

### Task 11: RTS Demo 脚手架

**Files:**
- Create: `examples/rts_demo/Cargo.toml`
- Create: `examples/rts_demo/Trunk.toml`
- Create: `examples/rts_demo/index.html`
- Create: `examples/rts_demo/src/main.rs`
- Modify: `Cargo.toml` (workspace members)

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "rts_demo"
version = "0.1.0"
edition = "2021"
description = "RTS 游戏原型 - Nova Engine 示例"

[dependencies]
nova_engine = { path = "../../crates/nova_engine" }
nova_map = { path = "../../crates/nova_map" }
bevy = { workspace = true, features = ["bevy_state"] }
bevy_rapier3d = { workspace = true }
bevy_egui = { workspace = true }
```

- [ ] **Step 2: 创建 Trunk.toml**

```toml
[build]
target = "index.html"

[watch]
watch = ["src", "index.html"]

[[hooks]]
stage = "pre_build"
command = "cargo"
command_arguments = ["fmt", "--", "--check"]
```

- [ ] **Step 3: 创建 index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nova Engine - RTS Demo</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html, body {
            width: 100%;
            height: 100%;
            overflow: hidden;
            background-color: #1a1a2e;
        }
        #nova-canvas {
            width: 100%;
            height: 100%;
            outline: none;
        }
        #nova-canvas:focus {
            outline: none;
        }
        .loading {
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: #fff;
            font-family: sans-serif;
            font-size: 18px;
            text-align: center;
        }
        .loading h1 {
            font-size: 32px;
            margin-bottom: 20px;
            color: #4af;
        }
        .loading p {
            color: #888;
        }
    </style>
</head>
<body>
    <div class="loading" id="loading">
        <h1>RTS Demo</h1>
        <p>加载中...</p>
    </div>
    <canvas id="nova-canvas" tabindex="0"></canvas>
    <script type="module">
        import init from './rts_demo.js';

        init().then(() => {
            document.getElementById('loading').style.display = 'none';
            const canvas = document.getElementById('nova-canvas');
            canvas.focus();
            canvas.addEventListener('click', () => canvas.focus());
        }).catch(err => {
            document.getElementById('loading').innerHTML = '<h1>加载失败</h1><p>' + err + '</p>';
        });
    </script>
</body>
</html>
```

- [ ] **Step 4: 创建 main.rs 入口**

```rust
//! RTS 游戏原型
//!
//! Nova Engine 示例 - 展示地图系统、单位控制、战斗系统

use bevy::prelude::*;
use nova_engine::prelude::*;
use nova_map::prelude::*;

mod components;
mod setup;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_systems(Startup, setup::setup_game)
        .run();
}
```

- [ ] **Step 5: 创建 components.rs**

```rust
//! 游戏组件定义

use bevy::prelude::*;

/// 单位标记
#[derive(Component)]
pub struct Unit;

/// 阵营
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Player,
    Enemy,
}

/// 生命值
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// 攻击能力
#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    pub timer: f32,
}

impl Attack {
    pub fn new(damage: f32, range: f32, cooldown: f32) -> Self {
        Self {
            damage,
            range,
            cooldown,
            timer: 0.0,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn reset_cooldown(&mut self) {
        self.timer = self.cooldown;
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer = (self.timer - delta).max(0.0);
    }
}

/// 移动能力
#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

/// 可选中标记
#[derive(Component)]
pub struct Selectable;

/// 当前被选中
#[derive(Component)]
pub struct Selected;

/// 攻击目标
#[derive(Component)]
pub struct AttackTarget(pub Entity);

/// 资源点
#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Crystal,
    Gas,
}

/// 游戏模式
#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameMode {
    #[default]
    Playing,
    Editor,
}
```

- [ ] **Step 6: 创建 setup.rs**

```rust
//! 场景初始化

use bevy::prelude::*;
use nova_map::prelude::*;

use crate::components::*;

/// 初始化游戏场景
pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 生成地图
    let config = MapGeneratorConfig {
        seed: 42,
        size: (64, 64),
        tile_size: 1.0,
        ..default()
    };
    let tilemap = MapGenerator::generate(&config);

    // 创建迷雾
    let fog = FogOfWar::new(tilemap.width(), tilemap.height());

    // 渲染地图
    spawn_terrain(&mut commands, &mut meshes, &mut materials, &tilemap);

    // 插入资源
    let map_width = tilemap.width() as f32 * tilemap.tile_size();
    let map_height = tilemap.height() as f32 * tilemap.tile_size();
    commands.insert_resource(tilemap);
    commands.insert_resource(fog);

    // 创建相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        RtsCameraController::default().with_map_bounds(map_width, map_height, 5.0),
    ));

    // 光照
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });

    // 生成玩家单位
    spawn_player_units(&mut commands, &mut meshes, &mut materials);

    // 生成敌方单位
    spawn_enemy_units(&mut commands, &mut meshes, &mut materials);
}

/// 渲染地形
fn spawn_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tilemap: &TileMap,
) {
    let tile_mesh = meshes.add(Cuboid::new(
        tilemap.tile_size() * 0.95,
        0.2,
        tilemap.tile_size() * 0.95,
    ));

    for (x, y, tile) in tilemap.iter() {
        let world_pos = tilemap.tile_to_world(x, y);
        let color = tile.terrain.color();

        let material = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.9,
            ..default()
        });

        commands.spawn((
            Mesh3d(tile_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos - Vec3::Y * 0.1),
        ));
    }
}

/// 生成玩家单位
fn spawn_player_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let unit_mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let unit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 1.0),
        ..default()
    });

    // 在左下角生成 3 个单位
    let positions = [
        Vec3::new(-25.0, 0.5, -25.0),
        Vec3::new(-23.0, 0.5, -25.0),
        Vec3::new(-24.0, 0.5, -23.0),
    ];

    for pos in positions {
        commands.spawn((
            Unit,
            Team::Player,
            Health::new(100.0),
            Attack::new(10.0, 5.0, 1.0),
            Movement::new(5.0),
            Selectable,
            Vision::new(8),
            Mesh3d(unit_mesh.clone()),
            MeshMaterial3d(unit_material.clone()),
            Transform::from_translation(pos),
        ));
    }
}

/// 生成敌方单位
fn spawn_enemy_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let unit_mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let unit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.3, 0.2),
        ..default()
    });

    // 在右上角生成 3 个敌人
    let positions = [
        Vec3::new(25.0, 0.5, 25.0),
        Vec3::new(23.0, 0.5, 25.0),
        Vec3::new(24.0, 0.5, 23.0),
    ];

    for pos in positions {
        commands.spawn((
            Unit,
            Team::Enemy,
            Health::new(100.0),
            Attack::new(10.0, 5.0, 1.0),
            Movement::new(4.0),
            Vision::new(8),
            Mesh3d(unit_mesh.clone()),
            MeshMaterial3d(unit_material.clone()),
            Transform::from_translation(pos),
        ));
    }
}
```

- [ ] **Step 7: 更新 workspace Cargo.toml**

在 members 中添加:
```toml
"examples/rts_demo",
```

- [ ] **Step 8: 验证编译**

Run: `cargo check -p rts_demo`
Expected: 编译成功

- [ ] **Step 9: 提交**

```bash
git add examples/rts_demo Cargo.toml
git commit -m "feat(examples): 添加 RTS Demo 脚手架"
```

---

### Task 12: 单位选中系统

**Files:**
- Create: `examples/rts_demo/src/selection.rs`
- Modify: `examples/rts_demo/src/main.rs`

- [ ] **Step 1: 实现选中系统**

```rust
//! 单位选中系统

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use nova_map::prelude::*;

use crate::components::*;

/// 选择框资源
#[derive(Resource, Default)]
pub struct SelectionBox {
    pub active: bool,
    pub start: Vec2,
    pub end: Vec2,
}

/// 选中系统插件
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionBox>()
            .add_systems(Update, (selection_system, render_selection_indicators));
    }
}

/// 主选中系统
fn selection_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    mut selection_box: ResMut<SelectionBox>,
    selectables: Query<(Entity, &Transform, &Team), With<Selectable>>,
    selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // 左键按下：开始框选
    if mouse_button.just_pressed(MouseButton::Left) {
        selection_box.active = true;
        selection_box.start = cursor_pos;
        selection_box.end = cursor_pos;
    }

    // 左键拖拽：更新框选
    if mouse_button.pressed(MouseButton::Left) && selection_box.active {
        selection_box.end = cursor_pos;
    }

    // 左键释放：完成选择
    if mouse_button.just_released(MouseButton::Left) && selection_box.active {
        selection_box.active = false;

        // 清除之前的选中
        for entity in selected.iter() {
            commands.entity(entity).remove::<Selected>();
        }

        let drag_distance = (selection_box.end - selection_box.start).length();

        if drag_distance < 5.0 {
            // 点击选择：选中点击位置的单位
            if let Some(world_pos) = screen_to_ground(cursor_pos, camera, camera_transform) {
                for (entity, transform, team) in selectables.iter() {
                    if *team != Team::Player {
                        continue;
                    }
                    let distance = (transform.translation - world_pos).length();
                    if distance < 1.0 {
                        commands.entity(entity).insert(Selected);
                        break;
                    }
                }
            }
        } else {
            // 框选：选中框内所有己方单位
            let min_x = selection_box.start.x.min(selection_box.end.x);
            let max_x = selection_box.start.x.max(selection_box.end.x);
            let min_y = selection_box.start.y.min(selection_box.end.y);
            let max_y = selection_box.start.y.max(selection_box.end.y);

            for (entity, transform, team) in selectables.iter() {
                if *team != Team::Player {
                    continue;
                }

                if let Some(screen_pos) =
                    camera.world_to_viewport(camera_transform, transform.translation)
                {
                    if screen_pos.x >= min_x
                        && screen_pos.x <= max_x
                        && screen_pos.y >= min_y
                        && screen_pos.y <= max_y
                    {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
        }
    }
}

/// 屏幕坐标转世界地面坐标
fn screen_to_ground(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let ray = camera.viewport_to_world(camera_transform, screen_pos).ok()?;

    // 与 Y=0 平面求交
    let t = -ray.origin.y / ray.direction.y;
    if t > 0.0 {
        Some(ray.origin + ray.direction * t)
    } else {
        None
    }
}

/// 渲染选中指示器
fn render_selection_indicators(
    selected: Query<&Transform, With<Selected>>,
    mut gizmos: Gizmos,
) {
    for transform in selected.iter() {
        let pos = transform.translation;
        gizmos.circle(
            Isometry3d::new(
                pos + Vec3::Y * 0.1,
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
            0.6,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}
```

- [ ] **Step 2: 更新 main.rs**

```rust
//! RTS 游戏原型

use bevy::prelude::*;
use nova_engine::prelude::*;
use nova_map::prelude::*;

mod components;
mod selection;
mod setup;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_systems(Startup, setup::setup_game)
        .run();
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p rts_demo`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add examples/rts_demo/src/selection.rs examples/rts_demo/src/main.rs
git commit -m "feat(rts_demo): 实现单位选中系统"
```

---

### Task 13: 移动系统

**Files:**
- Create: `examples/rts_demo/src/movement.rs`
- Modify: `examples/rts_demo/src/main.rs`

- [ ] **Step 1: 实现移动系统**

```rust
//! 移动系统

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use nova_map::prelude::*;

use crate::components::*;
use crate::selection::screen_to_ground;

/// 移动系统插件
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_command_system, path_follow_system));
    }
}

/// 移动指令系统
fn move_command_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    tilemap: Option<Res<TileMap>>,
    selected_units: Query<(Entity, &Transform), (With<Selected>, With<Movement>)>,
    enemies: Query<(Entity, &Transform, &Team)>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Some(world_pos) = screen_to_ground(cursor_pos, camera, camera_transform) else {
        return;
    };

    // 检查是否点击了敌人
    let mut clicked_enemy = None;
    for (entity, transform, team) in enemies.iter() {
        if *team == Team::Enemy {
            let distance = (transform.translation - world_pos).length();
            if distance < 1.0 {
                clicked_enemy = Some(entity);
                break;
            }
        }
    }

    // 如果点击敌人，设置攻击目标
    if let Some(enemy_entity) = clicked_enemy {
        for (entity, _) in selected_units.iter() {
            commands
                .entity(entity)
                .remove::<PathFollow>()
                .insert(AttackTarget(enemy_entity));
        }
        return;
    }

    // 否则移动到目标位置
    let Some(goal_tile) = tilemap.world_to_tile(world_pos) else {
        return;
    };

    for (entity, transform) in selected_units.iter() {
        let Some(start_tile) = tilemap.world_to_tile(transform.translation) else {
            continue;
        };

        if let Some(result) = Pathfinder::find_path(&tilemap, start_tile, goal_tile) {
            commands
                .entity(entity)
                .remove::<AttackTarget>()
                .insert(PathFollow::new(result.path));
        }
    }
}

/// 路径跟随系统
fn path_follow_system(
    time: Res<Time>,
    tilemap: Option<Res<TileMap>>,
    mut units: Query<(Entity, &mut Transform, &Movement, &mut PathFollow)>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, mut transform, movement, mut path) in units.iter_mut() {
        if path.finished {
            commands.entity(entity).remove::<PathFollow>();
            continue;
        }

        let Some(target_tile) = path.current_target() else {
            path.finished = true;
            continue;
        };

        let target_pos = tilemap.tile_to_world(target_tile.0, target_tile.1);
        let direction = target_pos - transform.translation;
        let distance = direction.length();

        if distance < 0.3 {
            // 到达当前目标点
            path.advance();
        } else {
            // 移动向目标
            let move_dir = direction.normalize();
            let move_amount = movement.speed * time.delta_secs();
            transform.translation += move_dir * move_amount.min(distance);

            // 朝向移动方向
            if direction.x.abs() > 0.01 || direction.z.abs() > 0.01 {
                let target_rotation = Quat::from_rotation_y((-direction.x).atan2(-direction.z));
                transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
            }
        }
    }
}

/// 导出 screen_to_ground 供其他模块使用
pub use crate::selection::screen_to_ground;
```

- [ ] **Step 2: 更新 selection.rs 导出函数**

在 `selection.rs` 中将 `screen_to_ground` 改为 `pub`:

```rust
/// 屏幕坐标转世界地面坐标
pub fn screen_to_ground(
    // ... 保持不变
```

- [ ] **Step 3: 更新 main.rs**

```rust
//! RTS 游戏原型

use bevy::prelude::*;
use nova_engine::prelude::*;
use nova_map::prelude::*;

mod components;
mod movement;
mod selection;
mod setup;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_systems(Startup, setup::setup_game)
        .run();
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p rts_demo`
Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add examples/rts_demo/src/movement.rs examples/rts_demo/src/selection.rs examples/rts_demo/src/main.rs
git commit -m "feat(rts_demo): 实现移动和寻路系统"
```

---

### Task 14: 战斗系统

**Files:**
- Create: `examples/rts_demo/src/combat.rs`
- Modify: `examples/rts_demo/src/main.rs`

- [ ] **Step 1: 实现战斗系统**

```rust
//! 战斗系统

use bevy::prelude::*;
use nova_map::prelude::*;

use crate::components::*;

/// 战斗系统插件
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                attack_cooldown_system,
                chase_target_system,
                combat_system,
                auto_target_system,
                death_system,
            ),
        );
    }
}

/// 攻击冷却更新
fn attack_cooldown_system(time: Res<Time>, mut units: Query<&mut Attack>) {
    for mut attack in units.iter_mut() {
        attack.tick(time.delta_secs());
    }
}

/// 追击目标系统
fn chase_target_system(
    tilemap: Option<Res<TileMap>>,
    mut attackers: Query<
        (Entity, &Transform, &Attack, &AttackTarget),
        (With<Movement>, Without<PathFollow>),
    >,
    targets: Query<&Transform, With<Health>>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, attacker_transform, attack, target) in attackers.iter_mut() {
        let Ok(target_transform) = targets.get(target.0) else {
            // 目标不存在，移除攻击目标
            commands.entity(entity).remove::<AttackTarget>();
            continue;
        };

        let distance = (attacker_transform.translation - target_transform.translation).length();

        // 如果不在攻击范围内，移动靠近
        if distance > attack.range {
            let Some(start_tile) = tilemap.world_to_tile(attacker_transform.translation) else {
                continue;
            };
            let Some(goal_tile) = tilemap.world_to_tile(target_transform.translation) else {
                continue;
            };

            if let Some(result) = Pathfinder::find_path(&tilemap, start_tile, goal_tile) {
                commands.entity(entity).insert(PathFollow::new(result.path));
            }
        }
    }
}

/// 战斗执行系统
fn combat_system(
    mut attackers: Query<(&Transform, &mut Attack, &AttackTarget, &Team)>,
    mut targets: Query<(&Transform, &mut Health, &Team)>,
) {
    for (attacker_transform, mut attack, target, attacker_team) in attackers.iter_mut() {
        let Ok((target_transform, mut target_health, target_team)) = targets.get_mut(target.0)
        else {
            continue;
        };

        // 不能攻击同队
        if attacker_team == target_team {
            continue;
        }

        let distance = (attacker_transform.translation - target_transform.translation).length();

        // 在范围内且冷却完成
        if distance <= attack.range && attack.can_attack() {
            target_health.take_damage(attack.damage);
            attack.reset_cooldown();
        }
    }
}

/// 自动索敌系统
fn auto_target_system(
    units: Query<
        (Entity, &Transform, &Team, &Attack),
        (With<Unit>, Without<AttackTarget>, Without<PathFollow>),
    >,
    potential_targets: Query<(Entity, &Transform, &Team), With<Health>>,
    mut commands: Commands,
) {
    for (entity, transform, team, attack) in units.iter() {
        let mut closest_enemy: Option<(Entity, f32)> = None;

        for (target_entity, target_transform, target_team) in potential_targets.iter() {
            // 跳过同队
            if team == target_team {
                continue;
            }

            let distance = (transform.translation - target_transform.translation).length();

            // 检测范围为攻击范围的 2 倍
            if distance <= attack.range * 2.0 {
                if closest_enemy.is_none() || distance < closest_enemy.unwrap().1 {
                    closest_enemy = Some((target_entity, distance));
                }
            }
        }

        if let Some((target_entity, _)) = closest_enemy {
            commands.entity(entity).insert(AttackTarget(target_entity));
        }
    }
}

/// 死亡清理系统
fn death_system(units: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in units.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

- [ ] **Step 2: 更新 main.rs**

```rust
//! RTS 游戏原型

use bevy::prelude::*;
use nova_engine::prelude::*;
use nova_map::prelude::*;

mod combat;
mod components;
mod movement;
mod selection;
mod setup;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_systems(Startup, setup::setup_game)
        .run();
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p rts_demo`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add examples/rts_demo/src/combat.rs examples/rts_demo/src/main.rs
git commit -m "feat(rts_demo): 实现战斗系统"
```

---

### Task 15: UI 系统

**Files:**
- Create: `examples/rts_demo/src/ui.rs`
- Modify: `examples/rts_demo/src/main.rs`

- [ ] **Step 1: 实现 UI 系统**

```rust
//! UI 系统

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::*;
use crate::selection::SelectionBox;

/// UI 系统插件
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (game_ui, selection_box_ui));
    }
}

/// 游戏主 UI
fn game_ui(
    mut contexts: EguiContexts,
    selected_units: Query<(&Health, &Attack), With<Selected>>,
    player_units: Query<Entity, (With<Unit>, With<Team>)>,
    enemy_units: Query<Entity, (With<Unit>, With<Team>)>,
) {
    // 选中单位信息面板
    egui::Window::new("单位信息")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            let selected_count = selected_units.iter().count();

            if selected_count == 0 {
                ui.label("未选中单位");
            } else if selected_count == 1 {
                // 单个单位详细信息
                if let Some((health, attack)) = selected_units.iter().next() {
                    ui.horizontal(|ui| {
                        ui.label("生命值:");
                        let progress = health.percentage();
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .text(format!("{:.0}/{:.0}", health.current, health.max)),
                        );
                    });
                    ui.label(format!("攻击力: {:.0}", attack.damage));
                    ui.label(format!("攻击范围: {:.1}", attack.range));
                }
            } else {
                // 多个单位简略信息
                ui.label(format!("已选中 {} 个单位", selected_count));

                let total_health: f32 = selected_units.iter().map(|(h, _)| h.current).sum();
                let max_health: f32 = selected_units.iter().map(|(h, _)| h.max).sum();
                ui.horizontal(|ui| {
                    ui.label("总生命值:");
                    ui.add(
                        egui::ProgressBar::new(total_health / max_health)
                            .text(format!("{:.0}/{:.0}", total_health, max_health)),
                    );
                });
            }
        });

    // 操作提示
    egui::Window::new("操作提示")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("左键: 选中/框选  |  右键: 移动/攻击  |  WASD: 移动相机  |  滚轮: 缩放");
        });

    // 单位统计
    egui::Window::new("统计")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            let player_count = player_units
                .iter()
                .filter(|_| true) // 实际应该检查 Team::Player
                .count();
            let enemy_count = enemy_units.iter().count() - player_count;

            ui.colored_label(egui::Color32::from_rgb(50, 150, 255), format!("我方: {}", player_count / 2));
            ui.colored_label(egui::Color32::from_rgb(255, 80, 50), format!("敌方: {}", enemy_count / 2));
        });
}

/// 选择框渲染
fn selection_box_ui(mut contexts: EguiContexts, selection_box: Res<SelectionBox>) {
    if !selection_box.active {
        return;
    }

    let painter = contexts.ctx_mut().layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("selection_box"),
    ));

    let min = egui::pos2(
        selection_box.start.x.min(selection_box.end.x),
        selection_box.start.y.min(selection_box.end.y),
    );
    let max = egui::pos2(
        selection_box.start.x.max(selection_box.end.x),
        selection_box.start.y.max(selection_box.end.y),
    );

    let rect = egui::Rect::from_min_max(min, max);

    // 绘制半透明填充
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_unmultiplied(0, 255, 0, 30));

    // 绘制边框
    painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::GREEN));
}
```

- [ ] **Step 2: 更新 main.rs**

```rust
//! RTS 游戏原型

use bevy::prelude::*;
use nova_engine::prelude::*;
use nova_map::prelude::*;

mod combat;
mod components;
mod movement;
mod selection;
mod setup;
mod ui;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(ui::UiPlugin)
        .add_systems(Startup, setup::setup_game)
        .run();
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p rts_demo`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add examples/rts_demo/src/ui.rs examples/rts_demo/src/main.rs
git commit -m "feat(rts_demo): 实现 UI 系统"
```

---

### Task 16: 最终验证和运行

**Files:**
- All rts_demo files

- [ ] **Step 1: WASM 编译检查**

Run: `cargo check --target wasm32-unknown-unknown -p rts_demo`
Expected: 编译成功

- [ ] **Step 2: 运行 Demo**

Run: `cd examples/rts_demo && trunk serve`
Expected:
- 打开浏览器访问 http://localhost:8080
- 显示 64x64 的彩色地形
- 可以用 WASD 移动相机
- 可以看到左下角 3 个蓝色单位（玩家）
- 可以看到右上角 3 个红色单位（敌人）
- 左键点击/框选可以选中玩家单位
- 右键点击地面，选中单位会移动过去
- 右键点击敌人，选中单位会追击并攻击

- [ ] **Step 3: 提交最终版本**

```bash
git add -A
git commit -m "feat(rts_demo): 完成 RTS Demo 原型"
```

---

## 后续扩展（可选）

以下功能不在当前 MVP 范围内，但可以后续添加：

1. **迷雾渲染 shader** - 实现真正的视觉迷雾效果
2. **编辑模式** - 按 E 切换，可以绘制地形
3. **小地图** - 右下角显示整体地图
4. **资源采集** - 单位可以采集资源点
5. **敌方 AI** - 敌人主动巡逻和攻击
6. **单位生产** - 建造建筑和生产单位
