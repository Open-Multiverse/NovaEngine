# Phase 1: 测试基础设施修复 - 研究报告

**研究日期：** 2026-03-20
**领域：** Rust / Bevy ECS 测试基础设施修复
**置信度：** HIGH（所有结论来自直接阅读实际源代码）

---

<user_constraints>
## 用户约束（来自 CONTEXT.md）

### 已锁定决策

**区域 A：CharacterStats / CharacterBundle**
- `CharacterStats` 是 `Attributes` 的 builder 包装，内部拥有 `Attributes` 实例
- `CharacterBundle` 包含 `stats: CharacterStats`、`state: CharacterState`、`transform: Transform`、`visibility: Visibility`
- `Attributes` 不单独出现在 Bundle 中，通过 `stats.attributes` 访问
- `CharacterStats` 实现 `Component`，Bundle 实现 `Default`
- 不新增独立数据字段；现有 `nova_character::Attributes` 保持不变

**区域 B：BehaviorTree Builder API**
- 在 `BehaviorTree` 上直接实现 `sequence()`、`action(f)`、`child(tree)` 方法
- `ActionNode` 新增 `Custom(Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>)` 变体
- `Custom` 变体使用 `Arc`（非 `Box`）
- 现有 `standard_soldier()` / `coward()` 预设树不修改

**区域 C：Blackboard 类型**
- `HashMap<String, Box<dyn Any + Send + Sync>>` 存储
- `Blackboard` 不实现 `Clone`
- 实现 `Component` 和 `Default`

**TESTFIX-05：**
- `BrowserCompatibility::supports_webgpu()` 在非 WASM 环境改返回 `false`

**新类型存放位置：**
| 类型 | 文件 |
|------|------|
| `CharacterStats`, `CharacterBundle` | `crates/nova_character/src/character.rs` |
| `AiAgent`, `Blackboard` | `crates/nova_ai/src/decision.rs` |
| BehaviorTree builder 方法 | `crates/nova_ai/src/behavior.rs` |

### Claude 酌情处理

无明确标注，以下问题在直接执行中酌情处理：
- `CharacterState::current()` 方法（测试调用，原代码不存在）
- `BehaviorTree::default()` 实现（测试调用，原代码不存在）
- `behavior_tree_system` 更新为包含 `Blackboard` 查询以支持 `Custom` 动作执行
- 集成测试文件位置问题（`tests/integration/` 需挂载到正确 crate）

### 延期想法（超出本阶段范围）

- `BehaviorTree::root.clone()` 的性能问题（Phase 7 处理）
- 任何非测试修复的功能实现
</user_constraints>

---

<phase_requirements>
## Phase 需求

| ID | 描述 | 研究支撑 |
|----|------|----------|
| TESTFIX-01 | `CharacterBundle` 和 `CharacterStats` 在 `nova_character` 中存在且可从集成测试导入 | `character.rs` 中添加两个类型；`prelude.rs` 已有 `pub use crate::character::*` 会自动导出 |
| TESTFIX-02 | `AiAgent`、`Blackboard` 在 `nova_ai` 中存在且可从集成测试导入 | `decision.rs` 中添加两个类型；`prelude.rs` 已有 `pub use crate::decision::*` 会自动导出 |
| TESTFIX-03 | `BehaviorTree::sequence()` / `action()` builder API 在 `nova_ai` 中可用 | `behavior.rs` 已有 `BehaviorTree`，在其上新增方法即可 |
| TESTFIX-04 | `cargo test --all` 全部通过，无编译错误 | 修复以上三项后，还需解决 `CharacterState::current()`、`BehaviorTree::default()`、集成测试文件挂载、`behavior_tree_system` 适配 |
| TESTFIX-05 | `BrowserCompatibility::supports_webgpu()` 非 WASM 返回 `false` | `nova_test/src/wasm.rs:111` 一行修改 |
</phase_requirements>

---

## 概要

Phase 1 是纯代码修复阶段，目标是让 `cargo test --all` 编译并通过。当前集成测试因引用了尚不存在的类型（`CharacterBundle`、`CharacterStats`、`AiAgent`、`Blackboard`）和 API（`BehaviorTree::sequence()`、`action()`）而无法编译。

