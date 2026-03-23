# Phase 02：角色移动系统 - 研究报告

**研究日期：** 2026-03-23
**领域：** Bevy ECS 移动系统 / A* 寻路集成 / CharacterState 状态驱动移动
**置信度：** HIGH（所有关键类型直接从项目源码验证）

---

<user_constraints>
## 用户约束（来自 CONTEXT.md）

### 锁定决策

**移动执行方式：**
- 始终经过 A* 寻路：MoveTo 指令触发后，移动系统调用 `Pathfinder::find_path` 计算路径，写入 `PathFollow`，再按路径点逐步移动
- 无 TileMap 时降级为直线移动：场景中没有 TileMap Resource 时，退化为直接线性插值到目标 Vec3（保证单元测试可用）
- 进入 Moving 状态时寻路一次：寻路仅在实体首次进入 `Moving` 状态时触发，结果写入 `PathFollow` 缓存，后续每帧跟随路径点移动，不重新寻路

**PathFollow 集成：**
- 移动系统负责写入 PathFollow：移动系统检测到实体从非 Moving 切换为 Moving 状态时，立即调用 A* 并将结果写入 `PathFollow` 组件
- 用 `PreviousCharacterState` 组件记录上一帧状态：每帧将当前 `CharacterState` 写入 `PreviousCharacterState`，移动系统通过对比两者判断是否首次进入 Moving
- 系统执行顺序：`update_previous_state_system` → `movement_system`（同一 `Update` 调度集，依赖链 after/before 声明）

**到达判定与状态切换：**
- 到达阈值：0.5 单位（与现有 `formation_follow_system` 的 `if distance > 0.5` 一致）
- 路径点到达后立即前进到下一点：调用 `PathFollow::advance()`，不停顿
- 终点到达后移动系统直接 `insert(CharacterState::Idle)`：同时设置 `PathFollow.finished = true`，下一帧移除 PathFollow 组件

**系统归属：**
- 移动系统放在 `nova_character` crate：移动逻辑属于"角色行为"范畴，关注 `CharacterState` 变化
- 将 `nova_map` 加入 `nova_character` 的 Cargo.toml 依赖：`nova_character` 目前仅依赖 `nova_core` 和 `bevy`，需要新增 `nova_map = { workspace = true }`
- `NovaCharacterPlugin` 注册新系统：在 `NovaCharacterPlugin::build()` 中追加 `update_previous_state_system` 和 `movement_system`

### Claude's Discretion

- `PreviousCharacterState` 的具体存储形式（可复用 `CharacterState` 枚举 + newtype wrapper，或简单 bool `was_moving`）
- 移动系统的 `Query` 参数结构（`With<PathFollow>` 拆分 vs 统一 Query）
- `PathFollow` 组件的生命周期管理（何时添加 / 何时移除）

### 延期想法（超出本 Phase 范围）

- A* 路径缓存（相同 start+goal 避免重复计算）— Phase 7 PERF-01
- 目标变化时重新寻路（追击场景）— Phase 4 AI 行为树完善后再处理
</user_constraints>

---

<phase_requirements>
## Phase 需求

| ID | 描述 | 研究支撑 |
|----|------|---------|
| MOVE-01 | AI 单位在 `CharacterState::Moving { target }` 时，其 `Transform` 每帧向 `target` 位置更新 | `formation_follow_system` 的 `diff.normalize() * step.min(distance)` 模式可直接复用；`Attributes.move_speed` 默认 5.0 |
| MOVE-02 | 单位到达目标后自动切换为 `CharacterState::Idle` | `stun_tick_system` 的直接赋值模式：`*state = CharacterState::Idle`；到达阈值 0.5 已验证 |
| MOVE-03 | 移动速度通过 `Attributes.speed` 或配置值驱动 | `Attributes.move_speed: f32`（默认 5.0）已存在于 `nova_character/src/attributes.rs` |
| MOVE-04 | 移动系统与寻路系统（`PathFollow`）集成，单位沿路径点移动 | `PathFollow::new/current_target/advance` API 已存在；`Pathfinder::find_path(tilemap, start, goal)` 签名已验证；`TileMap::world_to_tile` 和 `tile_to_world` 均可用 |
</phase_requirements>

---

## 摘要

