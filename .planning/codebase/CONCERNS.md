# Codebase Concerns

**Analysis Date:** 2026-03-20

## Tech Debt

**音频系统为空壳实现:**
- Issue: `nova_audio` 插件仅维护状态字段（`AudioState.last_played_sound`、`AudioState.current_music`），`AudioEvent::PlaySound` 不真正播放任何声音，没有调用 Bevy 的 `AudioSink` 或 `AudioBundle`，整个音频系统是占位符
- Files: `crates/nova_audio/src/plugin.rs`
- Impact: 所有音频事件被静默吞掉，游戏无声音输出；空间音频组件 `AudioSource` 已定义但从未被任何系统读取
- Fix approach: 集成 Bevy 原生音频系统（`AudioPlugin`/`AudioBundle`），将 `process_audio_events` 替换为真实的音频播放逻辑

**GPU 实例化渲染为统计层但无实际 GPU 提交:**
- Issue: `update_instance_batches` 收集 `Mat4` 矩阵到 `InstanceBatches.batches`，但没有系统将这些矩阵提交到 GPU 实例化缓冲区；`dirty` 标志置为 `false` 后从未被读取
- Files: `crates/nova_render/src/performance/instancing.rs`
- Impact: 实例化逻辑仅做统计，实际 draw call 没有减少；`InstancingStats.draw_call_reduction` 数字不反映真实性能提升
- Fix approach: 使用 Bevy 的 `ExtractComponent` + GPU buffer 真正提交实例化数据，或移除假统计以避免误导

**`CharacterStats` / `CharacterBundle` 在集成测试中引用但不存在:**
- Issue: `tests/integration/character_tests.rs` 导入 `nova_character::{CharacterBundle, CharacterState, CharacterStats}`，但 `nova_character` crate 中没有定义 `CharacterStats` 或 `CharacterBundle`；`nova_character::prelude` 只导出 `attributes::*`、`character::*`、`feedback::*`、`loader::*`、`state::*`
- Files: `tests/integration/character_tests.rs`, `crates/nova_character/src/lib.rs`, `crates/nova_character/src/prelude.rs`
- Impact: 集成测试无法编译通过，`cargo test` 会失败
- Fix approach: 在 `nova_character` 中添加 `CharacterStats` 类型（对 `Attributes` 的别名或新结构体）及 `CharacterBundle`，或修改测试使用现有的 `Attributes` + `CharacterState`

**`AiAgent`、`Blackboard`、`BehaviorTree::sequence()`/`action()` 在集成测试中引用但不存在:**
- Issue: `tests/integration/ai_tests.rs` 使用 `nova_ai::{AiAgent, BehaviorTree, Blackboard}` 及 `BehaviorTree::sequence().child(BehaviorTree::action(|blackboard| {...}))` builder 模式，但 `nova_ai` crate 只有 `BehaviorTree::new(root)` / `standard_soldier()` / `coward()`，没有 `AiAgent`、`Blackboard` 或 builder 方法
- Files: `tests/integration/ai_tests.rs`, `crates/nova_ai/src/behavior.rs`, `crates/nova_ai/src/lib.rs`
- Impact: 集成测试无法编译，实际使用的 API 形态与 crate 不匹配
- Fix approach: 在 `nova_ai` 中实现 `AiAgent` 组件、`Blackboard` 组件（键值存储），以及 `BehaviorTree::sequence()` / `action()` builder API；或修改测试使用现有 `BehaviorNode::Sequence` + `BehaviorTree::new()`

**`RenderTest` 截图捕获为未实现占位符:**
- Issue: `RenderTest::capture_and_compare` 包含 `// TODO: 实现截图捕获逻辑`，只执行预热帧然后打印日志，不做任何截图捕获或对比；`ScreenshotManager` 已导入但未使用
- Files: `crates/nova_test/src/render.rs:79`
- Impact: 渲染回归测试没有实际保障，`RenderTestBuilder.expected_path` 字段永远不被消费
- Fix approach: 使用 Bevy 的 `ScreenshotManager::save_screenshot_to_disk` 实现截图捕获，然后做像素差异比较

