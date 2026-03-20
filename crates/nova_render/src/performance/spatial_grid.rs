//! 空间哈希网格
//!
//! 加速空间查询，用于碰撞检测、范围查询等

use bevy::prelude::*;
use std::collections::HashMap;

/// 空间网格坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl GridCoord {
    pub fn from_world(pos: Vec3, cell_size: f32) -> Self {
        Self {
            x: (pos.x / cell_size).floor() as i32,
            y: (pos.y / cell_size).floor() as i32,
            z: (pos.z / cell_size).floor() as i32,
        }
    }

    pub fn to_world_center(self, cell_size: f32) -> Vec3 {
        Vec3::new(
            (self.x as f32 + 0.5) * cell_size,
            (self.y as f32 + 0.5) * cell_size,
            (self.z as f32 + 0.5) * cell_size,
        )
    }
}

/// 空间网格
#[derive(Resource)]
pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<GridCoord, Vec<Entity>>,
    pub entity_cells: HashMap<Entity, GridCoord>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            entity_cells: HashMap::new(),
        }
    }

    /// 获取指定位置所在的格子
    pub fn get_cell(&self, pos: Vec3) -> GridCoord {
        GridCoord::from_world(pos, self.cell_size)
    }

    /// 获取指定格子的所有实体
    pub fn get_entities_in_cell(&self, coord: GridCoord) -> Option<&Vec<Entity>> {
        self.cells.get(&coord)
    }

    /// 获取位置周围的所有实体（包括相邻格子）
    pub fn get_nearby_entities(&self, pos: Vec3, radius: f32) -> Vec<Entity> {
        let center = self.get_cell(pos);
        let radius_in_cells = (radius / self.cell_size).ceil() as i32;

        let mut result = Vec::new();

        for dx in -radius_in_cells..=radius_in_cells {
            for dy in -radius_in_cells..=radius_in_cells {
                for dz in -radius_in_cells..=radius_in_cells {
                    let coord = GridCoord {
                        x: center.x + dx,
                        y: center.y + dy,
                        z: center.z + dz,
                    };

                    if let Some(entities) = self.cells.get(&coord) {
                        result.extend(entities);
                    }
                }
            }
        }

        result
    }

    /// 更新实体位置
    pub fn update_entity(&mut self, entity: Entity, pos: Vec3) {
        let new_coord = self.get_cell(pos);

        // 检查是否需要移动
        if let Some(&old_coord) = self.entity_cells.get(&entity) {
            if old_coord == new_coord {
                return; // 还在同一个格子
            }

            // 从旧格子移除
            if let Some(entities) = self.cells.get_mut(&old_coord) {
                entities.retain(|&e| e != entity);
            }
        }

        // 添加到新格子
        self.cells.entry(new_coord).or_default().push(entity);

        self.entity_cells.insert(entity, new_coord);
    }

    /// 移除实体
    pub fn remove_entity(&mut self, entity: Entity) {
        if let Some(coord) = self.entity_cells.remove(&entity) {
            if let Some(entities) = self.cells.get_mut(&coord) {
                entities.retain(|&e| e != entity);
            }
        }
    }
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new(50.0)
    }
}

/// 空间网格条目（添加到需要被追踪的实体）
#[derive(Component)]
pub struct SpatialGridEntry;

/// 更新空间网格系统
pub fn update_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &GlobalTransform), With<SpatialGridEntry>>,
    mut removed: RemovedComponents<SpatialGridEntry>,
    settings: Res<super::PerformanceSettings>,
) {
    if !settings.enable_spatial_grid {
        return;
    }

    // 更新现有实体位置
    for (entity, transform) in query.iter() {
        grid.update_entity(entity, transform.translation());
    }

    // 清理已移除的实体
    for entity in removed.read() {
        grid.remove_entity(entity);
    }
}

/// 为实体添加空间网格追踪
pub fn add_to_spatial_grid(
    mut commands: Commands,
    query: Query<Entity, (With<Transform>, Without<SpatialGridEntry>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpatialGridEntry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_coord_conversion() {
        let cell_size = 50.0;

        // 测试世界坐标到格子坐标
        let coord = GridCoord::from_world(Vec3::new(75.0, 25.0, 100.0), cell_size);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 0);
        assert_eq!(coord.z, 2);

        // 测试格子坐标到世界坐标
        let world_pos = coord.to_world_center(cell_size);
        assert_eq!(world_pos, Vec3::new(75.0, 25.0, 125.0));
    }

    #[test]
    fn test_spatial_grid_update() {
        let mut grid = SpatialGrid::new(50.0);
        let entity = Entity::from_raw(1);

        // 添加实体
        grid.update_entity(entity, Vec3::new(30.0, 0.0, 0.0));

        let coord = GridCoord { x: 0, y: 0, z: 0 };
        assert!(grid.cells.get(&coord).unwrap().contains(&entity));

        // 移动实体到新格子
        grid.update_entity(entity, Vec3::new(80.0, 0.0, 0.0));

        let old_coord = GridCoord { x: 0, y: 0, z: 0 };
        let new_coord = GridCoord { x: 1, y: 0, z: 0 };

        assert!(!grid.cells.get(&old_coord).unwrap().contains(&entity));
        assert!(grid.cells.get(&new_coord).unwrap().contains(&entity));
    }

    #[test]
    fn test_nearby_query() {
        let mut grid = SpatialGrid::new(50.0);

        // 添加三个实体
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        let e3 = Entity::from_raw(3);

        grid.update_entity(e1, Vec3::new(10.0, 0.0, 0.0));
        grid.update_entity(e2, Vec3::new(60.0, 0.0, 0.0)); // 相邻格子
        grid.update_entity(e3, Vec3::new(500.0, 0.0, 0.0)); // 很远

        // 查询附近实体（半径 100）
        let nearby = grid.get_nearby_entities(Vec3::new(0.0, 0.0, 0.0), 100.0);

        assert!(nearby.contains(&e1));
        assert!(nearby.contains(&e2));
        assert!(!nearby.contains(&e3));
    }
}
