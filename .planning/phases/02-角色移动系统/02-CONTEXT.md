# Phase 2：角色移动系统 - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

让 `CharacterState::Moving { target: Vec3 }` 状态实际驱动单位的 `Transform` 每帧向目标更新，并与 `PathFollow` 寻路系统集成，使 AI 单位可以沿路径移动。

**不在本 Phase 范围内：**
- 战斗/攻击相关逻辑（Phase 3）
- AI 行为树完善（Phase 4）
- 性能优化 A* 缓存（Phase 7）

</domain>

<decisions>
## Implementation Decisions

### 移动执行方式
- **始终经过 A\* 寻路**：MoveTo 指令触发后，移动系统调用 `Pathfinder::find_path` 计算路径，写入 `PathFollow`，再按路径点逐步移动
- **无 TileMap 时降级为直线移动**：场景中没有 TileMap Resource 时，退化为直接线性插值到目标 Vec3（保证单元测试可用）
- **进入 Moving 状态时寻路一次**：寻路仅在实体首次进入 `Moving` 状态时触发，结果写入 `PathFollow` 缓存，后续每帧跟随路径点移动，不重新寻路

### PathFollow 集成
- **移动系统负责写入 PathFollow**：移动系统检测到实体从非 Moving 切换为 Moving 状态时，立即调用 A\* 并将结果写入 `PathFollow` 组件
- **用 `PreviousCharacterState` 组件记录上一帧状态**：每帧将当前 `CharacterState` 写入 `PreviousCharacterState`，移动系统通过对比两者判断是否首次进入 Moving
- **系统执行顺序**：`update_previous_state_system` → `movement_system`（同一 `Update` 调度集，依赖链 after/before 声明）

### 到达判定与状态切换
- **到达阈值：0.5 单位**（与现有 `formation_follow_system` 的 `if distance > 0.5` 一致）
- **路径点到达后立即前进到下一点**：调用 `PathFollow::advance()`，不停顿
- **终点到达后移动系统直接 `insert(CharacterState::Idle)`**：同时设置 `PathFollow.finished = true`，下一帧移除 PathFollow 组件

### 系统归属
- **移动系统放在 `nova_character` crate**：移动逻辑属于"角色行为"范畴，关注 `CharacterState` 变化
- **将 `nova_map` 加入 `nova_character` 的 Cargo.toml 依赖**：`nova_character` 目前仅依赖 `nova_core` 和 `bevy`，需要新增 `nova_map = { workspace = true }`
- **`NovaCharacterPlugin` 注册新系统**：在 `NovaCharacterPlugin::build()` 中追加 `update_previous_state_system` 和 `movement_system`

### Claude's Discretion
- `PreviousCharacterState` 的具体存储形式（可复用 `CharacterState` 枚举 + newtype wrapper，或简单 bool `was_moving`）
- 移动系统的 `Query` 参数结构（`With<PathFollow>` 拆分 vs 统一 Query）
- `PathFollow` 组件的生命周期管理（何时添加 / 何时移除）

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### 需求定义
- `.planning/REQUIREMENTS.md` §角色移动系统（MOVE） — MOVE-01 至 MOVE-04 的验收标准

### 现有核心类型
- `crates/nova_character/src/state.rs` — `CharacterState` 枚举定义（Moving/Idle/Attacking/Dead 等），`stun_tick_system` 参考
- `crates/nova_character/src/attributes.rs` — `Attributes.move_speed`（默认 5.0），驱动移动速度
- `crates/nova_map/src/pathfinding.rs` — `PathFollow` 结构体（path/current_index/finished），`Pathfinder::find_path` 签名
- `crates/nova_map/src/tilemap.rs` — `TileMap::tile_to_world(x, y)` 坐标转换，`TileMap::world_to_tile(pos)` 反向转换

### 参考实现模式
- `crates/nova_formation/src/movement.rs` — `formation_follow_system` 移动模式（diff.normalize() * step.min(distance)，阈值 0.5）— **直接复用此模式**
- `crates/nova_character/src/lib.rs` — `NovaCharacterPlugin::build()` — 新系统注册位置

### 无外部规格文档
No external specs — requirements are fully captured in decisions above and REQUIREMENTS.md.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `formation_follow_system` 的移动逻辑（`diff.normalize() * step.min(distance)`）— 直接复用，速度从 `Attributes.move_speed` 读取
- `PathFollow::advance()` — 已有推进路径索引的方法，`PathFollow::current_target()` 返回当前目标瓦片
- `TileMap::tile_to_world(x, y)` — 将 `(u32, u32)` 瓦片坐标转为 `Vec3` 世界坐标，移动系统需要此函数
- `Pathfinder` — 已有 A\* 实现，`find_path(start, goal, tilemap)` 返回 `Vec<(u32,u32)>`

### Established Patterns
- `stun_tick_system` — `CharacterState` 变更模式：直接 `*state = CharacterState::Idle`，移动系统可复用同样的 mut Query 写法
- `behavior_tree_system` — 通过 `commands.entity(entity).insert(CharacterState::Moving {...})` 写入状态（Commands 模式），移动系统用同样方式 insert Idle

### Integration Points
- `NovaCharacterPlugin::build()` — 移动系统需注册到这里（Update 调度）
- `nova_character/Cargo.toml` — 需新增 `nova_map = { workspace = true }` 依赖
- `behavior_tree_system`（nova_ai）— AI 通过 Commands 写入 Moving 状态，移动系统响应；两系统在同一 Update 帧运行，behavior_tree_system 写入后下一帧移动系统才响应（正常）

</code_context>

<specifics>
## Specific Ideas

- 无 TileMap 时直线降级行为要**静默**处理（不打日志），以免单元测试输出噪音
- `PreviousCharacterState` 只需存储"上一帧是否是 Moving"，不需要完整克隆状态枚举（节省 Clone 开销）

</specifics>

<deferred>
## Deferred Ideas

- A\* 路径缓存（相同 start+goal 避免重复计算）— Phase 7 PERF-01
- 目标变化时重新寻路（追击场景）— Phase 4 AI 行为树完善后再处理

</deferred>

---

*Phase: 02-角色移动系统*
*Context gathered: 2026-03-23*