**`BrowserCompatibility` 非 WASM 环境始终返回 `true`:**
- Issue: `crates/nova_test/src/wasm.rs` 中 `#[cfg(not(target_arch = "wasm32"))]` 实现的 `supports_webgpu()` 和 `supports_webgl2()` 都 `return true`，在原生测试环境跑不出有效信息
- Files: `crates/nova_test/src/wasm.rs:111-118`
- Impact: 兼容性检测函数在 CI（非 WASM）环境下永不失败，无法发现真实问题
- Fix approach: 非 WASM 实现应 `return false` 或 `cfg!(test)` 下明确标注跳过

**`NovaApp` 窗口标题修改有 Clone 泄漏:**
- Issue: `with_title` 在闭包中 `clone()` 标题字符串，每次窗口系统运行都会 clone；且 Startup 系统添加了 `get_single_mut` 冗余调用，若未来有多窗口会 panic
- Files: `crates/nova_core/src/app.rs:51-55`
- Impact: 轻微内存开销；多窗口场景下会 panic
- Fix approach: 通过 `WindowPlugin` 直接设置标题而不通过 Startup 系统

---

## Known Bugs

**`BehaviorTree::standard_soldier` 使用 `Entity::PLACEHOLDER` 作为追击目标:**
- Symptoms: 调用 `standard_soldier()` 生成的 AI 单位在"追击"分支会向 `Entity::PLACEHOLDER`（无效 entity）移动，导致移动逻辑无效或 panic
- Files: `crates/nova_ai/src/behavior.rs:102`
- Trigger: 任何使用 `standard_soldier()` 行为树且感知到敌人时
- Workaround: 手动构造 `BehaviorNode::Action(ActionNode::MoveTo(MoveTarget::Entity(real_entity)))` 替代

**`update_instance_batches` 在每帧清空后将 `dirty` 置为 `false`，但 `dirty` 从未被置为 `true`:**
- Symptoms: `InstanceBatches.dirty` 始终为 `false`，如果有系统依赖它触发 GPU 上传则永远不触发
- Files: `crates/nova_render/src/performance/instancing.rs:65`
- Trigger: 任何使用 `dirty` 标志的下游系统
- Workaround: 当前无下游系统读取 `dirty`，所以不造成可见错误，但属于逻辑缺陷

**`wasm_utils::sleep` 中多个 `.unwrap()` 在 WASM 环境下可能 panic:**
- Symptoms: `web_sys::window()` 返回 `None` 时（如 Worker 环境）、`set_timeout_with_callback` 失败时，直接 unwrap 会 panic
- Files: `crates/nova_test/src/wasm.rs:64-72`
- Trigger: 在非主线程 WASM 环境中调用 `sleep()`
- Workaround: 使用 `?` 传播或 `unwrap_or`

---

## Security Considerations

**无安全相关代码（低风险）:**
- Risk: 项目为本地游戏引擎，无网络请求、无用户输入校验漏洞、无 `unsafe` 代码块
- Files: 全局搜索未发现 `unsafe` 块（仅 `Entity::from_raw` 用于测试）
- Current mitigation: N/A
- Recommendations: 若后续加入 WASM 跨域加载资源，需校验来源；`CharacterDef::from_json` 直接 `serde_json::from_str` 无大小限制，处理不可信输入时需加上 `max_size` 检查

---

## Performance Bottlenecks

**A* 寻路无缓存，每次调用分配新 `HashMap` 和 `BinaryHeap`:**
- Problem: `Pathfinder::find_path` 每次调用创建新的 `came_from`、`g_score` HashMap 和 open_set heap，GC 压力大
- Files: `crates/nova_map/src/pathfinding.rs:72-74`
- Cause: 无对象池，无路径缓存，单帧多单位寻路时成本成倍增长
- Improvement path: 引入寻路请求队列，每帧限制计算单位数；或缓存最近计算的路径（key: start+goal）

