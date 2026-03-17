# Nova Map 与 RTS Demo 设计文档

> Nova Engine 地图系统模块 + RTS 游戏原型

## 概述

本文档描述 Nova Engine 新增的地图系统模块（nova_map）及配套的 RTS Demo 示例。目标是为引擎提供可复用的瓦片地图能力，并通过一个完整的 RTS 原型验证设计。

### 项目目标

- **nova_map 模块**：提供程序化地形生成、瓦片地图管理、战争迷雾、A* 寻路等 RTS/策略游戏通用能力
- **RTS Demo**：展示地图系统的完整应用，包含单位选中、移动、战斗等基础 RTS 玩法

### 设计原则

- **模块独立**：nova_map 不依赖具体游戏逻辑，可被 RTS、RPG、塔防等多种类型复用
- **ECS 友好**：核心数据结构可作为 Bevy Resource/Component 使用
- **渐进式**：先实现核心功能，编辑器等高级功能后续扩展

## 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────┐
│                 examples/rts_demo                    │
│  - 单位选中/框选                                      │
│  - 移动指令                                          │
│  - 战斗系统                                          │
│  - 资源点可视化                                       │
│  - 编辑模式                                          │
├──────────────┬──────────────┬───────────────────────┤
│  nova_map    │  nova_render │  nova_physics         │
│  - 瓦片地图   │  - 迷雾shader│  - 单位碰撞           │
│  - 地形生成   │  - 地形渲染  │                       │
│  - 高度图     │              │                       │
│  - 寻路 A*   │              │                       │
│  - 战争迷雾   │              │                       │
├──────────────┴──────────────┴───────────────────────┤
│                    nova_core                         │
└─────────────────────────────────────────────────────┘
```

### 模块职责

| 模块 | 职责 |
|------|------|
| nova_map | 瓦片地图数据结构、程序化生成、寻路、迷雾逻辑 |
| nova_render | 迷雾渲染 shader、地形可视化 |
| rts_demo | RTS 游戏逻辑、单位系统、战斗系统、UI |

## nova_map 模块设计

### 文件结构

```
crates/nova_map/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、插件定义
    ├── tile.rs          # Tile, TerrainType 定义
    ├── tilemap.rs       # TileMap 数据结构和查询方法
    ├── heightmap.rs     # 高度图数据结构
    ├── generator.rs     # 程序化地图生成器
    ├── fog.rs           # 战争迷雾系统
    ├── pathfinding.rs   # A* 寻路算法
    ├── camera.rs        # RTS 相机控制器
    ├── serialization.rs # 地图保存/加载
    └── prelude.rs       # 公共导出
```

### 核心数据结构

#### 瓦片与地形

```rust
/// 地形类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainType {
    /// 草地 - 正常移动速度
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
    pub fn move_cost(&self) -> Option<f32> {
        match self {
            Self::Grass => Some(1.0),
            Self::Desert => Some(1.25),
            Self::Forest => Some(1.67),
            Self::Water | Self::Mountain => None, // 不可通行
        }
    }

    /// 是否可建造
    pub fn buildable(&self) -> bool {
        matches!(self, Self::Grass | Self::Desert)
    }
}

/// 单个瓦片
#[derive(Clone, Debug)]
pub struct Tile {
    /// 地形类型
    pub terrain: TerrainType,
    /// 高度值（0.0 ~ 1.0）
    pub height: f32,
    /// 是否被占用（有建筑/资源）
    pub occupied: bool,
}

impl Tile {
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
}
```

#### 瓦片地图

```rust
/// 瓦片地图资源
#[derive(Resource, Clone)]
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
    /// 创建空地图
    pub fn new(width: u32, height: u32, tile_size: f32) -> Self;

    /// 获取指定位置的瓦片
    pub fn get(&self, x: u32, y: u32) -> Option<&Tile>;

    /// 获取指定位置的瓦片（可变）
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile>;

    /// 世界坐标转瓦片坐标
    pub fn world_to_tile(&self, world_pos: Vec3) -> Option<(u32, u32)>;

    /// 瓦片坐标转世界坐标（瓦片中心）
    pub fn tile_to_world(&self, x: u32, y: u32) -> Vec3;

    /// 获取瓦片的世界高度
    pub fn get_world_height(&self, x: u32, y: u32) -> f32;

    /// 迭代所有瓦片
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, &Tile)>;

    /// 获取相邻瓦片（用于寻路）
    pub fn neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)>;
}
```

### 程序化生成系统

#### 生成器配置

```rust
/// 地形权重配置
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct MapGeneratorConfig {
    /// 随机种子
    pub seed: u64,
    /// 地图尺寸（宽, 高）
    pub size: (u32, u32),
    /// 瓦片世界尺寸
    pub tile_size: f32,
    /// 高度变化幅度
    pub height_scale: f32,
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
}

