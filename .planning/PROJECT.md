# Nova Engine

## What This Is

Nova Engine 是一个基于 Bevy ECS 的 Web 3D 游戏引擎，使用 Rust + WebAssembly 构建，以 WebGPU 作为图形后端。引擎采用插件化分层架构，提供渲染、物理、AI、地图、动画、音频等完整子系统，目标是支持浏览器端可玩的 RTS 类游戏。

## Core Value

玩家可以在浏览器中运行一个功能完整的 RTS 演示——单位能移动、战斗、有 AI 决策，地图可以寻路，引擎编译无警告无失败测试。

## Requirements

### Validated

<!-- 从现有代码推断：已实现并结构稳定的能力 -->

- ✓ ECS 插件化架构（NovaApp builder + Plugin trait） — 现有代码
- ✓ 3D 渲染基础（WebGPU PBR + Bevy 0.15 管线） — 现有代码
- ✓ 相机控制系统（OrbitCamera / FpsCamera / RtsCameraController） — 现有代码
- ✓ 物理模拟（Rapier 3D 刚体 + 碰撞体 + 事件） — 现有代码
- ✓ egui UI 系统（FpsDisplay / DebugPanel / NovaButton） — 现有代码
- ✓ 关键帧 + 补间 + 状态机动画系统 — 现有代码
- ✓ 角色数据建模（Attributes / CharacterState / HealthBar） — 现有代码
- ✓ AI 感知 + 行为树骨架（BehaviorNode 枚举 + PerceptionEvent） — 现有代码
- ✓ 编队系统（FormationManager + 槽位分配 + 移动） — 现有代码
- ✓ 瓦片地图 + 程序化生成 + 战争迷雾 — 现有代码
- ✓ A* 寻路系统（Pathfinder + PathFollow） — 现有代码
- ✓ 资源注册 + 预加载系统（AssetRegistry） — 现有代码
- ✓ 音频系统结构（AudioEvent / SpatialAudioSettings） — 现有代码（占位）
- ✓ 场景序列化（SceneDefinition JSON 序列化） — 现有代码
- ✓ 性能优化框架（视锥剔除 / LOD / 实例化 / 空间网格） — 现有代码（部分占位）
- ✓ 工具链（Trunk WASM 构建 + nova_inspector + nova_test 框架） — 现有代码

### Active

<!-- 需要构建的新能力 -->

- [ ] 集成测试通过（CharacterBundle / AiAgent / Blackboard API 补齐）
- [ ] 移动执行系统（AI MoveTo → Transform 实际更新）
- [ ] 战斗伤害系统（Attacking 状态 → 伤害结算 → 扣血事件）
- [ ] 音频系统真实实现（AudioEvent → Bevy AudioSink 实际播放）
- [ ] GPU 实例化真实提交（InstanceBatches → GPU buffer）
- [ ] RTS Demo 端到端可玩（单位移动 + 战斗 + AI + 地图寻路集成）
- [ ] 行为树 builder API（sequence / action / condition builder 模式）
- [ ] 截图测试实现（RenderTest capture_and_compare 真实对比）
- [ ] 性能关键路径优化（A* 缓存 / FogOfWar 增量更新 / 行为树 clone 消除）

### Out of Scope

- 网络多人同步 — 当前目标为单机/本地演示
- 移动端原生 App — Web 优先，WASM 目标
- 编辑器 GUI — 仅 inspector 工具，无完整编辑器
- 视频/流媒体渲染 — 非游戏引擎核心功能

## Context

- 代码库已有完整的引擎分层结构（12 个 crate），但存在多处占位实现和 API 不一致问题
- 集成测试（`tests/integration/`）因 API 缺失无法编译，阻塞 CI
- 核心游戏循环（移动 + 战斗）的执行侧系统缺失，AI 行为树只写入状态但没有系统响应
- 性能优化层（实例化、LOD）有框架但 GPU 提交逻辑未实现
- RTS Demo 存在但依赖上述缺失系统

## Constraints

- **技术栈**: Rust + Bevy 0.15 + WebAssembly — 不换框架
- **兼容性**: 目标为 WebGPU 浏览器（Chrome 113+）
- **依赖锁定**: bevy_rapier3d 0.28 与 Bevy 0.15 绑定，升级需同步
- **WASM 体积**: 保持 wasm-opt 优化，避免引入非必要依赖

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 优先修复集成测试 API | 测试通过是后续所有阶段的门控 | — Pending |
| 补齐 MoveTo 执行系统 | AI 不能移动则演示无意义 | — Pending |
| 先实现战斗再优化性能 | 可玩性优先于性能调优 | — Pending |
| 音频集成 Bevy 原生 AudioPlugin | 最小改动路径，避免引入新依赖 | — Pending |

---
*Last updated: 2026-03-20 after initialization*