**`FogOfWar::add_vision` 每次调用都进行全范围矩形扫描:**
- Problem: `add_vision(center_x, center_y, range)` 遍历 `(2*range+1)^2` 个格子，对大 range 值（如 20）每帧调用 100 个单位时计算量 = `100 * 41^2 = 168,100` 次迭代
- Files: `crates/nova_map/src/fog.rs:88-132`
- Cause: 没有增量更新，每次调用全量重算视野影响区域
- Improvement path: 记录上一帧位置，仅在移动时更新；或使用视野锥形剔除

**LOD 系统每帧对所有 LOD 实体进行距离计算:**
- Problem: `update_lod` 对每个 LOD 实体都计算 `distance()`，无空间分区
- Files: `crates/nova_render/src/performance/lod.rs:51-92`
- Cause: 无空间索引，实体数量大时每帧 O(n) 遍历
- Improvement path: 与 `SpatialGrid` 结合使用，只对相机附近实体计算 LOD

**`behavior_tree_system` 每帧 clone 整棵行为树根节点:**
- Problem: `let root = tree.root.clone()` 在每帧每个 AI 单位处都发生，`BehaviorNode` 为递归枚举包含 `Vec<BehaviorNode>`，深拷贝成本随树深度增长
- Files: `crates/nova_ai/src/decision.rs:172`
- Cause: 因 borrow checker 冲突（同时借用 `tree` 和 `entity_commands`）而采用了 clone 规避
- Improvement path: 将行为树存储为独立 Resource（arena），通过 index 引用而非 clone；或分离 readonly BehaviorTree 和 mutable state

---

## Fragile Areas

**`SceneDefinition::to_json` / `from_json` 中使用 `unwrap()`（测试代码中）:**
- Files: `crates/nova_core/src/scene.rs:369-370`
- Why fragile: 测试中 `scene.to_json().unwrap()` 若序列化失败会 panic，错误信息不明确
- Safe modification: 改为 `expect("场景序列化失败")` 或在测试外使用 `?` 返回 `Result`
- Test coverage: 有 JSON roundtrip 测试，但仅覆盖正常路径

**`SceneTester::calculate_scene_bounds` 遍历所有 `Transform` 而非仅网格实体:**
- Files: `crates/nova_test/src/render.rs:125`
- Why fragile: `world.query::<&Transform>()` 会包含相机、灯光、UI 元素等非网格实体，导致包围盒计算偏大
- Safe modification: 添加 `With<Mesh3d>` 过滤器
- Test coverage: 测试使用裸 Transform 组件通过，不能反映真实场景的包围盒

**`Lod::new` 的 `assert!(!levels.is_empty())` 在运行时 panic:**
- Files: `crates/nova_render/src/performance/lod.rs:27`
- Why fragile: 若从配置文件加载空的 LOD 配置，会在创建时 panic 而非返回 `Error`
- Safe modification: 改为 `Result<Self, &'static str>` 返回值
- Test coverage: 测试仅覆盖正常带 level 的情况

**`update_animation_players` 直接写入 `*transform = sampled`，覆盖所有变换:**
- Files: `crates/nova_animation/src/player.rs:172`
- Why fragile: 动画采样结果完全覆盖 Transform，与物理引擎（Rapier）的位置更新产生冲突；若同时有物理和动画组件，两者会相互覆盖
- Safe modification: 使用独立的动画骨架 Transform 层，或添加 `With<AnimationDriven>` 标记排除物理实体
- Test coverage: 动画 player 测试没有物理组件，不能检测此冲突

---

## Scaling Limits

**`TileMap` 以 `Vec<Tile>` 存储全量瓦片，超大地图内存压力大:**
- Current capacity: 以默认 `f32` 大小估算，1000x1000 地图 = ~1M tiles
- Limit: 超过 4096x4096 时 clone 操作（`TileMap: Clone`）会导致显著延迟
- Scaling path: 分块存储（chunk-based），惰性加载；移除 `TileMap: Clone` 避免意外大 clone

