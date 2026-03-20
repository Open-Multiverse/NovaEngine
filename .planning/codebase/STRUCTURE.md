# Codebase Structure

**Analysis Date:** 2026-03-20

## Directory Layout

```
game_engine/                        # 工作空间根目录
├── crates/                         # 引擎核心 crate（库）
│   ├── nova_engine/                # 统一对外入口 crate
│   ├── nova_core/                  # App 生命周期、ECS 基础设施、输入、场景
│   ├── nova_render/                # 渲染、相机、光照、性能优化
│   ├── nova_physics/               # Rapier 3D 物理封装
│   ├── nova_ui/                    # egui UI 系统
│   ├── nova_animation/             # 关键帧/补间/状态机动画
│   ├── nova_audio/                 # 音频系统
│   ├── nova_assets/                # 资源管理
│   ├── nova_map/                   # 地图、寻路、战争迷雾
│   ├── nova_character/             # 角色属性与状态
│   ├── nova_ai/                    # AI 感知与决策
│   ├── nova_formation/             # 编队系统
│   └── nova_test/                  # 测试基础设施库
├── examples/                       # 可运行的示例项目（Bevy WASM apps）
│   ├── basic_demo/                 # 基础 3D 场景（物理、轨道相机、UI）
│   ├── physics_demo/               # 物理模拟演示
│   ├── animation_demo/             # 动画系统演示
│   ├── ui_demo/                    # UI 系统演示
│   ├── breakout_3d/                # Breakout 游戏示例
│   └── rts_demo/                   # RTS 游戏原型（综合示例）
├── tools/                          # 开发工具（原生二进制）
│   └── nova_inspector/             # ECS 实体检查器（调试工具）
├── tests/                          # 集成测试
│   └── integration/                # 跨 crate 集成测试
├── benches/                        # 工作空间级基准测试
├── docs/                           # 文档
│   └── superpowers/
│       ├── specs/                  # 规格说明（设计文档）
│       └── plans/                  # 实施计划
├── dist/                           # WASM 构建输出（Trunk 生成，不提交）
├── scripts/                        # 构建/工具脚本
├── .planning/                      # GSD 规划文档（不提交到 git）
├── .github/workflows/              # CI/CD 配置
├── Cargo.toml                      # 工作空间 Cargo 配置（含共享依赖版本）
├── Cargo.lock                      # 依赖锁定文件
├── Trunk.toml                      # Trunk WASM 构建配置（端口 8080）
├── index.html                      # Web 根入口（供 Trunk 使用）
├── rust-toolchain.toml             # Rust 工具链版本锁定
├── CLAUDE.md                       # 项目开发指南
└── IMPLEMENTATION.md               # 实施状态记录
```

## Directory Purposes

**`crates/nova_engine/`:**
- Purpose: 工作空间对外的统一 library crate
- Contains: `lib.rs`（`pub use` 所有子 crate）、`prelude.rs`（聚合所有子 prelude）
- Key files: `crates/nova_engine/src/lib.rs`、`crates/nova_engine/src/prelude.rs`

**`crates/nova_core/`:**
- Purpose: 引擎基础设施，所有其他 crate 的依赖起点
- Contains: App builder、插件定义、输入系统、场景序列化、调度阶段、核心组件
- Key files: `crates/nova_core/src/app.rs`、`crates/nova_core/src/plugin.rs`、`crates/nova_core/src/schedule.rs`

**`crates/nova_render/`:**
- Purpose: 所有渲染相关功能
- Contains: 相机、相机控制器、光照、网格构建器、材质预设、性能优化子模块
- Key files: `crates/nova_render/src/camera_controller.rs`、`crates/nova_render/src/performance/`

**`crates/nova_render/src/performance/`:**
- Purpose: 渲染性能优化系统
- Contains: `frustum_culling.rs`、`instancing.rs`、`lod.rs`、`spatial_grid.rs`
- Generated: No
- Committed: Yes

**`crates/nova_map/`:**
- Purpose: RTS/策略游戏地图系统
- Contains: 瓦片地图、程序化生成（noise）、A* 寻路、战争迷雾、高度图、RTS 相机、地图序列化
- Key files: `crates/nova_map/src/pathfinding.rs`、`crates/nova_map/src/fog.rs`、`crates/nova_map/src/generator.rs`

**`examples/rts_demo/`:**
- Purpose: 综合功能示例，演示地图、角色、AI、编队、战斗
- Contains: 按功能拆分的模块（`setup.rs`、`combat.rs`、`movement.rs`、`selection.rs`、`ui.rs`、`character_setup.rs`、`components.rs`）
- Key files: `examples/rts_demo/src/main.rs`

**`tools/nova_inspector/`:**
- Purpose: 原生桌面运行的 ECS 调试检查器
- Contains: `lib.rs`（NovaInspectorPlugin）、`main.rs`（启动入口）、`panels/`（UI 面板）、`state.rs`
- Key files: `tools/nova_inspector/src/lib.rs`

