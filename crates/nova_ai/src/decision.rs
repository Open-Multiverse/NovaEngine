//! 行为树执行器

use bevy::prelude::*;

use nova_character::{attributes::Attributes, state::CharacterState};

use crate::{
    behavior::{
        ActionNode, AttackTarget, BehaviorNode, BehaviorStatus, BehaviorTree, ConditionNode,
        MoveTarget,
    },
    blackboard::Blackboard,
    emotion::Emotion,
    perception::PerceivedEntities,
};

/// AI 控制标记组件——挂载此组件的实体受行为树系统驱动
#[derive(Component, Default, Clone, Debug)]
pub struct AiAgent;

impl AiAgent {
    pub fn new() -> Self {
        Self
    }
}

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
    blackboard: &mut Option<Mut<Blackboard>>,
) -> BehaviorStatus {
    match node {
        BehaviorNode::Sequence(children) => {
            for child in children {
                match evaluate_node(child, ctx, commands, blackboard) {
                    BehaviorStatus::Failure => return BehaviorStatus::Failure,
                    BehaviorStatus::Running => return BehaviorStatus::Running,
                    BehaviorStatus::Success => {}
                }
            }
            BehaviorStatus::Success
        }

        BehaviorNode::Selector(children) => {
            for child in children {
                match evaluate_node(child, ctx, commands, blackboard) {
                    BehaviorStatus::Success => return BehaviorStatus::Success,
                    BehaviorStatus::Running => return BehaviorStatus::Running,
                    BehaviorStatus::Failure => {}
                }
            }
            BehaviorStatus::Failure
        }

        BehaviorNode::Inverter(inner) => match evaluate_node(inner, ctx, commands, blackboard) {
            BehaviorStatus::Success => BehaviorStatus::Failure,
            BehaviorStatus::Failure => BehaviorStatus::Success,
            BehaviorStatus::Running => BehaviorStatus::Running,
        },

        BehaviorNode::Parallel(children) => {
            let mut any_running = false;
            for child in children {
                if evaluate_node(child, ctx, commands, blackboard) == BehaviorStatus::Running {
                    any_running = true;
                }
            }
            if any_running {
                BehaviorStatus::Running
            } else {
                BehaviorStatus::Success
            }
        }

        BehaviorNode::Condition(cond) => evaluate_condition(cond, ctx),

        BehaviorNode::Action(action) => execute_action(action, ctx, commands, blackboard),

        BehaviorNode::Repeater { .. } => BehaviorStatus::Running,
    }
}

fn evaluate_condition(cond: &ConditionNode, ctx: &BtContext) -> BehaviorStatus {
    let result = match cond {
        ConditionNode::HasPerceivedEnemy | ConditionNode::HasTarget => {
            ctx.perceived.closest_enemy.is_some()
        }
        ConditionNode::HealthBelow(threshold) => ctx.attributes.health.percentage() < *threshold,
        ConditionNode::EnemyInRange => ctx.perceived.closest_enemy.is_some(),
        ConditionNode::EnemyInAttackRange => ctx.perceived.closest_enemy.is_some(),
        ConditionNode::EmotionIs(target_emotion) => ctx
            .emotion
            .map(|e| e.current == *target_emotion)
            .unwrap_or(false),
        ConditionNode::IsInFormation => false, // 由 nova_formation 处理
    };

    if result {
        BehaviorStatus::Success
    } else {
        BehaviorStatus::Failure
    }
}

fn execute_action(
    action: &ActionNode,
    ctx: &BtContext,
    commands: &mut EntityCommands,
    blackboard: &mut Option<Mut<Blackboard>>,
) -> BehaviorStatus {
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
            }
            BehaviorStatus::Running
        }
        ActionNode::Flee => {
            if ctx.perceived.closest_enemy.is_some() {
                let retreat_pos = ctx.transform.translation + ctx.transform.back() * 10.0;
                commands.insert(CharacterState::Moving {
                    target: retreat_pos,
                });
            }
            BehaviorStatus::Running
        }
        ActionNode::Patrol { .. } | ActionNode::FollowLeader => BehaviorStatus::Running,
        ActionNode::Custom(f) => {
            if let Some(bb) = blackboard.as_deref_mut() {
                if f(bb) {
                    BehaviorStatus::Success
                } else {
                    BehaviorStatus::Failure
                }
            } else {
                // 没有 Blackboard 组件时安全降级
                BehaviorStatus::Failure
            }
        }
    }
}

/// 行为树执行系统（每帧 tick 所有有 BehaviorTree 的单位）
#[allow(clippy::type_complexity)]
pub fn behavior_tree_system(
    mut query: Query<(
        Entity,
        &Transform,
        &Attributes,
        &PerceivedEntities,
        Option<&Emotion>,
        &BehaviorTree,
        Option<&mut Blackboard>,
    )>,
    mut commands: Commands,
) {
    for (entity, transform, attributes, perceived, emotion, tree, mut blackboard) in query.iter_mut() {
        let ctx = BtContext {
            entity,
            transform,
            attributes,
            perceived,
            emotion,
        };

        let root = tree.root.clone();
        let mut entity_commands = commands.entity(entity);
        evaluate_node(&root, &ctx, &mut entity_commands, &mut blackboard);
    }
}
