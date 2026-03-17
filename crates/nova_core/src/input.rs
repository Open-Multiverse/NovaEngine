//! 输入处理模块
//!
//! 提供简化的键盘、鼠标输入处理 API

use bevy::prelude::*;

/// 输入状态资源
///
/// 提供简化的输入查询接口
#[derive(Resource, Default)]
pub struct InputState {
    /// 鼠标位置
    pub mouse_position: Vec2,
    /// 鼠标移动增量
    pub mouse_delta: Vec2,
    /// 滚轮增量
    pub scroll_delta: f32,
}

/// 输入动作映射
#[derive(Resource, Default)]
pub struct InputActions {
    actions: std::collections::HashMap<String, InputAction>,
}

/// 输入动作定义
#[derive(Clone)]
pub struct InputAction {
    /// 绑定的按键
    pub keys: Vec<KeyCode>,
    /// 绑定的鼠标按钮
    pub mouse_buttons: Vec<MouseButton>,
}

impl InputAction {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            mouse_buttons: Vec::new(),
        }
    }

    pub fn with_key(mut self, key: KeyCode) -> Self {
        self.keys.push(key);
        self
    }

    pub fn with_keys(mut self, keys: &[KeyCode]) -> Self {
        self.keys.extend_from_slice(keys);
        self
    }

    pub fn with_mouse(mut self, button: MouseButton) -> Self {
        self.mouse_buttons.push(button);
        self
    }
}

impl Default for InputAction {
    fn default() -> Self {
        Self::new()
    }
}

impl InputActions {
    /// 注册输入动作
    pub fn register(&mut self, name: impl Into<String>, action: InputAction) {
        self.actions.insert(name.into(), action);
    }

    /// 检查动作是否按下
    pub fn pressed(
        &self,
        name: &str,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        if let Some(action) = self.actions.get(name) {
            for key in &action.keys {
                if keyboard.pressed(*key) {
                    return true;
                }
            }
            for button in &action.mouse_buttons {
                if mouse.pressed(*button) {
                    return true;
                }
            }
        }
        false
    }

    /// 检查动作是否刚按下
    pub fn just_pressed(
        &self,
        name: &str,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        if let Some(action) = self.actions.get(name) {
            for key in &action.keys {
                if keyboard.just_pressed(*key) {
                    return true;
                }
            }
            for button in &action.mouse_buttons {
                if mouse.just_pressed(*button) {
                    return true;
                }
            }
        }
        false
    }

    /// 检查动作是否刚释放
    pub fn just_released(
        &self,
        name: &str,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        if let Some(action) = self.actions.get(name) {
            for key in &action.keys {
                if keyboard.just_released(*key) {
                    return true;
                }
            }
            for button in &action.mouse_buttons {
                if mouse.just_released(*button) {
                    return true;
                }
            }
        }
        false
    }
}

/// 轴向输入（用于移动等）
#[derive(Resource, Default)]
pub struct InputAxes {
    axes: std::collections::HashMap<String, InputAxis>,
}

/// 轴向输入定义
#[derive(Clone)]
pub struct InputAxis {
    /// 正向按键
    pub positive_keys: Vec<KeyCode>,
    /// 负向按键
    pub negative_keys: Vec<KeyCode>,
}

impl InputAxis {
    pub fn new() -> Self {
        Self {
            positive_keys: Vec::new(),
            negative_keys: Vec::new(),
        }
    }

    pub fn with_positive(mut self, key: KeyCode) -> Self {
        self.positive_keys.push(key);
        self
    }

    pub fn with_negative(mut self, key: KeyCode) -> Self {
        self.negative_keys.push(key);
        self
    }

    /// WASD 水平轴
    pub fn horizontal_wasd() -> Self {
        Self::new()
            .with_positive(KeyCode::KeyD)
            .with_negative(KeyCode::KeyA)
    }

    /// WASD 垂直轴
    pub fn vertical_wasd() -> Self {
        Self::new()
            .with_positive(KeyCode::KeyW)
            .with_negative(KeyCode::KeyS)
    }

    /// 箭头键水平轴
    pub fn horizontal_arrows() -> Self {
        Self::new()
            .with_positive(KeyCode::ArrowRight)
            .with_negative(KeyCode::ArrowLeft)
    }

    /// 箭头键垂直轴
    pub fn vertical_arrows() -> Self {
        Self::new()
            .with_positive(KeyCode::ArrowUp)
            .with_negative(KeyCode::ArrowDown)
    }
}

impl Default for InputAxis {
    fn default() -> Self {
        Self::new()
    }
}

impl InputAxes {
    /// 注册轴向输入
    pub fn register(&mut self, name: impl Into<String>, axis: InputAxis) {
        self.axes.insert(name.into(), axis);
    }

    /// 获取轴向值（-1.0 到 1.0）
    pub fn value(&self, name: &str, keyboard: &ButtonInput<KeyCode>) -> f32 {
        if let Some(axis) = self.axes.get(name) {
            let mut value: f32 = 0.0;

            for key in &axis.positive_keys {
                if keyboard.pressed(*key) {
                    value += 1.0;
                }
            }
            for key in &axis.negative_keys {
                if keyboard.pressed(*key) {
                    value -= 1.0;
                }
            }

            value.clamp(-1.0, 1.0)
        } else {
            0.0
        }
    }

    /// 获取 2D 向量输入
    pub fn vector2(
        &self,
        horizontal: &str,
        vertical: &str,
        keyboard: &ButtonInput<KeyCode>,
    ) -> Vec2 {
        Vec2::new(
            self.value(horizontal, keyboard),
            self.value(vertical, keyboard),
        )
    }
}

/// Nova 输入插件
pub struct NovaInputPlugin;

impl Plugin for NovaInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<InputActions>()
            .init_resource::<InputAxes>()
            .add_systems(PreUpdate, update_input_state);
    }
}

/// 更新输入状态
fn update_input_state(
    mut state: ResMut<InputState>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    windows: Query<&Window>,
) {
    // 重置增量
    state.mouse_delta = Vec2::ZERO;
    state.scroll_delta = 0.0;

    // 鼠标移动
    for event in mouse_motion.read() {
        state.mouse_delta += event.delta;
    }

    // 滚轮
    for event in mouse_wheel.read() {
        state.scroll_delta += event.y;
    }

    // 鼠标位置
    if let Ok(window) = windows.get_single() {
        if let Some(pos) = window.cursor_position() {
            state.mouse_position = pos;
        }
    }
}

/// 输入辅助函数
pub mod helpers {
    use super::*;

    /// 检查是否按下移动键
    pub fn is_moving(keyboard: &ButtonInput<KeyCode>) -> bool {
        keyboard.any_pressed([
            KeyCode::KeyW,
            KeyCode::KeyA,
            KeyCode::KeyS,
            KeyCode::KeyD,
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        ])
    }

    /// 获取 WASD 移动向量
    pub fn wasd_movement(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
        let mut dir = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            dir.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            dir.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            dir.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            dir.x -= 1.0;
        }

        if dir != Vec2::ZERO {
            dir = dir.normalize();
        }

        dir
    }

    /// 获取 3D 移动向量（包括上下）
    pub fn movement_3d(keyboard: &ButtonInput<KeyCode>) -> Vec3 {
        let horizontal = wasd_movement(keyboard);
        let mut vertical = 0.0;

        if keyboard.pressed(KeyCode::Space) {
            vertical += 1.0;
        }
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ControlLeft) {
            vertical -= 1.0;
        }

        Vec3::new(horizontal.x, vertical, horizontal.y)
    }
}