impl Default for MapGeneratorConfig {
    fn default() -> Self {
        Self {
            seed: 12345,
            size: (128, 128),
            tile_size: 1.0,
            height_scale: 5.0,
            water_level: 0.3,
            mountain_level: 0.8,
            terrain_weights: TerrainWeights::default(),
            noise_octaves: 4,
            noise_frequency: 0.02,
        }
    }
}
```

#### 生成器实现

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
        // 使用 Simplex Noise 叠加多个八度
        // 1. 创建噪声生成器（使用 seed）
        // 2. 对每个点采样多个八度的噪声
        // 3. 归一化到 0.0 ~ 1.0
    }

    /// 高度图转瓦片地图
    fn heightmap_to_tilemap(heightmap: &HeightMap, config: &MapGeneratorConfig) -> TileMap {
        // 根据高度值分配地形类型：
        // - height < water_level → Water
        // - height > mountain_level → Mountain
        // - 其他根据 terrain_weights 随机分配 Grass/Desert/Forest
    }
}

/// 高度图数据
pub struct HeightMap {
    width: u32,
    height: u32,
    data: Vec<f32>,
}
```

### 战争迷雾系统

```rust
/// 迷雾状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FogState {
    /// 未探索 - 完全黑暗，不显示地形
    #[default]
    Unexplored,
    /// 已探索 - 显示地形，不显示敌方单位
    Explored,
    /// 可见 - 在当前视野内
    Visible,
}

/// 战争迷雾资源
#[derive(Resource)]
pub struct FogOfWar {
    /// 地图宽度
    width: u32,
    /// 地图高度
    height: u32,
    /// 每个瓦片的迷雾状态
    states: Vec<FogState>,
    /// 视野计数（多少单位能看到该瓦片）
    vision_count: Vec<u32>,
}

impl FogOfWar {
    pub fn new(width: u32, height: u32) -> Self;

    /// 获取瓦片迷雾状态
    pub fn get_state(&self, x: u32, y: u32) -> FogState;

    /// 添加视野（单位进入某位置时调用）
    pub fn add_vision(&mut self, center: (u32, u32), range: u32, tilemap: &TileMap);

    /// 移除视野（单位离开某位置时调用）
    pub fn remove_vision(&mut self, center: (u32, u32), range: u32, tilemap: &TileMap);

    /// 更新迷雾状态（每帧调用）
    pub fn update(&mut self);

    /// 获取迷雾纹理数据（用于渲染）
    pub fn to_texture_data(&self) -> Vec<u8>;
}

/// 视野组件（附加到有视野的实体上）
#[derive(Component)]
pub struct Vision {
    /// 视野半径（瓦片数）
    pub range: u32,
    /// 高地视野加成
    pub height_bonus: u32,
    /// 上一帧位置（用于检测移动）
    pub last_tile: Option<(u32, u32)>,
}
```

### A* 寻路系统

```rust
/// 寻路请求
pub struct PathRequest {
    pub start: (u32, u32),
    pub goal: (u32, u32),
}

/// 寻路结果
pub struct PathResult {
    pub path: Vec<(u32, u32)>,
    pub cost: f32,
}

/// 寻路器
pub struct Pathfinder;

impl Pathfinder {
    /// A* 寻路
    pub fn find_path(tilemap: &TileMap, request: PathRequest) -> Option<PathResult> {
        // 标准 A* 实现：
        // 1. 使用 BinaryHeap 作为开放列表
        // 2. 启发函数使用曼哈顿距离或欧几里得距离
        // 3. 考虑地形移动代价
        // 4. 支持对角线移动（可选）
    }

    /// 批量寻路（用于多单位）
    pub fn find_paths_batch(
        tilemap: &TileMap,
        requests: Vec<PathRequest>,
    ) -> Vec<Option<PathResult>>;
}

/// 路径组件（附加到移动中的单位）
#[derive(Component)]
pub struct PathFollow {
    /// 路径点序列
    pub path: Vec<(u32, u32)>,
    /// 当前目标点索引
    pub current_index: usize,
    /// 是否到达终点
    pub finished: bool,
}
```

### RTS 相机控制器

