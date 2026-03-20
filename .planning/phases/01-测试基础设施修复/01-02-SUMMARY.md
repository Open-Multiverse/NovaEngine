---
phase: 01-测试基础设施修复
plan: "02"
subsystem: testing
tags: [bevy, ecs, behavior-tree, blackboard, ai, arc, type-erasure]

# Dependency graph
requires:
  - phase: 01-测试基础设施修复
    provides: nova_ai crate 基础结构（behavior.rs、decision.rs 已存在）
provides:
  - nova_ai::Blackboard 组件（泛型 set/get，类型擦除存储）
  - nova_ai::AiAgent 标记组件
  - ActionNode::Custom 变体（Arc 闭包，可 Clone）
  - BehaviorTree builder API（sequence/action/child/Default）
  - behavior_tree_system 支持 Custom 动作执行并写入 Blackboard
affects: [02-移动系统实现, wave-2-test-mounting]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "类型擦除黑板：HashMap<String, Box<dyn Any + Send + Sync>> 支持任意类型存储"
    - "Arc 包装闭包使 ActionNode::Custom 可 Clone"
    - "evaluate_node/execute_action 传递 &mut Option<Mut<Blackboard>> 避免 unsafe"

key-files:
  created:
    - crates/nova_ai/src/blackboard.rs
  modified:
    - crates/nova_ai/src/behavior.rs
    - crates/nova_ai/src/decision.rs
    - crates/nova_ai/src/lib.rs
    - crates/nova_ai/src/prelude.rs

key-decisions:
  - "使用替代方案 B（安全方案）：evaluate_node/execute_action 传递 blackboard 参数，而非在 BtContext 存储原始指针"
  - "ActionNode 去掉 #[derive(Debug)]，改为手动实现 Debug，因为 Arc<dyn Fn> 不实现 Debug"
  - "Blackboard 不实现 Clone，通过 Query<&mut Blackboard> 访问，无需克隆"

patterns-established:
  - "AI 黑板模式：行为树节点通过 Blackboard 共享运行时状态，键类型安全（downcast_ref）"
  - "BehaviorTree builder 模式：sequence().child(action(f)) 链式构造"

requirements-completed: [TESTFIX-02, TESTFIX-03]

# Metrics
duration: 3min
completed: 2026-03-20
---

# Phase 01 Plan 02: nova_ai Blackboard/AiAgent/Custom 动作补齐 Summary

**为 nova_ai crate 补齐 Blackboard 组件、AiAgent 标记组件、ActionNode::Custom 变体和 BehaviorTree builder API，使集成测试 ai_tests.rs 的 API 可以编译并执行**

## Performance

- **Duration:** 约 3 分钟
- **Started:** 2026-03-20T11:05:37Z
- **Completed:** 2026-03-20T11:08:06Z
- **Tasks:** 3
- **Files modified:** 5（含 1 新建）

## Accomplishments

- 新建 blackboard.rs：Blackboard 组件，泛型 set/get，类型擦除（Box<dyn Any + Send + Sync>）
- 在 behavior.rs 添加 ActionNode::Custom 变体（Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>）和 BehaviorTree builder API（sequence/action/child/Default）
- 在 decision.rs 添加 AiAgent 标记组件，更新 behavior_tree_system 支持 Custom 动作执行并写入 Blackboard

## Task Commits

每个任务已原子提交：

1. **任务 1：新建 blackboard.rs** - `3affcc1` (feat)
2. **任务 2+3：ActionNode::Custom + AiAgent + behavior_tree_system** - `79bf67a` (feat)

## Files Created/Modified

- `crates/nova_ai/src/blackboard.rs` — Blackboard 组件，set/get 泛型方法，手动 Debug 实现
- `crates/nova_ai/src/behavior.rs` — 添加 ActionNode::Custom 变体、手动 Debug、BehaviorTree builder 方法和 Default
- `crates/nova_ai/src/decision.rs` — 新增 AiAgent 组件，evaluate_node/execute_action 添加 blackboard 参数，behavior_tree_system Query 添加 Option<&mut Blackboard>
- `crates/nova_ai/src/lib.rs` — 注册 pub mod blackboard
- `crates/nova_ai/src/prelude.rs` — 添加 pub use crate::blackboard::*

## 核心 API 签名

```rust
// Blackboard
pub fn set<T: Any + Send + Sync>(&mut self, key: &str, value: T)
pub fn get<T: Any + 'static>(&self, key: &str) -> Option<&T>

// AiAgent
#[derive(Component, Default, Clone, Debug)]
pub struct AiAgent;
impl AiAgent { pub fn new() -> Self }

// ActionNode::Custom
Custom(Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>)

// BehaviorTree builder
pub fn sequence() -> Self
pub fn action(f: impl Fn(&mut Blackboard) -> bool + Send + Sync + 'static) -> Self
pub fn child(mut self, child: BehaviorTree) -> Self
impl Default for BehaviorTree { fn default() -> Self { Self::sequence() } }

// behavior_tree_system Query（新增 Option<&mut Blackboard>）
Query<(Entity, &Transform, &Attributes, &PerceivedEntities, Option<&Emotion>, &BehaviorTree, Option<&mut Blackboard>)>
```

## Decisions Made

- 使用"替代方案 B"（安全方案）：evaluate_node/execute_action 传递 `blackboard: &mut Option<Mut<Blackboard>>` 参数，而非在 BtContext 中存储原始指针（避免 unsafe）
- ActionNode 去掉 `#[derive(Debug)]`，改为手动实现，因为 `Arc<dyn Fn>` 不实现 Debug
- Blackboard 不实现 Clone，符合 ECS 组件的访问语义

## Deviations from Plan

无——计划执行完全按照规格，包括选择替代方案 B。

## Issues Encountered

任务 2 完成后 `cargo check -p nova_ai` 出现 non-exhaustive patterns 错误（decision.rs 中 execute_action 缺少 Custom 分支），这是预期内的中间状态——任务 3 立即修复了该错误，两个任务合并为一个提交。

## Next Phase Readiness

- nova_ai::AiAgent、nova_ai::Blackboard、nova_ai::BehaviorTree 可从外部 crate 导入
- BehaviorTree::sequence().child(BehaviorTree::action(f)) 链式调用可编译
- behavior_tree_system 处理 Custom 动作时写入 Blackboard
- Wave 2 的测试挂载计划（ai 相关测试）现在可以运行

## Self-Check: PASSED

- blackboard.rs: FOUND
- behavior.rs: FOUND
- decision.rs: FOUND
- commit 3affcc1: FOUND
- commit 79bf67a: FOUND

---
*Phase: 01-测试基础设施修复*
*Completed: 2026-03-20*
