//! 3D 打砖块游戏
//!
//! Nova Engine 示例 - 展示物理系统、碰撞检测、UI 和游戏状态管理

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use nova_engine::prelude::*;

// ============================================================================
// 游戏常量
// ============================================================================

/// 游戏区域宽度
const ARENA_WIDTH: f32 = 14.0;
/// 游戏区域高度
const ARENA_HEIGHT: f32 = 20.0;
/// 游戏区域深度
const ARENA_DEPTH: f32 = 2.0;
/// 墙壁厚度
const WALL_THICKNESS: f32 = 0.5;

/// 挡板宽度
const PADDLE_WIDTH: f32 = 3.0;
/// 挡板高度
const PADDLE_HEIGHT: f32 = 0.4;
/// 挡板深度
const PADDLE_DEPTH: f32 = 0.8;
/// 挡板移动速度
const PADDLE_SPEED: f32 = 15.0;
/// 挡板 Y 位置
const PADDLE_Y: f32 = -ARENA_HEIGHT / 2.0 + 2.0;

/// 球半径
const BALL_RADIUS: f32 = 0.3;
/// 球初始速度
const BALL_SPEED: f32 = 12.0;

/// 砖块宽度
const BRICK_WIDTH: f32 = 1.8;
/// 砖块高度
const BRICK_HEIGHT: f32 = 0.6;
/// 砖块深度
const BRICK_DEPTH: f32 = 0.8;
/// 砖块行数
const BRICK_ROWS: usize = 5;
/// 砖块列数
const BRICK_COLS: usize = 7;
/// 砖块起始 Y 位置
const BRICK_START_Y: f32 = ARENA_HEIGHT / 2.0 - 5.0;

// ============================================================================
// 游戏状态
// ============================================================================

/// 游戏状态枚举
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// 准备开始
    #[default]
    Ready,
    /// 游戏进行中
    Playing,
    /// 游戏结束
    GameOver,
    /// 胜利
    Victory,
}

/// 游戏数据资源
#[derive(Resource, Default)]
pub struct GameData {
    /// 当前分数
    pub score: u32,
    /// 剩余生命
    pub lives: u32,
    /// 剩余砖块数
    pub bricks_remaining: u32,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            bricks_remaining: (BRICK_ROWS * BRICK_COLS) as u32,
        }
    }
}

// ============================================================================
// 组件定义
// ============================================================================

/// 挡板组件
#[derive(Component)]
pub struct Paddle;

/// 球组件
#[derive(Component)]
pub struct Ball {
    /// 是否已发射
    pub launched: bool,
}

/// 砖块组件
#[derive(Component)]
pub struct Brick {
    /// 砖块分值
    pub points: u32,
}

/// 墙壁组件
#[derive(Component)]
pub struct Wall;

/// 底部边界（死亡区域）
#[derive(Component)]
pub struct DeathZone;

// ============================================================================
// 游戏入口
// ============================================================================

fn main() {
    NovaApp::new()
        .with_title("Nova Engine - 3D 打砖块")
        .add_plugin(NovaPhysicsPlugin)
        .add_plugin(NovaCollisionEventsPlugin)
        .add_plugin(NovaUiPlugin)
        .add_plugin(NovaInputPlugin)
        .add_plugin(BreakoutPlugin)
        .run();
}

/// 打砖块游戏插件
pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(GameData::new())
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (
                    paddle_movement,
                    launch_ball,
                    ball_velocity_fix,
                    handle_collisions,
                    check_death_zone,
                    check_victory,
                    game_ui,
                    restart_game,
                ),
            );
    }
}

// ============================================================================
// 场景设置
// ============================================================================

/// 初始化游戏场景
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光照
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // 环境光
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });

    // 创建墙壁
    spawn_walls(&mut commands, &mut meshes, &mut materials);

    // 创建挡板
    spawn_paddle(&mut commands, &mut meshes, &mut materials);

    // 创建球
    spawn_ball(&mut commands, &mut meshes, &mut materials);

    // 创建砖块
    spawn_bricks(&mut commands, &mut meshes, &mut materials);

    // 创建死亡区域
    spawn_death_zone(&mut commands);
}

