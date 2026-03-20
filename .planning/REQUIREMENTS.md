# Requirements: Nova Engine

**Defined:** 2026-03-20
**Core Value:** 玩家可以在浏览器中运行一个功能完整的 RTS 演示——单位能移动、战斗、有 AI 决策，地图可以寻路，引擎编译无警告无失败测试。

## v1 Requirements

### 测试基础设施修复（TESTFIX）

- [ ] **TESTFIX-01**: `CharacterBundle` 和 `CharacterStats` 类型在 `nova_character` 中存在且可从集成测试导入
- [ ] **TESTFIX-02**: `AiAgent`、`Blackboard` 组件在 `nova_ai` 中存在且可从集成测试导入
- [ ] **TESTFIX-03**: `BehaviorTree::sequence()` / `action()` builder API 在 `nova_ai` 中可用
- [ ] **TESTFIX-04**: `cargo test --all` 全部通过，无编译错误
- [ ] **TESTFIX-05**: `BrowserCompatibility::supports_webgpu()` 在非 WASM 环境返回 `false` 而非 `true`

### 角色移动系统（MOVE）

- [ ] **MOVE-01**: AI 单位在 `CharacterState::Moving { target }` 时，其 `Transform` 每帧向 `target` 位置更新
- [ ] **MOVE-02**: 单位到达目标后自动切换为 `CharacterState::Idle`
- [ ] **MOVE-03**: 移动速度通过 `Attributes.speed` 或配置值驱动
- [ ] **MOVE-04**: 移动系统与寻路系统（`PathFollow`）集成，单位沿路径点移动

### 战斗伤害系统（COMBAT）

- [ ] **COMBAT-01**: AI 单位在 `CharacterState::Attacking { target }` 时，对目标触发伤害结算
- [ ] **COMBAT-02**: 伤害结算扣除目标 `Attributes.health`
- [ ] **COMBAT-03**: 单位血量归零后进入死亡状态（`CharacterState::Dead`）并从场景移除
- [ ] **COMBAT-04**: 攻击冷却（`AttackCooldown`）正确阻止连续攻击
- [ ] **COMBAT-05**: 战斗事件可被外部系统监听（`CombatEvent` 或类似事件）

### AI 行为树完善（AI）

- [ ] **AI-01**: `BehaviorTree::standard_soldier()` 使用真实感知到的敌人 Entity 而非 `Entity::PLACEHOLDER`
- [ ] **AI-02**: 行为树条件节点（`HasTarget`、`EnemyInRange`、`HealthBelow`）有对应的求值实现和单元测试
- [ ] **AI-03**: `Blackboard` 组件作为 AI 状态存储，可被行为树节点读写
- [ ] **AI-04**: `AiAgent` 组件标记 AI 控制实体，行为树系统仅处理带此组件的实体

### 音频系统真实实现（AUDIO）

- [ ] **AUDIO-01**: `AudioEvent::PlaySound` 实际通过 Bevy `AudioPlugin` 播放声音
- [ ] **AUDIO-02**: `AudioEvent::PlayMusic` 播放背景音乐并支持循环
- [ ] **AUDIO-03**: `AudioEvent::StopMusic` 停止当前背景音乐
- [ ] **AUDIO-04**: `SpatialAudioSettings` 组件使声音具有 3D 空间位置衰减

### 渲染性能真实化（RENDER）

- [ ] **RENDER-01**: `InstanceBatches` 中收集的矩阵实际提交 GPU 实例化绘制，或移除假统计代码
- [ ] **RENDER-02**: `InstanceBatches.dirty` 标志在批次更新时正确置为 `true`
- [ ] **RENDER-03**: `SceneTester::calculate_scene_bounds` 使用 `With<Mesh3d>` 过滤器，仅计算网格实体包围盒

### 截图测试实现（RENDERTEST）

- [ ] **RENDERTEST-01**: `RenderTest::capture_and_compare` 使用 Bevy `ScreenshotManager` 实际捕获帧
- [ ] **RENDERTEST-02**: 捕获的截图与预期图像做像素差异比较，超过阈值则测试失败

