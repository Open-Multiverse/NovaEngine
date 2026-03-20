# Nova Character 人物建模系统实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 nova_character、nova_ai、nova_formation 三个新 crate，扩展 nova_animation，让 rts_demo 中的单位"活起来"。

**Architecture:** 分四个阶段：先实现角色数据层（nova_character），再实现 AI 决策层（nova_ai），然后实现编队系统（nova_formation），最后扩展动画系统并在 rts_demo 集成验证。每个 crate 独立可编译测试，再逐步集成。

**Tech Stack:** Rust, Bevy 0.15, serde/serde_json（角色配置加载），nova_core，nova_map（已有）

---

## File Structure

```
crates/nova_character/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、NovaCharacterPlugin
    ├── prelude.rs       # 公共导出
    ├── character.rs     # Character 组件、CharacterBundle、CharacterType
    ├── attributes.rs    # Attributes、Health 属性系统
    ├── state.rs         # CharacterState 状态机
    ├── feedback.rs      # DamageNumber、HitFlash、HealthBar、StatusIndicator
    └── loader.rs        # CharacterDef、从 JSON 加载角色配置

crates/nova_ai/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、NovaAiPlugin
    ├── prelude.rs       # 公共导出
    ├── perception.rs    # Perception、PerceivedEntities、PerceptionEvent 感知系统
    ├── behavior.rs      # BehaviorNode、ActionNode、ConditionNode 行为树数据
    ├── decision.rs      # BehaviorTreeExecutor 行为树执行器（Bevy System）
    ├── personality.rs   # Personality 性格特质
    ├── emotion.rs       # Emotion、EmotionType 情绪系统
    └── tactics.rs       # 战术行为（追击、撤退）辅助函数

crates/nova_formation/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、NovaFormationPlugin
    ├── prelude.rs       # 公共导出
    ├── formation.rs     # Formation、FormationManager（Resource）
    ├── patterns.rs      # FormationPattern、slot_offset 计算
    ├── slots.rs         # SlotAssignment 槽位分配算法
    └── movement.rs      # 编队移动系统（保持阵型）

crates/nova_animation/src/（新增文件）
├── state_machine.rs     # AnimationStateMachine、AnimationState、AnimationTransition（Task 13）
└── procedural.rs        # ProceduralIdle 程序化待机动画（Task 15）
# 注：skeleton.rs（骨骼动画）超出 MVP 范围，暂不实现

examples/rts_demo/src/（新增/修改文件）
├── character_setup.rs   # 使用 nova_character 定义角色、从 JSON 配置加载
├── ai_behaviors.rs      # 定义具体行为树、集成 nova_ai
├── formation_commands.rs # 编队命令处理
└── main.rs              # 集成新插件（修改）
```

---

## Phase 1: nova_character 角色数据模块

### Task 1: nova_character 脚手架

**Files:**
- Create: `crates/nova_character/Cargo.toml`
- Create: `crates/nova_character/src/lib.rs`
- Create: `crates/nova_character/src/prelude.rs`
- Modify: `Cargo.toml`（workspace members）

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "nova_character"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Nova Engine 人物建模系统 - 角色数据、属性、状态、视觉反馈"

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
criterion = { workspace = true }
```

- [ ] **Step 2: 创建 src/lib.rs（含空模块声明）**

```rust
//! Nova Character - 人物建模系统
//!
//! 负责角色"是什么"：数据、属性、状态、视觉反馈。
//! 不负责决策（nova_ai）和编队（nova_formation）。

pub mod attributes;
pub mod character;
pub mod feedback;
pub mod loader;
pub mod prelude;
pub mod state;

use bevy::prelude::*;

/// 角色系统插件
pub struct NovaCharacterPlugin;

impl Plugin for NovaCharacterPlugin {
    fn build(&self, _app: &mut App) {
        // 后续任务填充
    }
}
```

- [ ] **Step 3: 创建 src/prelude.rs（空占位）**

```rust
//! 公共导出

pub use crate::attributes::*;
pub use crate::character::*;
pub use crate::feedback::*;
pub use crate::loader::*;
pub use crate::state::*;
pub use crate::NovaCharacterPlugin;
```

- [ ] **Step 4: 为每个子模块创建空文件**

每个文件初始内容（以 `attributes.rs` 为例）：
```rust
//! 属性系统
```

需创建：
- `crates/nova_character/src/attributes.rs`
- `crates/nova_character/src/character.rs`
- `crates/nova_character/src/feedback.rs`
- `crates/nova_character/src/loader.rs`
- `crates/nova_character/src/state.rs`

- [ ] **Step 5: 将 nova_character 加入 workspace**

在根 `Cargo.toml` 的 `[workspace]` members 中添加：
```toml
"crates/nova_character",
```

在 `[workspace.dependencies]` 中添加：
```toml
nova_character = { path = "crates/nova_character" }
```

- [ ] **Step 6: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功，无错误

- [ ] **Step 7: 提交**

```bash
git add crates/nova_character Cargo.toml
git commit -m "feat(nova_character): 初始化角色系统模块脚手架"
```

---

### Task 2: Attributes 属性系统

**Files:**
- Modify: `crates/nova_character/src/attributes.rs`

- [ ] **Step 1: 实现 Health 和 Attributes**

```rust
//! 属性系统 - 角色数值数据

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 生命值
#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 { self.current / self.max } else { 0.0 }
    }
}

/// 角色属性组件
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Attributes {
    pub health: Health,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,   // 攻击间隔（秒）
    pub vision_range: f32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            health: Health::new(100.0),
            attack: 10.0,
            defense: 5.0,
            move_speed: 5.0,
            attack_range: 2.0,
            attack_speed: 1.0,
            vision_range: 10.0,
        }
    }
}

impl Attributes {
    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
    }
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_character/src/attributes.rs
git commit -m "feat(nova_character): 实现 Attributes 属性系统"
```

---

### Task 3: CharacterState 状态机

**Files:**
- Modify: `crates/nova_character/src/state.rs`

- [ ] **Step 1: 实现 CharacterState**

```rust
//! 角色状态机 - 描述角色当前在做什么

use bevy::prelude::*;

/// 角色状态
#[derive(Component, Clone, Debug, Default, Reflect, PartialEq)]
pub enum CharacterState {
    /// 待机
    #[default]
    Idle,
    /// 移动中
    Moving { target: Vec3 },
    /// 攻击中
    Attacking { target: Entity },
    /// 眩晕
    Stunned { remaining: f32 },
    /// 死亡
    Dead,
}

impl CharacterState {
    pub fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    pub fn is_moving(&self) -> bool {
        matches!(self, Self::Moving { .. })
    }

    pub fn is_attacking(&self) -> bool {
        matches!(self, Self::Attacking { .. })
    }

    pub fn is_stunned(&self) -> bool {
        matches!(self, Self::Stunned { .. })
    }
}

/// 攻击冷却计时器
#[derive(Component, Clone, Debug, Reflect)]
pub struct AttackCooldown {
    pub timer: f32,
    pub max: f32,
}

impl AttackCooldown {
    pub fn new(interval: f32) -> Self {
        Self { timer: 0.0, max: interval }
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer = (self.timer - delta).max(0.0);
    }

