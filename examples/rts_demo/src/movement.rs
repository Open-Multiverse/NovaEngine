//! 移动系统

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use nova_map::prelude::*;

use crate::components::*;
use crate::selection::screen_to_ground;

/// 移动系统插件
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_command_system, path_follow_system));
    }
}

/// 移动指令系统
fn move_command_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    tilemap: Option<Res<TileMap>>,
    selected_units: Query<(Entity, &Transform), (With<Selected>, With<Movement>)>,
    enemies: Query<(Entity, &Transform, &Team)>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Some(world_pos) = screen_to_ground(cursor_pos, camera, camera_transform) else {
        return;
    };

    // 检查是否点击了敌人
    let mut clicked_enemy = None;
    for (entity, transform, team) in enemies.iter() {
        if *team == Team::Enemy {
            let distance = (transform.translation - world_pos).length();
            if distance < 1.0 {
                clicked_enemy = Some(entity);
                break;
            }
        }
    }

    // 如果点击敌人，设置攻击目标
    if let Some(enemy_entity) = clicked_enemy {
        for (entity, _) in selected_units.iter() {
            commands
                .entity(entity)
                .remove::<PathFollow>()
                .insert(AttackTarget(enemy_entity));
        }
        return;
    }

    // 否则移动到目标位置
    let Some(goal_tile) = tilemap.world_to_tile(world_pos) else {
        return;
    };

    for (entity, transform) in selected_units.iter() {
        let Some(start_tile) = tilemap.world_to_tile(transform.translation) else {
            continue;
        };

        if let Some(result) = Pathfinder::find_path(&tilemap, start_tile, goal_tile) {
            commands
                .entity(entity)
                .remove::<AttackTarget>()
                .insert(PathFollow::new(result.path));
        }
    }
}

/// 路径跟随系统
fn path_follow_system(
    time: Res<Time>,
    tilemap: Option<Res<TileMap>>,
    mut units: Query<(Entity, &mut Transform, &Movement, &mut PathFollow)>,
    mut commands: Commands,
) {
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, mut transform, movement, mut path) in units.iter_mut() {
        if path.finished {
            commands.entity(entity).remove::<PathFollow>();
            continue;
        }

        let Some(target_tile) = path.current_target() else {
            path.finished = true;
            continue;
        };

        let target_pos = tilemap.tile_to_world(target_tile.0, target_tile.1);
        let direction = target_pos - transform.translation;
        let distance = direction.length();

        if distance < 0.3 {
            // 到达当前目标点
            path.advance();
        } else {
            // 移动向目标
            let move_dir = direction.normalize();
            let move_amount = movement.speed * time.delta_secs();
            transform.translation += move_dir * move_amount.min(distance);

            // 朝向移动方向
            if direction.x.abs() > 0.01 || direction.z.abs() > 0.01 {
                let target_rotation = Quat::from_rotation_y((-direction.x).atan2(-direction.z));
                transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
            }
        }
    }
}