通过阅读实际源代码，已确认需要修改的文件共 **5 个**，并发现了 CONTEXT.md 未明确列出的 **3 处附加修复点**（`CharacterState::current()`、`BehaviorTree::default()`、`behavior_tree_system` 签名更新）。这些均属于编译/测试失败的直接原因，需一并处理。

**主要建议：** 不引入任何新依赖，所有修复均为在现有文件中添加类型和方法，变更范围小且风险低。

---

## 标准技术栈

### 核心（已存在，无需改动）

| 库 | 版本 | 用途 |
|----|------|------|
| bevy | 0.15 | ECS 运行时、`Component`、`Bundle`、`Transform`、`Visibility` |
| std::any::Any | stdlib | `Blackboard` 类型擦除存储 |
| std::collections::HashMap | stdlib | `Blackboard` 内部存储 |
| std::sync::Arc | stdlib | `ActionNode::Custom` 闭包共享所有权 |

### 无需新增依赖

Phase 1 所有修复均使用现有依赖，`nova_ai/Cargo.toml` 和 `nova_character/Cargo.toml` 不需要变更。

---

## 架构模式

### 推荐项目修改结构

```
需要修改的文件（共 6 个）：
├── crates/nova_character/src/character.rs     # 添加 CharacterStats, CharacterBundle
├── crates/nova_character/src/state.rs         # 添加 CharacterState::current() 方法
├── crates/nova_ai/src/behavior.rs             # ActionNode::Custom 变体 + BehaviorTree builder 方法
├── crates/nova_ai/src/decision.rs             # AiAgent, Blackboard 类型 + behavior_tree_system 更新
├── crates/nova_test/src/wasm.rs               # L111: true → false
└── （集成测试挂载问题——见下文）
```

### 模式 1：CharacterStats as builder wrapper

**概念：** `CharacterStats` 包装 `Attributes`，提供 builder 风格 API，同时作为 ECS `Component`。

```rust
// crates/nova_character/src/character.rs
// 置信度：HIGH（来自 CONTEXT.md 锁定决策 + 直接查看 Attributes 结构）

use bevy::prelude::*;
use crate::attributes::Attributes;

#[derive(Component, Default)]
pub struct CharacterStats {
    pub name: String,
    pub attributes: Attributes,
}

impl CharacterStats {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }
    pub fn with_health(mut self, hp: f32) -> Self {
        self.attributes.health = crate::attributes::Health::new(hp);
        self
    }
    pub fn with_attack(mut self, val: f32) -> Self {
        self.attributes.attack = val;
        self
    }
    pub fn with_defense(mut self, val: f32) -> Self {
        self.attributes.defense = val;
        self
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn max_health(&self) -> f32 { self.attributes.health.max }
    pub fn attack(&self) -> f32 { self.attributes.attack }
    pub fn defense(&self) -> f32 { self.attributes.defense }
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub stats: CharacterStats,
    pub state: CharacterState,
    pub transform: Transform,
    pub visibility: Visibility,
}
```

注意：`CharacterState` 已实现 `Default`（`#[default] Idle`），`Transform` 和 `Visibility` 已在 Bevy 中实现 `Default`。`CharacterStats` 需手动实现 `Default`（`name` 为空字符串，`attributes` 用 `Attributes::default()`）。

### 模式 2：CharacterState::current() 方法

**问题：** `character_tests.rs:52` 调用 `state.current()`，但 `CharacterState` 是枚举 Component，不存在此方法。

**修复：** 在 `state.rs` 中为 `CharacterState` 添加 `current()` 方法，返回 `&Self`：

```rust
// crates/nova_character/src/state.rs
impl CharacterState {
    pub fn current(&self) -> &Self { self }
    // ... 已有方法不变
}
```

这样 `matches!(state.current(), CharacterState::Idle)` 等价于 `matches!(state, CharacterState::Idle)`，语义正确。

### 模式 3：BehaviorTree builder API + ActionNode::Custom

**核心约束：**
- `ActionNode::Custom` 用 `Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>`
- `BehaviorTree` 需实现 `Default`（测试第 13 行 `BehaviorTree::default()`）