    pub fn can_attack(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn reset(&mut self) {
        self.timer = self.max;
    }
}
```

- [ ] **Step 2: 实现眩晕计时系统**

在同一文件末尾添加：

```rust
/// 眩晕倒计时系统
pub fn stun_tick_system(time: Res<Time>, mut query: Query<&mut CharacterState>) {
    for mut state in query.iter_mut() {
        if let CharacterState::Stunned { remaining } = state.as_mut() {
            *remaining -= time.delta_secs();
            if *remaining <= 0.0 {
                *state = CharacterState::Idle;
            }
        }
    }
}
```

- [ ] **Step 3: 更新 lib.rs 注册系统**

修改 `NovaCharacterPlugin::build`：
```rust
fn build(&self, app: &mut App) {
    app
        .register_type::<CharacterState>()
        .register_type::<AttackCooldown>()
        .register_type::<Attributes>()
        .add_systems(Update, state::stun_tick_system);
}
```

（需要 `use crate::{attributes::Attributes, state::*};`）

- [ ] **Step 4: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_character/src/state.rs crates/nova_character/src/lib.rs
git commit -m "feat(nova_character): 实现 CharacterState 状态机"
```

---

### Task 4: Character 组件和 CharacterBundle

**Files:**
- Modify: `crates/nova_character/src/character.rs`

- [ ] **Step 1: 实现 CharacterId、CharacterType、Character**

```rust
//! 角色标识与类型定义

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 角色唯一标识
#[derive(
    Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize,
)]
pub struct CharacterId(pub u64);

/// 角色类型
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum CharacterType {
    Infantry,    // 步兵
    Archer,      // 弓箭手
    Mage,        // 法师
    Knight,      // 骑士
    Custom(u32), // 自定义
}

impl Default for CharacterType {
    fn default() -> Self {
        Self::Infantry
    }
}

/// 角色标识组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub character_type: CharacterType,
}

impl Character {
    pub fn new(id: u64, name: impl Into<String>, character_type: CharacterType) -> Self {
        Self {
            id: CharacterId(id),
            name: name.into(),
            character_type,
        }
    }
}

/// 阵营
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum Faction {
    Player,
    Enemy,
    Neutral,
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_character/src/character.rs
git commit -m "feat(nova_character): 实现 Character 组件和 CharacterType"
```

---

### Task 5: 视觉反馈系统

**Files:**
- Modify: `crates/nova_character/src/feedback.rs`

- [ ] **Step 1: 实现 DamageNumber 组件和事件**

```rust
//! 视觉反馈系统 - 伤害数字、受击闪烁、生命条、状态图标

use bevy::prelude::*;

/// 伤害数字组件
#[derive(Component, Clone, Debug)]
pub struct DamageNumber {
    pub value: f32,
    pub is_crit: bool,
    pub lifetime: f32,       // 剩余显示时间（秒）
    pub velocity: Vec3,      // 漂浮速度
}

/// 生成伤害数字事件
#[derive(Event, Clone, Debug)]
pub struct SpawnDamageNumber {
    pub position: Vec3,
    pub damage: f32,
    pub is_crit: bool,
}

/// 受击闪烁组件
#[derive(Component, Clone, Debug)]
pub struct HitFlash {
    pub timer: f32,           // 剩余闪烁时间
    pub total: f32,           // 总闪烁时间
}

impl HitFlash {
    pub fn new(duration: f32) -> Self {
        Self { timer: duration, total: duration }
    }
}

/// 触发受击闪烁事件
#[derive(Event, Clone, Debug)]
pub struct TriggerHitFlash {
    pub entity: Entity,
    pub duration: f32,
}
```

- [ ] **Step 2: 实现 HealthBar 和 StatusIndicator**

在同一文件追加：

```rust
/// 头顶生命条组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct HealthBar {
    pub width: f32,
    pub height: f32,
    pub offset: Vec3,        // 相对实体的偏移
    pub show_when_full: bool,
    pub ally_color: Color,
    pub enemy_color: Color,
}

impl Default for HealthBar {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 0.1,
            offset: Vec3::new(0.0, 1.2, 0.0),
            show_when_full: false,
            ally_color: Color::srgb(0.2, 0.8, 0.2),
            enemy_color: Color::srgb(0.9, 0.2, 0.2),
        }
    }
}

/// 状态图标类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusIconType {
    Moving,
    Attacking,
    Stunned,
    Slowed,
    Enraged,
    Fearful,
}

/// 头顶状态图标组件
#[derive(Component, Clone, Debug)]
pub struct StatusIndicator {
    pub icons: Vec<StatusIconType>,
    pub offset: Vec3,
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self {
            icons: vec![],
            offset: Vec3::new(0.0, 1.5, 0.0),
        }
    }
}
```

- [ ] **Step 3: 实现伤害数字漂浮系统和死亡事件**

在文件末尾添加：

```rust
/// 单位死亡事件
#[derive(Event, Clone, Debug)]
pub struct UnitDiedEvent {
    pub entity: Entity,
    pub position: Vec3,
}

/// 伤害数字漂浮和销毁系统
pub fn damage_number_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut num, mut transform) in query.iter_mut() {
        num.lifetime -= time.delta_secs();
        transform.translation += num.velocity * time.delta_secs();

        if num.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// 受击闪烁系统
pub fn hit_flash_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash)>,
    mut commands: Commands,
) {
    for (entity, mut flash) in query.iter_mut() {
        flash.timer -= time.delta_secs();
        if flash.timer <= 0.0 {
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}
```

- [ ] **Step 4: 更新 lib.rs 注册事件和系统**

```rust
fn build(&self, app: &mut App) {
    app
        .register_type::<CharacterState>()
        .register_type::<AttackCooldown>()
        .register_type::<Attributes>()
        .register_type::<HealthBar>()
        .add_event::<feedback::SpawnDamageNumber>()
        .add_event::<feedback::TriggerHitFlash>()
        .add_event::<feedback::UnitDiedEvent>()
        .add_systems(
            Update,
            (
                state::stun_tick_system,
                feedback::damage_number_system,
                feedback::hit_flash_system,
            ),
        );
}
```

- [ ] **Step 5: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功

- [ ] **Step 6: 提交**

```bash
git add crates/nova_character/src/feedback.rs crates/nova_character/src/lib.rs
git commit -m "feat(nova_character): 实现视觉反馈系统（伤害数字、受击闪烁、生命条）"
```

---

### Task 6: 角色配置 JSON 加载

**Files:**
- Modify: `crates/nova_character/src/loader.rs`

- [ ] **Step 1: 实现 CharacterDef 和反序列化结构**

```rust
//! 角色配置加载 - 支持从 JSON 定义角色

use serde::{Deserialize, Serialize};

use crate::attributes::Attributes;
use crate::character::CharacterType;

/// 属性定义（JSON 中的数值）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributesDef {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub vision_range: f32,
}

impl AttributesDef {
    pub fn to_attributes(&self) -> Attributes {
        use crate::attributes::Health;
        Attributes {
            health: Health::new(self.health),
            attack: self.attack,
            defense: self.defense,
            move_speed: self.move_speed,
            attack_range: self.attack_range,
            attack_speed: self.attack_speed,
            vision_range: self.vision_range,
        }
    }
}

/// 模型定义
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelDef {
    Primitive {
        shape: PrimitiveShapeDef,
        color: [f32; 4],
    },
    Gltf {
        path: String,
    },
}

/// 原始形状定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrimitiveShapeDef {
    Capsule { radius: f32, height: f32 },
    Cube { size: f32 },
    Sphere { radius: f32 },
}

/// 性格定义（可选）
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PersonalityDef {
    #[serde(default = "default_0_5")]
    pub aggression: f32,
    #[serde(default = "default_0_5")]
    pub courage: f32,
    #[serde(default = "default_0_5")]
    pub discipline: f32,
}

fn default_0_5() -> f32 { 0.5 }

/// 角色定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterDef {
    pub id: String,
    pub name: String,
    pub character_type: String,
    pub attributes: AttributesDef,
    pub model: ModelDef,
    #[serde(default)]
    pub personality: PersonalityDef,
}

impl CharacterDef {
    pub fn character_type(&self) -> CharacterType {
        match self.character_type.as_str() {
            "Infantry" => CharacterType::Infantry,
            "Archer" => CharacterType::Archer,
            "Mage" => CharacterType::Mage,
            "Knight" => CharacterType::Knight,
            _ => CharacterType::Infantry,
        }
    }
}

/// 角色配置文件（包含多个角色定义）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub characters: Vec<CharacterDef>,
}

impl CharacterConfig {
    /// 从 JSON 字符串加载
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 按 id 查找角色定义
    pub fn find(&self, id: &str) -> Option<&CharacterDef> {
        self.characters.iter().find(|c| c.id == id)
    }
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p nova_character
```

Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_character/src/loader.rs
git commit -m "feat(nova_character): 实现角色配置 JSON 加载"
```

---

## Phase 2: nova_ai AI 决策模块

### Task 7: nova_ai 脚手架

**Files:**
- Create: `crates/nova_ai/Cargo.toml`
- Create: `crates/nova_ai/src/lib.rs`
- Create: `crates/nova_ai/src/prelude.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "nova_ai"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Nova Engine AI 决策系统 - 感知、行为树、性格情绪"

[dependencies]
nova_core = { workspace = true }
nova_character = { workspace = true }
bevy = { workspace = true }
```

- [ ] **Step 2: 创建 src/lib.rs**

```rust
//! Nova AI - AI 决策系统
//!
//! 负责角色"怎么想"：感知、行为树决策、性格情绪。

pub mod behavior;
pub mod decision;
pub mod emotion;
pub mod perception;
pub mod personality;
pub mod prelude;
pub mod tactics;

use bevy::prelude::*;

/// AI 系统插件
pub struct NovaAiPlugin;

impl Plugin for NovaAiPlugin {
    fn build(&self, _app: &mut App) {
        // 后续任务填充
    }
}
```

- [ ] **Step 3: 创建空模块文件**

为每个子模块创建空文件（`//! 模块名` 占位）：
- `crates/nova_ai/src/behavior.rs`
- `crates/nova_ai/src/decision.rs`
- `crates/nova_ai/src/emotion.rs`
- `crates/nova_ai/src/perception.rs`
- `crates/nova_ai/src/personality.rs`
- `crates/nova_ai/src/tactics.rs`

`src/prelude.rs`：
```rust
//! 公共导出
pub use crate::behavior::*;
pub use crate::decision::*;
pub use crate::emotion::*;
pub use crate::perception::*;
pub use crate::personality::*;
pub use crate::NovaAiPlugin;
```

- [ ] **Step 4: 将 nova_ai 加入 workspace**

在根 `Cargo.toml` members 中添加：
```toml
"crates/nova_ai",
```

在 `[workspace.dependencies]` 中添加：
```toml
nova_ai = { path = "crates/nova_ai" }
```

- [ ] **Step 5: 验证编译**

```bash
cargo check -p nova_ai
```

Expected: 编译成功

- [ ] **Step 6: 提交**

```bash
git add crates/nova_ai Cargo.toml
git commit -m "feat(nova_ai): 初始化 AI 决策系统模块脚手架"
```

---

### Task 8: 感知系统

**Files:**
- Modify: `crates/nova_ai/src/perception.rs`

- [ ] **Step 1: 实现感知组件和感知结果**

```rust
//! 感知系统 - 单位能"看到/听到"什么

use bevy::prelude::*;

/// 感知能力组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Perception {
    /// 视觉范围（世界单位）
    pub vision_range: f32,
    /// 视野角度（度，360 为全向）
    pub vision_angle: f32,
    /// 听觉范围
    pub hearing_range: f32,
}

impl Default for Perception {
    fn default() -> Self {
        Self {
            vision_range: 10.0,
            vision_angle: 360.0,
            hearing_range: 6.0,
        }
    }
}

impl Perception {
    pub fn new(vision_range: f32) -> Self {
        Self {
            vision_range,
            ..default()
        }
    }

    /// 检查目标是否在视野内
    pub fn can_see(&self, self_transform: &Transform, target_pos: Vec3) -> bool {
        let diff = target_pos - self_transform.translation;
        let distance = diff.length();

        if distance > self.vision_range {
            return false;
        }

        if self.vision_angle >= 360.0 {
            return true;
        }

        // 计算角度差
        let forward = self_transform.forward();
        let to_target = diff.normalize();
        let angle = forward.dot(to_target).acos().to_degrees();
        angle <= self.vision_angle / 2.0
    }
}

/// 感知结果组件（每帧更新）
#[derive(Component, Default, Clone, Debug)]
pub struct PerceivedEntities {
    pub visible: Vec<Entity>,
    pub heard: Vec<Entity>,
    pub closest_enemy: Option<Entity>,
    pub closest_ally: Option<Entity>,
}

impl PerceivedEntities {
    pub fn clear(&mut self) {
        self.visible.clear();
        self.heard.clear();
        self.closest_enemy = None;
        self.closest_ally = None;
    }
}

/// 感知事件
#[derive(Event, Clone, Debug)]
pub enum PerceptionEvent {
    EnemySpotted { perceiver: Entity, enemy: Entity },
    EnemyLost { perceiver: Entity, enemy: Entity },
    AllyUnderAttack { perceiver: Entity, ally: Entity },
}
```

- [ ] **Step 2: 实现感知更新系统**

```rust
use nova_character::character::Faction;

/// 感知更新系统（每帧扫描可见实体）
pub fn perception_update_system(
    mut perceivers: Query<(
        Entity,
        &Transform,
        &Perception,
        &Faction,
        &mut PerceivedEntities,
    )>,
    potential_targets: Query<(Entity, &Transform, &Faction)>,
) {
    for (perceiver_entity, perceiver_transform, perception, perceiver_faction, mut perceived) in
        perceivers.iter_mut()
    {
        perceived.clear();

        let mut closest_enemy_dist = f32::INFINITY;
        let mut closest_ally_dist = f32::INFINITY;

        for (target_entity, target_transform, target_faction) in potential_targets.iter() {
            if target_entity == perceiver_entity {
                continue;
            }

            if perception.can_see(perceiver_transform, target_transform.translation) {
                perceived.visible.push(target_entity);

                let dist = (perceiver_transform.translation - target_transform.translation)
                    .length();

                if *target_faction != *perceiver_faction {
                    if dist < closest_enemy_dist {
                        closest_enemy_dist = dist;
                        perceived.closest_enemy = Some(target_entity);
                    }
                } else if dist < closest_ally_dist {
                    closest_ally_dist = dist;
                    perceived.closest_ally = Some(target_entity);
                }
            }
        }
    }
}
```

- [ ] **Step 3: 更新 lib.rs**

```rust
impl Plugin for NovaAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<perception::PerceptionEvent>()
            .add_systems(
                Update,
                perception::perception_update_system.in_set(AiSet::Perception),
            )
            .configure_sets(Update, AiSet::Perception.before(AiSet::Decision));
    }
}

/// AI 系统执行顺序
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiSet {
    Perception,
    Decision,
}
```

- [ ] **Step 4: 验证编译**

```bash
cargo check -p nova_ai
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_ai/src/perception.rs crates/nova_ai/src/lib.rs
git commit -m "feat(nova_ai): 实现感知系统"
```

---

### Task 9: 性格与情绪系统

**Files:**
- Modify: `crates/nova_ai/src/personality.rs`
- Modify: `crates/nova_ai/src/emotion.rs`

- [ ] **Step 1: 实现 Personality**

```rust
//! 性格特质 - 影响 AI 决策权重

use bevy::prelude::*;

/// 性格组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct Personality {
    /// 0-1，高=主动进攻，低=防守
    pub aggression: f32,
    /// 0-1，高=不畏死亡，低=容易逃跑
    pub courage: f32,
    /// 0-1，高=严格执行命令，低=自由行动
    pub discipline: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            courage: 0.5,
            discipline: 0.5,
        }
    }
}

impl Personality {
    pub fn soldier() -> Self {
        Self {
            aggression: 0.6,
            courage: 0.7,
            discipline: 0.8,
        }
    }

    pub fn coward() -> Self {
        Self {
            aggression: 0.2,
            courage: 0.2,
            discipline: 0.4,
        }
    }

    pub fn berserker() -> Self {
        Self {
            aggression: 0.95,
            courage: 0.9,
            discipline: 0.2,
        }
    }
}
```

- [ ] **Step 2: 实现 Emotion**

```rust
//! 情绪系统 - 动态变化，影响行为

use bevy::prelude::*;
use crate::personality::Personality;

/// 情绪类型
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
pub enum EmotionType {
    /// 平静 - 正常行为
    #[default]
    Calm,
    /// 愤怒 - 攻击加成，防御降低
    Angry,
    /// 恐惧 - 倾向逃跑
    Fearful,
    /// 狂暴 - 无视命令，疯狂攻击
    Berserk,
}

/// 情绪组件
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Emotion {
    pub current: EmotionType,
    /// 情绪强度 0-1
    pub intensity: f32,
    /// 情绪持续计时器（归零后恢复平静）
    pub duration: f32,
}

impl Emotion {
    /// 受伤时的情绪反应
    pub fn on_damage(&mut self, damage_percent: f32, personality: &Personality) {
        if damage_percent > 0.3 {
            if personality.courage < 0.3 {
                self.current = EmotionType::Fearful;
                self.intensity = 0.8;
                self.duration = 5.0;
            } else if personality.aggression > 0.7 {
                self.current = EmotionType::Angry;
                self.intensity = 0.6;
                self.duration = 4.0;
            }
        }
    }

    /// 盟友死亡时的情绪反应
    pub fn on_ally_death(&mut self, personality: &Personality) {
        if personality.aggression > 0.8 {
            self.current = EmotionType::Berserk;
            self.intensity = 1.0;
            self.duration = 8.0;
        } else if personality.courage < 0.4 {
            self.current = EmotionType::Fearful;
            self.intensity = 0.9;
            self.duration = 6.0;
        }
    }

    /// 情绪冷却（逐渐恢复平静）
    pub fn tick(&mut self, delta: f32) {
        if self.duration > 0.0 {
            self.duration -= delta;
            if self.duration <= 0.0 {
                self.current = EmotionType::Calm;
                self.intensity = 0.0;
            }
        }
    }
}

/// 情绪冷却系统
pub fn emotion_tick_system(time: Res<Time>, mut query: Query<&mut Emotion>) {
    for mut emotion in query.iter_mut() {
        emotion.tick(time.delta_secs());
    }
}
```

- [ ] **Step 3: 更新 lib.rs 加入情绪系统**

```rust
.add_systems(Update, emotion::emotion_tick_system)
```

- [ ] **Step 4: 验证编译**

```bash
cargo check -p nova_ai
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_ai/src/personality.rs crates/nova_ai/src/emotion.rs crates/nova_ai/src/lib.rs
git commit -m "feat(nova_ai): 实现性格与情绪系统"
```

---

### Task 10: 行为树数据结构

**Files:**
- Modify: `crates/nova_ai/src/behavior.rs`

- [ ] **Step 1: 实现行为树节点类型**

```rust
//! 行为树数据结构

use bevy::prelude::*;

/// 行为树执行结果
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorStatus {
    /// 成功
    Success,
    /// 失败
    Failure,
    /// 运行中
    Running,
}

/// 移动目标
#[derive(Clone, Debug)]
pub enum MoveTarget {
    Position(Vec3),
    Entity(Entity),
    Offset(Vec3), // 相对当前位置
}

/// 攻击目标
#[derive(Clone, Debug)]
pub enum AttackTarget {
    Entity(Entity),
    Closest,
}

/// 行为树节点（递归枚举）
#[derive(Clone, Debug)]
pub enum BehaviorNode {
    // ---- 叶子节点 ----
    Action(ActionNode),
    Condition(ConditionNode),

    // ---- 组合节点 ----
    /// 顺序执行：全部成功才成功，一个失败即失败
    Sequence(Vec<BehaviorNode>),
    /// 选择执行：一个成功即成功，全部失败才失败
    Selector(Vec<BehaviorNode>),
    /// 并行执行（全部 Running 时返回 Running）
    Parallel(Vec<BehaviorNode>),

    // ---- 装饰节点 ----
    /// 反转结果
    Inverter(Box<BehaviorNode>),
    /// 固定重复 N 次
    Repeater { node: Box<BehaviorNode>, times: u32, current: u32 },
}

/// 行为动作节点
#[derive(Clone, Debug)]
pub enum ActionNode {
    Idle,
    MoveTo(MoveTarget),
    Attack(AttackTarget),
    Flee,
    Patrol { points: Vec<Vec3>, current: usize },
    FollowLeader,
}

/// 条件节点
#[derive(Clone, Debug)]
pub enum ConditionNode {
    HasTarget,
    HealthBelow(f32),              // 百分比，如 0.3 表示 30%
    EnemyInRange,
    EnemyInAttackRange,
    IsInFormation,
    EmotionIs(super::emotion::EmotionType),
    HasPerceivedEnemy,
}

/// 行为树组件（每个 AI 单位挂载）
#[derive(Component, Clone, Debug)]
pub struct BehaviorTree {
    pub root: BehaviorNode,
}

impl BehaviorTree {
    pub fn new(root: BehaviorNode) -> Self {
        Self { root }
    }

    /// 创建标准战士行为树
    pub fn standard_soldier() -> Self {
        Self::new(BehaviorNode::Selector(vec![
            // 如果有感知到敌人且在攻击范围内：攻击
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition(ConditionNode::EnemyInAttackRange),
                BehaviorNode::Action(ActionNode::Attack(AttackTarget::Closest)),
            ]),
            // 如果有感知到敌人：追击
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition(ConditionNode::HasPerceivedEnemy),
                BehaviorNode::Action(ActionNode::MoveTo(MoveTarget::Entity(Entity::PLACEHOLDER))),
            ]),
            // 否则：待机
            BehaviorNode::Action(ActionNode::Idle),
        ]))
    }

    /// 创建胆小鬼行为树
    pub fn coward() -> Self {
        Self::new(BehaviorNode::Selector(vec![
            // 血量低于 30% 就逃跑
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition(ConditionNode::HealthBelow(0.3)),
                BehaviorNode::Action(ActionNode::Flee),
            ]),
            // 否则尝试攻击
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition(ConditionNode::EnemyInAttackRange),
                BehaviorNode::Action(ActionNode::Attack(AttackTarget::Closest)),
            ]),
            BehaviorNode::Action(ActionNode::Idle),
        ]))
    }
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p nova_ai
```

Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add crates/nova_ai/src/behavior.rs
git commit -m "feat(nova_ai): 实现行为树数据结构"
```

---

### Task 11: 行为树执行器

**Files:**
- Modify: `crates/nova_ai/src/decision.rs`

- [ ] **Step 1: 实现行为树求值函数**

```rust
//! 行为树执行器

use bevy::prelude::*;

use nova_character::{attributes::Attributes, character::Faction, state::CharacterState};

use crate::{
    behavior::{ActionNode, AttackTarget, BehaviorNode, BehaviorStatus, BehaviorTree, ConditionNode, MoveTarget},
    emotion::{Emotion, EmotionType},
    perception::PerceivedEntities,
};

/// 执行行为树所需的上下文（只读）
pub struct BtContext<'a> {
    pub entity: Entity,
    pub transform: &'a Transform,
    pub attributes: &'a Attributes,
    pub perceived: &'a PerceivedEntities,
    pub emotion: Option<&'a Emotion>,
}

