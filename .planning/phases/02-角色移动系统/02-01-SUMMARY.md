---
phase: 02-角色移动系统
plan: "01"
subsystem: character

tags: [bevy, ecs, movement, pathfinding, a-star]

# Dependency graph
requires:
  - phase: 01-测试基础设施修复
    provides: CharacterBundle, CharacterState, Attributes API
  - phase: 02-角色移动系统
    provides: TileMap, PathFollow, Pathfinder (nova_map)
provides:
  - PreviousCharacterState 组件（记录上一帧移动状态）
  - update_previous_state_system（状态历史更新）
  - movement_system（A* 寻路 + 直线降级移动）
affects:
  - nova_ai（AI 行为树写入 Moving 状态后，movement_system 响应）
  - nova_formation（编队移动系统可复用移动模式）
  - rts_demo（单位可实际移动）

# Tech tracking
tech-stack:
  added: [nova_map 依赖]
  patterns:
    - "ECS 组件模式：PreviousCharacterState 作为状态历史记录"
    - "系统执行顺序：update_previous_state_system → movement_system"
    - "Option<Res<TileMap>> 模式：可选资源静默降级"
    - "Commands 延迟删除：PathFollow 完成后 remove 组件"

key-files:
  created:
    - crates/nova_character/src/movement.rs
  modified:
    - crates/nova_character/Cargo.toml
    - crates/nova_character/src/character.rs
    - crates/nova_character/src/lib.rs

key-decisions:
  - "PreviousCharacterState 使用 bool newtype 而非完整 CharacterState 克隆，避免 Entity/Vec3 字段的克隆开销"
  - "系统执行顺序确保 update_previous_state_system 在 movement_system 之前，正确检测首次进入 Moving"
  - "无 TileMap 时静默降级为直线移动，不打任何日志（符合 CONTEXT.md 决策）"
  - "到达判定距离 0.5 单位，与 nova_formation 移动模式保持一致"

patterns-established:
  - "状态历史模式：使用 PreviousXxx 组件记录上一帧状态，用于检测状态转换"
  - "可选资源降级：Option<Res<T>> 模式处理可能不存在的资源"
  - "移动插值模式：diff.normalize() * step.min(distance) 防止超越目标"

requirements-completed: [MOVE-01, MOVE-02, MOVE-03, MOVE-04]

# Metrics
duration: 8min
completed: 2026-03-23
---

# Phase 02 Plan 01: 角色移动系统实现 Summary

**CharacterState::Moving 驱动的 Transform 更新系统，集成 A* 寻路与直线降级移动**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-23T06:21:00Z
- **Completed:** 2026-03-23T06:28:58Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- 创建 `movement.rs` 包含完整的移动系统实现
- 实现 `PreviousCharacterState` 组件用于检测首次进入 Moving 状态
- 实现 `update_previous_state_system` 在 `movement_system` 之前更新状态历史
- 实现 `movement_system` 支持 A* 寻路（有 TileMap 时）和直线降级（无 TileMap 时）
- `CharacterBundle` 新增 `prev_state` 字段，确保所有角色实体携带状态历史组件
- `NovaCharacterPlugin` 注册两个新系统并设置正确的执行顺序

## Task Commits

Each task was committed atomically:

1. **Task 1: 新增 nova_map 依赖 + 创建 movement.rs** - `f2fb7a2` (feat)
2. **Task 2: 更新 CharacterBundle 和 NovaCharacterPlugin** - `64cf03a` (feat)

## Files Created/Modified

- `crates/nova_character/Cargo.toml` - 新增 nova_map 依赖
- `crates/nova_character/src/movement.rs` - 新建：PreviousCharacterState 组件、update_previous_state_system、movement_system
- `crates/nova_character/src/character.rs` - CharacterBundle 新增 prev_state 字段
- `crates/nova_character/src/lib.rs` - 注册 movement 模块、导出 PreviousCharacterState、注册两个系统到 Update

## Decisions Made

- **PreviousCharacterState 使用 bool newtype**：避免克隆含 Entity/Vec3 的完整 CharacterState，减少内存拷贝
- **系统执行顺序明确指定**：`movement_system.after(update_previous_state_system)` 确保状态历史正确更新后再做移动判断
- **无 TileMap 时静默降级**：符合 CONTEXT.md 决策，不输出任何 warn/info 日志
- **到达判定距离 0.5 单位**：与 nova_formation 中的移动模式保持一致

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None. All compilation checks passed on first attempt.

Pre-existing compilation errors in `nova_test` and `nova_inspector` crates are unrelated to this plan's changes (they involve Bevy API version mismatches and missing features).

## Verification

```bash
# nova_character 编译通过
cargo check -p nova_character
# Finished dev profile [unoptimized + debuginfo] target(s)

# 下游依赖 crate 编译通过
cargo check -p nova_character -p nova_ai -p nova_formation -p nova_engine
# Finished dev profile [unoptimized + debuginfo] target(s)
```

## Next Phase Readiness

- AI 行为树写入 `CharacterState::Moving { target }` 后，`movement_system` 将实际驱动单位移动
- 单位到达目标后自动切换为 `Idle` 状态
- 为 Phase 3（战斗系统）和 RTS Demo 端到端可玩性奠定基础

---

*Phase: 02-角色移动系统*
*Completed: 2026-03-23*