/// 创建墙壁
fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.4),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });

    // 左墙
    commands.spawn((
        Wall,
        Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, ARENA_HEIGHT, ARENA_DEPTH))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(-ARENA_WIDTH / 2.0 - WALL_THICKNESS / 2.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, ARENA_HEIGHT / 2.0, ARENA_DEPTH / 2.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // 右墙
    commands.spawn((
        Wall,
        Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, ARENA_HEIGHT, ARENA_DEPTH))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(ARENA_WIDTH / 2.0 + WALL_THICKNESS / 2.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, ARENA_HEIGHT / 2.0, ARENA_DEPTH / 2.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // 顶墙
    commands.spawn((
        Wall,
        Mesh3d(meshes.add(Cuboid::new(
            ARENA_WIDTH + WALL_THICKNESS * 2.0,
            WALL_THICKNESS,
            ARENA_DEPTH,
        ))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(0.0, ARENA_HEIGHT / 2.0 + WALL_THICKNESS / 2.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(
            (ARENA_WIDTH + WALL_THICKNESS * 2.0) / 2.0,
            WALL_THICKNESS / 2.0,
            ARENA_DEPTH / 2.0,
        ),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // 背景墙
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            ARENA_WIDTH + WALL_THICKNESS * 2.0,
            ARENA_HEIGHT + WALL_THICKNESS * 2.0,
            0.1,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.05, 0.1),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -ARENA_DEPTH / 2.0 - 0.1),
    ));
}

/// 创建挡板
fn spawn_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let paddle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 1.0),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });

    commands.spawn((
        Paddle,
        Mesh3d(meshes.add(Cuboid::new(PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_DEPTH))),
        MeshMaterial3d(paddle_material),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0, PADDLE_DEPTH / 2.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}

/// 创建球
fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let ball_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.2),
        emissive: LinearRgba::new(1.0, 0.8, 0.0, 1.0),
        ..default()
    });

    commands.spawn((
        Ball { launched: false },
        Mesh3d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial3d(ball_material),
        Transform::from_xyz(0.0, PADDLE_Y + PADDLE_HEIGHT / 2.0 + BALL_RADIUS + 0.1, 0.0),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity::zero(),
        GravityScale(0.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::TRANSLATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED,
    ));
}

