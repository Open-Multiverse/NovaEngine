# Phase 1 决策上下文：测试基础设施修复

**Phase 目标：** `cargo test --all` 编译通过且全部测试通过
**讨论日期：** 2026-03-20
**状态：** 决策已锁定，可供 researcher 和 planner 直接使用

---

## 区域 A：CharacterBundle / CharacterStats 设计

### 决策

**CharacterStats 是 Attributes 的 builder 包装，内部拥有 Attributes 实例。**

```rust
pub struct CharacterStats {
    pub name: String,
    pub attributes: Attributes,
}

impl CharacterStats {
    pub fn new(name: &str) -> Self { ... }
    pub fn with_health(self, hp: f32) -> Self { ... }
    pub fn with_attack(self, val: f32) -> Self { ... }
    pub fn with_defense(self, val: f32) -> Self { ... }

    // 委托访问器（满足测试）
    pub fn name(&self) -> &str { &self.name }
    pub fn max_health(&self) -> f32 { self.attributes.health.max }
    pub fn attack(&self) -> f32 { self.attributes.attack }
    pub fn defense(&self) -> f32 { self.attributes.defense }
}
```

**CharacterBundle 结构：**

```rust
#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub stats: CharacterStats,      // 内含 Attributes（唯一数据来源）
    pub state: CharacterState,
    pub transform: Transform,
    pub visibility: Visibility,
}
```

- `Attributes` **不**单独出现在 Bundle 中，通过 `stats.attributes` 访问
- Bundle 实现 `Default`（满足测试 `CharacterBundle::default()`）
- `CharacterStats` 实现 `Component`

### 约束

- 不新增独立数据字段，`CharacterStats` 仅是命名包装 + builder
- 现有 `nova_character::Attributes` 保持不变

---

## 区域 B：BehaviorTree Builder API

### 决策

**在 `BehaviorTree` 上直接实现 builder 方法；在 `ActionNode` 中新增 `Custom` 变体支持闭包。**

```rust
// ActionNode 新增变体
pub enum ActionNode {
    Idle,
    MoveTo(MoveTarget),
    Attack(AttackTarget),
    Flee,
    Patrol { points: Vec<Vec3>, current: usize },
    FollowLeader,
    Custom(Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>),  // 新增
}

// BehaviorTree 新增 builder 方法
impl BehaviorTree {
    /// 创建顺序节点（从测试入口）
    pub fn sequence() -> Self {
        Self { root: BehaviorNode::Sequence(vec![]) }
    }

    /// 创建自定义动作节点（接受闭包）
    pub fn action(f: impl Fn(&mut Blackboard) -> bool + Send + Sync + 'static) -> Self {
        Self { root: BehaviorNode::Action(ActionNode::Custom(Arc::new(f))) }
    }

    /// 链式添加子节点（仅对 Sequence/Selector/Parallel 有效）
    pub fn child(mut self, child: BehaviorTree) -> Self {
        match &mut self.root {
            BehaviorNode::Sequence(children)
            | BehaviorNode::Selector(children)
            | BehaviorNode::Parallel(children) => {
                children.push(child.root);
            }
            _ => {}
        }
        self
    }
}
```

### 约束

- `Custom` 变体使用 `Arc`（非 `Box`）以便在行为树系统中共享（避免 clone 问题）
- 现有 `standard_soldier()` / `coward()` 预设树**不修改**
- `ActionNode::Custom` 的闭包参数类型固定为 `&mut Blackboard`（满足测试签名）

---

## 区域 C：Blackboard 类型系统

### 决策

**`Blackboard` 用 `HashMap<String, Box<dyn Any + Send + Sync>>` 存储；不实现 `Clone`。**

```rust
use std::{any::Any, collections::HashMap};

#[derive(Component, Default)]
pub struct Blackboard {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Blackboard {
    pub fn set<T: Any + Send + Sync>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }

    pub fn get<T: Any + 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref()
    }
}
```

**Blackboard 不实现 Clone** — 行为树系统通过 `Query<&mut Blackboard>` 访问，无需 clone。

### 约束

- `BehaviorTree` 的 `root.clone()` 问题（Phase 7 范围）在本阶段不处理
- `ActionNode::Custom` 闭包接收 `&mut Blackboard`（已与 B 区域对齐）
- `Blackboard` 实现 `Component` 和 `Default`（满足测试 `Blackboard::default()`）

---

## 额外修复（无灰色地带，直接执行）

### TESTFIX-05：BrowserCompatibility 非 WASM 返回值

**问题：** `crates/nova_test/src/wasm.rs:111` — 非 WASM 下 `supports_webgpu()` 返回 `true`，但测试期望 `false`

**修复：** 直接改为 `false`

```rust
#[cfg(not(target_arch = "wasm32"))]
pub fn supports_webgpu() -> bool {
    false  // 非 WASM 环境不支持 WebGPU
}
```

---

## 新类型的存放位置

| 类型 | 存放位置 |
|------|----------|
| `CharacterStats` | `crates/nova_character/src/character.rs`（与 `Character` 同文件） |
| `CharacterBundle` | `crates/nova_character/src/character.rs` |
| `AiAgent` | `crates/nova_ai/src/decision.rs`（新组件，标记 AI 控制实体） |
| `Blackboard` | `crates/nova_ai/src/decision.rs`（或新建 `blackboard.rs`） |
| BehaviorTree builder 方法 | `crates/nova_ai/src/behavior.rs`（已有 `BehaviorTree` 定义处） |

### AiAgent 定义（简单标记组件）

```rust
#[derive(Component, Default)]
pub struct AiAgent;

impl AiAgent {
    pub fn new() -> Self { Self }
}
```

---

## 代码上下文（供 researcher 参考）

```
nova_character/src/
  character.rs     → 添加 CharacterStats, CharacterBundle（L1-42 现有代码）
  attributes.rs    → Attributes 不变（L41-63 现有）
  state.rs         → CharacterState 不变（L7-19 现有）
  prelude.rs       → 需更新导出

nova_ai/src/
  behavior.rs      → BehaviorTree 添加 builder 方法，ActionNode 添加 Custom（L81-125）
  decision.rs      → 添加 AiAgent, Blackboard 类型（L150-176 现有 behavior_tree_system 需更新 Query）
  prelude.rs       → 需更新导出

nova_test/src/
  wasm.rs          → L111 改 true → false
```

---

## 集成测试验证点

所有决策完成后，以下测试应通过：

1. `tests/integration/character_tests.rs` — `CharacterBundle::default()` 可生成，`CharacterStats` 方法返回正确值
2. `tests/integration/ai_tests.rs` — `AiAgent::new()`、`BehaviorTree::sequence().child(BehaviorTree::action(|bb| {...}))`、`Blackboard::default()` 均可用，行为树执行后 blackboard 写入正确
3. `BrowserCompatibility::supports_webgpu()` 在原生测试返回 `false`

---

*Phase 1 context — ready for research and planning*
