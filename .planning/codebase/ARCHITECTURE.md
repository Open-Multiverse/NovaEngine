# Architecture

**Analysis Date:** 2026-03-20

## Pattern Overview

**Overall:** Bevy ECS Plugin Architecture（基于 Bevy ECS 的插件化分层引擎）

**Key Characteristics:**
- 所有功能封装为独立 Bevy `Plugin`，通过 `NovaApp::add_plugin()` 组合
- 数据（`Component` / `Resource`）与逻辑（`System`）严格分离，遵循 ECS 范式
- 统一入口 crate `nova_engine` 通过 `prelude` 模块重导出所有子系统
- 所有 crate 均提供自己的 `prelude.rs`，消费者只需 `use nova_engine::prelude::*`
- 系统执行顺序通过 `NovaSystemSet` 枚举和 Bevy 调度阶段控制

## Layers

**引擎入口层（nova_engine）:**
- Purpose: 统一对外 API，聚合所有子 crate 的 prelude
- Location: `crates/nova_engine/src/`
- Contains: `lib.rs`（重导出所有子 crate）、`prelude.rs`（聚合 prelude）
- Depends on: 所有其他 `nova_*` crate
- Used by: 游戏示例、应用代码

**核心层（nova_core）:**
- Purpose: App 生命周期管理、ECS 基础设施、输入系统、场景序列化
- Location: `crates/nova_core/src/`
- Contains: `app.rs`（NovaApp builder）、`plugin.rs`（NovaDefaultPlugins）、`schedule.rs`（NovaSystemSet）、`components.rs`（GameTime 等）、`input.rs`（InputState/InputActions/InputAxes）、`scene.rs`（SceneDefinition JSON 序列化）
- Depends on: `bevy`
- Used by: 所有其他 nova_* crate、游戏代码

**渲染层（nova_render）:**
- Purpose: 3D 渲染、相机控制、光照、网格、材质、性能优化
- Location: `crates/nova_render/src/`
- Contains: `camera.rs`、`camera_controller.rs`（OrbitCamera/FpsCamera）、`light.rs`、`mesh.rs`、`material.rs`、`performance/`（视锥剔除、实例化、LOD、空间网格）
- Depends on: `nova_core`、`bevy`
- Used by: 游戏代码、nova_engine

**物理层（nova_physics）:**
- Purpose: 基于 Rapier 3D 的物理模拟
- Location: `crates/nova_physics/src/`
- Contains: `rigidbody.rs`（RigidBodyConfig）、`collider.rs`（ColliderConfig）、`events.rs`（NovaCollisionEvent）
- Depends on: `nova_core`、`bevy_rapier3d`
- Used by: 游戏代码、nova_engine

**UI 层（nova_ui）:**
- Purpose: 基于 egui 的即时模式 UI
- Location: `crates/nova_ui/src/`
- Contains: `context.rs`（UI 状态/主题）、`widgets.rs`（FpsDisplay、NovaButton、DebugPanel）
- Depends on: `nova_core`、`bevy_egui`
- Used by: 游戏代码、工具

**动画层（nova_animation）:**
- Purpose: 关键帧动画、补间动画、动画状态机、程序化动画
- Location: `crates/nova_animation/src/`
- Contains: `clip.rs`、`player.rs`、`tween.rs`、`state_machine.rs`（AnimationStateMachine）、`procedural.rs`（ProceduralIdle）
- Depends on: `nova_core`、`bevy`
- Used by: 游戏代码、nova_character

**角色层（nova_character）:**
- Purpose: 角色数据建模（属性、状态、视觉反馈）——"角色是什么"
- Location: `crates/nova_character/src/`
- Contains: `character.rs`、`attributes.rs`（Attributes）、`state.rs`（CharacterState/AttackCooldown）、`feedback.rs`（HealthBar/SpawnDamageNumber）、`loader.rs`
- Depends on: `nova_core`、`nova_animation`、`bevy`
- Used by: RTS demo、游戏代码

