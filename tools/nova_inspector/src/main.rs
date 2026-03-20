//! Nova Inspector CLI 入口

use bevy::prelude::*;
use nova_inspector::NovaInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NovaInspectorPlugin)
        .add_systems(Startup, setup_demo_scene)
        .run();
}

fn setup_demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 创建相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 创建光源
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 4.0,
            std::f32::consts::PI / 4.0,
            0.0,
        )),
    ));

    // 创建立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 创建地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));

    info!("Nova Inspector started. Press F12 to toggle inspector.");
}
