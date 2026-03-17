//! 物理系统演示
//!
//! 展示刚体、碰撞器和物理交互

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use nova_engine::prelude::*;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - 物理演示")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaPhysicsPlugin)
        .add_startup_system(setup)
        .add_system(spawn_falling_objects)
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
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光源
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // 地面（静态刚体）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.5, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.25, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.25, 10.0),
    ));

    // 初始堆叠的立方体
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        ..default()
    });

    for y in 0..5 {
        for x in -2..=2 {
            commands.spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(cube_material.clone()),
                Transform::from_xyz(x as f32 * 1.1, 1.0 + y as f32 * 1.1, 0.0),
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5, 0.5),
                Restitution::coefficient(0.3),
            ));
        }
    }

    // 斜坡
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.5, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.7),
            ..default()
        })),
        Transform::from_xyz(-6.0, 2.0, 0.0).with_rotation(Quat::from_rotation_z(0.3)),
        RigidBody::Fixed,
        Collider::cuboid(4.0, 0.25, 2.0),
    ));

    // 初始化计时器
    commands.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

/// 定时生成下落物体的资源
#[derive(Resource)]
struct SpawnTimer(Timer);

/// 生成下落的球体
fn spawn_falling_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        // 随机位置和颜色
        let x = (time.elapsed_secs() * 1.7).sin() * 5.0;
        let z = (time.elapsed_secs() * 2.3).cos() * 5.0;
        let hue = (time.elapsed_secs() * 0.5) % 1.0;

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::hsl(hue * 360.0, 0.8, 0.5),
                ..default()
            })),
            Transform::from_xyz(x, 15.0, z),
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
        ));
    }
}
