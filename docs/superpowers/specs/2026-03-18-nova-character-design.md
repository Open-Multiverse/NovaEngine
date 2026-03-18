# Nova Character 人物建模系统设计文档

> Nova Engine 人物系统模块 - 让游戏单位"活起来"

## 概述

本文档描述 Nova Engine 的人物建模能力，目标是让游戏单位在三个维度上表现出"活着"的感觉：

- **视觉上活的**：动画、待机动作、表情反应、视觉特效
- **逻辑上活的**：战斗 AI、巡逻行为、编队阵型、性格情绪
- **交互上活的**：物理碰撞、感知系统、地形交互、环境交互

### 应用场景

- RTS/策略游戏，上帝视角
- 小规模场景（10-30 个活动单位）
- 渐进式模型支持（先几何体，后外部模型）

### 设计原则

- **分层模块**：按职责拆分，可独立测试和复用
- **ECS 友好**：核心数据结构作为 Bevy Component/Resource
- **渐进式**：从简单几何体平滑过渡到骨骼动画模型

## 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    examples/rts_demo                         │
│  - 整合所有模块                                               │
│  - 游戏特定逻辑                                               │
├────────────┬────────────┬─────────────┬────────────────────┤
│ nova_ai    │nova_formation│nova_character│ nova_animation    │
│ - 行为树    │ - 编队管理   │ - 人物数据    │ - 状态机 (扩展)   │
│ - 决策系统  │ - 阵型计算   │ - 属性系统    │ - 骨骼动画       │
│ - 感知系统  │ - 群体移动   │ - 状态管理    │ - 程序化动画     │
│ - 性格/情绪 │             │ - 视觉反馈    │                  │
├────────────┴────────────┴─────────────┴────────────────────┤
│                    nova_physics (已有)                       │
│  - 碰撞检测、刚体、触发器                                      │
├────────────────────────────────────────────────────────────┤
│                    nova_map (已有)                           │
│  - 地形数据、寻路、迷雾                                        │
├────────────────────────────────────────────────────────────┤
│                    nova_core (已有)                          │
└─────────────────────────────────────────────────────────────┘
```

### 模块职责边界

| 模块 | 核心职责 | 不负责 |
|------|---------|--------|
| `nova_character` | 人物是什么（数据、属性、状态） | 如何决策、如何移动 |
| `nova_ai` | 人物怎么想（感知、决策、情绪） | 具体动画播放 |
| `nova_formation` | 群体怎么走（编队、阵型） | 单个单位的行为 |
| `nova_animation` | 人物怎么动（动画播放、状态机） | 决策什么时候播放 |

### 设计说明

**属性与感知的关系**：`Attributes.vision_range` 是角色的基础视野属性，`Perception.vision_range` 从 `Attributes` 初始化，可被 buff/debuff 修改。`Perception` 是运行时状态，`Attributes` 是基础数据。

**行为树执行模型**：采用每帧 tick 模式，在 `CharacterSet::Decision` 阶段执行。对于性能敏感场景，可考虑分帧执行（每帧只 tick 部分单位）。

## nova_character 模块设计

### 文件结构

```
crates/nova_character/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、插件定义
    ├── prelude.rs       # 公共导出
    ├── character.rs     # Character 组件、CharacterBundle
    ├── attributes.rs    # 属性系统（生命、攻击、防御等）
    ├── state.rs         # 状态机（Idle、Moving、Attacking 等）
    ├── feedback.rs      # 视觉反馈（伤害数字、状态图标）
    └── loader.rs        # 角色配置加载（JSON/RON）
```

### 核心组件

```rust
/// 角色标识 - 区分不同类型角色
#[derive(Component)]
pub struct Character {
    pub id: CharacterId,          // 唯一标识
    pub name: String,             // 显示名称
    pub character_type: CharacterType,  // 战士/弓箭手/法师等
}