本 Phase 核心任务是将已有的 `CharacterState::Moving { target }` 状态与 `Transform` 更新、A* 寻路系统打通，使 AI 单位能够实际移动。项目已有所有必要基础设施：`PathFollow` 组件、`Pathfinder::find_path`、`TileMap` 坐标转换函数，以及 `formation_follow_system` 中可直接复用的移动模式。

需要新增的内容：(1) `nova_character/Cargo.toml` 新增 `nova_map` 依赖；(2) `PreviousCharacterState` 组件（简单 bool 包装即可）；(3) `update_previous_state_system` 和 `movement_system` 两个系统，注册到 `NovaCharacterPlugin`。

**主要建议：** 直接复制 `formation_follow_system` 的移动核心逻辑，将硬编码速度 `5.0` 替换为 `Attributes.move_speed`，在此基础上加入 PathFollow 路径点跟踪和状态切换逻辑。

---

## 标准技术栈

### 核心（已存在于项目）

| 类型/函数 | 位置 | 用途 | 置信度 |
|-----------|------|------|--------|
| `CharacterState::Moving { target: Vec3 }` | `nova_character/src/state.rs:12` | 驱动移动的状态枚举变体 | HIGH |
| `Attributes.move_speed: f32` | `nova_character/src/attributes.rs:45` | 移动速度源，默认 5.0 | HIGH |
| `PathFollow` 组件 | `nova_map/src/pathfinding.rs:152` | 路径点序列 + 当前索引 + 完成标志 | HIGH |
| `PathFollow::new(path)` | `nova_map/src/pathfinding.rs:163` | 从 `Vec<(u32,u32)>` 创建路径跟随组件 | HIGH |
| `PathFollow::current_target()` | `nova_map/src/pathfinding.rs:172` | 返回 `Option<(u32,u32)>` 当前目标瓦片 | HIGH |
| `PathFollow::advance()` | `nova_map/src/pathfinding.rs:176` | 前进到下一路径点；超出范围时置 `finished = true` | HIGH |
| `Pathfinder::find_path(tilemap, start, goal)` | `nova_map/src/pathfinding.rs:55` | A* 寻路，返回 `Option<PathResult>`；`PathResult.path: Vec<(u32,u32)>` | HIGH |
| `TileMap::world_to_tile(world_pos: Vec3)` | `nova_map/src/tilemap.rs:100` | 世界坐标转瓦片坐标，返回 `Option<(u32,u32)>` | HIGH |
| `TileMap::tile_to_world(x, y)` | `nova_map/src/tilemap.rs:123` | 瓦片坐标转世界坐标（瓦片中心），返回 `Vec3` | HIGH |
| `TileMap`（Resource） | `nova_map/src/tilemap.rs:9` | 通过 `Option<Res<TileMap>>` 访问，无 TileMap 时降级直线移动 | HIGH |

### 新增依赖

```toml
# nova_character/Cargo.toml [dependencies] 新增：
nova_map = { workspace = true }
```

---

## 架构模式

### 推荐文件结构

```
crates/nova_character/src/
├── movement.rs          # 新建：PreviousCharacterState + 两个系统
├── state.rs             # 已有：CharacterState 枚举
├── attributes.rs        # 已有：Attributes（含 move_speed）
├── lib.rs               # 修改：注册新系统 + pub mod movement
└── ...
```

### 模式 1：`PreviousCharacterState` 组件设计

**决策区域（Claude's Discretion）**

推荐使用简单 bool newtype，避免 Clone `CharacterState` 枚举（枚举含 `Vec3`/`Entity` 字段）：

```rust
// 来源：CONTEXT.md 设计决策 + state.rs 类型分析
/// 记录上一帧是否处于 Moving 状态（用于首次进入 Moving 检测）
#[derive(Component, Debug, Default)]
pub struct PreviousCharacterState {
    pub was_moving: bool,
}
```

**替代方案**（若未来需要记录更多状态变化）：存储完整 `CharacterState` 克隆，需要 `CharacterState: Clone`（已派生）。

### 模式 2：系统执行顺序声明

```rust
// 来源：nova_character/src/lib.rs 现有模式 + CONTEXT.md 系统顺序决策
app.add_systems(
    Update,
    (
        update_previous_state_system,
        movement_system.after(update_previous_state_system),
        state::stun_tick_system,
        // ...其他已有系统
    ),
);
```

