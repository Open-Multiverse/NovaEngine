//! 单位选中系统

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use nova_map::prelude::*;

use crate::components::*;

/// 选择框资源
#[derive(Resource, Default)]
pub struct SelectionBox {
    pub active: bool,
    pub start: Vec2,
    pub end: Vec2,
}

/// 选中系统插件
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionBox>()
            .add_systems(Update, (selection_system, render_selection_indicators));
    }
}

/// 主选中系统
fn selection_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<RtsCameraController>>,
    mut selection_box: ResMut<SelectionBox>,
    selectables: Query<(Entity, &Transform, &Team), With<Selectable>>,
    selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // 左键按下：开始框选
    if mouse_button.just_pressed(MouseButton::Left) {
        selection_box.active = true;
        selection_box.start = cursor_pos;
        selection_box.end = cursor_pos;
    }

    // 左键拖拽：更新框选
    if mouse_button.pressed(MouseButton::Left) && selection_box.active {
        selection_box.end = cursor_pos;
    }

    // 左键释放：完成选择
    if mouse_button.just_released(MouseButton::Left) && selection_box.active {
        selection_box.active = false;

        // 清除之前的选中
        for entity in selected.iter() {
            commands.entity(entity).remove::<Selected>();
        }

        let drag_distance = (selection_box.end - selection_box.start).length();

        if drag_distance < 5.0 {
            // 点击选择：选中点击位置的单位
            if let Some(world_pos) = screen_to_ground(cursor_pos, camera, camera_transform) {
                for (entity, transform, team) in selectables.iter() {
                    if *team != Team::Player {
                        continue;
                    }
                    let distance = (transform.translation - world_pos).length();
                    if distance < 1.0 {
                        commands.entity(entity).insert(Selected);
                        break;
                    }
                }
            }
        } else {
            // 框选：选中框内所有己方单位
            let min_x = selection_box.start.x.min(selection_box.end.x);
            let max_x = selection_box.start.x.max(selection_box.end.x);
            let min_y = selection_box.start.y.min(selection_box.end.y);
            let max_y = selection_box.start.y.max(selection_box.end.y);

            for (entity, transform, team) in selectables.iter() {
                if *team != Team::Player {
                    continue;
                }

                if let Ok(screen_pos) =
                    camera.world_to_viewport(camera_transform, transform.translation)
                {
                    if screen_pos.x >= min_x
                        && screen_pos.x <= max_x
                        && screen_pos.y >= min_y
                        && screen_pos.y <= max_y
                    {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
        }
    }
}

/// 屏幕坐标转世界地面坐标
pub fn screen_to_ground(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let ray = camera.viewport_to_world(camera_transform, screen_pos).ok()?;

    // 与 Y=0 平面求交
    let t = -ray.origin.y / ray.direction.y;
    if t > 0.0 {
        Some(ray.origin + ray.direction * t)
    } else {
        None
    }
}

/// 渲染选中指示器
fn render_selection_indicators(selected: Query<&Transform, With<Selected>>, mut gizmos: Gizmos) {
    for transform in selected.iter() {
        let pos = transform.translation;
        gizmos.circle(
            Isometry3d::new(
                pos + Vec3::Y * 0.1,
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
            0.6,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}