```rust
// crates/nova_ai/src/behavior.rs
use std::sync::Arc;
use crate::decision::Blackboard;  // Blackboard 定义在 decision.rs

pub enum ActionNode {
    Idle,
    MoveTo(MoveTarget),
    Attack(AttackTarget),
    Flee,
    Patrol { points: Vec<Vec3>, current: usize },
    FollowLeader,
    Custom(Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>),
}

impl BehaviorTree {
    pub fn sequence() -> Self {
        Self { root: BehaviorNode::Sequence(vec![]) }
    }
    pub fn action(f: impl Fn(&mut Blackboard) -> bool + Send + Sync + 'static) -> Self {
        Self { root: BehaviorNode::Action(ActionNode::Custom(Arc::new(f))) }
    }
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

impl Default for BehaviorTree {
    fn default() -> Self {
        Self::sequence()  // 默认为空 Sequence 节点
    }
}
```

**关键问题：** `ActionNode` 当前派生 `Clone` 和 `Debug`。添加 `Custom` 变体后，`Arc<dyn Fn...>` 可以 `Clone`（Arc 的 clone 是引用计数增加），但无法自动派生 `Debug`。解决方案：为 `ActionNode` 手动实现 `Debug`，或移除 `#[derive(Clone, Debug)]` 改为手动实现。同样，`BehaviorNode` 派生 `Clone`，其包含 `ActionNode`，传递受影响。

**推荐方案：** 手动实现 `Debug` for `ActionNode`（`Custom` 变体打印固定字符串 `"Custom(fn)"`），保留 `Clone`（因为 Arc clone 是低成本操作）。

### 模式 4：Blackboard 和 AiAgent 组件

```rust
// crates/nova_ai/src/decision.rs
use std::{any::Any, collections::HashMap};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AiAgent;

impl AiAgent {
    pub fn new() -> Self { Self }
}

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

### 模式 5：behavior_tree_system 适配 Custom 动作

**当前问题：** `behavior_tree_system` 的 `Query` 不包含 `Blackboard`，但 `ActionNode::Custom` 需要 `&mut Blackboard` 参数。

**解决方案：** 更新 `behavior_tree_system` 的 Query，加入 `Option<&mut Blackboard>`；在 `execute_action` 中处理 `Custom` 变体：

```rust
// decision.rs behavior_tree_system 更新后的 Query
pub fn behavior_tree_system(
    mut query: Query<(
        Entity,
        &Transform,
        &Attributes,
        &PerceivedEntities,
        Option<&Emotion>,
        &BehaviorTree,
        Option<&mut Blackboard>,  // 新增
    )>,
    mut commands: Commands,
) { ... }
```

`evaluate_node` 和 `execute_action` 签名需传入 `Option<&mut Blackboard>`，在 `ActionNode::Custom` 分支中调用闭包。

**注意：** Bevy 0.15 中 `Query` 包含可变和不可变引用时需注意借用规则。`Option<&mut Blackboard>` 可用，但 `iter_mut()` 才能获取可变引用。应将该 Query 改为 `query.iter_mut()`。

### 模式 6：集成测试文件挂载问题

**现状：** `tests/integration/` 目录位于工作区根目录，**没有对应的 `Cargo.toml`**。该目录下的文件 `use nova_character::...` 和 `use nova_test::TestApp`，说明这些文件需要以某个依赖这两个 crate 的包的集成测试身份运行。

**Cargo 约定：** 包的集成测试放在包根目录下的 `tests/` 子目录。工作区根目录不是一个 Cargo 包，无法直接拥有集成测试。

**推荐方案（置信度 HIGH）：** 将 `tests/integration/` 移入 `crates/nova_test/tests/integration/`，并确认 `nova_test` 的 Cargo.toml 已列出所有需要的依赖（当前已包含 `nova_character`、`nova_ai` 等）。这样 `cargo test -p nova_test` 或 `cargo test --all` 会自动执行这些集成测试。

具体步骤：
1. 在 `crates/nova_test/` 下创建 `tests/` 目录
2. 将 `tests/integration/` 移动或复制到 `crates/nova_test/tests/integration/`
3. Cargo 会将 `tests/integration/mod.rs`（或每个 `.rs` 文件）识别为独立的集成测试二进制

**替代方案：** 新建一个 `crates/nova_integration_tests/` 包，但这会增加额外的 crate，代价更高。

---

## 不要手写的模块

| 问题 | 不要自己写 | 使用 |
|------|------------|------|
| 类型擦除动态存储 | 自定义 trait object 系统 | `std::any::Any + Box<dyn Any>` |
| 引用计数共享闭包 | 自定义智能指针 | `std::sync::Arc` |
| ECS Bundle | 手动管理多组件插入 | `#[derive(Bundle)]` |

