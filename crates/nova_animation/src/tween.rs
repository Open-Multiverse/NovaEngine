//! 补间动画系统

use bevy::prelude::*;

/// 缓动函数类型
#[derive(Debug, Clone, Copy, Default)]
pub enum NovaEaseFunction {
    #[default]
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
}

impl NovaEaseFunction {
    /// 计算缓动值
    pub fn ease(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            NovaEaseFunction::Linear => t,
            NovaEaseFunction::QuadIn => t * t,
            NovaEaseFunction::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
            NovaEaseFunction::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            NovaEaseFunction::CubicIn => t * t * t,
            NovaEaseFunction::CubicOut => 1.0 - (1.0 - t).powi(3),
            NovaEaseFunction::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

/// 位置补间组件
#[derive(Component)]
pub struct PositionTween {
    pub start: Vec3,
    pub end: Vec3,
    pub duration: f32,
    pub elapsed: f32,
    pub ease: NovaEaseFunction,
    pub loop_mode: LoopMode,
}

/// 循环模式
#[derive(Debug, Clone, Copy, Default)]
pub enum LoopMode {
    #[default]
    Once,
    Loop,
    PingPong,
}

impl PositionTween {
    pub fn new(start: Vec3, end: Vec3, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
            ease: NovaEaseFunction::default(),
            loop_mode: LoopMode::default(),
        }
    }

    pub fn with_ease(mut self, ease: NovaEaseFunction) -> Self {
        self.ease = ease;
        self
    }

    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    /// 获取当前进度 (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    /// 是否已完成
    pub fn is_finished(&self) -> bool {
        matches!(self.loop_mode, LoopMode::Once) && self.elapsed >= self.duration
    }
}

/// 补间动画更新系统
pub fn update_position_tweens(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PositionTween)>,
) {
    for (mut transform, mut tween) in &mut query {
        tween.elapsed += time.delta_secs();

        let t = match tween.loop_mode {
            LoopMode::Once => tween.progress(),
            LoopMode::Loop => (tween.elapsed % tween.duration) / tween.duration,
            LoopMode::PingPong => {
                let cycle = (tween.elapsed / tween.duration) as i32;
                let t = (tween.elapsed % tween.duration) / tween.duration;
                if cycle % 2 == 0 {
                    t
                } else {
                    1.0 - t
                }
            }
        };

        let eased_t = tween.ease.ease(t);
        transform.translation = tween.start.lerp(tween.end, eased_t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_linear() {
        let ease = NovaEaseFunction::Linear;
        assert!((ease.ease(0.0) - 0.0).abs() < 0.001);
        assert!((ease.ease(0.5) - 0.5).abs() < 0.001);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_quad_in() {
        let ease = NovaEaseFunction::QuadIn;
        assert!((ease.ease(0.0) - 0.0).abs() < 0.001);
        assert!((ease.ease(0.5) - 0.25).abs() < 0.001);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_quad_out() {
        let ease = NovaEaseFunction::QuadOut;
        assert!((ease.ease(0.0) - 0.0).abs() < 0.001);
        assert!((ease.ease(0.5) - 0.75).abs() < 0.001);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_clamps_input() {
        let ease = NovaEaseFunction::Linear;
        // 超出范围的输入应该被 clamp
        assert!((ease.ease(-0.5) - 0.0).abs() < 0.001);
        assert!((ease.ease(1.5) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_position_tween_new() {
        let tween = PositionTween::new(Vec3::ZERO, Vec3::ONE, 2.0);
        assert_eq!(tween.start, Vec3::ZERO);
        assert_eq!(tween.end, Vec3::ONE);
        assert_eq!(tween.duration, 2.0);
        assert_eq!(tween.elapsed, 0.0);
    }

    #[test]
    fn test_position_tween_progress() {
        let mut tween = PositionTween::new(Vec3::ZERO, Vec3::ONE, 2.0);
        assert_eq!(tween.progress(), 0.0);

        tween.elapsed = 1.0;
        assert!((tween.progress() - 0.5).abs() < 0.001);

        tween.elapsed = 2.0;
        assert!((tween.progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_position_tween_is_finished() {
        let mut tween = PositionTween::new(Vec3::ZERO, Vec3::ONE, 2.0);
        assert!(!tween.is_finished());

        tween.elapsed = 2.0;
        assert!(tween.is_finished());

        // Loop 模式永不完成
        tween.loop_mode = LoopMode::Loop;
        assert!(!tween.is_finished());
    }

    #[test]
    fn test_position_tween_builder() {
        let tween = PositionTween::new(Vec3::ZERO, Vec3::ONE, 1.0)
            .with_ease(NovaEaseFunction::QuadIn)
            .with_loop(LoopMode::PingPong);

        assert!(matches!(tween.ease, NovaEaseFunction::QuadIn));
        assert!(matches!(tween.loop_mode, LoopMode::PingPong));
    }
}