```rust
/// RTS 相机控制器组件
#[derive(Component)]
pub struct RtsCameraController {
    /// 移动速度（世界单位/秒）
    pub move_speed: f32,
    /// 缩放速度
    pub zoom_speed: f32,
    /// 缩放范围（最小高度, 最大高度）
    pub zoom_range: (f32, f32),
    /// 边缘滚动触发区域（像素）
    pub edge_scroll_margin: f32,
    /// 是否启用边缘滚动
    pub edge_scroll_enabled: bool,
    /// 相机移动边界（世界坐标）
    pub bounds: Option<Rect>,
}

impl Default for RtsCameraController {
    fn default() -> Self {
        Self {
            move_speed: 20.0,
            zoom_speed: 10.0,
            zoom_range: (10.0, 50.0),
            edge_scroll_margin: 20.0,
            edge_scroll_enabled: true,
            bounds: None,
        }
    }
}
```

### 地图序列化

```rust
/// 地图文件格式
#[derive(Serialize, Deserialize)]
pub struct MapFile {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub tiles: Vec<TileData>,
    pub spawn_points: Vec<SpawnPoint>,
    pub resource_nodes: Vec<ResourceNodeData>,
}

#[derive(Serialize, Deserialize)]
pub struct TileData {
    pub terrain: String,  // "grass", "desert", etc.
    pub height: f32,
    pub occupied: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SpawnPoint {
    pub x: u32,
    pub y: u32,
    pub team: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceNodeData {
    pub x: u32,
    pub y: u32,
    pub resource_type: String,
    pub amount: u32,
}

impl TileMap {
    /// 从文件加载
    pub fn load(path: &str) -> Result<Self, MapLoadError>;

    /// 保存到文件
    pub fn save(&self, path: &str) -> Result<(), MapSaveError>;
}
```

### 插件定义

```rust
/// Nova Map 插件
pub struct NovaMapPlugin;

impl Plugin for NovaMapPlugin {
    fn build(&self, app: &mut App) {
        app
            // 注册类型
            .register_type::<Vision>()
            .register_type::<PathFollow>()
            .register_type::<RtsCameraController>()
            // 添加系统
            .add_systems(Update, (
                rts_camera_system,
                vision_update_system,
                fog_update_system,
                path_follow_system,
            ));
    }
}
```

## RTS Demo 设计

### 文件结构

```
examples/rts_demo/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── assets/
│   └── maps/
│       └── default.json
└── src/
    ├── main.rs           # 入口、插件组装
    ├── components.rs     # 游戏组件定义
    ├── setup.rs          # 场景初始化
    ├── selection.rs      # 单位选中系统
    ├── movement.rs       # 移动指令系统
    ├── combat.rs         # 战斗系统
    ├── ai.rs             # 敌方 AI
    ├── fog_render.rs     # 迷雾渲染集成
    ├── ui.rs             # HUD 和小地图
    └── editor.rs         # 编辑模式
```

### 游戏组件

```rust
// ============================================================================
// 单位组件
// ============================================================================

/// 单位标记
#[derive(Component)]
pub struct Unit;

/// 阵营
#[derive(Component, Clone, Copy, PartialEq, Eq)]
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
    /// 伤害值
    pub damage: f32,
    /// 攻击范围（世界单位）
    pub range: f32,
    /// 攻击间隔（秒）
    pub cooldown: f32,
    /// 当前冷却计时
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
    /// 移动速度（世界单位/秒）
    pub speed: f32,
}

// ============================================================================
// 选择组件
// ============================================================================

/// 可选中标记
#[derive(Component)]
pub struct Selectable;

/// 当前被选中标记
#[derive(Component)]
pub struct Selected;

/// 攻击目标
#[derive(Component)]
pub struct AttackTarget(pub Entity);

/// 移动目标
#[derive(Component)]
pub struct MoveTarget(pub Vec3);

// ============================================================================
// 资源组件
// ============================================================================

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

// ============================================================================
// 游戏状态
// ============================================================================

/// 游戏模式
#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameMode {
    #[default]
    Playing,
    Editor,
    Paused,
}

/// 选择框资源
#[derive(Resource, Default)]
pub struct SelectionBox {
    pub active: bool,
    pub start: Vec2,
    pub end: Vec2,
}
```

### 系统实现概要

#### 选中系统