/// 求值行为树节点，返回状态并写入命令
pub fn evaluate_node(
    node: &BehaviorNode,
    ctx: &BtContext,
    commands: &mut EntityCommands,
) -> BehaviorStatus {
    match node {
        BehaviorNode::Sequence(children) => {
            for child in children {
                match evaluate_node(child, ctx, commands) {
                    BehaviorStatus::Failure => return BehaviorStatus::Failure,
                    BehaviorStatus::Running => return BehaviorStatus::Running,
                    BehaviorStatus::Success => {}
                }
            }
            BehaviorStatus::Success
        }

        BehaviorNode::Selector(children) => {
            for child in children {
                match evaluate_node(child, ctx, commands) {
                    BehaviorStatus::Success => return BehaviorStatus::Success,
                    BehaviorStatus::Running => return BehaviorStatus::Running,
                    BehaviorStatus::Failure => {}
                }
            }
            BehaviorStatus::Failure
        }

        BehaviorNode::Inverter(inner) => {
            match evaluate_node(inner, ctx, commands) {
                BehaviorStatus::Success => BehaviorStatus::Failure,
                BehaviorStatus::Failure => BehaviorStatus::Success,
                BehaviorStatus::Running => BehaviorStatus::Running,
            }
        }

        BehaviorNode::Parallel(children) => {
            let mut any_running = false;
            for child in children {
                if evaluate_node(child, ctx, commands) == BehaviorStatus::Running {
                    any_running = true;
                }
            }
            if any_running { BehaviorStatus::Running } else { BehaviorStatus::Success }
        }

        BehaviorNode::Condition(cond) => evaluate_condition(cond, ctx),

        BehaviorNode::Action(action) => execute_action(action, ctx, commands),

        BehaviorNode::Repeater { .. } => BehaviorStatus::Running,
    }
}