**`tests/integration/`:**
- Purpose: 跨 crate 集成测试
- Contains: `mod.rs`、`ai_tests.rs`、`character_tests.rs`、`map_tests.rs`
- Key files: `tests/integration/mod.rs`

**`dist/`:**
- Purpose: Trunk 构建的 WASM 产物
- Generated: Yes（`trunk build` 生成）
- Committed: No（`.gitignore` 应排除）

## Key File Locations

**Entry Points:**
- `examples/basic_demo/src/main.rs`: 基础示例应用入口
- `examples/rts_demo/src/main.rs`: RTS 综合示例入口
- `tools/nova_inspector/src/main.rs`: Inspector 工具入口
- `index.html`: Web WASM 加载根页面

**Configuration:**
- `Cargo.toml`: 工作空间配置，所有共享依赖版本在此锁定（`[workspace.dependencies]`）
- `Trunk.toml`: Web 构建配置，输出到 `dist/`，服务端口 8080
- `rust-toolchain.toml`: Rust 工具链版本

**Core Logic:**
- `crates/nova_core/src/app.rs`: `NovaApp` builder，所有游戏应用的起点
- `crates/nova_core/src/schedule.rs`: `NovaSystemSet`，定义系统执行顺序
- `crates/nova_engine/src/prelude.rs`: 统一 prelude，游戏代码的单一 import 源

**Testing:**
- `crates/nova_test/src/lib.rs`: `TestApp` 测试基础设施
- `tests/integration/`: 集成测试入口

## Naming Conventions

**Files:**
- 模块文件：`snake_case.rs`（如 `camera_controller.rs`、`state_machine.rs`）
- 每个 crate 必须有 `lib.rs`、`prelude.rs`、`plugin.rs`（少数例外）

**Directories:**
- Crate 目录：`nova_<feature>` 格式（如 `nova_render`、`nova_physics`）
- 示例目录：`<feature>_demo` 或描述性名称（如 `rts_demo`、`breakout_3d`）

**Types:**
- Plugin 结构体：`Nova<Feature>Plugin`（如 `NovaRenderPlugin`、`NovaAiPlugin`）
- Component 结构体：描述性 PascalCase（如 `OrbitCameraController`、`CharacterState`）
- Resource 结构体：描述性 PascalCase（如 `GameTime`、`FormationManager`、`AssetRegistry`）
- System 函数：`snake_case` 动词短语（如 `formation_follow_system`、`perception_update_system`）
- SystemSet 枚举：PascalCase（如 `NovaSystemSet::Physics`、`AiSet::Perception`）

## Where to Add New Code

**新引擎功能 crate（如 nova_network）:**
- 新建 `crates/nova_network/src/lib.rs`、`plugin.rs`、`prelude.rs`
- 在根 `Cargo.toml` `[workspace.members]` 和 `[workspace.dependencies]` 中注册
- 在 `crates/nova_engine/src/lib.rs` 中 `pub use nova_network;`
- 在 `crates/nova_engine/src/prelude.rs` 中 `pub use nova_network::prelude::*;`

**在现有 crate 中添加新组件/系统:**
- 组件定义：在相应 crate 的功能模块中添加（如渲染组件加入 `crates/nova_render/src/`）
- 在对应 `plugin.rs` 的 `build()` 中注册系统：`app.add_systems(Update, my_system)`
- 在 `prelude.rs` 中导出新的公共类型

**新示例项目:**
- 新建 `examples/<name>/src/main.rs` 和 `examples/<name>/Cargo.toml`
- 在根 `Cargo.toml` `[workspace.members]` 中注册

**共享测试工具:**
- 实现放入 `crates/nova_test/src/`
- 通过 `crates/nova_test/src/lib.rs` 和 `prelude.rs` 导出

**集成测试:**
- 测试文件放入 `tests/integration/`，在 `tests/integration/mod.rs` 中声明模块

**工具（原生桌面应用）:**
- 新建 `tools/<name>/src/main.rs` 和 `tools/<name>/Cargo.toml`
- 在根 `Cargo.toml` `[workspace.members]` 中注册

## Special Directories

**`.planning/`:**
- Purpose: GSD 工作流规划文档（ARCHITECTURE.md、STACK.md 等）
- Generated: No（手动/工具生成）
- Committed: 视团队约定

**`target/`:**
- Purpose: Cargo 编译产物缓存
- Generated: Yes
- Committed: No

**`dist/`:**
- Purpose: Trunk WASM 构建输出
- Generated: Yes
- Committed: No

**`crates/nova_test/`:**
- Purpose: 专用测试库，提供 `TestApp`（无窗口的轻量 App）和断言工具
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-03-20*