### 模式 3：核心移动逻辑（直接复用 formation_follow_system）

```rust
// 来源：nova_formation/src/movement.rs:31-38（直接复用模式）
let diff = target_world_pos - transform.translation;
let distance = diff.length();

if distance > 0.5 {
    let step = attrs.move_speed * time.delta_secs();
    transform.translation += diff.normalize() * step.min(distance);
}
```

### 模式 4：CharacterState 直接赋值

```rust
// 来源：nova_character/src/state.rs:83（stun_tick_system 模式）
// 通过 mut Query 直接赋值，无需 Commands
*state = CharacterState::Idle;
```

### 模式 5：movement_system 整体逻辑流

```
每帧对每个实体：
1. 读取 CharacterState — 若非 Moving，跳过
2. 读取 PreviousCharacterState.was_moving
3. 若首次进入 Moving（!was_moving）：
   a. 读取 Option<Res<TileMap>>
   b. 有 TileMap：world_to_tile(start) + world_to_tile(target) → find_path → PathFollow::new → commands.insert
   c. 无 TileMap：直接记录 target 用于直线移动（不插入 PathFollow）
4. 若有 PathFollow 组件：
   a. current_target() → tile_to_world → diff/distance 计算
   b. distance > 0.5：更新 Transform
   c. distance <= 0.5：advance()
   d. PathFollow.finished：*state = Idle + commands.remove::<PathFollow>
5. 若无 PathFollow（直线降级）：
   a. 从 CharacterState::Moving { target } 读取目标
   b. 标准 diff/distance 移动
   c. distance <= 0.5：*state = Idle
```

### 反模式（需避免）

- **使用 Commands 更改 CharacterState**：当前帧内用 `mut Query<&mut CharacterState>` 直接赋值更安全（避免一帧延迟）；仅终点到达时考虑 Commands（若需同帧移除 PathFollow）
- **每帧重新寻路**：CONTEXT.md 明确锁定"进入 Moving 状态时寻路一次"
- **直接克隆 CharacterState 到 PreviousCharacterState**：`CharacterState::Attacking { target: Entity }` 可能指向已销毁实体，只需存 bool

---

## 不要手写（已有实现）

| 问题 | 不要写 | 直接使用 | 原因 |
|------|--------|----------|------|
| A* 算法 | 自定义路径搜索 | `Pathfinder::find_path` | 项目已有完整 A* + 8方向 + 对角线代价 |
| 路径点推进 | 自定义索引管理 | `PathFollow::advance()` + `current_target()` | 已有完整 finished 标志逻辑 |
| 坐标转换 | 手算瓦片/世界坐标 | `TileMap::tile_to_world` / `world_to_tile` | 已考虑地图中心偏移和高度 |
| 移动插值 | 自定义 lerp/移动 | 复用 `formation_follow_system` 模式 | 经过验证的 `step.min(distance)` 防止超越目标 |

---

## 常见陷阱

### 陷阱 1：`world_to_tile` 返回 None 时未处理

**触发场景：** 单位位置在地图边界外，或 target 超出地图范围时 `world_to_tile` 返回 `None`。
**后果：** 寻路无法触发，单位静止不动，状态永远不切换为 Idle。
**规避：** 寻路失败时（find_path 返回 None，或 world_to_tile 返回 None）直接降级为直线移动到 target；不能 panic。

### 陷阱 2：`PathFollow` 空路径（start == goal）

**触发场景：** `Pathfinder::find_path` 在 start == goal 时返回 `Some(PathResult { path: vec![], cost: 0.0 })`（已确认，见 pathfinding.rs:56-61）。
**后果：** `PathFollow::new(vec![])` 创建后 `current_target()` 立即返回 `None`，`advance()` 立即置 `finished = true`。
**规避：** 在写入 PathFollow 前检查 `path.is_empty()`；若为空，直接设置 `CharacterState::Idle`，不插入 PathFollow 组件。

### 陷阱 3：系统顺序错误导致状态检测失效