/// 创建砖块
fn spawn_bricks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let brick_mesh = meshes.add(Cuboid::new(BRICK_WIDTH, BRICK_HEIGHT, BRICK_DEPTH));

    // 砖块颜色 - 从上到下不同颜色代表不同分值
    let brick_colors = [
        Color::srgb(1.0, 0.2, 0.2), // 红色 - 50分
        Color::srgb(1.0, 0.5, 0.2), // 橙色 - 40分
        Color::srgb(1.0, 1.0, 0.2), // 黄色 - 30分
        Color::srgb(0.2, 1.0, 0.2), // 绿色 - 20分
        Color::srgb(0.2, 0.5, 1.0), // 蓝色 - 10分
    ];

    let points_per_row = [50, 40, 30, 20, 10];

    let total_width = BRICK_COLS as f32 * (BRICK_WIDTH + 0.2) - 0.2;
    let start_x = -total_width / 2.0 + BRICK_WIDTH / 2.0;

    for row in 0..BRICK_ROWS {
        let y = BRICK_START_Y - row as f32 * (BRICK_HEIGHT + 0.3);
        let color = brick_colors[row % brick_colors.len()];
        let points = points_per_row[row % points_per_row.len()];

        let brick_material = materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.3,
            perceptual_roughness: 0.5,
            ..default()
        });

        for col in 0..BRICK_COLS {
            let x = start_x + col as f32 * (BRICK_WIDTH + 0.2);

            commands.spawn((
                Brick { points },
                Mesh3d(brick_mesh.clone()),
                MeshMaterial3d(brick_material.clone()),
                Transform::from_xyz(x, y, 0.0),
                RigidBody::Fixed,
                Collider::cuboid(BRICK_WIDTH / 2.0, BRICK_HEIGHT / 2.0, BRICK_DEPTH / 2.0),
                Restitution::coefficient(1.0),
                Friction::coefficient(0.0),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}

/// 创建死亡区域（底部）
fn spawn_death_zone(commands: &mut Commands) {
    commands.spawn((
        DeathZone,
        Transform::from_xyz(0.0, -ARENA_HEIGHT / 2.0 - 1.0, 0.0),
        Collider::cuboid(ARENA_WIDTH / 2.0, 0.5, ARENA_DEPTH / 2.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

// ============================================================================
// 游戏系统
// ============================================================================

/// 挡板移动
fn paddle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::GameOver || *game_state.get() == GameState::Victory {
        return;
    }

    let Ok(mut transform) = paddle_query.get_single_mut() else {
        return;
    };

    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_x = transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    let max_x = ARENA_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    transform.translation.x = new_x.clamp(-max_x, max_x);
}

/// 发射球
fn launch_ball(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ball_query: Query<(&mut Ball, &mut Velocity, &Transform)>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *game_state.get() != GameState::Ready && *game_state.get() != GameState::Playing {
        return;
    }

    let Ok((mut ball, mut velocity, ball_transform)) = ball_query.get_single_mut() else {
        return;
    };

    // 如果球未发射，让球跟随挡板
    if !ball.launched {
        if let Ok(paddle_transform) = paddle_query.get_single() {
            // 球跟随挡板（通过速度来模拟，因为我们用的是动态刚体）
            let target_x = paddle_transform.translation.x;
            let current_x = ball_transform.translation.x;
            velocity.linvel.x = (target_x - current_x) * 20.0;
        }

        // 按空格发射
        if keyboard.just_pressed(KeyCode::Space) {
            ball.launched = true;
            // 给球一个初始速度（稍微向上偏斜）
            let angle = std::f32::consts::PI / 4.0 + (std::f32::consts::PI / 6.0) * rand_f32();
            velocity.linvel = Vec3::new(angle.cos() * BALL_SPEED, angle.sin() * BALL_SPEED, 0.0);

            if *game_state.get() == GameState::Ready {
                next_state.set(GameState::Playing);
            }
        }
    }
}

/// 简单的伪随机数生成（用于球发射角度）
/// 使用静态计数器，WASM 兼容
fn rand_f32() -> f32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(12345);
    let val = COUNTER.fetch_add(1, Ordering::Relaxed);
    // 简单的 LCG 随机数
    let next = val.wrapping_mul(1103515245).wrapping_add(12345);
    ((next % 1000) as f32 / 1000.0) - 0.5
}

/// 保持球速度恒定
fn ball_velocity_fix(mut ball_query: Query<(&Ball, &mut Velocity)>) {
    for (ball, mut velocity) in ball_query.iter_mut() {
        if !ball.launched {
            continue;
        }

        let current_speed = velocity.linvel.length();
        if current_speed > 0.1 && current_speed != BALL_SPEED {
            velocity.linvel = velocity.linvel.normalize() * BALL_SPEED;
        }

        // 确保 Z 方向速度为 0
        velocity.linvel.z = 0.0;

        // 防止球水平移动（角度太小）
        if velocity.linvel.y.abs() < 2.0 && velocity.linvel.x.abs() > 0.1 {
            velocity.linvel.y = if velocity.linvel.y >= 0.0 { 2.0 } else { -2.0 };
            velocity.linvel = velocity.linvel.normalize() * BALL_SPEED;
        }
    }
}

/// 处理碰撞事件
fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    brick_query: Query<(Entity, &Brick)>,
    mut game_data: ResMut<GameData>,
) {
    let Ok(ball_entity) = ball_query.get_single() else {
        return;
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // 检查是否是球和砖块的碰撞
            let (ball_e, other_e) = if *e1 == ball_entity {
                (*e1, *e2)
            } else if *e2 == ball_entity {
                (*e2, *e1)
            } else {
                continue;
            };

            // 如果碰撞的是砖块
            if let Ok((brick_entity, brick)) = brick_query.get(other_e) {
                if ball_e == ball_entity {
                    // 增加分数
                    game_data.score += brick.points;
                    game_data.bricks_remaining = game_data.bricks_remaining.saturating_sub(1);

                    // 销毁砖块
                    commands.entity(brick_entity).despawn_recursive();
                }
            }
        }
    }
}

/// 检查死亡区域
fn check_death_zone(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    death_zone_query: Query<Entity, With<DeathZone>>,
    mut game_data: ResMut<GameData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *game_state.get() != GameState::Playing {
        return;
    }

    let Ok(ball_entity) = ball_query.get_single() else {
        return;
    };

    let Ok(death_zone_entity) = death_zone_query.get_single() else {
        return;
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let hit_death_zone = (*e1 == ball_entity && *e2 == death_zone_entity)
                || (*e2 == ball_entity && *e1 == death_zone_entity);

            if hit_death_zone {
                game_data.lives = game_data.lives.saturating_sub(1);

                // 删除旧球
                commands.entity(ball_entity).despawn_recursive();

                if game_data.lives == 0 {
                    next_state.set(GameState::GameOver);
                } else {
                    // 重新生成球
                    spawn_ball(&mut commands, &mut meshes, &mut materials);
                }
            }
        }
    }
}

/// 检查胜利条件
fn check_victory(
    game_data: Res<GameData>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *game_state.get() == GameState::Playing && game_data.bricks_remaining == 0 {
        next_state.set(GameState::Victory);
    }
}

/// 重新开始游戏
fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ball_query: Query<Entity, With<Ball>>,
    brick_query: Query<Entity, With<Brick>>,
    paddle_query: Query<Entity, With<Paddle>>,
) {
    if *game_state.get() != GameState::GameOver && *game_state.get() != GameState::Victory {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        // 重置游戏数据
        *game_data = GameData::new();

        // 删除旧实体
        for entity in ball_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in brick_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in paddle_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // 重新生成
        spawn_paddle(&mut commands, &mut meshes, &mut materials);
        spawn_ball(&mut commands, &mut meshes, &mut materials);
        spawn_bricks(&mut commands, &mut meshes, &mut materials);

        next_state.set(GameState::Ready);
    }
}

