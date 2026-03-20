# Roadmap: Nova Engine

## Overview

从现有的结构完整但系统空缺的代码库出发，逐步补齐使引擎真正可运行所需的核心机制：首先修复阻塞 CI 的集成测试，再依次实现移动执行、战斗伤害、AI 行为树完善、音频真实播放、渲染诚实化、性能关键路径优化，最终完成在浏览器中可玩的 RTS 演示。

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: 测试基础设施修复** - 补齐缺失 API，使全部集成测试可编译并通过
- [ ] **Phase 2: 角色移动系统** - 实现 MoveTo 执行侧，AI 单位 Transform 真正移动
- [ ] **Phase 3: 战斗伤害系统** - 实现 Attacking 状态的伤害结算、扣血与死亡流程
- [ ] **Phase 4: AI 行为树完善** - 修复 Entity::PLACEHOLDER bug，补齐条件节点求值
- [ ] **Phase 5: 音频真实实现** - 将音频占位符替换为 Bevy AudioPlugin 真实播放
- [ ] **Phase 6: 渲染诚实化与截图测试** - 修正 GPU 实例化统计虚报，实现截图回归测试
- [ ] **Phase 7: 性能关键路径优化** - A* 缓存、FogOfWar 增量更新、行为树 clone 消除
- [ ] **Phase 8: RTS Demo 端到端集成** - 整合所有系统，Demo 在浏览器中可玩

## Phase Details

### Phase 1: 测试基础设施修复
**Goal**: 集成测试可以编译并全部通过，CI 不再因缺失类型或错误 API 而失败
**Depends on**: Nothing (first phase)
**Requirements**: TESTFIX-01, TESTFIX-02, TESTFIX-03, TESTFIX-04, TESTFIX-05
**Success Criteria** (what must be TRUE):
  1. `cargo test --all` 执行完成，无编译错误，无失败测试
  2. `CharacterBundle` 和 `CharacterStats` 可从 `nova_character` 正常导入和使用
  3. `AiAgent`、`Blackboard`、`BehaviorTree::sequence()`/`action()` 可从 `nova_ai` 正常导入和使用
  4. `BrowserCompatibility::supports_webgpu()` 在原生（非 WASM）环境返回 `false`
**Plans**: 3 plans

Plans:
- [ ] 01-01-PLAN.md — 在 nova_character 中添加 CharacterStats/CharacterBundle 类型及 CharacterState::current() 方法
- [ ] 01-02-PLAN.md — 新建 blackboard.rs，添加 AiAgent 组件，为 BehaviorTree 实现 builder API 和 ActionNode::Custom
- [ ] 01-03-PLAN.md — 挂载集成测试到 nova_test crate，修复 supports_webgpu() 非 WASM 返回值，验证 cargo test --all 全绿

### Phase 2: 角色移动系统
**Goal**: AI 单位在被指令移动时，Transform 每帧真正向目标更新，并与 PathFollow 寻路集成
**Depends on**: Phase 1
**Requirements**: MOVE-01, MOVE-02, MOVE-03, MOVE-04
**Success Criteria** (what must be TRUE):
  1. 带 `CharacterState::Moving { target }` 的单位每帧位置向目标靠近，可在场景中观察到
  2. 单位到达目标后 `CharacterState` 自动切换为 `Idle`，停止移动
  3. 移动速度与 `Attributes.speed` 值成正比，修改该值可观察到速度变化
  4. 单位跟随 `PathFollow` 路径点移动，遇到地形障碍时绕行而非穿越
**Plans**: 3 plans

Plans:
- [ ] 02-01: 实现 movement_system（读取 CharacterState::Moving，更新 Transform）
- [ ] 02-02: 集成 PathFollow 路径跟随，实现到达判定与状态切换

### Phase 3: 战斗伤害系统
**Goal**: 单位进入 Attacking 状态后触发完整的伤害结算链：扣血、死亡移除、攻击冷却、事件广播
**Depends on**: Phase 2
**Requirements**: COMBAT-01, COMBAT-02, COMBAT-03, COMBAT-04, COMBAT-05
**Success Criteria** (what must be TRUE):
  1. 处于 `CharacterState::Attacking { target }` 的单位对目标造成伤害，目标血条数值下降
  2. 目标血量归零后从场景中消失（Dead 状态 + 实体移除）
  3. 攻击冷却期间同一单位无法再次造成伤害（可通过调高冷却值验证）
  4. `CombatEvent` 可被外部监听器捕获，日志中可见伤害事件记录
**Plans**: 3 plans

Plans:
- [ ] 03-01: 实现 combat_system（查询 Attacking 目标，扣除 health，发送 CombatEvent）
- [ ] 03-02: 实现死亡判定与实体清理（health <= 0 → Dead 状态 → despawn）
- [ ] 03-03: 实现 AttackCooldown 计时器，阻止连续攻击

### Phase 4: AI 行为树完善
**Goal**: 行为树使用真实感知数据驱动决策，条件节点有正确求值，AI 单位行为可预测且可测试
**Depends on**: Phase 3
**Requirements**: AI-01, AI-02, AI-03, AI-04
**Success Criteria** (what must be TRUE):
  1. `standard_soldier()` AI 单位追击时移动目标是真实感知到的敌方实体，不再使用占位 Entity
  2. `HasTarget`、`EnemyInRange`、`HealthBelow` 条件节点可独立单元测试，结果符合预期
  3. `Blackboard` 可被行为树节点读写，键值变化可通过调试面板观察
  4. 只有带 `AiAgent` 组件的实体受行为树系统处理，其他实体不受影响
