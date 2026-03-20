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