fn evaluate_condition(cond: &ConditionNode, ctx: &BtContext) -> BehaviorStatus {
    let result = match cond {
        ConditionNode::HasPerceivedEnemy | ConditionNode::HasTarget => {
            ctx.perceived.closest_enemy.is_some()
        }
        ConditionNode::HealthBelow(threshold) => {
            ctx.attributes.health.percentage() < *threshold
        }
        ConditionNode::EnemyInRange => ctx.perceived.closest_enemy.is_some(),
        ConditionNode::EnemyInAttackRange => {
            // 简化版：只检查是否有最近敌人在感知列表中
            ctx.perceived.closest_enemy.is_some()
        }
        ConditionNode::EmotionIs(target_emotion) => {
            ctx.emotion.map(|e| e.current == *target_emotion).unwrap_or(false)
        }
        ConditionNode::IsInFormation => false, // 由 nova_formation 处理
    };

    if result { BehaviorStatus::Success } else { BehaviorStatus::Failure }
}

fn execute_action(action: &ActionNode, ctx: &BtContext, commands: &mut EntityCommands) -> BehaviorStatus {
    match action {
        ActionNode::Idle => {
            commands.insert(CharacterState::Idle);
            BehaviorStatus::Running
        }
        ActionNode::Attack(target) => {
            let target_entity = match target {
                AttackTarget::Closest => ctx.perceived.closest_enemy,
                AttackTarget::Entity(e) => Some(*e),
            };
            if let Some(enemy) = target_entity {
                commands.insert(CharacterState::Attacking { target: enemy });
                BehaviorStatus::Running
            } else {
                BehaviorStatus::Failure
            }
        }
        ActionNode::MoveTo(target) => {
            let target_pos = match target {
                MoveTarget::Position(p) => Some(*p),
                MoveTarget::Entity(_) => None, // 通过追击系统处理
                MoveTarget::Offset(o) => Some(ctx.transform.translation + *o),
            };
            if let Some(pos) = target_pos {
                commands.insert(CharacterState::Moving { target: pos });
                BehaviorStatus::Running
            } else {
                BehaviorStatus::Running // 等待追击系统处理
            }
        }
        ActionNode::Flee => {
            // 向远离最近敌人的方向移动
            if let Some(_enemy) = ctx.perceived.closest_enemy {
                // 简化版：向后退
                let retreat_pos = ctx.transform.translation + ctx.transform.back() * 10.0;
                commands.insert(CharacterState::Moving { target: retreat_pos });
            }
            BehaviorStatus::Running
        }
        ActionNode::Patrol { .. } | ActionNode::FollowLeader => {
            BehaviorStatus::Running
        }
    }
}