/// 属性组件 - 数值相关
#[derive(Component)]
pub struct Attributes {
    pub health: Health,           // 当前/最大生命
    pub attack: f32,              // 攻击力
    pub defense: f32,             // 防御力
    pub move_speed: f32,          // 移动速度
    pub attack_range: f32,        // 攻击范围
    pub attack_speed: f32,        // 攻击间隔
    pub vision_range: f32,        // 视野范围
}

/// 角色状态 - 当前在做什么
#[derive(Component, Default)]
pub enum CharacterState {
    #[default]
    Idle,                         // 待机
    Moving { target: Vec3 },      // 移动中
    Attacking { target: Entity }, // 攻击中
    Stunned { duration: f32 },    // 眩晕
    Dead,                         // 死亡
}
```

### 视觉反馈事件

```rust
/// 视觉反馈请求 - 通过事件触发
pub enum FeedbackEvent {
    DamageNumber { entity: Entity, amount: f32, crit: bool },
    StatusIcon { entity: Entity, icon: StatusIconType },
    HitFlash { entity: Entity },
}
```

### 角色配置（支持外部加载）

```rust
/// 角色类型定义（可从 JSON 加载）
#[derive(Deserialize)]
pub struct CharacterDef {
    pub id: String,
    pub name: String,
    pub character_type: String,
    pub base_attributes: AttributesDef,
    pub model: ModelDef,
}

#[derive(Deserialize)]
pub enum ModelDef {
    Primitive { shape: String, color: [f32; 4] },
    Gltf { path: String },
}
```

## nova_ai 模块设计

### 文件结构

```
crates/nova_ai/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、插件定义
    ├── prelude.rs       # 公共导出
    ├── perception.rs    # 感知系统（视觉、听觉）
    ├── behavior.rs      # 行为树节点定义
    ├── decision.rs      # 决策系统（行为树执行器）
    ├── personality.rs   # 性格系统
    ├── emotion.rs       # 情绪系统
    └── tactics.rs       # 战术行为（追击、撤退、包围）
```

### 感知系统

```rust
/// 感知组件 - 单位能"看到/听到"什么
#[derive(Component)]
pub struct Perception {
    pub vision_range: f32,        // 视觉范围
    pub vision_angle: f32,        // 视野角度（360 为全向）
    pub hearing_range: f32,       // 听觉范围
}

/// 感知结果 - 每帧更新
#[derive(Component, Default)]
pub struct PerceivedEntities {
    pub visible: Vec<Entity>,     // 当前可见的实体
    pub heard: Vec<Entity>,       // 当前听到的实体
    pub closest_enemy: Option<Entity>,
    pub closest_ally: Option<Entity>,
}

/// 感知事件 - 用于触发反应
pub enum PerceptionEvent {
    EnemySpotted { perceiver: Entity, enemy: Entity },
    EnemyLost { perceiver: Entity, enemy: Entity },
    AllyUnderAttack { perceiver: Entity, ally: Entity },
}
```

### 行为树系统

```rust
/// 行为节点类型
pub enum BehaviorNode {
    // 叶子节点 - 实际行为
    Action(ActionNode),
    Condition(ConditionNode),

    // 组合节点
    Sequence(Vec<BehaviorNode>),   // 顺序执行，全成功才成功
    Selector(Vec<BehaviorNode>),   // 选择执行，一个成功即成功
    Parallel(Vec<BehaviorNode>),   // 并行执行

    // 装饰节点
    Inverter(Box<BehaviorNode>),   // 反转结果
    Repeater { node: Box<BehaviorNode>, times: u32 },
}

/// 预定义行为动作
pub enum ActionNode {
    Idle,                          // 待机
    MoveTo(MoveTarget),            // 移动到目标
    Attack(AttackTarget),          // 攻击目标
    Flee,                          // 逃跑
    Patrol { points: Vec<Vec3> },  // 巡逻
    FollowLeader,                  // 跟随队长
}

