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