---

## 常见陷阱

### 陷阱 1：ActionNode Clone 与 Debug 冲突

**出错原因：** `ActionNode` 当前 `#[derive(Clone, Debug)]`，添加 `Custom(Arc<dyn Fn(...)>)` 后，`dyn Fn` 不能自动派生 `Debug`，编译报错。

**如何避免：** 移除 `ActionNode` 的 `#[derive(Debug)]`，改为手动实现：

```rust
impl std::fmt::Debug for ActionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionNode::Custom(_) => write!(f, "Custom(fn)"),
            // 其他变体使用 derive 行为
            _ => write!(f, "{:?}", self), // 不能这样写！会递归
        }
    }
}
```

正确做法是完整手写 `Debug` impl，或者用 `#[derive(Debug)]` 但对 `Custom` 字段使用 `#[debug(skip)]`（需要 `derivative` crate，不推荐引入新依赖）。最简单是完整手写 `Debug` for `ActionNode`，所有非 `Custom` 变体直接 `write!(f, "ActionNode::{}", variant_name)` 或使用结构化输出。

同样，`BehaviorNode` 的 `Clone` 和 `Debug` 也会受传递影响，因为 `BehaviorNode::Action(ActionNode)` 包含 `ActionNode`。

**警告信号：** 看到 `the trait Debug is not implemented for dyn Fn(...)` 错误。

### 陷阱 2：behavior_tree_system 中的 Bevy 借用规则

**出错原因：** Bevy 系统中同一 Query 包含 `&BehaviorTree`（不可变）和 `&mut Blackboard`（可变）在同一实体上是合法的（不同 Component），但需要使用 `query.iter_mut()` 而不是 `query.iter()`，否则无法获取 `Blackboard` 的可变引用。

**如何避免：** 将 Query 中所有可变组件统一用 `iter_mut()`，对不可变引用仍以 `&` 访问。

### 陷阱 3：nova_ai 中 decision.rs 和 behavior.rs 的循环引用

**出错原因：** `behavior.rs` 中的 `ActionNode::Custom` 需要 `Blackboard` 类型（定义在 `decision.rs`），而 `decision.rs` 的 `behavior_tree_system` 已经 use 了 `behavior.rs` 中的类型。如果两个文件互相引用会产生循环依赖。

**如何避免：** 将 `Blackboard` 定义在 `decision.rs` 或单独的 `blackboard.rs`，然后在 `behavior.rs` 中通过 `use crate::decision::Blackboard` 引用。当前 `decision.rs` 已经 `use crate::behavior::*`，所以 `behavior.rs` 反过来引用 `decision.rs` 中的类型会产生循环。

**解决方案：** 新建 `crates/nova_ai/src/blackboard.rs`，将 `Blackboard` 定义在那里，然后 `behavior.rs` 和 `decision.rs` 都 `use crate::blackboard::Blackboard`。这需要在 `lib.rs` 中添加 `pub mod blackboard`，在 `prelude.rs` 中添加 `pub use crate::blackboard::*`。

**替代方案：** 如果 `behavior.rs` 只用 `Blackboard` 类型引用（不引用 `decision.rs` 中的任何函数），Rust 允许在 `behavior.rs` 中 `use crate::decision::Blackboard`，因为这不是真正的循环（`decision.rs` use 了 `behavior.rs` 的类型，`behavior.rs` 只 use `decision.rs` 的 `Blackboard` 类型）。需验证编译是否通过。

### 陷阱 4：CharacterStats Default 实现

**出错原因：** `CharacterBundle` 派生 `Default` 要求所有字段都实现 `Default`。`CharacterStats` 需要实现 `Default`（`name` 默认空字符串，`attributes` 使用 `Attributes::default()`）。

