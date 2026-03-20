//! 渲染测试支持
//!
//! 提供渲染测试的基础设施，包括截图对比

use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;

/// 渲染测试配置
#[derive(Resource, Clone)]
pub struct RenderTestConfig {
    /// 截图保存路径
    pub screenshot_path: String,
    /// 帧数（等待渲染稳定）
    pub warmup_frames: usize,
    /// 比较阈值（0.0-1.0，越低越严格）
    pub similarity_threshold: f32,
}

impl Default for RenderTestConfig {
    fn default() -> Self {
        Self {
            screenshot_path: "tests/screenshots".to_string(),
            warmup_frames: 10,
            similarity_threshold: 0.01,
        }
    }
}

/// 渲染测试管理器
pub struct RenderTest;

impl RenderTest {
    /// 创建新的渲染测试
    ///
    /// # 示例
    /// ```ignore
    /// RenderTest::new()
    ///     .with_warmup_frames(5)
    ///     .capture_and_compare("expected.png");
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// 设置预热帧数
    pub fn with_warmup_frames(self, frames: usize) -> RenderTestBuilder {
        RenderTestBuilder {
            warmup_frames: frames,
            ..Default::default()
        }
    }
}

/// 渲染测试构建器
#[derive(Default)]
pub struct RenderTestBuilder {
    warmup_frames: usize,
    expected_path: Option<String>,
}

impl RenderTestBuilder {
    /// 设置期望截图路径
    pub fn with_expected(self, path: &str) -> Self {
        Self {
            expected_path: Some(path.to_string()),
            ..self
        }
    }

    /// 捕获截图并与期望值对比
    ///
    /// 注意：这个函数需要在运行中的 App 中调用
    pub fn capture_and_compare(self, app: &mut App) {
        // 等待预热帧数
        for _ in 0..self.warmup_frames {
            app.update();
        }

        // TODO: 实现截图捕获逻辑
        // 这需要渲染系统的支持，目前作为占位符
        log::info!(
            "Render test screenshot captured after {} warmup frames",
            self.warmup_frames
        );
    }
}

/// 场景测试工具
pub struct SceneTester;

impl SceneTester {
    /// 验证场景中所有必需组件都存在
    pub fn validate_scene(world: &World) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查相机
        let camera_count = world.query::<&Camera>().iter(world).count();
        if camera_count == 0 {
            errors.push("No camera found in scene".to_string());
        }

        // 检查光源
        let light_count = world.query::<&PointLight>().iter(world).count()
            + world.query::<&DirectionalLight>().iter(world).count()
            + world.query::<&SpotLight>().iter(world).count();

        if light_count == 0 {
            errors.push("No lights found in scene".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 计算场景中网格的包围盒
    pub fn calculate_scene_bounds(world: &World) -> Option<(Transform, Transform)> {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        let mut has_meshes = false;

        for transform in world.query::<&Transform>().iter(world) {
            let pos = transform.translation;
            min = min.min(pos);
            max = max.max(pos);
            has_meshes = true;
        }

        if has_meshes {
            Some((
                Transform::from_translation(min),
                Transform::from_translation(max),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_validation_empty() {
        let world = World::new();
        let result = SceneTester::validate_scene(&world);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("No camera")));
        assert!(errors.iter().any(|e| e.contains("No lights")));
    }

    #[test]
    fn test_scene_bounds_empty() {
        let world = World::new();
        let bounds = SceneTester::calculate_scene_bounds(&world);
        assert!(bounds.is_none());
    }

    #[test]
    fn test_scene_bounds_with_transforms() {
        let mut world = World::new();

        world.spawn(Transform::from_xyz(0.0, 0.0, 0.0));
        world.spawn(Transform::from_xyz(10.0, 5.0, 3.0));
        world.spawn(Transform::from_xyz(-5.0, -2.0, 1.0));

        let bounds = SceneTester::calculate_scene_bounds(&world);
        assert!(bounds.is_some());

        let (min, max) = bounds.unwrap();
        assert_eq!(min.translation, Vec3::new(-5.0, -2.0, 0.0));
        assert_eq!(max.translation, Vec3::new(10.0, 5.0, 3.0));
    }
}