```rust
/// 单位选中系统
pub fn selection_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    mut selection_box: ResMut<SelectionBox>,
    selectables: Query<(Entity, &Transform, &Team), With<Selectable>>,
    mut commands: Commands,
    selected: Query<Entity, With<Selected>>,
) {
    // 1. 左键按下：记录起点，开始框选
    // 2. 左键拖拽：更新框选框
    // 3. 左键释放：
    //    - 如果拖拽距离小，视为点击，选中点击位置的单位
    //    - 如果拖拽距离大，视为框选，选中框内所有己方单位
    // 4. 清除之前的选中状态，添加新的 Selected 组件
}

/// 渲染选择框
pub fn render_selection_box(
    selection_box: Res<SelectionBox>,
    mut gizmos: Gizmos,
) {
    if selection_box.active {
        // 绘制半透明选择框
    }
}
```

#### 移动系统

```rust
/// 移动指令系统
pub fn move_command_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    tilemap: Res<TileMap>,
    selected_units: Query<Entity, (With<Selected>, With<Movement>)>,
    mut commands: Commands,
) {
    // 右键点击地面：为所有选中单位设置移动目标
    if mouse_button.just_pressed(MouseButton::Right) {
        // 1. 射线检测获取点击的世界坐标
        // 2. 转换为瓦片坐标
        // 3. 检查是否可通行
        // 4. 为每个选中单位计算路径（A*）
        // 5. 添加 PathFollow 组件
    }
}

/// 路径跟随系统
pub fn path_follow_system(
    time: Res<Time>,
    tilemap: Res<TileMap>,
    mut units: Query<(Entity, &mut Transform, &Movement, &mut PathFollow)>,
    mut commands: Commands,
) {
    for (entity, mut transform, movement, mut path) in units.iter_mut() {
        if path.finished {
            continue;
        }

        // 1. 获取当前目标点的世界坐标
        // 2. 计算移动方向
        // 3. 移动单位
        // 4. 如果到达当前点，前进到下一个点
        // 5. 如果到达终点，标记完成并移除组件
    }
}
```

#### 战斗系统

```rust
/// 攻击指令系统
pub fn attack_command_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    selected_units: Query<Entity, (With<Selected>, With<Attack>)>,
    targets: Query<(Entity, &Transform, &Team), With<Health>>,
    mut commands: Commands,
) {
    // 右键点击敌方单位：设置攻击目标
    if mouse_button.just_pressed(MouseButton::Right) {
        // 1. 射线检测获取点击的实体
        // 2. 检查是否是敌方单位
        // 3. 为所有选中单位添加 AttackTarget 组件
    }
}

/// 战斗执行系统
pub fn combat_system(
    time: Res<Time>,
    mut attackers: Query<(Entity, &Transform, &mut Attack, &AttackTarget, &Team)>,
    mut targets: Query<(Entity, &Transform, &mut Health, &Team)>,
    mut commands: Commands,
) {
    for (attacker_entity, attacker_transform, mut attack, target, attacker_team) in attackers.iter_mut() {
        attack.tick(time.delta_secs());

        // 1. 检查目标是否还存在
        // 2. 计算与目标的距离
        // 3. 如果在攻击范围内且冷却完成，造成伤害
        // 4. 如果目标死亡，移除 AttackTarget
    }
}

/// 死亡清理系统
pub fn death_system(
    units: Query<(Entity, &Health)>,
    mut commands: Commands,
) {
    for (entity, health) in units.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// 自动索敌系统
pub fn auto_target_system(
    units: Query<(Entity, &Transform, &Team, &Attack), Without<AttackTarget>>,
    potential_targets: Query<(Entity, &Transform, &Team), With<Health>>,
    mut commands: Commands,
) {
    // 为没有攻击目标的单位自动寻找范围内的敌人
}
```

#### 敌方 AI

```rust
/// 简单敌方 AI
pub fn enemy_ai_system(
    enemy_units: Query<(Entity, &Transform, &Attack), (With<Unit>, With<Team>)>,
    player_units: Query<(Entity, &Transform), (With<Unit>, With<Team>)>,
    tilemap: Res<TileMap>,
    mut commands: Commands,
) {
    // 简单行为：
    // 1. 如果视野内有敌人，移动并攻击
    // 2. 否则在出生点附近巡逻
}
```

### UI 系统

```rust
/// 游戏 HUD
pub fn game_ui(
    mut contexts: EguiContexts,
    selected_units: Query<(&Health, &Attack), With<Selected>>,
    game_mode: Res<State<GameMode>>,
) {
    // 显示选中单位信息
    egui::Window::new("单位信息")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
        .show(contexts.ctx_mut(), |ui| {
            for (health, attack) in selected_units.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("生命: {:.0}/{:.0}", health.current, health.max));
                    ui.label(format!("攻击: {:.0}", attack.damage));
                });
            }
        });

    // 游戏模式提示
    egui::Window::new("提示")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("左键: 选中/框选  右键: 移动/攻击  E: 编辑模式");
        });
}

/// 小地图（简化版）
pub fn minimap_ui(
    mut contexts: EguiContexts,
    tilemap: Res<TileMap>,
    fog: Res<FogOfWar>,
    units: Query<(&Transform, &Team), With<Unit>>,
) {
    // 右下角显示小地图
    // 显示地形、迷雾、单位位置
}
```