**如何避免：** 为 `CharacterStats` 添加 `#[derive(Default)]` 或手动 `impl Default`。`String::default()` 返回空字符串，`Attributes` 已实现 `Default`，所以可以直接 `#[derive(Default)]`。

### 陷阱 5：集成测试中的 assert_has_component! 宏

**出错原因：** `assert_has_component!` 宏定义在 `nova_test/src/assertions.rs`，使用 `#[macro_export]`，但集成测试中需要 `use nova_test::*` 或明确引入宏。宏在 Rust 2018+ 中需要通过 `use` 导入或者在同 crate 中直接使用。

**如何避免：** 确认集成测试文件头部有 `use nova_test::*;`（未在当前测试文件中看到，但 `#[macro_export]` 宏在 `use nova_test::*` 后可用）。当前 `character_tests.rs` 只 `use nova_character::...` 和 `use nova_test::TestApp`，没有导入宏。需要添加 `use nova_test::assert_has_component;` 或 `use nova_test::*;`。

---

## 代码示例（来自实际源码）

### CharacterState 现有 Default

```rust
// crates/nova_character/src/state.rs（已存在）
#[derive(Component, Clone, Debug, Default, Reflect, PartialEq)]
pub enum CharacterState {
    #[default]
    Idle,
    // ...
}
```

说明 `CharacterState` 已有 `Default`，`CharacterBundle` 的 `state` 字段可以参与 `Bundle::default()`。

### Attributes 现有字段结构

```rust
// crates/nova_character/src/attributes.rs（已存在）
pub struct Attributes {
    pub health: Health,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub vision_range: f32,
}
```

`CharacterStats::with_health()` 需修改 `health` 字段（`Health::new(hp)`），`with_attack()` 修改 `attack`，`with_defense()` 修改 `defense`。

### BehaviorTree 现有结构

```rust
// crates/nova_ai/src/behavior.rs（已存在）
#[derive(Component, Clone, Debug)]
pub struct BehaviorTree {
    pub root: BehaviorNode,
}
```

添加 `Default` impl 和 builder 方法时，`#[derive(Clone, Debug)]` 在 `ActionNode::Custom` 加入后的处理方式见陷阱 1。

---

## 技术现状

| 旧方式 | 当前方式 | 说明 |
|--------|----------|------|
| CharacterBundle 不存在 | 需新增 | Phase 1 目标 |
| CharacterStats 不存在 | 需新增 | Phase 1 目标 |
| AiAgent 不存在 | 需新增 | Phase 1 目标 |
| Blackboard 不存在 | 需新增 | Phase 1 目标 |
| BehaviorTree 无 builder API | 需添加方法 | Phase 1 目标 |
| supports_webgpu() 非 WASM 返回 true | 改为 false | Phase 1 目标 |

---

## 开放问题

1. **集成测试文件挂载位置**
   - 已知：`tests/integration/` 在工作区根目录，无 Cargo.toml
   - 不确定：是应该移入 `nova_test/tests/` 还是另有打算
   - 建议：移入 `crates/nova_test/tests/integration/`，最小侵入且符合 Cargo 约定

2. **assert_has_component! 宏导入**
   - 已知：测试文件中未导入该宏，但使用了它
   - 不确定：是否依赖隐式 `#[macro_export]` 特性（在 Rust 2018 中 `#[macro_export]` 宏通过 `use crate_name::macro_name` 可访问）
   - 建议：在测试文件中确认宏已正确导入，或添加 `use nova_test::assert_has_component;`

3. **behavior_tree_system 执行 Custom 动作时的上下文**
   - 已知：测试验证 `blackboard.get::<bool>("test")` 返回 true，说明 Custom 闭包必须真正被执行
   - 不确定：现有 `behavior_tree_system` 需要多大程度重构才能支持 Custom 执行（需要 Blackboard 可变访问）
   - 建议：见模式 5，将 `Option<&mut Blackboard>` 加入 Query，`iter()` 改为 `iter_mut()`

---

## 验证架构

### 测试框架

| 属性 | 值 |
|------|----|
| 框架 | `cargo test`（Rust 内置）|
| 配置文件 | `Cargo.toml`（workspace 级别）|
| 快速运行命令 | `cargo test -p nova_character && cargo test -p nova_ai && cargo test -p nova_test` |
| 完整套件命令 | `cargo test --all` |