// ============================================================================
// UI 系统
// ============================================================================

/// 游戏 UI
fn game_ui(
    mut contexts: EguiContexts,
    game_data: Res<GameData>,
    game_state: Res<State<GameState>>,
) {
    // 分数和生命显示
    egui::Window::new("游戏状态")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("分数: {}", game_data.score))
                        .size(24.0)
                        .color(egui::Color32::YELLOW),
                );
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new(format!("生命: {}", game_data.lives))
                        .size(24.0)
                        .color(egui::Color32::RED),
                );
            });
        });

    // 游戏状态提示
    match game_state.get() {
        GameState::Ready => {
            egui::Window::new("开始提示")
                .title_bar(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 50.0))
                .show(contexts.ctx_mut(), |ui| {
                    ui.label(
                        egui::RichText::new("按 空格键 发射球")
                            .size(28.0)
                            .color(egui::Color32::WHITE),
                    );
                    ui.label(
                        egui::RichText::new("A/D 或 ←/→ 移动挡板")
                            .size(18.0)
                            .color(egui::Color32::GRAY),
                    );
                });
        }
        GameState::GameOver => {
            egui::Window::new("游戏结束")
                .title_bar(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("游戏结束")
                                .size(48.0)
                                .color(egui::Color32::RED),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!("最终分数: {}", game_data.score))
                                .size(32.0)
                                .color(egui::Color32::YELLOW),
                        );
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("按 R 重新开始")
                                .size(24.0)
                                .color(egui::Color32::WHITE),
                        );
                    });
                });
        }
        GameState::Victory => {
            egui::Window::new("胜利")
                .title_bar(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("恭喜通关!")
                                .size(48.0)
                                .color(egui::Color32::GREEN),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!("最终分数: {}", game_data.score))
                                .size(32.0)
                                .color(egui::Color32::YELLOW),
                        );
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("按 R 重新开始")
                                .size(24.0)
                                .color(egui::Color32::WHITE),
                        );
                    });
                });
        }
        _ => {}
    }
}
