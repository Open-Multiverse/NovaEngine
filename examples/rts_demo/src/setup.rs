//! 场景初始化

use bevy::prelude::*;
use nova_map::prelude::*;

use crate::components::*;

/// 初始化游戏场景
pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 生成地图
    let config = MapGeneratorConfig {
        seed: 42,
        size: (64, 64),
        tile_size: 1.0,
        ..default()
    };
    let tilemap = MapGenerator::generate(&config);

    // 创建迷雾
    let fog = FogOfWar::new(tilemap.width(), tilemap.height());

    // 渲染地图
    spawn_terrain(&mut commands, &mut meshes, &mut materials, &tilemap);

    // 插入资源
    let map_width = tilemap.width() as f32 * tilemap.tile_size();
    let map_height = tilemap.height() as f32 * tilemap.tile_size();
    commands.insert_resource(tilemap);
    commands.insert_resource(fog);

    // 创建相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        RtsCameraController::default().with_map_bounds(map_width, map_height, 5.0),
    ));

    // 光照（禁用阴影以提升性能）
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });

    // 生成玩家单位
    spawn_player_units(&mut commands, &mut meshes, &mut materials);

    // 生成敌方单位
    spawn_enemy_units(&mut commands, &mut meshes, &mut materials);
}

/// 渲染地形
fn spawn_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tilemap: &TileMap,
) {
    let tile_mesh = meshes.add(Cuboid::new(
        tilemap.tile_size() * 0.95,
        0.2,
        tilemap.tile_size() * 0.95,
    ));

    // 预创建每种地形类型的材质（优化：避免每格子创建独立材质）
    use nova_map::prelude::TerrainType;
    use std::collections::HashMap;

    let terrain_materials: HashMap<TerrainType, Handle<StandardMaterial>> = [
        TerrainType::Grass,
        TerrainType::Desert,
        TerrainType::Water,
        TerrainType::Mountain,
        TerrainType::Forest,
    ]
    .into_iter()
    .map(|terrain| {
        let material = materials.add(StandardMaterial {
            base_color: terrain.color(),
            perceptual_roughness: 0.9,
            ..default()
        });
        (terrain, material)
    })
    .collect();

    for (x, y, tile) in tilemap.iter() {
        let world_pos = tilemap.tile_to_world(x, y);
        let material = terrain_materials
            .get(&tile.terrain)
            .cloned()
            .unwrap_or_else(|| terrain_materials[&TerrainType::Grass].clone());

        commands.spawn((
            Mesh3d(tile_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos - Vec3::Y * 0.1),
        ));
    }
}

/// 生成玩家单位
fn spawn_player_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let unit_mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let unit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 1.0),
        ..default()
    });

    // 在左下角生成 3 个单位
    let positions = [
        Vec3::new(-25.0, 0.5, -25.0),
        Vec3::new(-23.0, 0.5, -25.0),
        Vec3::new(-24.0, 0.5, -23.0),
    ];

    for pos in positions {
        commands.spawn((
            Unit,
            Team::Player,
            Health::new(100.0),
            Attack::new(10.0, 5.0, 1.0),
            Movement::new(5.0),
            Selectable,
            Vision::new(8),
            Mesh3d(unit_mesh.clone()),
            MeshMaterial3d(unit_material.clone()),
            Transform::from_translation(pos),
        ));
    }
}

/// 生成敌方单位
fn spawn_enemy_units(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let unit_mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let unit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.3, 0.2),
        ..default()
    });

    // 在右上角生成 3 个敌人
    let positions = [
        Vec3::new(25.0, 0.5, 25.0),
        Vec3::new(23.0, 0.5, 25.0),
        Vec3::new(24.0, 0.5, 23.0),
    ];

    for pos in positions {
        commands.spawn((
            Unit,
            Team::Enemy,
            Health::new(100.0),
            Attack::new(10.0, 5.0, 1.0),
            Movement::new(4.0),
            Vision::new(8),
            Mesh3d(unit_mesh.clone()),
            MeshMaterial3d(unit_material.clone()),
            Transform::from_translation(pos),
        ));
    }
}