/// 预定义条件
pub enum ConditionNode {
    HasTarget,                     // 有攻击目标
    HealthBelow(f32),              // 血量低于百分比
    EnemyInRange,                  // 敌人在攻击范围内
    IsInFormation,                 // 在编队中
    EmotionIs(EmotionType),        // 情绪状态检查
}
```

### 性格与情绪

```rust
/// 性格特质 - 影响决策权重
#[derive(Component)]
pub struct Personality {
    pub aggression: f32,      // 0-1，高=主动进攻，低=防守
    pub courage: f32,         // 0-1，高=不畏死亡，低=容易逃跑
    pub discipline: f32,      // 0-1，高=严格执行命令，低=自由行动
}

/// 情绪状态 - 动态变化，影响行为
#[derive(Component, Default)]
pub struct Emotion {
    pub current: EmotionType,
    pub intensity: f32,       // 情绪强度 0-1
}

#[derive(Default, Clone, Copy)]
pub enum EmotionType {
    #[default]
    Calm,        // 平静 - 正常行为
    Angry,       // 愤怒 - 攻击加成，防御降低
    Fearful,     // 恐惧 - 倾向逃跑
    Berserk,     // 狂暴 - 无视命令，疯狂攻击
}
```

### 情绪触发规则

```rust
impl Emotion {
    pub fn on_damage(&mut self, damage_percent: f32, personality: &Personality) {
        if damage_percent > 0.3 {
            if personality.courage < 0.3 {
                self.current = EmotionType::Fearful;
            } else if personality.aggression > 0.7 {
                self.current = EmotionType::Angry;
            }
        }
    }

    pub fn on_ally_death(&mut self, personality: &Personality) {
        if personality.aggression > 0.8 {
            self.current = EmotionType::Berserk;
        }
    }
}
```

## nova_formation 模块设计

### 文件结构

```
crates/nova_formation/
├── Cargo.toml
└── src/
    ├── lib.rs           # 模块入口、插件定义
    ├── prelude.rs       # 公共导出
    ├── formation.rs     # 编队数据结构
    ├── patterns.rs      # 阵型模式（方阵、楔形等）
    ├── movement.rs      # 编队移动系统
    └── slots.rs         # 槽位分配算法
```

### 编队数据结构

```rust
/// 编队资源 - 管理所有编队
#[derive(Resource, Default)]
pub struct FormationManager {
    formations: HashMap<FormationId, Formation>,
    next_id: u32,
}

/// 单个编队
pub struct Formation {
    pub id: FormationId,
    pub leader: Entity,              // 队长（编队中心参考）
    pub members: Vec<Entity>,        // 成员列表
    pub pattern: FormationPattern,   // 阵型模式
    pub spacing: f32,                // 单位间距
    pub facing: Vec3,                // 编队朝向
}

/// 编队成员组件
#[derive(Component)]
pub struct FormationMember {
    pub formation_id: FormationId,
    pub slot_index: usize,           // 在阵型中的位置索引
    pub local_offset: Vec3,          // 相对队长的偏移
}
```

### 阵型模式

```rust
/// 预定义阵型
#[derive(Clone)]
pub enum FormationPattern {
    /// 方阵 - 适合步兵
    Square { rows: u32, cols: u32 },

    /// 楔形 - 适合冲锋
    Wedge { depth: u32 },

    /// 横线 - 适合远程单位
    Line,

    /// 圆形 - 防御阵型
    Circle { radius: f32 },

    /// 自定义 - 指定每个槽位的相对位置
    Custom { slots: Vec<Vec3> },
}

impl FormationPattern {
    /// 计算阵型中第 n 个单位的相对位置
    pub fn slot_offset(&self, index: usize, spacing: f32) -> Vec3;
}
```

### 槽位分配策略

```rust
pub enum SlotAssignment {
    /// 按加入顺序
    Sequential,
    /// 按距离最优分配（匈牙利算法）
    Optimal,
    /// 按单位类型（近战在前，远程在后）
    ByUnitType,
}
```

## nova_animation 扩展设计

### 新增文件

```
crates/nova_animation/src/
├── ... (现有文件)
├── state_machine.rs  # 新增：动画状态机
├── procedural.rs     # 新增：程序化动画
└── skeleton.rs       # 新增：骨骼动画支持
```

### 动画状态机

```rust
/// 动画状态机组件
#[derive(Component)]
pub struct AnimationStateMachine {
    pub current_state: AnimationState,
    pub states: HashMap<AnimationState, AnimationStateConfig>,
    pub transitions: Vec<AnimationTransition>,
}

