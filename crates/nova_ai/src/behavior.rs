//! 行为树数据结构

use std::sync::Arc;
use bevy::prelude::*;
use crate::blackboard::Blackboard;

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
    Repeater {
        node: Box<BehaviorNode>,
        times: u32,
        current: u32,
    },
}

/// 行为动作节点
#[derive(Clone)]
pub enum ActionNode {
    Idle,
    MoveTo(MoveTarget),
    Attack(AttackTarget),
    Flee,
    Patrol { points: Vec<Vec3>, current: usize },
    FollowLeader,
    /// 自定义闭包动作，接收可变 Blackboard 引用，返回 bool（true=成功，false=失败）
    /// 使用 Arc 使其可 Clone（引用计数，低成本）
    Custom(Arc<dyn Fn(&mut Blackboard) -> bool + Send + Sync>),
}

impl std::fmt::Debug for ActionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionNode::Idle => write!(f, "ActionNode::Idle"),
            ActionNode::MoveTo(t) => write!(f, "ActionNode::MoveTo({:?})", t),
            ActionNode::Attack(t) => write!(f, "ActionNode::Attack({:?})", t),
            ActionNode::Flee => write!(f, "ActionNode::Flee"),
            ActionNode::Patrol { points, current } => {
                write!(f, "ActionNode::Patrol {{ points: {:?}, current: {} }}", points, current)
            }
            ActionNode::FollowLeader => write!(f, "ActionNode::FollowLeader"),
            ActionNode::Custom(_) => write!(f, "ActionNode::Custom(fn)"),
        }
    }
}

/// 条件节点
#[derive(Clone, Debug)]
pub enum ConditionNode {
    HasTarget,
    HealthBelow(f32), // 百分比，如 0.3 表示 30%
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

    /// 创建空顺序节点（用作 builder 起点）
    pub fn sequence() -> Self {
        Self { root: BehaviorNode::Sequence(vec![]) }
    }

    /// 创建自定义动作节点，接受返回 bool 的闭包
    pub fn action(f: impl Fn(&mut Blackboard) -> bool + Send + Sync + 'static) -> Self {
        Self { root: BehaviorNode::Action(ActionNode::Custom(Arc::new(f))) }
    }

    /// 链式添加子节点（仅对 Sequence/Selector/Parallel 有效，其他节点类型静默忽略）
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
        Self::sequence()
    }
}
