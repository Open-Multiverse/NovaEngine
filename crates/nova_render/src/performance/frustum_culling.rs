//! 视锥剔除系统
//!
//! 只渲染在相机视锥体内的物体

use bevy::prelude::*;

/// 视锥剔除标记
#[derive(Component)]
pub struct FrustumCulled {
    pub is_visible: bool,
    pub last_check: f32,
}

impl Default for FrustumCulled {
    fn default() -> Self {
        Self {
            is_visible: true,
            last_check: 0.0,
        }
    }
}

/// AABB 包围盒
#[derive(Component, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn from_transform_and_scale(transform: &Transform) -> Self {
        let scale = transform.scale;
        let pos = transform.translation;

        // 假设物体中心在原点，尺寸为 1x1x1
        let half_size = scale * 0.5;

        Self {
            min: pos - half_size,
            max: pos + half_size,
        }
    }

    pub fn intersects_frustum_distance(&self, transform: &GlobalTransform, camera_pos: Vec3) -> bool {
        // 简化版本：只测试距离
        let center = (self.min + self.max) * 0.5;
        let world_center = transform.transform_point(center);
        let distance = world_center.distance(camera_pos);

        // 假设视锥远裁剪面为 1000
        distance < 1000.0
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            min: Vec3::splat(-0.5),
            max: Vec3::splat(0.5),
        }
    }
}

/// 更新视锥剔除
pub fn update_frustum_culling(
    mut query: Query<(
        Entity,
        &GlobalTransform,
        &mut FrustumCulled,
        &Aabb,
        Option<&mut Visibility>,
    )>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    settings: Res<super::PerformanceSettings>,
) {
    if !settings.enable_frustum_culling {
        // 禁用剔除时，所有物体可见
        for (_, _, _, _, visibility) in query.iter_mut() {
            if let Some(mut vis) = visibility {
                *vis = Visibility::Visible;
            }
        }
        return;
    }

    // 获取主相机位置
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    // 简单的距离剔除（简化版视锥剔除）
    for (_, transform, mut culled, aabb, visibility) in query.iter_mut() {
        let world_pos = transform.translation();
        let camera_pos = camera_transform.translation();
        let distance = world_pos.distance(camera_pos);

        // 距离裁剪：超过 500 单位不可见
        // 实际项目中应使用真实的视锥测试
        let is_visible = distance < 500.0;

        culled.is_visible = is_visible;

        // 更新 Visibility 组件
        if let Some(mut vis) = visibility {
            *vis = if is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// 为所有网格实体添加视锥剔除组件
pub fn add_frustum_culling_to_meshes(
    mut commands: Commands,
    query: Query<Entity, (With<Mesh3d>, Without<FrustumCulled>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((FrustumCulled::default(), Aabb::default()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_creation() {
        let transform = Transform::from_xyz(10.0, 0.0, 0.0).with_scale(Vec3::splat(2.0));

        let aabb = Aabb::from_transform_and_scale(&transform);

        assert_eq!(aabb.min, Vec3::new(9.0, -1.0, -1.0));
        assert_eq!(aabb.max, Vec3::new(11.0, 1.0, 1.0));
    }
}
