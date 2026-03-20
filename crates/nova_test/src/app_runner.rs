//! App 测试运行器
//!
//! 提供用于测试的最小化 App 环境

use bevy::prelude::*;
use std::time::Duration;

/// 测试用 App 构建器
pub struct TestApp {
    app: App,
    frame_count: usize,
    max_frames: Option<usize>,
}

impl TestApp {
    /// 创建新的测试 App
    ///
    /// # 示例
    /// ```
    /// use nova_test::TestApp;
    ///
    /// let mut app = TestApp::new();
    /// app.run_frames(5);
    /// ```
    pub fn new() -> Self {
        let mut app = App::new();

        // 添加最小化插件
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default());

        Self {
            app,
            frame_count: 0,
            max_frames: None,
        }
    }

    /// 添加插件
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.app.add_plugins(plugin);
        self
    }

    /// 添加系统
    pub fn add_system<M>(&mut self, system: impl IntoSystemConfigs<M>) -> &mut Self {
        self.app.add_systems(Update, system);
        self
    }

    /// 添加启动系统
    pub fn add_startup_system<M>(&mut self, system: impl IntoSystemConfigs<M>) -> &mut Self {
        self.app.add_systems(Startup, system);
        self
    }

    /// 设置最大运行帧数（防止死循环）
    pub fn with_max_frames(mut self, frames: usize) -> Self {
        self.max_frames = Some(frames);
        self
    }

    /// 运行指定帧数
    pub fn run_frames(&mut self, frames: usize) -> &mut Self {
        let max = self.max_frames.unwrap_or(frames * 2);

        for _ in 0..frames {
            if self.frame_count >= max {
                panic!("TestApp exceeded maximum frame count {}", max);
            }

            self.app.update();
            self.frame_count += 1;
        }

        self
    }

    /// 运行直到条件满足或超时
    pub fn run_until<F>(&mut self, condition: F, timeout_frames: usize) -> &mut Self
    where
        F: Fn(&World) -> bool,
    {
        for _ in 0..timeout_frames {
            if condition(self.world()) {
                return self;
            }
            self.app.update();
            self.frame_count += 1;
        }

        panic!("Condition not met within {} frames", timeout_frames);
    }

    /// 运行指定时间
    pub fn run_for(&mut self, duration: Duration) -> &mut Self {
        let start = self.app.world().resource::<Time>().elapsed();

        loop {
            let current = self.app.world().resource::<Time>().elapsed();
            if current - start >= duration {
                break;
            }

            self.app.update();
            self.frame_count += 1;
        }

        self
    }

    /// 获取世界引用
    pub fn world(&self) -> &World {
        self.app.world()
    }

    /// 获取世界可变引用
    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }

    /// 获取资源
    pub fn resource<T: Resource>(&self) -> &T {
        self.app.world().resource::<T>()
    }

    /// 获取 App 引用（用于高级操作）
    pub fn app(&self) -> &App {
        &self.app
    }

    /// 获取 App 可变引用
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// 获取当前帧数
    pub fn frame_count(&self) -> usize {
        self.frame_count
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let mut app = TestApp::new();
        assert_eq!(app.frame_count(), 0);

        app.run_frames(5);
        assert_eq!(app.frame_count(), 5);
    }

    #[test]
    fn test_app_system_execution() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let mut app = TestApp::new();
        app.add_system(|| {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        });

        app.run_frames(3);

        assert_eq!(COUNTER.load(Ordering::SeqCst), 3);
    }
}
