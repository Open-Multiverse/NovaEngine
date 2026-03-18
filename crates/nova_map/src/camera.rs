//! RTS 相机控制器

use bevy::prelude::*;

/// RTS 相机控制器组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct RtsCameraController {
    /// 移动速度（世界单位/秒）
    pub move_speed: f32,
    /// 缩放速度
    pub zoom_speed: f32,
    /// 缩放范围（最小高度, 最大高度）
    pub zoom_min: f32,
    pub zoom_max: f32,
    /// 边缘滚动触发区域（像素）
    pub edge_margin: f32,
    /// 是否启用边缘滚动
    pub edge_scroll_enabled: bool,
    /// 相机移动边界
    pub bounds_min: Option<Vec2>,
    pub bounds_max: Option<Vec2>,
}

impl Default for RtsCameraController {
    fn default() -> Self {
        Self {
            move_speed: 20.0,
            zoom_speed: 10.0,
            zoom_min: 10.0,
            zoom_max: 50.0,
            edge_margin: 20.0,
            edge_scroll_enabled: true,
            bounds_min: None,
            bounds_max: None,
        }
    }
}

impl RtsCameraController {
    /// 设置边界
    pub fn with_bounds(mut self, min: Vec2, max: Vec2) -> Self {
        self.bounds_min = Some(min);
        self.bounds_max = Some(max);
        self
    }

    /// 根据地图尺寸设置边界
    pub fn with_map_bounds(self, width: f32, height: f32, margin: f32) -> Self {
        let half_w = width / 2.0 + margin;
        let half_h = height / 2.0 + margin;
        self.with_bounds(Vec2::new(-half_w, -half_h), Vec2::new(half_w, half_h))
    }
}

/// RTS 相机控制系统
pub fn rts_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &RtsCameraController)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    for (mut transform, controller) in cameras.iter_mut() {
        let mut move_dir = Vec3::ZERO;

        // 键盘移动
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            move_dir.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            move_dir.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            move_dir.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            move_dir.x += 1.0;
        }

        // 边缘滚动
        if controller.edge_scroll_enabled {
            if let Some(cursor_pos) = window.cursor_position() {
                let margin = controller.edge_margin;
                let width = window.width();
                let height = window.height();

                if cursor_pos.x < margin {
                    move_dir.x -= 1.0;
                }
                if cursor_pos.x > width - margin {
                    move_dir.x += 1.0;
                }
                if cursor_pos.y < margin {
                    move_dir.z -= 1.0;
                }
                if cursor_pos.y > height - margin {
                    move_dir.z += 1.0;
                }
            }
        }

        // 应用移动
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            transform.translation += move_dir * controller.move_speed * time.delta_secs();
        }

        // 应用边界限制
        if let (Some(min), Some(max)) = (controller.bounds_min, controller.bounds_max) {
            transform.translation.x = transform.translation.x.clamp(min.x, max.x);
            transform.translation.z = transform.translation.z.clamp(min.y, max.y);
        }
    }
}

/// RTS 相机缩放系统（需要 bevy 的 mouse wheel 事件）
pub fn rts_camera_zoom_system(
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    mut cameras: Query<(&mut Transform, &RtsCameraController)>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut scroll_delta = 0.0;
    for ev in scroll_evr.read() {
        scroll_delta += match ev.unit {
            MouseScrollUnit::Line => ev.y * 3.0,
            MouseScrollUnit::Pixel => ev.y * 0.1,
        };
    }

    if scroll_delta.abs() < 0.001 {
        return;
    }

    for (mut transform, controller) in cameras.iter_mut() {
        let current_y = transform.translation.y;
        let new_y = (current_y - scroll_delta * controller.zoom_speed)
            .clamp(controller.zoom_min, controller.zoom_max);
        transform.translation.y = new_y;
    }
}