### Phase 需求 → 测试映射

| 需求 ID | 行为 | 测试类型 | 自动化命令 | 测试文件存在？ |
|---------|------|----------|------------|---------------|
| TESTFIX-01 | CharacterBundle/CharacterStats 可导入和使用 | integration | `cargo test -p nova_test -- character` | ❌ Wave 0 需挂载 |
| TESTFIX-02 | AiAgent/Blackboard 可导入和使用 | integration | `cargo test -p nova_test -- ai` | ❌ Wave 0 需挂载 |
| TESTFIX-03 | BehaviorTree builder API 可用并执行 | integration | `cargo test -p nova_test -- behavior_tree` | ❌ Wave 0 需挂载 |
| TESTFIX-04 | cargo test --all 全部通过 | all | `cargo test --all` | 取决于以上 |
| TESTFIX-05 | supports_webgpu() 非 WASM 返回 false | unit | `cargo test -p nova_test -- supports_webgpu` | ❌ Wave 0（单元测试未覆盖此断言）|

### 采样率

- **每任务提交后：** `cargo check --all-targets`（快速验证编译）
- **每 wave 合并后：** `cargo test --all`
- **Phase 验收：** `cargo test --all` 全绿，无编译警告

### Wave 0 缺口

- [ ] 集成测试文件移动到 `crates/nova_test/tests/integration/`（或等效位置）
- [ ] 确认 `assert_has_component!` 宏在集成测试中可访问
- [ ] `character_tests.rs:52` 的 `state.current()` 方法需添加到 `CharacterState`
- [ ] `ai_tests.rs:13` 的 `BehaviorTree::default()` 需实现 `Default` for `BehaviorTree`

---

## 信息来源

### 主要来源（HIGH 置信度）

- 直接读取 `crates/nova_character/src/character.rs` — 确认 `CharacterStats`/`CharacterBundle` 不存在
- 直接读取 `crates/nova_character/src/attributes.rs` — 确认 `Attributes` 字段结构
- 直接读取 `crates/nova_character/src/state.rs` — 确认 `CharacterState` 结构，无 `current()` 方法
- 直接读取 `crates/nova_character/src/prelude.rs` — 确认 `pub use crate::character::*` 已存在
- 直接读取 `crates/nova_ai/src/behavior.rs` — 确认 `BehaviorTree`/`ActionNode` 现状，无 builder API，无 Default
- 直接读取 `crates/nova_ai/src/decision.rs` — 确认 `Blackboard`/`AiAgent` 不存在，`behavior_tree_system` 无 Blackboard Query
- 直接读取 `crates/nova_ai/src/prelude.rs` — 确认 `pub use crate::decision::*` 已存在
- 直接读取 `crates/nova_test/src/wasm.rs:111` — 确认 `supports_webgpu()` 返回 `true`（需改 `false`）
- 直接读取 `tests/integration/character_tests.rs` — 确认实际测试 API 签名和断言
- 直接读取 `tests/integration/ai_tests.rs` — 确认实际测试 API 签名和断言
- 直接读取 `crates/nova_test/Cargo.toml` — 确认依赖已包含所有需要的 crate
- 直接读取 workspace `Cargo.toml` — 确认 Bevy 0.15，无额外依赖需引入

### 二级来源（MEDIUM 置信度）

- Rust 语言规范：`Arc<dyn Fn(...)>` 实现 `Clone`（引用计数），`dyn Fn` 不实现 `Debug`
- Cargo 文档约定：集成测试放在 `tests/` 目录，工作区根目录不是 Cargo 包

---

## 元数据

**置信度分解：**
- 需修改文件列表：HIGH — 直接读取源码确认
- 修改内容（CharacterStats/Bundle/Blackboard/AiAgent）：HIGH — CONTEXT.md 有精确代码模板
- ActionNode::Custom 的 Debug 派生冲突：HIGH — Rust 类型系统确定性问题
- 集成测试文件挂载方案：MEDIUM — Cargo 约定，未验证 nova_test 运行集成测试
- behavior_tree_system 重构范围：MEDIUM — 需要实际编译验证

**研究日期：** 2026-03-20
**有效期：** 稳定（代码不会自行变化），直到下次代码修改