**触发场景：** `movement_system` 在 `update_previous_state_system` 之前运行。
**后果：** `PreviousCharacterState.was_moving` 与当前帧相同，永远无法检测到"首次进入 Moving"。
**规避：** 用 `.after(update_previous_state_system)` 明确声明依赖顺序。

### 陷阱 4：`update_previous_state_system` 处理实体集不一致

**触发场景：** `PreviousCharacterState` 组件未随 `CharacterState` 一起 spawn。
**后果：** 新生成的 Moving 单位 Query 中找不到 `PreviousCharacterState`，系统跳过该实体。
**规避：** 将 `PreviousCharacterState` 加入 `CharacterBundle`（默认 `was_moving: false`），或在 movement_system 中用 `Option<&PreviousCharacterState>` 查询并默认 `was_moving = false`。

### 陷阱 5：无 TileMap 时的日志噪音

**触发场景：** 单元测试环境无 TileMap Resource，移动系统降级为直线移动。
**后果：** 每帧打 warn! 日志，测试输出噪音。
**规避：** 无 TileMap 时静默降级，不打任何日志（CONTEXT.md 明确要求）。

### 陷阱 6：PathFollow 组件移除时序

**触发场景：** 在 movement_system 内同帧通过 `Commands::remove` 移除 PathFollow，但当帧 Query 仍能读到它。
**后果：** 逻辑上已完成但本帧还会执行一次多余的移动步骤。
**规避：** 这是 Bevy Commands 延迟语义的正常行为，不是 bug；设置 `PathFollow.finished = true` 作为本帧跳过标志，`remove` 命令下帧生效。

---

## 代码示例

### 查询签名参考（基于现有模式）

```rust
// 来源：nova_character/src/state.rs:77（stun_tick_system）
// 直接 mut Query 赋值模式
pub fn movement_system(
    time: Res<Time>,
    tilemap: Option<Res<TileMap>>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut CharacterState,
        &Attributes,
        Option<&mut PathFollow>,
        Option<&PreviousCharacterState>,
    ), With<CharacterState>>,
) { ... }
```

### `update_previous_state_system` 参考

```rust
// 来源：CONTEXT.md 决策 + state.rs 类型
pub fn update_previous_state_system(
    mut query: Query<(&CharacterState, &mut PreviousCharacterState)>,
) {
    for (state, mut prev) in query.iter_mut() {
        prev.was_moving = state.is_moving();
    }
}
```

### 寻路触发示例

```rust
// 来源：nova_map/src/pathfinding.rs API + tilemap.rs API（直接从源码验证）
if let Some(tilemap) = tilemap.as_deref() {
    if let (Some(start_tile), Some(goal_tile)) = (
        tilemap.world_to_tile(transform.translation),
        tilemap.world_to_tile(target),
    ) {
        if let Some(result) = Pathfinder::find_path(tilemap, start_tile, goal_tile) {
            if result.path.is_empty() {
                *state = CharacterState::Idle;
            } else {
                commands.entity(entity).insert(PathFollow::new(result.path));
            }
        } else {
            // 寻路失败（目标不可达），直线降级
        }
    }
} else {
    // 无 TileMap，直线移动（直接用 target Vec3）
}
```

---

## 技术现状

| 旧方式 | 当前方式 | 影响 |
|--------|----------|------|
| `CharacterState::Moving` 无系统响应（Phase 2 前的状态） | Phase 2 新增 `movement_system` 响应 Moving 状态 | AI 单位从静止变为可移动 |
| `formation_follow_system` 硬编码速度 5.0 | 角色移动系统从 `Attributes.move_speed` 读取速度 | 支持 MOVE-03 速度可配置 |
| `PathFollow` 仅有数据结构，无系统驱动 | Phase 2 移动系统负责写入和消费 PathFollow | MOVE-04 寻路集成实现 |

---

## 开放问题

1. **`PreviousCharacterState` 应加入 `CharacterBundle` 还是按需插入？**
   - 已知：CharacterBundle 在 `nova_character/src/character.rs:104` 定义，当前字段：stats/state/transform/visibility
   - 未知：是否所有 CharacterBundle 实体都需要移动能力（可能存在纯静态角色）
   - 建议：加入 CharacterBundle（默认 `was_moving: false`），保证所有角色实体一致，简化 Query