/// 行为树执行系统（每帧 tick 所有有 BehaviorTree 的单位）
pub fn behavior_tree_system(
    mut query: Query<(
        Entity,
        &Transform,
        &Attributes,
        &PerceivedEntities,
        Option<&Emotion>,
        &BehaviorTree,
    )>,
    mut commands: Commands,
) {
    for (entity, transform, attributes, perceived, emotion, tree) in query.iter_mut() {
        let ctx = BtContext {
            entity,
            transform,
            attributes,
            perceived,
            emotion,
        };

        let root = tree.root.clone();
        let mut entity_commands = commands.entity(entity);
        evaluate_node(&root, &ctx, &mut entity_commands);
    }
}
```

- [ ] **Step 2: 更新 lib.rs 注册决策系统**

```rust
.add_systems(
    Update,
    decision::behavior_tree_system.in_set(AiSet::Decision),
)
```

- [ ] **Step 3: 验证编译**

```bash
cargo check -p nova_ai
```

Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add crates/nova_ai/src/decision.rs crates/nova_ai/src/lib.rs
git commit -m "feat(nova_ai): 实现行为树执行器"
```

---

## Phase 3: nova_formation 编队系统

### Task 12: nova_formation 脚手架 + 阵型计算

**Files:**
- Create: `crates/nova_formation/Cargo.toml`
- Create: `crates/nova_formation/src/lib.rs`
- Create: `crates/nova_formation/src/prelude.rs`
- Create: `crates/nova_formation/src/formation.rs`
- Create: `crates/nova_formation/src/patterns.rs`
- Create: `crates/nova_formation/src/slots.rs`
- Create: `crates/nova_formation/src/movement.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "nova_formation"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Nova Engine 编队系统 - 阵型、编队移动"

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
```

- [ ] **Step 2: 创建 src/patterns.rs（阵型计算核心）**

```rust
//! 阵型模式 - 计算每个槽位的相对偏移

use bevy::prelude::*;

/// 阵型模式
#[derive(Clone, Debug)]
pub enum FormationPattern {
    /// 方阵 - 步兵常用
    Square { rows: u32, cols: u32 },
    /// 楔形 - 冲锋
    Wedge { depth: u32 },
    /// 横线 - 远程
    Line,
    /// 圆形 - 防御
    Circle { radius: f32 },
    /// 自定义
    Custom { slots: Vec<Vec3> },
}

impl FormationPattern {
    /// 计算第 index 个槽位的相对偏移（基于 spacing）
    pub fn slot_offset(&self, index: usize, spacing: f32) -> Vec3 {
        match self {
            FormationPattern::Square { rows, cols } => {
                let col = (index as u32) % cols;
                let row = (index as u32) / cols;
                Vec3::new(
                    col as f32 * spacing - ((*cols - 1) as f32 * spacing / 2.0),
                    0.0,
                    row as f32 * spacing,
                )
            }
            FormationPattern::Wedge { depth } => {
                let row = (index as u32) % (*depth + 1);
                let col_offset = index as i32 - (row * (row + 1) / 2) as i32;
                Vec3::new(
                    col_offset as f32 * spacing,
                    0.0,
                    row as f32 * spacing,
                )
            }
            FormationPattern::Line => Vec3::new(
                index as f32 * spacing - 0.0,
                0.0,
                0.0,
            ),
            FormationPattern::Circle { radius } => {
                let angle = (index as f32 / 8.0) * std::f32::consts::TAU;
                Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
            }
            FormationPattern::Custom { slots } => {
                slots.get(index).copied().unwrap_or(Vec3::ZERO)
            }
        }
    }

    /// 计算该阵型最大支持多少单位
    pub fn capacity(&self) -> Option<usize> {
        match self {
            FormationPattern::Square { rows, cols } => Some((*rows * *cols) as usize),
            FormationPattern::Custom { slots } => Some(slots.len()),
            _ => None, // 无限
        }
    }
}
```

- [ ] **Step 3: 创建 src/formation.rs（编队数据结构）**

