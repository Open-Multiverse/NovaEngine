//! 战斗系统

use bevy::prelude::*;
use nova_map::prelude::*;

use crate::components::*;

/// 战斗系统插件
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                attack_cooldown_system,
                chase_target_system,
                combat_system,
                auto_target_system,
                death_system,
            ),
        );
    }
}

/// 攻击冷却更新
fn attack_cooldown_system(time: Res<Time>, mut units: Query<&mut Attack>) {
    for mut attack in units.iter_mut() {
        attack.tick(time.delta_secs());
    }
}

/// 追击目标系统
fn chase_target_system(
    tilemap: Option<Res<TileMap>>,
    mut attackers: Query<
        (Entity, &Transform, &Attack, &AttackTarget),
        (With<Movement>, Without<PathFollow>),
    >,
    targets: Query<&Transform, With<Health>>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, attacker_transform, attack, target) in attackers.iter_mut() {
        let Ok(target_transform) = targets.get(target.0) else {
            // 目标不存在，移除攻击目标
            commands.entity(entity).remove::<AttackTarget>();
            continue;
        };

        let distance = (attacker_transform.translation - target_transform.translation).length();

        // 如果不在攻击范围内，移动靠近
        if distance > attack.range {
            let Some(start_tile) = tilemap.world_to_tile(attacker_transform.translation) else {
                continue;
            };
            let Some(goal_tile) = tilemap.world_to_tile(target_transform.translation) else {
                continue;
            };

            if let Some(result) = Pathfinder::find_path(&tilemap, start_tile, goal_tile) {
                commands.entity(entity).insert(PathFollow::new(result.path));
            }
        }
    }
}

/// 战斗执行系统
fn combat_system(
    mut attackers: Query<(&Transform, &mut Attack, &AttackTarget, &Team)>,
    mut targets: Query<(&Transform, &mut Health, &Team)>,
) {
    for (attacker_transform, mut attack, target, attacker_team) in attackers.iter_mut() {
        let Ok((target_transform, mut target_health, target_team)) = targets.get_mut(target.0)
        else {
            continue;
        };

        // 不能攻击同队
        if attacker_team == target_team {
            continue;
        }

        let distance = (attacker_transform.translation - target_transform.translation).length();

        // 在范围内且冷却完成
        if distance <= attack.range && attack.can_attack() {
            target_health.take_damage(attack.damage);
            attack.reset_cooldown();
        }
    }
}

/// 自动索敌系统
fn auto_target_system(
    units: Query<
        (Entity, &Transform, &Team, &Attack),
        (With<Unit>, Without<AttackTarget>, Without<PathFollow>),
    >,
    potential_targets: Query<(Entity, &Transform, &Team), With<Health>>,
    mut commands: Commands,
) {
    for (entity, transform, team, attack) in units.iter() {
        let mut closest_enemy: Option<(Entity, f32)> = None;

        for (target_entity, target_transform, target_team) in potential_targets.iter() {
            // 跳过同队
            if team == target_team {
                continue;
            }

            let distance = (transform.translation - target_transform.translation).length();

            // 检测范围为攻击范围的 2 倍
            if distance <= attack.range * 2.0 {
                if closest_enemy.is_none() || distance < closest_enemy.unwrap().1 {
                    closest_enemy = Some((target_entity, distance));
                }
            }
        }

        if let Some((target_entity, _)) = closest_enemy {
            commands.entity(entity).insert(AttackTarget(target_entity));
        }
    }
}

/// 死亡清理系统
fn death_system(units: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in units.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