2. **PathFollow 组件生命周期：是否需要 Remove 系统？**
   - 已知：CONTEXT.md 决策"下一帧移除 PathFollow 组件"，通过 `commands.entity(e).remove::<PathFollow>()` 实现
   - 已知：Bevy Commands 是延迟执行的，同帧设置 `finished = true` 可作为跳过标志
   - 建议：在 movement_system 末尾通过 Commands 移除，本帧用 `finished` 标志跳过额外执行

---

## 验证架构

### 测试框架

| 属性 | 值 |
|------|-----|
| 框架 | Rust `cargo test`（内置）|
| 集成测试位置 | `crates/nova_test/tests/integration/` |
| 单元测试位置 | 各 crate `src/*.rs` 内 `#[cfg(test)]` 模块 |
| 快速运行命令 | `cargo test -p nova_character` |
| 全套运行命令 | `cargo test --all` |
| TestApp 工具 | `nova_test::TestApp`（已验证，Phase 1 使用中）|

### 需求 → 测试映射

| 需求 ID | 行为 | 测试类型 | 自动化命令 | 文件已存在？ |
|---------|------|----------|------------|-------------|
| MOVE-01 | Moving 状态下 Transform 每帧更新 | 集成测试 | `cargo test -p nova_test test_movement_updates_transform` | 待建（Wave 0）|
| MOVE-02 | 到达目标后切换为 Idle | 集成测试 | `cargo test -p nova_test test_movement_state_transition_to_idle` | 待建（Wave 0）|
| MOVE-03 | move_speed 影响移动速度 | 集成测试 | `cargo test -p nova_test test_movement_speed_from_attributes` | 待建（Wave 0）|
| MOVE-04 | 沿 PathFollow 路径点移动 | 集成测试 | `cargo test -p nova_test test_pathfollow_integration` | 待建（Wave 0）|

### 采样率

- **每次任务提交后：** `cargo test -p nova_character && cargo test -p nova_test`
- **每个 Wave 合并后：** `cargo test --all`
- **Phase 门控：** 全套测试绿色通过后再运行 `/gsd:verify-work`

### Wave 0 缺口

- [ ] `crates/nova_test/tests/integration/movement_tests.rs` — 覆盖 MOVE-01 至 MOVE-04
- [ ] `crates/nova_character/src/movement.rs` — 本文件内可含 `#[cfg(test)]` 单元测试（PreviousCharacterState 逻辑）
- [ ] 无需新增 nova_character 对 nova_test 的依赖（nova_test 已依赖 nova_character，见 Cargo.toml）

---

## 来源

### 主要来源（HIGH 置信度，直接阅读源码）

- `crates/nova_character/src/state.rs` — CharacterState 枚举、stun_tick_system 模式
- `crates/nova_character/src/attributes.rs` — Attributes.move_speed 字段（默认 5.0）
- `crates/nova_character/src/character.rs` — CharacterBundle 结构
- `crates/nova_character/src/lib.rs` — NovaCharacterPlugin::build() 系统注册位置
- `crates/nova_character/Cargo.toml` — 当前依赖（确认缺少 nova_map）
- `crates/nova_map/src/pathfinding.rs` — PathFollow/Pathfinder 完整 API
- `crates/nova_map/src/tilemap.rs` — TileMap::world_to_tile / tile_to_world 签名
- `crates/nova_formation/src/movement.rs` — formation_follow_system 移动模式（直接复用参考）
- `crates/nova_test/src/app_runner.rs` — TestApp 测试基础设施
- `crates/nova_test/tests/integration/character_tests.rs` — 集成测试模式参考

### 决策来源（HIGH 置信度，用户锁定）

- `.planning/phases/02-角色移动系统/02-CONTEXT.md` — 所有架构决策

---

## 元数据

**置信度分解：**

- 标准技术栈：HIGH — 所有类型和函数均直接从项目源码验证
- 架构模式：HIGH — 基于已验证的现有系统模式（stun_tick_system、formation_follow_system）
- 常见陷阱：HIGH — 基于 Pathfinder/PathFollow 源码的边界条件分析
- 测试映射：MEDIUM — 测试用例名称待实现时确定，但框架和模式已验证

**研究日期：** 2026-03-23
**有效期预估：** 30 天（Bevy 0.15 锁定版本，稳定期长）
