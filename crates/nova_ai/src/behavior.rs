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
    EmotionIs(crate::emotion::EmotionType),
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
