//! 动画系统演示
//!
//! 展示补间动画和关键帧动画

use bevy::prelude::*;
use nova_engine::prelude::*;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - 动画演示")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaAnimationPlugin)
        .add_startup_system(setup)
        .run();
}

/// 场景初始化
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光源
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        })),
    ));

    // === 线性移动动画 ===
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(-4.0, 0.5, 0.0),
        PositionTween::new(Vec3::new(-4.0, 0.5, 0.0), Vec3::new(-4.0, 3.0, 0.0), 2.0)
            .with_ease(NovaEaseFunction::Linear)
            .with_loop(LoopMode::PingPong),
    ));

    // === 缓入动画 ===
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2),
            ..default()
        })),
        Transform::from_xyz(-1.5, 0.5, 0.0),
        PositionTween::new(Vec3::new(-1.5, 0.5, 0.0), Vec3::new(-1.5, 3.0, 0.0), 2.0)
            .with_ease(NovaEaseFunction::QuadIn)
            .with_loop(LoopMode::PingPong),
    ));

    // === 缓出动画 ===
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            ..default()
        })),
        Transform::from_xyz(1.0, 0.5, 0.0),
        PositionTween::new(Vec3::new(1.0, 0.5, 0.0), Vec3::new(1.0, 3.0, 0.0), 2.0)
            .with_ease(NovaEaseFunction::QuadOut)
            .with_loop(LoopMode::PingPong),
    ));

    // === 缓入缓出动画 ===
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.2),
            ..default()
        })),
        Transform::from_xyz(3.5, 0.5, 0.0),
        PositionTween::new(Vec3::new(3.5, 0.5, 0.0), Vec3::new(3.5, 3.0, 0.0), 2.0)
            .with_ease(NovaEaseFunction::QuadInOut)
            .with_loop(LoopMode::PingPong),
    ));

    // === 循环移动的球体 ===
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.4, 0.8),
            emissive: LinearRgba::new(0.5, 0.2, 0.5, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 2.0, 3.0),
        PositionTween::new(Vec3::new(-3.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 3.0), 3.0)
            .with_ease(NovaEaseFunction::CubicInOut)
            .with_loop(LoopMode::PingPong),
    ));
}
