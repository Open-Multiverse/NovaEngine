//! 相机控制器
//!
//! 提供轨道相机控制，支持鼠标旋转、缩放、平移

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

/// 轨道相机控制器组件
#[derive(Component, Debug)]
pub struct OrbitCameraController {
    /// 目标点（相机围绕此点旋转）
    pub target: Vec3,
    /// 到目标点的距离
    pub distance: f32,
    /// 最小距离
    pub min_distance: f32,
    /// 最大距离
    pub max_distance: f32,
    /// 水平旋转角度（弧度）
    pub yaw: f32,
    /// 垂直旋转角度（弧度）
    pub pitch: f32,
    /// 最小俯仰角
    pub min_pitch: f32,
    /// 最大俯仰角
    pub max_pitch: f32,
    /// 旋转灵敏度
    pub rotate_sensitivity: f32,
    /// 缩放灵敏度
    pub zoom_sensitivity: f32,
    /// 平移灵敏度
    pub pan_sensitivity: f32,
    /// 是否启用
    pub enabled: bool,
    /// 旋转按钮（默认右键）
    pub rotate_button: MouseButton,
    /// 平移按钮（默认中键）
    pub pan_button: MouseButton,
}

impl Default for OrbitCameraController {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 10.0,
            min_distance: 1.0,
            max_distance: 100.0,
            yaw: 0.0,
            pitch: -0.5,
            min_pitch: -1.5,
            max_pitch: 1.5,
            rotate_sensitivity: 0.005,
            zoom_sensitivity: 1.0,
            pan_sensitivity: 0.01,
            enabled: true,
            rotate_button: MouseButton::Right,
            pan_button: MouseButton::Middle,
        }
    }
}

impl OrbitCameraController {
    /// 创建新的轨道相机控制器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置目标点
    pub fn with_target(mut self, target: Vec3) -> Self {
        self.target = target;
        self
    }

    /// 设置初始距离
    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance.clamp(self.min_distance, self.max_distance);
        self
    }

    /// 设置距离范围
    pub fn with_distance_range(mut self, min: f32, max: f32) -> Self {
        self.min_distance = min;
        self.max_distance = max;
        self.distance = self.distance.clamp(min, max);
        self
    }

    /// 设置初始旋转角度
    pub fn with_rotation(mut self, yaw: f32, pitch: f32) -> Self {
        self.yaw = yaw;
        self.pitch = pitch.clamp(self.min_pitch, self.max_pitch);
        self
    }

    /// 计算相机位置
    pub fn calculate_position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.cos();
        self.target + Vec3::new(x, -y, z)
    }

    /// 计算相机 Transform
    pub fn calculate_transform(&self) -> Transform {
        let position = self.calculate_position();
        Transform::from_translation(position).looking_at(self.target, Vec3::Y)
    }
}

/// 轨道相机控制器插件
pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit_camera_system);
    }
}

/// 轨道相机控制系统
fn orbit_camera_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrbitCameraController)>,
) {
    // 收集鼠标移动
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta += event.delta;
    }

    // 收集滚轮
    let mut scroll = 0.0;
    for event in mouse_wheel.read() {
        scroll += event.y;
    }

    for (mut transform, mut controller) in &mut query {
        if !controller.enabled {
            continue;
        }

        // 旋转（右键拖动）
        if mouse_button.pressed(controller.rotate_button) && delta != Vec2::ZERO {
            controller.yaw -= delta.x * controller.rotate_sensitivity;
            controller.pitch -= delta.y * controller.rotate_sensitivity;
            controller.pitch = controller
                .pitch
                .clamp(controller.min_pitch, controller.max_pitch);
        }

        // 平移（中键拖动）
        if mouse_button.pressed(controller.pan_button) && delta != Vec2::ZERO {
            let right = transform.right();
            let up = transform.up();
            let pan = (right * -delta.x + up * delta.y)
                * controller.pan_sensitivity
                * controller.distance
                * 0.1;
            controller.target += pan;
        }

        // 缩放（滚轮）
        if scroll != 0.0 {
            controller.distance -= scroll * controller.zoom_sensitivity;
            controller.distance = controller
                .distance
                .clamp(controller.min_distance, controller.max_distance);
        }

        // 更新相机 Transform
        *transform = controller.calculate_transform();
    }
}

/// FPS 风格相机控制器
#[derive(Component, Debug)]
pub struct FpsCameraController {
    /// 移动速度
    pub speed: f32,
    /// 鼠标灵敏度
    pub sensitivity: f32,
    /// 水平旋转角度
    pub yaw: f32,
    /// 垂直旋转角度
    pub pitch: f32,
    /// 是否启用
    pub enabled: bool,
}

impl Default for FpsCameraController {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sensitivity: 0.003,
            yaw: 0.0,
            pitch: 0.0,
            enabled: true,
        }
    }
}

impl FpsCameraController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }
}

/// FPS 相机控制器插件
pub struct FpsCameraPlugin;

impl Plugin for FpsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fps_camera_system);
    }
}

/// FPS 相机控制系统
fn fps_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FpsCameraController)>,
) {
    // 收集鼠标移动
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta += event.delta;
    }

    for (mut transform, mut controller) in &mut query {
        if !controller.enabled {
            continue;
        }

        // 鼠标旋转（右键按住时）
        if mouse_button.pressed(MouseButton::Right) && delta != Vec2::ZERO {
            controller.yaw -= delta.x * controller.sensitivity;
            controller.pitch -= delta.y * controller.sensitivity;
            controller.pitch = controller.pitch.clamp(-1.5, 1.5);

            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
        }

        // 键盘移动
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction -= transform.forward().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction += transform.forward().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= transform.right().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += transform.right().as_vec3();
        }
        if keyboard.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }
        if keyboard.pressed(KeyCode::ShiftLeft) {
            direction -= Vec3::Y;
        }

        if direction != Vec3::ZERO {
            let speed = if keyboard.pressed(KeyCode::ControlLeft) {
                controller.speed * 2.0
            } else {
                controller.speed
            };
            transform.translation += direction.normalize() * speed * time.delta_secs();
        }
    }
}
