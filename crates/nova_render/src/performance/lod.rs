//! LOD (Level of Detail) 系统
//!
//! 根据距离切换不同精度的模型

use bevy::prelude::*;

/// LOD 级别配置
#[derive(Clone)]
pub struct LodLevel {
    /// 距离阈值（小于此距离使用此级别）
    pub distance: f32,
    /// 网格句柄
    pub mesh: Handle<Mesh>,
    /// 材质句柄（可选，None 则使用原材质）
    pub material: Option<Handle<StandardMaterial>>,
}

/// LOD 组件
#[derive(Component)]
pub struct Lod {
    pub levels: Vec<LodLevel>,
    pub current_level: usize,
}

impl Lod {
    pub fn new(levels: Vec<LodLevel>) -> Self {
        assert!(!levels.is_empty(), "LOD must have at least one level");

        Self {
            levels,
            current_level: 0,
        }
    }

    /// 根据距离选择最佳 LOD 级别
    pub fn select_level(&self, distance: f32) -> usize {
        for (i, level) in self.levels.iter().enumerate() {
            if distance < level.distance {
                return i;
            }
        }
        // 使用最远级别
        self.levels.len() - 1
    }
}

/// LOD 目标（需要更新的 Mesh 组件）
#[derive(Component)]
pub struct LodTarget;

/// 更新 LOD 系统
pub fn update_lod(
    mut lod_query: Query<(Entity, &GlobalTransform, &mut Lod)>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut mesh_query: Query<&mut Mesh3d>,
    mut material_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    settings: Res<super::PerformanceSettings>,
) {
    if !settings.enable_lod {
        return;
    }

    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_pos = camera_transform.translation();

    for (entity, transform, mut lod) in lod_query.iter_mut() {
        let distance = transform.translation().distance(camera_pos);
        let new_level = lod.select_level(distance);

        // 只在级别变化时更新
        if new_level != lod.current_level {
            lod.current_level = new_level;

            // 更新网格
            if let Ok(mut mesh) = mesh_query.get_mut(entity) {
                mesh.0 = lod.levels[new_level].mesh.clone();
            }

            // 更新材质（如果有）
            if let Some(ref material) = lod.levels[new_level].material {
                if let Ok(mut mat) = material_query.get_mut(entity) {
                    mat.0 = material.clone();
                }
            }
        }
    }
}

/// 构建器模式创建 LOD
pub struct LodBuilder {
    levels: Vec<LodLevel>,
}

impl LodBuilder {
    pub fn new(base_mesh: Handle<Mesh>) -> Self {
        Self {
            levels: vec![LodLevel {
                distance: f32::MAX,
                mesh: base_mesh,
                material: None,
            }],
        }
    }

    pub fn with_level(mut self, distance: f32, mesh: Handle<Mesh>) -> Self {
        // 插入到正确位置（按距离排序）
        let level = LodLevel {
            distance,
            mesh,
            material: None,
        };

        let pos = self
            .levels
            .iter()
            .position(|l| l.distance > distance)
            .unwrap_or(self.levels.len());

        self.levels.insert(pos, level);

        // 确保最后一个有无限距离
        if let Some(last) = self.levels.last_mut() {
            last.distance = f32::MAX;
        }

        self
    }

    pub fn with_material(mut self, level: usize, material: Handle<StandardMaterial>) -> Self {
        if let Some(l) = self.levels.get_mut(level) {
            l.material = Some(material);
        }
        self
    }

    pub fn build(self) -> Lod {
        Lod::new(self.levels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod_level_selection() {
        let lod = Lod::new(vec![
            LodLevel {
                distance: 10.0,
                mesh: Handle::weak_from_u128(1),
                material: None,
            },
            LodLevel {
                distance: 50.0,
                mesh: Handle::weak_from_u128(2),
                material: None,
            },
            LodLevel {
                distance: f32::MAX,
                mesh: Handle::weak_from_u128(3),
                material: None,
            },
        ]);

        // 近距离：使用最高精度
        assert_eq!(lod.select_level(5.0), 0);

        // 中等距离
        assert_eq!(lod.select_level(30.0), 1);

        // 远距离：使用最低精度
        assert_eq!(lod.select_level(100.0), 2);
    }

    #[test]
    fn test_lod_builder() {
        let lod = LodBuilder::new(Handle::weak_from_u128(1))
            .with_level(10.0, Handle::weak_from_u128(2))
            .with_level(50.0, Handle::weak_from_u128(3))
            .build();

        assert_eq!(lod.levels.len(), 3);
        assert_eq!(lod.levels[0].distance, 10.0);
        assert_eq!(lod.levels[1].distance, 50.0);
        assert_eq!(lod.levels[2].distance, f32::MAX);
    }
}