```rust
//! 编队数据结构

use std::collections::HashMap;

use bevy::prelude::*;

use crate::patterns::FormationPattern;

/// 编队 ID
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormationId(pub u32);

/// 单个编队
#[derive(Clone, Debug)]
pub struct Formation {
    pub id: FormationId,
    pub leader: Entity,
    pub members: Vec<Entity>,
    pub pattern: FormationPattern,
    pub spacing: f32,
    pub facing: Vec3,
}

impl Formation {
    pub fn new(id: FormationId, leader: Entity, pattern: FormationPattern, spacing: f32) -> Self {
        Self {
            id,
            leader,
            members: vec![],
            pattern,
            spacing,
            facing: Vec3::NEG_Z,
        }
    }

    pub fn add_member(&mut self, entity: Entity) {
        self.members.push(entity);
    }

    pub fn remove_member(&mut self, entity: Entity) {
        self.members.retain(|&e| e != entity);
    }

    pub fn slot_world_pos(&self, slot_index: usize, leader_pos: Vec3) -> Vec3 {
        let offset = self.pattern.slot_offset(slot_index, self.spacing);
        // 根据朝向旋转偏移
        let rotation = Quat::from_rotation_y(self.facing.x.atan2(self.facing.z));
        leader_pos + rotation * offset
    }
}

/// 编队管理器（Resource）
#[derive(Resource, Default)]
pub struct FormationManager {
    formations: HashMap<FormationId, Formation>,
    next_id: u32,
}

impl FormationManager {
    pub fn create(
        &mut self,
        leader: Entity,
        pattern: FormationPattern,
        spacing: f32,
    ) -> FormationId {
        let id = FormationId(self.next_id);
        self.next_id += 1;
        self.formations.insert(id, Formation::new(id, leader, pattern, spacing));
        id
    }

    pub fn get(&self, id: FormationId) -> Option<&Formation> {
        self.formations.get(&id)
    }

    pub fn get_mut(&mut self, id: FormationId) -> Option<&mut Formation> {
        self.formations.get_mut(&id)
    }

    pub fn dissolve(&mut self, id: FormationId) {
        self.formations.remove(&id);
    }

    pub fn formations(&self) -> impl Iterator<Item = &Formation> {
        self.formations.values()
    }
}

/// 编队成员组件
#[derive(Component, Clone, Debug)]
pub struct FormationMember {
    pub formation_id: FormationId,
    pub slot_index: usize,
    /// 相对队长的槽位偏移（缓存值，避免每帧重算）
    pub local_offset: Vec3,
}

impl FormationMember {
    pub fn new(formation_id: FormationId, slot_index: usize, local_offset: Vec3) -> Self {
        Self { formation_id, slot_index, local_offset }
    }
}
```

- [ ] **Step 4: 创建 src/slots.rs（槽位分配）**

```rust
//! 槽位分配算法

use bevy::prelude::*;

/// 槽位分配策略
pub enum SlotAssignment {
    /// 按加入顺序
    Sequential,
    /// 按距离目标槽位最近分配
    ByDistance,
}

impl SlotAssignment {
    /// 为 entities 分配槽位索引，返回 (entity, slot_index) 列表
    pub fn assign(
        &self,
        entities: &[Entity],
        transforms: &[(Entity, Vec3)],
        slot_positions: &[Vec3],
    ) -> Vec<(Entity, usize)> {
        match self {
            SlotAssignment::Sequential => {
                entities.iter().enumerate().map(|(i, &e)| (e, i)).collect()
            }
            SlotAssignment::ByDistance => {
                // 贪心分配：每个 entity 找最近的未占用槽位
                let mut assigned = vec![false; slot_positions.len()];
                let mut result = Vec::with_capacity(entities.len());

                for &entity in entities {
                    let pos = transforms
                        .iter()
                        .find(|(e, _)| *e == entity)
                        .map(|(_, p)| *p)
                        .unwrap_or(Vec3::ZERO);

                    let best_slot = slot_positions
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !assigned[*i])
                        .min_by(|(_, a), (_, b)| {
                            let da = (*a - pos).length_squared();
                            let db = (*b - pos).length_squared();
                            da.partial_cmp(&db).unwrap()
                        })
                        .map(|(i, _)| i);

                    if let Some(slot) = best_slot {
                        assigned[slot] = true;
                        result.push((entity, slot));
                    }
                }

                result
            }
        }
    }
}
```

- [ ] **Step 5: 创建 src/movement.rs（编队移动）**

```rust
//! 编队移动系统

use bevy::prelude::*;

use crate::formation::{FormationManager, FormationMember};

/// 编队移动目标（Resource）
#[derive(Resource, Default)]
pub struct FormationMoveTarget {
    pub targets: std::collections::HashMap<crate::formation::FormationId, Vec3>,
}

/// 编队成员跟随系统
pub fn formation_follow_system(
    time: Res<Time>,
    manager: Res<FormationManager>,
    leaders: Query<&Transform>,
    mut members: Query<(&mut Transform, &FormationMember)>,
) {
    for (mut member_transform, member) in members.iter_mut() {
        let Some(formation) = manager.get(member.formation_id) else {
            continue;
        };

        let Ok(leader_transform) = leaders.get(formation.leader) else {
            continue;
        };

        let target_pos =
            formation.slot_world_pos(member.slot_index, leader_transform.translation);

        let diff = target_pos - member_transform.translation;
        let distance = diff.length();

        // 只在距目标超过 0.5 时移动
        if distance > 0.5 {
            let speed = 5.0_f32;
            let step = speed * time.delta_secs();
            member_transform.translation += diff.normalize() * step.min(distance);
        }
    }
}
```

- [ ] **Step 6: 创建 src/lib.rs**

```rust
//! Nova Formation - 编队系统

pub mod formation;
pub mod movement;
pub mod patterns;
pub mod prelude;
pub mod slots;

use bevy::prelude::*;
pub use formation::{Formation, FormationId, FormationManager, FormationMember};
pub use movement::FormationMoveTarget;
pub use patterns::FormationPattern;
pub use slots::SlotAssignment;

pub struct NovaFormationPlugin;

impl Plugin for NovaFormationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FormationManager>()
            .init_resource::<FormationMoveTarget>()
            .add_systems(Update, movement::formation_follow_system);
    }
}
```

`src/prelude.rs`：
```rust
//! 公共导出
pub use crate::formation::*;
pub use crate::movement::*;
pub use crate::patterns::*;
pub use crate::slots::*;
pub use crate::NovaFormationPlugin;
```

- [ ] **Step 7: 将 nova_formation 加入 workspace**

```toml
# Cargo.toml members 添加：
"crates/nova_formation",

# [workspace.dependencies] 添加：
nova_formation = { path = "crates/nova_formation" }
```

- [ ] **Step 8: 验证编译**

```bash
cargo check -p nova_formation
```

Expected: 编译成功

- [ ] **Step 9: 提交**

```bash
git add crates/nova_formation Cargo.toml
git commit -m "feat(nova_formation): 实现编队系统（阵型、槽位分配、编队移动）"
```

---

## Phase 4: nova_animation 扩展 + rts_demo 集成

### Task 13: nova_animation 扩展——动画状态机

**Files:**
- Create: `crates/nova_animation/src/state_machine.rs`
- Modify: `crates/nova_animation/src/lib.rs`（声明模块）

- [ ] **Step 1: 实现 AnimationStateMachine 组件**