/// 动画状态
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Walk,
    Run,
    Attack,
    Hit,
    Die,
    Custom(u32),
}

/// 状态配置
pub struct AnimationStateConfig {
    pub clip_name: String,
    pub loop_mode: LoopMode,
    pub speed: f32,
}

/// 状态转换规则
pub struct AnimationTransition {
    pub from: AnimationState,
    pub to: AnimationState,
    pub condition: TransitionCondition,
    pub blend_duration: f32,
}
```

### 程序化待机动画

```rust
/// 程序化待机动画 - 让静止单位看起来"活着"
#[derive(Component)]
pub struct ProceduralIdle {
    pub enabled: bool,
    pub sway_amplitude: f32,      // 摇摆幅度
    pub sway_speed: f32,          // 摇摆速度
    pub breathe_scale: f32,       // 呼吸缩放幅度
    pub phase: f32,               // 相位（避免同步）
}
```

### 骨骼动画支持

```rust
/// 骨骼动画配置（用于 glTF 模型）
#[derive(Component)]
pub struct SkeletonAnimation {
    pub animation_clips: HashMap<String, Handle<AnimationClip>>,
    pub current_clip: Option<String>,
}
```

## 视觉反馈系统

### 伤害数字

```rust
/// 伤害数字组件
#[derive(Component)]
pub struct DamageNumber {
    pub value: f32,
    pub is_crit: bool,
    pub lifetime: f32,
    pub velocity: Vec3,
}

/// 伤害数字生成事件
#[derive(Event)]
pub struct SpawnDamageNumber {
    pub position: Vec3,
    pub damage: f32,
    pub is_crit: bool,
}
```

### 受击闪烁

```rust
/// 受击闪烁组件
#[derive(Component)]
pub struct HitFlash {
    pub timer: f32,
    pub flash_color: Color,
    pub original_color: Color,
}

/// 触发受击闪烁事件
#[derive(Event)]
pub struct TriggerHitFlash {
    pub entity: Entity,
    pub duration: f32,
}
```

### 状态图标与生命条

```rust
/// 头顶状态图标
#[derive(Component)]
pub struct StatusIndicator {
    pub icons: Vec<StatusIcon>,
    pub offset: Vec3,
}

#[derive(Clone, Copy)]
pub enum StatusIconType {
    MovingTo,
    Attacking,
    Stunned,
    Slowed,
    Enraged,
    Fearful,
}

/// 头顶生命条
#[derive(Component)]
pub struct HealthBar {
    pub width: f32,
    pub height: f32,
    pub offset: Vec3,
    pub show_when_full: bool,
    pub ally_color: Color,
    pub enemy_color: Color,
}
```

## 模块集成与数据流

### 数据流

```
┌─────────────────────────────────────────────────────────────────────┐
│                           游戏循环                                   │
└─────────────────────────────────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        ▼                         ▼                         ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│  感知阶段      │        │  决策阶段      │        │  执行阶段      │
│  (Perception) │───────▶│  (Decision)   │───────▶│  (Execution)  │
└───────────────┘        └───────────────┘        └───────────────┘
        │                         │                         │
        ▼                         ▼                         ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│ nova_ai       │        │ nova_ai       │        │ nova_character│
│ perception.rs │        │ decision.rs   │        │ state.rs      │
└───────────────┘        └───────────────┘        └───────────────┘
                                                          │
        ┌─────────────────────────┬─────────────────────────┤
        ▼                         ▼                         ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│nova_formation │        │ nova_physics  │        │nova_animation │
└───────────────┘        └───────────────┘        └───────────────┘
                                                          │
                                                          ▼
                                                  ┌───────────────┐
                                                  │nova_character │
                                                  │ feedback.rs   │
                                                  └───────────────┘