**`AssetRegistry` 使用 `HashSet<String>` 存储资源路径（字符串持有者）:**
- Current capacity: 合理
- Limit: 大量资源注册时字符串内存占用较高，且 `all_registered()` 返回的顺序不确定影响加载顺序
- Scaling path: 改为 `HashSet<AssetId>` 或 interned string

---

## Dependencies at Risk

**`bevy_rapier3d = "0.28"` 版本耦合 Bevy 0.15:**
- Risk: `bevy_rapier3d` 0.28 与 Bevy 0.15 绑定；Bevy 升级到 0.16 时需等待 `bevy_rapier3d` 跟进
- Impact: Bevy 升级路径被 Rapier 版本阻断
- Migration plan: 评估迁移到 `avian3d`（原 `bevy_xpbd`），其版本发布周期与 Bevy 更同步

**`bevy_egui = "0.31"` 同样强耦合 Bevy 版本:**
- Risk: Bevy 升级后 `bevy_egui` 需单独发布新版
- Impact: UI 系统随 Bevy 升级阻塞
- Migration plan: 持续关注 `bevy_egui` 发布节奏；或评估 Bevy 原生 UI（`bevy_ui`）替代

---

## Missing Critical Features

**实际音频播放:**
- Problem: `nova_audio` 全为占位符，项目目前没有任何声音
- Blocks: 所有需要音效/背景音乐的游戏功能
- Files: `crates/nova_audio/src/plugin.rs`

**角色伤害/攻击系统缺失核心逻辑:**
- Problem: `CharacterState::Attacking { target }` 定义了攻击状态，但没有任何系统处理攻击状态下的伤害结算（查询目标、扣血、触发事件）；`ActionNode::Attack` 在 `decision.rs` 中命令插入 `CharacterState::Attacking`，但后续无伤害系统响应
- Blocks: 任何战斗玩法
- Files: `crates/nova_character/src/state.rs`, `crates/nova_ai/src/decision.rs`

**移动执行系统缺失:**
- Problem: `ActionNode::MoveTo` 将状态设为 `CharacterState::Moving { target }`，但没有系统在此状态下实际更新 `Transform` 使单位向目标移动
- Blocks: AI 单位实际移动
- Files: `crates/nova_ai/src/decision.rs:130-138`, `crates/nova_character/src/state.rs`

---

## Test Coverage Gaps

**行为树条件节点未测试:**
- What's not tested: `ConditionNode::HasTarget`、`EnemyInRange`、`HealthBelow`、`EmotionIs` 等条件节点的求值逻辑（`evaluate_node` 中的 `BehaviorNode::Condition` 分支）
- Files: `crates/nova_ai/src/decision.rs`
- Risk: 条件判断错误会导致 AI 行为异常但无法自动发现
- Priority: High

**物理事件转发未测试:**
- What's not tested: `nova_physics/src/events.rs` 中 Rapier 碰撞事件转 Nova 碰撞事件的逻辑
- Files: `crates/nova_physics/src/events.rs`
- Risk: 碰撞触发器失效时无法感知
- Priority: High

**编队移动系统未测试:**
- What's not tested: `nova_formation` 的 `movement.rs` 系统中实际移动逻辑；仅有模式计算（`patterns.rs`）和槽位分配（`slots.rs`）的单元测试
- Files: `crates/nova_formation/src/movement.rs`
- Risk: 编队移动在多单位场景下的边界情况未被覆盖
- Priority: Medium

**WASM 测试只在浏览器环境运行，CI 中无法自动执行:**
- What's not tested: `nova_test/src/wasm.rs` 中的浏览器兼容性测试、WASM 环境断言
- Files: `crates/nova_test/src/wasm.rs`
- Risk: WASM 特定 bug（如 WebGPU 上下文创建失败）在合并时不被捕获
- Priority: Medium

**LOD 级别切换的材质更新路径未测试:**
- What's not tested: `update_lod` 中的 `material` 分支，即 LOD 切换时同时更新材质的逻辑
- Files: `crates/nova_render/src/performance/lod.rs:84-89`
- Risk: 材质不随 LOD 切换，视觉效果与预期不符
- Priority: Low

---

*Concerns audit: 2026-03-20*