```rust
//! 动画状态机 - 管理角色动画状态切换

use std::collections::HashMap;

use bevy::prelude::*;

/// 动画状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    Attack,
    Hit,
    Die,
    Custom(u32),
}

/// 循环模式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopMode {
    Once,
    Loop,
    PingPong,
}

/// 单个状态的配置
#[derive(Clone, Debug)]
pub struct AnimationStateConfig {
    /// 动画片段名称（对应 AnimationPlayer 中的片段）
    pub clip_name: String,
    pub loop_mode: LoopMode,
    pub speed: f32,
}

/// 状态转换触发条件
#[derive(Clone, Debug)]
pub enum TransitionCondition {
    /// 立即切换
    Immediate,
    /// 当前动画播放完毕后切换
    OnClipEnd,
    /// 超时后切换（秒）
    AfterSeconds(f32),
}

/// 状态转换规则
#[derive(Clone, Debug)]
pub struct AnimationTransition {
    pub from: AnimationState,
    pub to: AnimationState,
    pub condition: TransitionCondition,
    /// 混合时间（秒）
    pub blend_duration: f32,
}

/// 动画状态机组件
#[derive(Component, Clone, Debug)]
pub struct AnimationStateMachine {
    pub current_state: AnimationState,
    pub states: HashMap<AnimationState, AnimationStateConfig>,
    pub transitions: Vec<AnimationTransition>,
    /// 当前状态已持续时间
    pub state_time: f32,
}

impl AnimationStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: AnimationState::Idle,
            states: HashMap::new(),
            transitions: vec![],
            state_time: 0.0,
        }
    }

    pub fn with_state(mut self, state: AnimationState, config: AnimationStateConfig) -> Self {
        self.states.insert(state, config);
        self
    }

    pub fn with_transition(mut self, transition: AnimationTransition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// 强制切换到新状态
    pub fn transition_to(&mut self, new_state: AnimationState) {
        if self.current_state != new_state {
            self.current_state = new_state;
            self.state_time = 0.0;
        }
    }

    /// 获取当前状态的配置
    pub fn current_config(&self) -> Option<&AnimationStateConfig> {
        self.states.get(&self.current_state)
    }
}

impl Default for AnimationStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// 动画状态机计时系统
pub fn animation_state_machine_system(
    time: Res<Time>,
    mut query: Query<&mut AnimationStateMachine>,
) {
    for mut asm in query.iter_mut() {
        asm.state_time += time.delta_secs();
    }
}
```

- [ ] **Step 2: 更新 nova_animation/src/lib.rs 声明新模块**

在现有 `pub mod` 列表中添加：
```rust
pub mod state_machine;
```

并追加导出：
```rust
pub use state_machine::{
    AnimationState, AnimationStateMachine, AnimationStateConfig, AnimationTransition,
    LoopMode, TransitionCondition,
};
```

- [ ] **Step 3: 在 plugin.rs 注册系统**

```rust
app.add_systems(Update, crate::state_machine::animation_state_machine_system);
```

- [ ] **Step 4: 验证编译**

```bash
cargo check -p nova_animation
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_animation/src/state_machine.rs \
        crates/nova_animation/src/lib.rs \
        crates/nova_animation/src/plugin.rs
git commit -m "feat(nova_animation): 添加动画状态机（AnimationStateMachine）"
```

---

### Task 15: nova_animation 扩展——程序化待机动画

**Files:**
- Create: `crates/nova_animation/src/procedural.rs`
- Modify: `crates/nova_animation/src/lib.rs`

- [ ] **Step 1: 实现 ProceduralIdle 组件**

```rust
//! 程序化待机动画 - 让静止单位看起来"活着"

use bevy::prelude::*;

/// 程序化待机动画组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct ProceduralIdle {
    pub enabled: bool,
    /// 摇摆幅度（弧度）
    pub sway_amplitude: f32,
    /// 摇摆速度
    pub sway_speed: f32,
    /// 呼吸缩放幅度
    pub breathe_scale: f32,
    /// 相位偏移（避免所有单位同步）
    pub phase: f32,
}

impl ProceduralIdle {
    pub fn new_with_phase(phase: f32) -> Self {
        Self {
            enabled: true,
            sway_amplitude: 0.05,
            sway_speed: 1.2,
            breathe_scale: 0.02,
            phase,
        }
    }
}

impl Default for ProceduralIdle {
    fn default() -> Self {
        Self::new_with_phase(0.0)
    }
}

/// 程序化待机动画系统
pub fn procedural_idle_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ProceduralIdle)>,
) {
    let t = time.elapsed_secs();

    for (mut transform, idle) in query.iter_mut() {
        if !idle.enabled {
            continue;
        }

        let phase = idle.phase;
        let sway = (t * idle.sway_speed + phase).sin() * idle.sway_amplitude;
        let breathe = 1.0 + (t * idle.sway_speed * 0.7 + phase).sin() * idle.breathe_scale;

        // 轻微左右摇摆
        transform.rotation = Quat::from_rotation_z(sway);
        // 轻微缩放模拟呼吸
        transform.scale = Vec3::splat(breathe);
    }
}
```

- [ ] **Step 2: 更新 nova_animation/src/lib.rs**

```rust
pub mod clip;
pub mod player;
pub mod plugin;
pub mod prelude;
pub mod procedural;  // 新增
pub mod tween;

pub use plugin::NovaAnimationPlugin;
pub use procedural::{ProceduralIdle, procedural_idle_system};  // 新增
```

- [ ] **Step 3: 更新 plugin.rs 注册系统**

找到 `crates/nova_animation/src/plugin.rs`，在 `Plugin::build` 中添加：
```rust
app.add_systems(Update, crate::procedural::procedural_idle_system);
```

- [ ] **Step 4: 验证编译**

```bash
cargo check -p nova_animation
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_animation/src/procedural.rs crates/nova_animation/src/lib.rs crates/nova_animation/src/plugin.rs
git commit -m "feat(nova_animation): 添加程序化待机动画（ProceduralIdle）"
```

---

### Task 16: rts_demo 集成 nova_character

**Files:**
- Create: `examples/rts_demo/src/character_setup.rs`
- Create: `examples/rts_demo/assets/characters.json`
- Modify: `examples/rts_demo/Cargo.toml`
- Modify: `examples/rts_demo/src/main.rs`
- Modify: `examples/rts_demo/src/setup.rs`

- [ ] **Step 1: 更新 Cargo.toml 加入依赖**

在 `examples/rts_demo/Cargo.toml` 的 `[dependencies]` 中添加：
```toml
nova_character = { path = "../../crates/nova_character" }
nova_ai = { path = "../../crates/nova_ai" }
nova_formation = { path = "../../crates/nova_formation" }
nova_animation = { path = "../../crates/nova_animation" }
```

- [ ] **Step 2: 创建角色配置 JSON**

`examples/rts_demo/assets/characters.json`：
```json
{
  "characters": [
    {
      "id": "soldier",
      "name": "士兵",
      "character_type": "Infantry",
      "attributes": {
        "health": 100.0,
        "attack": 10.0,
        "defense": 5.0,
        "move_speed": 5.0,
        "attack_range": 1.5,
        "attack_speed": 1.0,
        "vision_range": 10.0
      },
      "model": {
        "type": "Primitive",
        "shape": { "Capsule": { "radius": 0.3, "height": 0.8 } },
        "color": [0.2, 0.6, 1.0, 1.0]
      },
      "personality": {
        "aggression": 0.6,
        "courage": 0.7,
        "discipline": 0.8
      }
    },
    {
      "id": "enemy_grunt",
      "name": "敌兵",
      "character_type": "Infantry",
      "attributes": {
        "health": 80.0,
        "attack": 8.0,
        "defense": 3.0,
        "move_speed": 4.0,
        "attack_range": 1.5,
        "attack_speed": 1.2,
        "vision_range": 8.0
      },
      "model": {
        "type": "Primitive",
        "shape": { "Capsule": { "radius": 0.3, "height": 0.8 } },
        "color": [1.0, 0.3, 0.2, 1.0]
      },
      "personality": {
        "aggression": 0.7,
        "courage": 0.5,
        "discipline": 0.5
      }
    }
  ]
}
```

- [ ] **Step 3: 创建 character_setup.rs**

