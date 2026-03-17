//! Nova Engine 基础示例
//!
//! 展示一个完整的 3D 场景：
//! - 旋转的立方体
//! - 物理下落的球体
//! - 地面平面
//! - 基础光照
//! - FPS 显示和调试 UI

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use nova_engine::prelude::*;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - 基础示例")
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(NovaRenderPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(NovaAnimationPlugin)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .add_system(spawn_falling_spheres)
        .add_system(ui_system)
        .run();
}

/// 演示状态资源
#[derive(Resource, Default)]
struct DemoState {
    /// 是否显示调试面板
    show_debug: bool,
    /// 球体生成计时器
    spawn_timer: f32,
    /// 已生成的球体数量
    sphere_count: u32,
}

/// 场景设置
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 初始化状态
    commands.insert_resource(DemoState::default());

    // 相机（带轨道控制器）
    commands.spawn((
        NovaCamera3d::new()
            .with_position(Vec3::new(8.0, 8.0, 8.0))
            .looking_at(Vec3::ZERO, Vec3::Y)
            .bundle(),
        OrbitCameraController::new()
            .with_target(Vec3::ZERO)
            .with_distance(12.0)
            .with_rotation(0.8, -0.5),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.4, 0.0)),
    ));

    // 环境光
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // 地面（带物理碰撞）
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(PredefinedMaterials::ground().build())),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
    ));

    // 旋转立方体（红色）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(PredefinedMaterials::red().build())),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));

    // 静态立方体（蓝色，带物理）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.5, 2.0))),
        MeshMaterial3d(materials.add(PredefinedMaterials::blue().build())),
        Transform::from_xyz(3.0, 0.25, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(1.0, 0.25, 1.0),
    ));

    // 金属立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(
            materials.add(PredefinedMaterials::metal(Color::srgb(0.8, 0.7, 0.2)).build()),
        ),
        Transform::from_xyz(-3.0, 0.5, 0.0),
    ));
}

/// 旋转立方体标记组件
#[derive(Component)]
struct RotatingCube;

/// 下落球体标记组件
#[derive(Component)]
struct FallingSphere;

/// 旋转立方体系统
fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.8);
    }
}

/// 定时生成下落的球体
fn spawn_falling_spheres(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<DemoState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spheres: Query<Entity, With<FallingSphere>>,
) {
    // 更新计时器
    state.spawn_timer += time.delta_secs();

    // 每 2 秒生成一个球体
    if state.spawn_timer >= 2.0 && state.sphere_count < 20 {
        state.spawn_timer = 0.0;
        state.sphere_count += 1;

        // 随机位置
        let x = (state.sphere_count as f32 * 1.7).sin() * 3.0;
        let z = (state.sphere_count as f32 * 2.3).cos() * 3.0;

        // 随机颜色
        let hue = (state.sphere_count as f32 * 0.15) % 1.0;
        let color = Color::hsl(hue * 360.0, 0.7, 0.5);

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.3).mesh().ico(3).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_xyz(x, 5.0, z),
            RigidBody::Dynamic,
            Collider::ball(0.3),
            FallingSphere,
        ));
    }

    // 清理掉落太远的球体
    for entity in spheres.iter() {
        // 这里简化处理，实际应该检查位置
        // 可以添加位置检测来移除掉落出界的球体
        let _ = entity;
    }
}

/// UI 系统
fn ui_system(
    mut contexts: EguiContexts,
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<DemoState>,
    spheres: Query<&Transform, With<FallingSphere>>,
) {
    let ctx = contexts.ctx_mut();

    // FPS 显示
    FpsDisplay::show_corner(ctx, &diagnostics);

    // 调试面板
    egui::Window::new("Nova Engine Demo")
        .default_pos(egui::pos2(10.0, 10.0))
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("调试信息");
            ui.separator();

            // 时间信息
            ui.label(format!("运行时间: {:.1}s", time.elapsed_secs()));
            ui.label(format!("帧时间: {:.2}ms", time.delta_secs() * 1000.0));

            // FPS
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    ui.label(format!("FPS: {:.0}", value));
                }
            }

            ui.separator();
            ui.heading("场景信息");

            // 球体数量
            ui.label(format!("球体数量: {}", spheres.iter().count()));
            ui.label(format!("已生成: {}/20", state.sphere_count));

            ui.separator();

            // 控制按钮
            if NovaButton::primary(ui, "重置球体").clicked() {
                state.sphere_count = 0;
            }

            ui.checkbox(&mut state.show_debug, "显示详细调试");

            if state.show_debug {
                ui.separator();
                ui.collapsing("球体位置", |ui| {
                    for (i, transform) in spheres.iter().enumerate() {
                        ui.label(format!(
                            "#{}: ({:.1}, {:.1}, {:.1})",
                            i,
                            transform.translation.x,
                            transform.translation.y,
                            transform.translation.z
                        ));
                    }
                });
            }
        });
}
