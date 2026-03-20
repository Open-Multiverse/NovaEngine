//! GPU 实例化渲染
//!
//! 合并相同材质/网格的渲染调用

use bevy::prelude::*;
use std::collections::HashMap;

/// 实例批次键
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceKey {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl InstanceKey {
    fn from_components(mesh: &Mesh3d, material: &MeshMaterial3d<StandardMaterial>) -> Self {
        Self {
            mesh: mesh.0.clone(),
            material: material.0.clone(),
        }
    }
}

/// 实例批次数据
#[derive(Resource, Default)]
pub struct InstanceBatches {
    pub batches: HashMap<InstanceKey, Vec<Mat4>>,
    pub dirty: bool,
}

/// 可实例化标记
#[derive(Component)]
pub struct Instanced;

/// 更新实例批次
pub fn update_instance_batches(
    mut batches: ResMut<InstanceBatches>,
    query: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>, &GlobalTransform), With<Instanced>>,
    settings: Res<super::PerformanceSettings>,
) {
    if !settings.enable_instancing {
        return;
    }

    // 清空旧批次
    batches.batches.clear();

    // 收集所有可实例化实体
    for (mesh, material, transform) in query.iter() {
        let key = InstanceKey::from_components(mesh, material);

        batches
            .batches
            .entry(key)
            .or_default()
            .push(transform.compute_matrix());
    }

    // 移除未达到阈值的批次（使用 CPU 渲染）
    let threshold = settings.instancing_threshold;
    batches
        .batches
        .retain(|_, matrices| matrices.len() >= threshold);

    batches.dirty = false;
}

/// 为批量实体添加实例化组件
pub fn enable_instancing_for_similar_entities(
    mut commands: Commands,
    query: Query<(Entity, &Mesh3d, &MeshMaterial3d<StandardMaterial>), Without<Instanced>>,
) {
    // 统计每种类型的数量
    let mut type_counts: HashMap<InstanceKey, Vec<Entity>> = HashMap::new();

    for (entity, mesh, material) in query.iter() {
        let key = InstanceKey::from_components(mesh, material);
        type_counts.entry(key).or_default().push(entity);
    }

    // 为数量 > 50 的组添加实例化标记
    for (_, entities) in type_counts {
        if entities.len() >= 50 {
            for entity in entities {
                commands.entity(entity).insert(Instanced);
            }
        }
    }
}

/// 实例化统计
#[derive(Resource, Default)]
pub struct InstancingStats {
    pub batch_count: usize,
    pub total_instances: usize,
    pub draw_call_reduction: usize,
}

/// 更新实例化统计
pub fn update_instancing_stats(batches: Res<InstanceBatches>, mut stats: ResMut<InstancingStats>) {
    stats.batch_count = batches.batches.len();
    stats.total_instances = batches.batches.values().map(|v| v.len()).sum();

    // 估算减少的 Draw Call 数量
    stats.draw_call_reduction = stats.total_instances.saturating_sub(stats.batch_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_key_hash() {
        let key1 = InstanceKey {
            mesh: Handle::weak_from_u128(1),
            material: Handle::weak_from_u128(2),
        };

        let key2 = InstanceKey {
            mesh: Handle::weak_from_u128(1),
            material: Handle::weak_from_u128(2),
        };

        assert_eq!(key1, key2);
    }
}