**AI 层（nova_ai）:**
- Purpose: AI 决策系统（感知、行为树、情绪、战术）——"角色怎么想"
- Location: `crates/nova_ai/src/`
- Contains: `perception.rs`（PerceptionEvent）、`decision.rs`（行为树执行）、`behavior.rs`、`emotion.rs`、`personality.rs`、`tactics.rs`
- Depends on: `nova_core`、`nova_character`、`bevy`
- Used by: RTS demo

**编队层（nova_formation）:**
- Purpose: 单位编队管理与移动
- Location: `crates/nova_formation/src/`
- Contains: `formation.rs`（Formation/FormationManager）、`movement.rs`（formation_follow_system）、`patterns.rs`（FormationPattern）、`slots.rs`（SlotAssignment）
- Depends on: `nova_core`、`bevy`
- Used by: RTS demo

**地图层（nova_map）:**
- Purpose: 瓦片地图、程序化生成、寻路、战争迷雾、RTS 相机
- Location: `crates/nova_map/src/`
- Contains: `tilemap.rs`、`tile.rs`（TerrainType）、`generator.rs`（MapGenerator）、`pathfinding.rs`（Pathfinder/PathFollow）、`fog.rs`（FogOfWar/Vision）、`camera.rs`（RtsCameraController）、`heightmap.rs`、`serialization.rs`（MapFile）
- Depends on: `nova_core`、`noise`、`bevy`
- Used by: RTS demo

**资源层（nova_assets）:**
- Purpose: 资源注册、预加载、资源组、加载状态追踪
- Location: `crates/nova_assets/src/`
- Contains: `loader.rs`（AssetRegistry/AssetLoadState）、`handle.rs`（类型安全句柄）
- Depends on: `nova_core`、`bevy`
- Used by: 游戏代码

**音频层（nova_audio）:**
- Purpose: 背景音乐、音效、空间音频、音量控制
- Location: `crates/nova_audio/src/`
- Contains: `source.rs`（AudioSource/SpatialAudioSettings）、`plugin.rs`（AudioEvent）
- Depends on: `nova_core`、`bevy`
- Used by: 游戏代码

## Data Flow

**游戏帧更新流程:**

1. `PreUpdate` 阶段：`NovaInputPlugin` 更新 `InputState` 资源（鼠标位置、增量、滚轮）
2. `Update` 阶段（`AiSet::Perception`）：`perception_update_system` 读取场景状态，发送 `PerceptionEvent`
3. `Update` 阶段（`AiSet::Decision`，在 Perception 之后）：`behavior_tree_system` 消费感知事件，驱动角色行为
4. `Update` 阶段：`formation_follow_system` 更新编队中各单位的目标位置
5. `Update` 阶段：动画系统（`tween` / `player`）推进动画状态
6. `Update` 阶段：`orbit_camera_system` / `fps_camera_system` 响应输入，更新相机 `Transform`
7. `PostUpdate` 阶段：Bevy 内置渲染管线（PBR、WebGPU）执行绘制
8. `Update` 阶段：`fog_vision_system` 追踪单位移动，更新 `FogOfWar` 状态

**系统执行顺序（NovaSystemSet）:**
```
Input → Logic → Physics → Animation → PreRender → Ui
```
实际在 `FixedUpdate` 中运行物理，`Update` 中运行游戏逻辑。

**场景加载数据流:**
1. 触发 `LoadSceneEvent { json }`
2. `SceneDefinition::from_json()` 反序列化（`serde_json`）
3. 遍历 `SceneEntity` 树，通过 `commands.spawn()` 创建 ECS 实体
4. 发送 `SceneLoadedEvent { name }`

**State Management:**
- 游戏全局时间通过 `GameTime` resource 管理（支持 time scale）
- 角色状态通过 `CharacterState` component 存储（Bevy Reflect 注册）
- 地图状态通过 `TileMap` resource 存储
- 编队状态通过 `FormationManager` resource 管理
- 资源加载状态通过 `AssetLoadState` resource 追踪

## Key Abstractions