**Plans**: 3 plans

Plans:
- [ ] 04-01: 修复 standard_soldier Entity::PLACEHOLDER bug，接入真实感知事件
- [ ] 04-02: 为 HasTarget / EnemyInRange / HealthBelow 条件节点实现求值逻辑并补充单元测试
- [ ] 04-03: 确认 AiAgent / Blackboard 组件在行为树系统中正确过滤与读写

### Phase 5: 音频真实实现
**Goal**: AudioEvent 事件触发真实的音频播放，背景音乐和音效可在浏览器中听到
**Depends on**: Phase 1
**Requirements**: AUDIO-01, AUDIO-02, AUDIO-03, AUDIO-04
**Success Criteria** (what must be TRUE):
  1. 发送 `AudioEvent::PlaySound` 后浏览器中可听到对应音效
  2. 发送 `AudioEvent::PlayMusic` 后背景音乐循环播放
  3. 发送 `AudioEvent::StopMusic` 后背景音乐立即停止
  4. 带 `SpatialAudioSettings` 的声源随距离衰减，远处声音比近处小
**Plans**: 3 plans

Plans:
- [ ] 05-01: 将 process_audio_events 替换为 Bevy AudioPlugin / AudioBundle 真实调用
- [ ] 05-02: 实现 SpatialAudioSettings 的 3D 衰减逻辑

### Phase 6: 渲染诚实化与截图测试
**Goal**: GPU 实例化统计反映真实 draw call，截图测试可捕获帧并做像素对比
**Depends on**: Phase 1
**Requirements**: RENDER-01, RENDER-02, RENDER-03, RENDERTEST-01, RENDERTEST-02
**Success Criteria** (what must be TRUE):
  1. `InstanceBatches` 的实例数据实际提交 GPU 绘制（或假统计代码被移除，不再误导性地报告 draw call 减少）
  2. `InstanceBatches.dirty` 标志在批次内容变化时置为 `true`，状态与数据同步
  3. `SceneTester::calculate_scene_bounds` 仅统计带 `Mesh3d` 的实体，相机/灯光不计入包围盒
  4. `RenderTest::capture_and_compare` 实际捕获一帧截图并与预期图像做像素差异比较，超过阈值则失败
**Plans**: 3 plans

Plans:
- [ ] 06-01: 修复 InstanceBatches dirty 标志逻辑，实现真实 GPU 提交或移除假统计
- [ ] 06-02: 修复 calculate_scene_bounds 添加 With<Mesh3d> 过滤器
- [ ] 06-03: 实现 RenderTest::capture_and_compare 截图捕获与像素对比

### Phase 7: 性能关键路径优化
**Goal**: A* 路径缓存、FogOfWar 增量更新、行为树无每帧 clone，消除已知的性能关键瓶颈
**Depends on**: Phase 4
**Requirements**: PERF-01, PERF-02, PERF-03
**Success Criteria** (what must be TRUE):
  1. 对同一 start+goal 的重复寻路请求直接返回缓存结果，可通过日志统计命中率验证
  2. 单位静止时 `FogOfWar` 更新函数不被调用，只有移动单位触发视野重算
  3. `behavior_tree_system` 中不再出现每帧整棵行为树的深拷贝，内存分配曲线平稳
**Plans**: 3 plans

Plans:
- [ ] 07-01: 为 Pathfinder::find_path 添加 LRU 路径缓存
- [ ] 07-02: 为 FogOfWar::add_vision 添加位置变化检测，仅增量更新
- [ ] 07-03: 重构 behavior_tree_system 消除 root.clone()，改用 index 引用或分离只读结构

### Phase 8: RTS Demo 端到端集成
**Goal**: RTS Demo 在 Trunk WASM 构建后可在 Chrome 浏览器中运行，玩家可选单位移动、单位战斗、AI 自主决策
**Depends on**: Phase 4, Phase 5, Phase 7
**Requirements**: DEMO-01, DEMO-02, DEMO-03, DEMO-04
**Success Criteria** (what must be TRUE):
  1. 玩家可在浏览器中框选单位并点击地图位置，单位沿 A* 路径移动到目标
  2. 单位移动过程中战争迷雾随视野更新，已探索区域保持可见
  3. AI 单位在感知到敌方单位后自动发起攻击，战斗过程可观察到血条变化与单位消失
  4. `trunk build --release` 成功，产物在 Chrome 113+ 中无控制台错误地运行
**Plans**: 3 plans

Plans:
- [ ] 08-01: 补齐 RTS Demo 对移动/战斗/AI 系统的调用连接
- [ ] 08-02: 验证 WASM 构建与浏览器运行，修复任何 wasm32 编译问题

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8
Note: Phase 5 (音频) 和 Phase 6 (渲染) 依赖 Phase 1 但与 Phase 2-4 可并行执行。

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. 测试基础设施修复 | 1/3 | In Progress|  |
| 2. 角色移动系统 | 0/2 | Not started | - |
| 3. 战斗伤害系统 | 0/3 | Not started | - |
| 4. AI 行为树完善 | 0/3 | Not started | - |
| 5. 音频真实实现 | 0/2 | Not started | - |
| 6. 渲染诚实化与截图测试 | 0/3 | Not started | - |
| 7. 性能关键路径优化 | 0/3 | Not started | - |
| 8. RTS Demo 端到端集成 | 0/2 | Not started | - |
