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
        self.formations
            .insert(id, Formation::new(id, leader, pattern, spacing));
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
        Self {
            formation_id,
            slot_index,
            local_offset,
        }
    }
}