### 编辑模式

```rust
/// 编辑器状态
#[derive(Resource, Default)]
pub struct EditorState {
    pub mode: EditorMode,
    pub brush_size: u32,
    pub selected_terrain: TerrainType,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    #[default]
    View,
    PaintTerrain,
    AdjustHeight,
    PlaceSpawn,
    PlaceResource,
}

/// 编辑器 UI
pub fn editor_ui(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut tilemap: ResMut<TileMap>,
) {
    egui::Window::new("地图编辑器")
        .show(contexts.ctx_mut(), |ui| {
            // 模式选择
            ui.horizontal(|ui| {
                ui.selectable_value(&mut editor_state.mode, EditorMode::View, "查看");
                ui.selectable_value(&mut editor_state.mode, EditorMode::PaintTerrain, "地形");
                ui.selectable_value(&mut editor_state.mode, EditorMode::AdjustHeight, "高度");
            });

            // 地形选择
            if editor_state.mode == EditorMode::PaintTerrain {
                ui.separator();
                ui.label("地形类型:");
                // 地形按钮...
            }

            // 笔刷大小
            ui.separator();
            ui.add(egui::Slider::new(&mut editor_state.brush_size, 1..=5).text("笔刷"));

            // 保存/加载
            ui.separator();
            if ui.button("保存地图").clicked() {
                // tilemap.save(...)
            }
            if ui.button("加载地图").clicked() {
                // tilemap = TileMap::load(...)
            }
        });
}

/// 编辑器绘制系统
pub fn editor_paint_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    editor_state: Res<EditorState>,
    mut tilemap: ResMut<TileMap>,
    game_mode: Res<State<GameMode>>,
) {
    if *game_mode.get() != GameMode::Editor {
        return;
    }

    // 左键按住时绘制地形
}
```

## 迷雾渲染集成

在 `nova_render` 中添加迷雾后处理：

```rust
/// 迷雾渲染插件
pub struct FogRenderPlugin;

impl Plugin for FogRenderPlugin {
    fn build(&self, app: &mut App) {
        // 1. 创建迷雾纹理资源
        // 2. 添加迷雾更新系统（将 FogOfWar 数据同步到 GPU 纹理）
        // 3. 添加后处理 shader
    }
}
```

迷雾 shader 伪代码：

```wgsl
@fragment
fn fog_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_pos = reconstruct_world_position(in.uv);
    let tile_coord = world_to_tile(world_pos);
    let fog_state = sample_fog_texture(tile_coord);

    var color = textureSample(scene_texture, scene_sampler, in.uv);

    if fog_state == UNEXPLORED {
        color = vec4(0.0, 0.0, 0.0, 1.0);  // 纯黑
    } else if fog_state == EXPLORED {
        color = color * 0.5;  // 半暗
    }
    // VISIBLE: 保持原色

    return color;
}
```

## 依赖关系

### nova_map/Cargo.toml

```toml
[package]
name = "nova_map"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_core = { path = "../nova_core" }
bevy = { version = "0.15", default-features = false }
noise = "0.9"          # Simplex 噪声
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### rts_demo/Cargo.toml

```toml
[package]
name = "rts_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_engine = { path = "../../crates/nova_engine" }
nova_map = { path = "../../crates/nova_map" }
bevy = "0.15"
bevy_egui = "0.31"
```

## 成功标准

### nova_map 模块

1. **地图生成**：能通过配置生成包含多种地形和高度变化的瓦片地图
2. **寻路**：A* 寻路能正确绕过障碍物，考虑地形代价
3. **迷雾**：战争迷雾能正确追踪单位视野，区分三种状态
4. **相机**：RTS 相机支持 WASD/边缘滚动/缩放
5. **序列化**：地图能保存为 JSON 并重新加载

### RTS Demo

1. **可运行**：在浏览器中流畅运行
2. **单位控制**：能选中、框选、移动玩家单位
3. **战斗**：单位能自动攻击范围内敌人
4. **寻路**：单位能绑过障碍物到达目标
5. **迷雾**：未探索区域不可见，已探索但视野外显示地形
6. **编辑**：能切换到编辑模式修改地形

---

*文档版本: 1.0*
*创建日期: 2026-03-17*