### RTS Demo 集成（DEMO）

- [ ] **DEMO-01**: RTS Demo 启动后，玩家可选中单位并点击地图发出移动命令
- [ ] **DEMO-02**: 单位按 A* 路径移动到目标位置，战争迷雾随单位更新
- [ ] **DEMO-03**: AI 单位自动感知敌方并发起攻击（行为树驱动）
- [ ] **DEMO-04**: RTS Demo 在 Trunk WASM 构建后可在 Chrome 浏览器中运行

### 性能关键路径优化（PERF）

- [ ] **PERF-01**: `Pathfinder::find_path` 对相同 start+goal 请求使用缓存，避免重复 A* 计算
- [ ] **PERF-02**: `FogOfWar::add_vision` 仅在单位位置改变时更新，避免每帧全量重算
- [ ] **PERF-03**: `behavior_tree_system` 不再每帧 clone 整棵行为树，改用 index 引用或分离 readonly/mutable 结构

## v2 Requirements

### 编辑器与工具

- **TOOL-01**: nova_inspector 支持实时修改组件属性
- **TOOL-02**: 场景编辑器支持拖拽放置实体

### 扩展渲染

- **REND-V2-01**: 阴影贴图支持
- **REND-V2-02**: 后处理效果（Bloom、SSAO）
- **REND-V2-03**: 粒子系统

### 网络

- **NET-01**: 帧同步本地多人（同机）
- **NET-02**: WebRTC P2P 网络基础

## Out of Scope

| Feature | Reason |
|---------|--------|
| 网络多人同步（服务器） | 超出 v1 范围，需要独立基础设施 |
| 移动端原生 App | Web 优先，WASM 目标足够 |
| 完整可视化编辑器 | nova_inspector 满足调试需求 |
| 视频渲染/截帧导出 | 非游戏引擎核心功能 |
| Bevy 升级到 0.16 | 需等待 bevy_rapier3d 跟进，v1 锁定 0.15 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TESTFIX-01 | Phase 1 | Pending |
| TESTFIX-02 | Phase 1 | Pending |
| TESTFIX-03 | Phase 1 | Pending |
| TESTFIX-04 | Phase 1 | Pending |
| TESTFIX-05 | Phase 1 | Pending |
| MOVE-01 | Phase 2 | Pending |
| MOVE-02 | Phase 2 | Pending |
| MOVE-03 | Phase 2 | Pending |
| MOVE-04 | Phase 2 | Pending |
| COMBAT-01 | Phase 3 | Pending |
| COMBAT-02 | Phase 3 | Pending |
| COMBAT-03 | Phase 3 | Pending |
| COMBAT-04 | Phase 3 | Pending |
| COMBAT-05 | Phase 3 | Pending |
| AI-01 | Phase 4 | Pending |
| AI-02 | Phase 4 | Pending |
| AI-03 | Phase 4 | Pending |
| AI-04 | Phase 4 | Pending |
| AUDIO-01 | Phase 5 | Pending |
| AUDIO-02 | Phase 5 | Pending |
| AUDIO-03 | Phase 5 | Pending |
| AUDIO-04 | Phase 5 | Pending |
| RENDER-01 | Phase 6 | Pending |
| RENDER-02 | Phase 6 | Pending |
| RENDER-03 | Phase 6 | Pending |
| RENDERTEST-01 | Phase 6 | Pending |
| RENDERTEST-02 | Phase 6 | Pending |
| DEMO-01 | Phase 8 | Pending |
| DEMO-02 | Phase 8 | Pending |
| DEMO-03 | Phase 8 | Pending |
| DEMO-04 | Phase 8 | Pending |
| PERF-01 | Phase 7 | Pending |
| PERF-02 | Phase 7 | Pending |
| PERF-03 | Phase 7 | Pending |

**Coverage:**
- v1 requirements: 34 total
- Mapped to phases: 34
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-20*
*Last updated: 2026-03-20 — traceability filled by roadmapper*
