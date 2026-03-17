//! UI 系统演示
//!
//! 展示 egui UI 组件和交互

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use nova_engine::prelude::*;

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - UI 演示")
        .with_window_size(1280.0, 720.0)
        .add_plugin(NovaUiPlugin)
        .add_startup_system(setup)
        .add_system(ui_demo_system)
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
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        })),
    ));

    // 可控制的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.8),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        ControllableCube,
    ));

    // 初始化 UI 状态
    commands.insert_resource(UiDemoState::default());
}

/// 可控制的立方体标记
#[derive(Component)]
struct ControllableCube;

/// UI 演示状态
#[derive(Resource)]
struct UiDemoState {
    cube_position: Vec3,
    cube_scale: f32,
    cube_color: [f32; 3],
    rotation_speed: f32,
    show_debug: bool,
    selected_tab: usize,
}

impl Default for UiDemoState {
    fn default() -> Self {
        Self {
            cube_position: Vec3::new(0.0, 0.5, 0.0),
            cube_scale: 1.0,
            cube_color: [0.5, 0.5, 0.8],
            rotation_speed: 0.0,
            show_debug: false,
            selected_tab: 0,
        }
    }
}

/// UI 演示系统
fn ui_demo_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiDemoState>,
    mut cube_query: Query<&mut Transform, With<ControllableCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_material_query: Query<&MeshMaterial3d<StandardMaterial>, With<ControllableCube>>,
    time: Res<Time>,
) {
    // 主控制面板
    egui::Window::new("🎮 控制面板")
        .default_pos([10.0, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            // 选项卡
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(state.selected_tab == 0, "变换")
                    .clicked()
                {
                    state.selected_tab = 0;
                }
                if ui
                    .selectable_label(state.selected_tab == 1, "外观")
                    .clicked()
                {
                    state.selected_tab = 1;
                }
                if ui
                    .selectable_label(state.selected_tab == 2, "动画")
                    .clicked()
                {
                    state.selected_tab = 2;
                }
            });

            ui.separator();

            match state.selected_tab {
                0 => {
                    // 变换控制
                    ui.heading("位置");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::Slider::new(&mut state.cube_position.x, -5.0..=5.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Y:");
                        ui.add(egui::Slider::new(&mut state.cube_position.y, 0.5..=5.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Z:");
                        ui.add(egui::Slider::new(&mut state.cube_position.z, -5.0..=5.0));
                    });

                    ui.separator();

                    ui.heading("缩放");
                    ui.add(egui::Slider::new(&mut state.cube_scale, 0.5..=3.0));

                    if ui.button("重置位置").clicked() {
                        state.cube_position = Vec3::new(0.0, 0.5, 0.0);
                        state.cube_scale = 1.0;
                    }
                }
                1 => {
                    // 外观控制
                    ui.heading("颜色");
                    ui.horizontal(|ui| {
                        ui.label("R:");
                        ui.add(egui::Slider::new(&mut state.cube_color[0], 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("G:");
                        ui.add(egui::Slider::new(&mut state.cube_color[1], 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("B:");
                        ui.add(egui::Slider::new(&mut state.cube_color[2], 0.0..=1.0));
                    });

                    // 预设颜色
                    ui.separator();
                    ui.label("预设颜色:");
                    ui.horizontal(|ui| {
                        if ui.button("红色").clicked() {
                            state.cube_color = [0.8, 0.2, 0.2];
                        }
                        if ui.button("绿色").clicked() {
                            state.cube_color = [0.2, 0.8, 0.2];
                        }
                        if ui.button("蓝色").clicked() {
                            state.cube_color = [0.2, 0.2, 0.8];
                        }
                    });
                }
                2 => {
                    // 动画控制
                    ui.heading("旋转动画");
                    ui.add(egui::Slider::new(&mut state.rotation_speed, 0.0..=5.0).text("速度"));

                    if ui.button("停止旋转").clicked() {
                        state.rotation_speed = 0.0;
                    }
                }
                _ => {}
            }
        });

    // 调试信息窗口
    egui::Window::new("📊 调试信息")
        .default_pos([10.0, 400.0])
        .collapsible(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.checkbox(&mut state.show_debug, "显示详细信息");

            if state.show_debug {
                ui.separator();
                ui.label(format!("FPS: {:.1}", 1.0 / time.delta_secs()));
                ui.label(format!("时间: {:.2}s", time.elapsed_secs()));
                ui.label(format!(
                    "立方体位置: ({:.2}, {:.2}, {:.2})",
                    state.cube_position.x, state.cube_position.y, state.cube_position.z
                ));
            }
        });

    // 应用变换到立方体
    if let Ok(mut transform) = cube_query.get_single_mut() {
        transform.translation = state.cube_position;
        transform.scale = Vec3::splat(state.cube_scale);

        // 旋转动画
        if state.rotation_speed > 0.0 {
            transform.rotate_y(state.rotation_speed * time.delta_secs());
        }
    }

    // 应用颜色
    if let Ok(material_handle) = cube_material_query.get_single() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(
                state.cube_color[0],
                state.cube_color[1],
                state.cube_color[2],
            );
        }
    }
}
