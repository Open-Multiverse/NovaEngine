//! Nova App - 应用生命周期管理

use bevy::prelude::*;

/// Nova 应用构建器
///
/// 封装 Bevy App，提供简化的游戏引擎配置接口
pub struct NovaApp {
    app: App,
}

impl Default for NovaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl NovaApp {
    /// 创建新的 Nova 应用
    pub fn new() -> Self {
        let mut app = App::new();

        // 配置默认插件，针对 WebGPU 优化
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Nova Engine".into(),
                        canvas: Some("#nova-canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        );

        Self { app }
    }

    /// 设置窗口标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        // 注意：窗口已在 new() 中创建，这里需要通过资源修改
        // 在实际运行时会生效
        let title = title.into();
        self.app
            .add_systems(Startup, move |mut windows: Query<&mut Window>| {
                if let Ok(mut window) = windows.get_single_mut() {
                    window.title = title.clone();
                }
            });
        self
    }

    /// 设置窗口大小
    pub fn with_window_size(mut self, width: f32, height: f32) -> Self {
        self.app
            .add_systems(Startup, move |mut windows: Query<&mut Window>| {
                if let Ok(mut window) = windows.get_single_mut() {
                    window.resolution.set(width, height);
                }
            });
        self
    }

    /// 添加插件
    pub fn add_plugin<T: Plugin>(mut self, plugin: T) -> Self {
        self.app.add_plugins(plugin);
        self
    }

    /// 添加启动系统
    pub fn add_startup_system<M>(mut self, system: impl IntoSystemConfigs<M>) -> Self {
        self.app.add_systems(Startup, system);
        self
    }

    /// 添加更新系统
    pub fn add_system<M>(mut self, system: impl IntoSystemConfigs<M>) -> Self {
        self.app.add_systems(Update, system);
        self
    }

    /// 插入资源
    pub fn insert_resource<R: Resource>(mut self, resource: R) -> Self {
        self.app.insert_resource(resource);
        self
    }

    /// 获取内部 Bevy App 的可变引用
    pub fn inner_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// 运行应用
    pub fn run(mut self) {
        self.app.run();
    }
}