```

### 系统执行顺序

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterSet {
    Perception,    // 感知
    Decision,      // 决策
    Movement,      // 移动（包括编队）
    Combat,        // 战斗
    Animation,     // 动画
    Feedback,      // 视觉反馈
}

app.configure_sets(
    Update,
    (
        CharacterSet::Perception,
        CharacterSet::Decision,
        CharacterSet::Movement,
        CharacterSet::Combat,
        CharacterSet::Animation,
        CharacterSet::Feedback,
    ).chain()
);
```

### 插件组合

```rust
/// 完整角色系统插件
pub struct NovaCharacterFullPlugin;

impl Plugin for NovaCharacterFullPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(NovaCharacterPlugin)
            .add_plugins(NovaAiPlugin)
            .add_plugins(NovaFormationPlugin)
            .add_plugins(NovaAnimationPlugin);
    }
}
```

## 渐进式模型支持

### 模型抽象层

```rust
/// 角色模型配置
#[derive(Clone)]
pub enum CharacterModel {
    Primitive(PrimitiveModel),
    Gltf(GltfModel),
}

/// 程序化模型
#[derive(Clone)]
pub struct PrimitiveModel {
    pub shape: PrimitiveShape,
    pub color: Color,
    pub scale: Vec3,
}

#[derive(Clone)]
pub enum PrimitiveShape {
    Capsule { radius: f32, height: f32 },
    Cube { size: f32 },
    Sphere { radius: f32 },
}

/// glTF 模型配置
#[derive(Clone)]
pub struct GltfModel {
    pub path: String,
    pub scale: Vec3,
    pub animation_map: HashMap<AnimationState, String>,
}
```

### 角色定义文件示例

```json
{
  "characters": [
    {
      "id": "soldier",
      "name": "士兵",
      "character_type": "Infantry",
      "attributes": {
        "health": 100,
        "attack": 10,
        "defense": 5,
        "move_speed": 5.0,
        "attack_range": 1.5,
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
    }
  ]
}
```

### 推荐的外部模型来源

| 来源 | 特点 | 许可 |
|------|------|------|
| Mixamo | 免费人物模型+动画 | 商用免费 |
| Sketchfab | 大量免费/付费模型 | 按模型许可 |
| Kenney | 简约风格游戏资源 | CC0 |
| Quaternius | 低多边形角色 | CC0 |

**格式推荐**：glTF/GLB 格式，Bevy 原生支持。

## 依赖关系

### nova_character/Cargo.toml

```toml
[package]
name = "nova_character"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_core = { path = "../nova_core" }
bevy = { version = "0.15", default-features = false }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### nova_ai/Cargo.toml

```toml
[package]
name = "nova_ai"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_core = { path = "../nova_core" }
nova_character = { path = "../nova_character" }
bevy = { version = "0.15", default-features = false }
```

### nova_formation/Cargo.toml

```toml
[package]
name = "nova_formation"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_core = { path = "../nova_core" }
bevy = { version = "0.15", default-features = false }
```

## 成功标准

### nova_character 模块

1. 角色数据组件正确定义并可序列化
2. 状态机支持基本状态（Idle/Moving/Attacking/Dead）
3. 视觉反馈（伤害数字、生命条）正常显示
4. 支持从 JSON 加载角色定义

### nova_ai 模块

1. 感知系统正确检测可见实体
2. 行为树能执行基本行为（移动、攻击、巡逻）
3. 性格影响决策（勇敢/懦弱表现不同）
4. 情绪动态变化并影响行为

### nova_formation 模块

1. 支持创建/解散编队
2. 方阵、楔形、横线阵型正确计算位置
3. 编队整体移动时保持阵型
4. 成员能绕过障碍后回归阵型

### nova_animation 扩展

1. 动画状态机正确切换状态
2. 程序化待机动画让单位"呼吸"
3. 骨骼动画能加载并播放 glTF 模型动画

### 集成验证

1. 在 rts_demo 中单位表现出"活着"的感觉
2. 选中单位能移动、攻击、编队
3. 单位死亡有动画和视觉反馈
4. 性能满足 30 个单位同屏 60fps

---

*文档版本: 1.0*
*创建日期: 2026-03-18*