**NovaApp（应用构建器）:**
- Purpose: 封装 Bevy `App`，提供链式 builder API
- Examples: `crates/nova_core/src/app.rs`
- Pattern: Builder pattern，`with_title().add_plugin().add_startup_system().run()`

**Plugin（插件接口）:**
- Purpose: 每个功能模块通过实现 Bevy `Plugin` trait 注入到引擎
- Examples: `NovaRenderPlugin`、`NovaPhysicsPlugin`、`NovaAiPlugin`、`NovaMapPlugin`
- Pattern: 在 `build()` 中注册 `init_resource`、`add_event`、`add_systems`、`configure_sets`

**prelude 模式:**
- Purpose: 每个 crate 暴露 `prelude.rs`，消费者通过 `use nova_xxx::prelude::*` 获取所有常用类型
- Examples: `crates/nova_engine/src/prelude.rs`（聚合所有子 prelude）
- Pattern: 透明重导出，隐藏内部模块路径

**Component（ECS 组件）:**
- Purpose: 纯数据载体，附加在实体上
- Examples: `OrbitCameraController`、`CharacterState`、`Vision`、`PathFollow`、`Attributes`
- Pattern: `#[derive(Component)]`，使用 builder methods 配置初始值

**SystemSet（执行顺序）:**
- Purpose: 控制同一调度阶段内系统的相对执行顺序
- Examples: `NovaSystemSet`（引擎级别）、`AiSet`（AI 子系统内部）
- Pattern: `#[derive(SystemSet)]` enum，通过 `.in_set()` 和 `configure_sets().before()/.after()` 排序

## Entry Points

**游戏应用入口:**
- Location: 每个 example 的 `src/main.rs`（如 `examples/basic_demo/src/main.rs`）
- Triggers: WASM 加载后由 `wasm-bindgen` 调用 `main()`，或原生执行
- Responsibilities: 创建 `NovaApp`，注册所需插件，添加启动和更新系统，调用 `.run()`

**Web 入口:**
- Location: `index.html`（根目录）
- Triggers: 浏览器加载，由 Trunk 构建为 WASM 包
- Responsibilities: 提供 `#nova-canvas` canvas 元素，加载 WASM bundle

**引擎库入口:**
- Location: `crates/nova_engine/src/lib.rs`
- Triggers: 被游戏 crate 依赖时编译进目标
- Responsibilities: 重导出所有子 crate，提供统一的 `nova_engine::prelude::*`

**Inspector 工具入口:**
- Location: `tools/nova_inspector/src/main.rs`
- Triggers: 原生二进制直接运行（非 WASM）
- Responsibilities: 启动带 `NovaInspectorPlugin` 的 Bevy App，供开发调试使用

## Error Handling

**Strategy:** 无集中式错误处理层；遵循 Bevy 惯例，运行时错误通过 Bevy 日志系统（`info!`/`warn!`/`error!`）输出

**Patterns:**
- 可失败操作使用 `if let Some(...) = ...` 模式提前返回（如 `fog_vision_system` 中对可选 `Res` 的处理）
- 场景序列化/反序列化通过 `Result<_, serde_json::Error>` 向调用方传播错误
- 物理碰撞通过 `EventWriter<NovaCollisionEvent>` 发布事件，由业务层监听处理
- ECS Query 使用 Bevy 的 `get_single()` / `get_single_mut()` 返回 `Result`，一般用 `if let Ok` 处理

## Cross-Cutting Concerns

**Logging:** 使用 Bevy 内置 `bevy::log`（底层为 `log` crate），在 `NovaDefaultPlugins` 中配置 `Level::INFO`；代码中直接使用 `info!()`/`warn!()`

**Validation:** 无集中验证层；组件数值约束通过 builder 方法的 `clamp()` 调用（如 `OrbitCameraController::with_distance()` 使用 `clamp(min, max)`）

**Authentication:** 不适用（纯本地游戏引擎，无网络认证需求）

---

*Architecture analysis: 2026-03-20*