```rust
//! 角色生成辅助 - 使用 nova_character 定义的角色类型

use bevy::prelude::*;

use nova_ai::{
    behavior::BehaviorTree,
    emotion::Emotion,
    perception::{Perception, PerceivedEntities},
    personality::Personality,
};
use nova_animation::procedural::ProceduralIdle;
use nova_character::{
    attributes::Attributes,
    character::{Character, CharacterType, Faction},
    feedback::{HealthBar, StatusIndicator},
    state::{AttackCooldown, CharacterState},
};

/// 生成玩家角色
pub fn spawn_player_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    unit_id: u64,
    phase: f32,
) -> Entity {
    let mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 1.0),
        ..default()
    });

    let attrs = Attributes::default();
    let perception_range = attrs.vision_range;
    let attack_speed = attrs.attack_speed;

    commands.spawn((
        // 标识
        Character::new(unit_id, "士兵", CharacterType::Infantry),
        Faction::Player,
        // 属性与状态
        attrs,
        CharacterState::Idle,
        AttackCooldown::new(attack_speed),
        // AI
        Perception::new(perception_range),
        PerceivedEntities::default(),
        Personality::soldier(),
        Emotion::default(),
        BehaviorTree::standard_soldier(),
        // 视觉反馈
        HealthBar::default(),
        StatusIndicator::default(),
        ProceduralIdle::new_with_phase(phase),
        // 渲染
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
    ))
    .id()
}

/// 生成敌方角色
pub fn spawn_enemy_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    unit_id: u64,
    phase: f32,
) -> Entity {
    let mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.3, 0.2),
        ..default()
    });

    let attrs = Attributes {
        health: nova_character::attributes::Health::new(80.0),
        attack: 8.0,
        attack_speed: 1.2,
        vision_range: 8.0,
        ..default()
    };
    let perception_range = attrs.vision_range;
    let attack_speed = attrs.attack_speed;

    commands.spawn((
        Character::new(unit_id, "敌兵", CharacterType::Infantry),
        Faction::Enemy,
        attrs,
        CharacterState::Idle,
        AttackCooldown::new(attack_speed),
        Perception::new(perception_range),
        PerceivedEntities::default(),
        Personality::soldier(),
        Emotion::default(),
        BehaviorTree::standard_soldier(),
        HealthBar::default(),
        StatusIndicator::default(),
        ProceduralIdle::new_with_phase(phase),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
    ))
    .id()
}
```

- [ ] **Step 4: 更新 main.rs 注册新插件**

将 `examples/rts_demo/src/main.rs` 中的 `fn main()` 替换为：

```rust
//! RTS 游戏原型

use nova_ai::NovaAiPlugin;
use nova_animation::NovaAnimationPlugin;
use nova_character::NovaCharacterPlugin;
use nova_engine::prelude::*;
use nova_formation::NovaFormationPlugin;
use nova_map::prelude::*;

mod character_setup;
mod combat;
mod components;
mod movement;
mod selection;
mod setup;
mod ui;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - RTS Demo")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaMapWithFogPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(NovaCharacterPlugin)
        .add_plugin(NovaAiPlugin)
        .add_plugin(NovaFormationPlugin)
        .add_plugin(NovaAnimationPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(setup::setup_game)
        .run();
}
```

- [ ] **Step 5: 更新 setup.rs 使用新函数**

将 `spawn_player_units` 和 `spawn_enemy_units` 函数替换为调用 `character_setup` 中的函数。**注意：setup.rs 中现有函数需要与 `nova_map::prelude::Vision` 对齐**——新角色组件已内含 `Perception`，不再需要旧的 `Vision` 组件。用以下完整函数替换对应函数：

```rust
use crate::character_setup::{spawn_enemy_unit, spawn_player_unit};

fn spawn_player_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let positions = [
        Vec3::new(-25.0, 0.5, -25.0),
        Vec3::new(-23.0, 0.5, -25.0),
        Vec3::new(-24.0, 0.5, -23.0),
    ];
    for (i, pos) in positions.iter().enumerate() {
        spawn_player_unit(commands, meshes, materials, *pos, i as u64, i as f32 * 1.3);
    }
}

fn spawn_enemy_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let positions = [
        Vec3::new(25.0, 0.5, 25.0),
        Vec3::new(23.0, 0.5, 25.0),
        Vec3::new(24.0, 0.5, 23.0),
    ];
    for (i, pos) in positions.iter().enumerate() {
        spawn_enemy_unit(commands, meshes, materials, *pos, (10 + i) as u64, i as f32 * 1.7);
    }
}
```

- [ ] **Step 6: 验证编译**

```bash
cargo check -p rts_demo
```

Expected: 编译成功

- [ ] **Step 7: 提交**

```bash
git add examples/rts_demo/src/character_setup.rs \
        examples/rts_demo/assets/characters.json \
        examples/rts_demo/Cargo.toml \
        examples/rts_demo/src/main.rs \
        examples/rts_demo/src/setup.rs
git commit -m "feat(rts_demo): 集成 nova_character、nova_ai、nova_formation、nova_animation"
```

---

### Task 17: nova_engine 更新导出

**Files:**
- Modify: `crates/nova_engine/Cargo.toml`
- Modify: `crates/nova_engine/src/lib.rs`

- [ ] **Step 1: 更新 nova_engine/Cargo.toml**

```toml
nova_character = { workspace = true }
nova_ai = { workspace = true }
nova_formation = { workspace = true }
```

- [ ] **Step 2: 更新 nova_engine/src/lib.rs**

```rust
pub use nova_character;
pub use nova_ai;
pub use nova_formation;
```

- [ ] **Step 3: 验证编译**

```bash
cargo check -p nova_engine
```

Expected: 编译成功

- [ ] **Step 4: 全量 WASM 编译检查**

```bash
cargo check --target wasm32-unknown-unknown -p rts_demo
```

Expected: 编译成功

- [ ] **Step 5: 提交**

```bash
git add crates/nova_engine/Cargo.toml crates/nova_engine/src/lib.rs
git commit -m "feat(nova_engine): 导出 nova_character、nova_ai、nova_formation"
```

---

### Task 18: 最终验证

**Files:** 所有相关文件

- [ ] **Step 1: 全量编译检查（native）**

```bash
cargo check --all-targets
```

Expected: 无错误，只允许 warnings

- [ ] **Step 2: WASM 编译检查**

```bash
cargo check --target wasm32-unknown-unknown -p rts_demo
```

Expected: 编译成功

- [ ] **Step 3: Clippy 检查**

```bash
cargo clippy -p nova_character -p nova_ai -p nova_formation -p rts_demo -- -D warnings
```

Expected: 无错误（warnings 允许修复）

- [ ] **Step 4: 格式化**

```bash
cargo fmt --all
```

- [ ] **Step 5: 运行 Demo 验证**

```bash
cd examples/rts_demo && trunk serve
```

打开浏览器 http://localhost:8080，验证：
- 单位有轻微的程序化待机摇摆（"呼吸感"）
- 玩家单位和敌方单位有生命条显示
- 受伤时出现受击闪烁效果
- AI 单位能自动发现并攻击敌人
- 通过感知系统，单位进入视野范围内时才被发现

- [ ] **Step 6: 最终提交**

```bash
git add -A
git commit -m "feat: Nova Character 人物建模系统完成集成"
```

---

## 依赖关系总结

```
nova_core
  └── nova_character   ← Task 1-6（属性、状态、反馈、配置加载）
        └── nova_ai    ← Task 7-11（感知、行为树、性格情绪）
nova_core
  └── nova_formation   ← Task 12（编队、阵型、槽位）
nova_animation         ← Task 13（状态机）、Task 15（程序化动画）
rts_demo               ← Task 16-18（集成验证）
```

## 设计规格成功标准覆盖

| 成功标准 | 覆盖任务 |
|---------|---------|
| 角色数据组件可序列化 | Task 2、4 |
| 状态机支持基本状态 | Task 3 |
| 视觉反馈（伤害数字、生命条） | Task 5 |
| 死亡视觉反馈（UnitDiedEvent） | Task 5 |
| JSON 加载角色定义 | Task 6 |
| 感知系统检测可见实体 | Task 8 |
| 行为树执行基本行为 | Task 10、11 |
| 性格影响决策 | Task 9 |
| 情绪动态变化 | Task 9 |
| 编队创建/解散 | Task 12 |
| 方阵、楔形、横线阵型 | Task 12 |
| 编队整体移动 | Task 12 |
| 动画状态机切换 | Task 13 |
| 程序化待机动画 | Task 15 |
| rts_demo 单位"活着"感 | Task 16 |
| 选中单位移动/攻击 | Task 16（集成现有系统）|
| 性能 30 个单位 60fps | Task 18（验证）|

---

*文档版本: 1.0*
*创建日期: 2026-03-19*
*对应设计文档: docs/superpowers/specs/2026-03-18-nova-character-design.md*
