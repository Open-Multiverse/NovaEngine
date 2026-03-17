//! Nova Engine 基础示例
//!
//! 展示一个简单的 3D 场景：
//! - 旋转的立方体
//! - 地面平面
//! - 基础光照

use bevy::prelude::*;
use nova_engine::prelude::*;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - 基础示例")
        .add_plugin(NovaRenderPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaAnimationPlugin)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

/// 场景设置
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn(
        NovaCamera3d::new()
            .with_position(Vec3::new(5.0, 5.0, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y)
            .bundle(),
    );

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 旋转立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));
}

/// 旋转立方体标记组件
#[derive(Component)]
struct RotatingCube;

/// 旋转立方体系统
fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}
