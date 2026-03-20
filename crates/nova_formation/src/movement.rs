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
    leaders: Query<&Transform, Without<FormationMember>>,
    mut members: Query<(&mut Transform, &FormationMember)>,
) {
    for (mut member_transform, member) in members.iter_mut() {
        let Some(formation) = manager.get(member.formation_id) else {
            continue;
        };

        let Ok(leader_transform) = leaders.get(formation.leader) else {
            continue;
        };

        let target_pos = formation.slot_world_pos(member.slot_index, leader_transform.translation);

        let diff = target_pos - member_transform.translation;
        let distance = diff.length();

        if distance > 0.5 {
            let speed = 5.0_f32;
            let step = speed * time.delta_secs();
            member_transform.translation += diff.normalize() * step.min(distance);
        }
    }
}
