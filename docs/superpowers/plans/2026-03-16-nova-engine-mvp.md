# Nova Engine MVP 实施计划

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建 Nova Engine MVP，实现核心渲染、物理、UI、动画系统，并在 Web 浏览器中运行一个完整的 3D demo。

**Architecture:** 基于 Bevy 构建独立 API 层，使用 Cargo workspace 管理多 crate 结构。每个子系统封装为独立 crate，通过 nova_engine 入口 crate 统一导出。

**Tech Stack:** Rust, WebAssembly, Bevy 0.15+, Rapier3D, egui, wgpu (WebGPU)

---

## File Structure

```
nova_engine/
├── Cargo.toml                          # Workspace 根配置
├── rust-toolchain.toml                 # Rust 版本锁定
├── .cargo/config.toml                  # WASM 构建配置
├── Trunk.toml                          # Web 构建配置
├── index.html                          # Web 入口模板
├── .gitignore                          # Git 忽略配置
├── LICENSE-MIT                         # MIT 许可证
├── LICENSE-APACHE                      # Apache 2.0 许可证
├── README.md                           # 项目说明
├── crates/
│   ├── nova_core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # 模块入口
│   │       ├── app.rs                  # Nova App 封装
│   │       ├── plugin.rs               # 插件系统
│   │       ├── schedule.rs             # 调度阶段定义
│   │       └── prelude.rs              # 公共导出
│   ├── nova_render/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs               # 渲染插件
│   │       ├── camera.rs               # 相机组件
│   │       ├── light.rs                # 光照组件
│   │       ├── mesh.rs                 # 网格组件
│   │       ├── material.rs             # 材质组件
│   │       └── prelude.rs
│   ├── nova_physics/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs               # 物理插件
│   │       ├── rigid_body.rs           # 刚体组件
│   │       ├── collider.rs             # 碰撞器组件
│   │       ├── events.rs               # 碰撞事件
│   │       └── prelude.rs
│   ├── nova_ui/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs               # UI 插件
│   │       ├── context.rs              # UI 上下文
│   │       ├── widgets.rs              # 基础组件
│   │       └── prelude.rs
│   ├── nova_animation/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs               # 动画插件
│   │       ├── clip.rs                 # 动画片段
│   │       ├── player.rs               # 动画播放器
│   │       └── prelude.rs
│   └── nova_engine/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # 统一入口
│           └── prelude.rs              # 全局 prelude
├── examples/
│   └── basic_demo/
│       ├── Cargo.toml
│       ├── src/
│       │   └── main.rs
│       ├── assets/
│       │   └── .gitkeep
│       ├── index.html
│       └── Trunk.toml
└── .github/
    └── workflows/
        └── ci.yml                      # CI 配置
```

---

## Chunk 1: 项目基础设施

### Task 1: 初始化 Cargo Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.cargo/config.toml`
- Create: `.gitignore`

- [ ] **Step 1: 创建 workspace 根 Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/nova_core",
    "crates/nova_render",
    "crates/nova_physics",
    "crates/nova_ui",
    "crates/nova_animation",
    "crates/nova_engine",
    "examples/basic_demo",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/nova_engine"
rust-version = "1.80"

[workspace.dependencies]
# Internal crates
nova_core = { path = "crates/nova_core" }
nova_render = { path = "crates/nova_render" }
nova_physics = { path = "crates/nova_physics" }
nova_ui = { path = "crates/nova_ui" }
nova_animation = { path = "crates/nova_animation" }
nova_engine = { path = "crates/nova_engine" }

# External dependencies
bevy = { version = "0.15", default-features = false, features = [
    "bevy_asset",
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_pbr",
    "bevy_gltf",
    "bevy_winit",
    "webgpu",
    "tonemapping_luts",
    "ktx2",
    "zstd",
] }
bevy_rapier3d = "0.28"
bevy_egui = "0.31"
log = "0.4"
```

- [ ] **Step 2: 创建 rust-toolchain.toml**

```toml
[toolchain]
channel = "stable"
targets = ["wasm32-unknown-unknown"]
```

- [ ] **Step 3: 创建 .cargo/config.toml**

```toml
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"

[build]
rustflags = ["--cfg=web_sys_unstable_apis"]

[alias]
wasm = "build --release --target wasm32-unknown-unknown"
```

- [ ] **Step 4: 创建 .gitignore**

```gitignore
/target
Cargo.lock
*.swp
*.swo
.DS_Store
dist/
.superpowers/
```

- [ ] **Step 5: 验证 workspace 配置**

Run: `cargo metadata --format-version 1 | head -20`
Expected: JSON 输出显示 workspace 信息

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml rust-toolchain.toml .cargo/config.toml .gitignore
git commit -m "chore: initialize cargo workspace with wasm support"
```

---

### Task 2: 创建许可证和 README

**Files:**
- Create: `LICENSE-MIT`
- Create: `LICENSE-APACHE`
- Create: `README.md`

- [ ] **Step 1: 创建 MIT 许可证**

```text
MIT License

Copyright (c) 2026 Nova Engine Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 2: 创建 Apache 2.0 许可证**

```text
                              Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
   other entities that control, are controlled by, or are under common
   control with that entity. For the purposes of this definition,
   "control" means (i) the power, direct or indirect, to cause the
   direction or management of such entity, whether by contract or
   otherwise, or (ii) ownership of fifty percent (50%) or more of the
   outstanding shares, or (iii) beneficial ownership of such entity.

   "You" (or "Your") shall mean an individual or Legal Entity
   exercising permissions granted by this License.

   "Source" form shall mean the preferred form for making modifications,
   including but not limited to software source code, documentation
   source, and configuration files.

   "Object" form shall mean any form resulting from mechanical
   transformation or translation of a Source form, including but
   not limited to compiled object code, generated documentation,
   and conversions to other media types.

   "Work" shall mean the work of authorship, whether in Source or
   Object form, made available under the License, as indicated by a
   copyright notice that is included in or attached to the work.

   "Derivative Works" shall mean any work, whether in Source or Object
   form, that is based on (or derived from) the Work and for which the
   editorial revisions, annotations, elaborations, or other modifications
   represent, as a whole, an original work of authorship.

   "Contribution" shall mean any work of authorship, including
   the original version of the Work and any modifications or additions
   to that Work or Derivative Works thereof, that is intentionally
   submitted to the Licensor for inclusion in the Work by the copyright owner.

   "Contributor" shall mean Licensor and any Legal Entity on behalf of whom
   a Contribution has been received by Licensor and subsequently incorporated
   within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   patent license to make, have made, use, offer to sell, sell, import,
   and otherwise transfer the Work.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
       Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
       stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
       that You distribute, all copyright, patent, trademark, and
       attribution notices from the Source form of the Work; and

   (d) If the Work includes a "NOTICE" text file as part of its
       distribution, then any Derivative Works that You distribute must
       include a readable copy of the attribution notices contained
       within such NOTICE file.

5. Submission of Contributions. Unless You explicitly state otherwise,
   any Contribution intentionally submitted for inclusion in the Work
   by You to the Licensor shall be under the terms and conditions of
   this License, without any additional terms or conditions.

6. Trademarks. This License does not grant permission to use the trade
   names, trademarks, service marks, or product names of the Licensor.

7. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

8. Limitation of Liability. In no event and under no legal theory shall
   any Contributor be liable to You for damages.

9. Accepting Warranty or Additional Liability. You may act only on Your
   own behalf and on Your sole responsibility.

END OF TERMS AND CONDITIONS
```

- [ ] **Step 3: 创建 README.md**

```markdown
# Nova Engine

A Web 3D game engine built with Rust and WebAssembly.

## Features

- **WebGPU Rendering** - Modern graphics API for high-performance 3D
- **Physics** - Powered by Rapier3D
- **UI** - Immediate mode UI with egui
- **Animation** - Skeletal animation support

## Quick Start

```rust
use nova_engine::prelude::*;

fn main() {
    Nova::app()
        .add_plugin(NovaDefaults)
        .add_system(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera3d::default());
    cmd.spawn(DirectionalLight::sun());
    cmd.spawn((Mesh::cube(1.0), Material::default()));
}
```

## Building for Web

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve examples/basic_demo
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))
```

- [ ] **Step 4: Commit**

```bash
git add LICENSE-MIT LICENSE-APACHE README.md
git commit -m "docs: add dual license and README"
```

---

### Task 3: Web 构建配置

**Files:**
- Create: `Trunk.toml`
- Create: `index.html`

- [ ] **Step 1: 创建 Trunk.toml**

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
ignore = ["dist", "target"]

[serve]
address = "127.0.0.1"
port = 8080
open = true
```

- [ ] **Step 2: 创建 index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nova Engine</title>
    <style>
        html, body {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            background: #1a1a2e;
        }
        canvas {
            display: block;
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <link data-trunk rel="rust" data-wasm-opt="z" />
</body>
</html>
```

- [ ] **Step 3: Commit**

```bash
git add Trunk.toml index.html
git commit -m "build: add trunk web build configuration"
```

---

## Chunk 2: Nova Core

### Task 4: 创建 nova_core crate 基础结构

**Files:**
- Create: `crates/nova_core/Cargo.toml`
- Create: `crates/nova_core/src/lib.rs`
- Create: `crates/nova_core/src/schedule.rs`

- [ ] **Step 1: 创建 nova_core/Cargo.toml**

```toml
[package]
name = "nova_core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
bevy = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_core/src/schedule.rs**

```rust
//! Nova 调度阶段定义

pub use bevy::prelude::{
    First, Last, PostStartup, PostUpdate, PreStartup, PreUpdate, Startup, Update,
};

/// Nova 调度阶段
pub struct Schedules;

impl Schedules {
    /// 游戏启动时运行一次
    pub const STARTUP: Startup = Startup;
    /// 每帧更新
    pub const UPDATE: Update = Update;
}
```

- [ ] **Step 3: 创建 nova_core/src/lib.rs**

```rust
//! Nova Engine 核心模块
//!
//! 提供 ECS 封装、App 生命周期和插件系统。

pub mod schedule;

pub mod prelude {
    pub use crate::schedule::*;
    pub use bevy::prelude::{
        App, Bundle, Commands, Component, Deref, DerefMut, Entity, Event, EventReader,
        EventWriter, In, IntoSystem, Local, Or, Plugin, Query, Res, ResMut, Resource,
        StartupSet, SystemSet, With, Without, World,
    };
    pub use bevy::math::{Vec2, Vec3, Vec4, Quat, Mat4};
    pub use bevy::transform::components::Transform;
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_core`
Expected: 编译成功，无错误

- [ ] **Step 5: Commit**

```bash
git add crates/nova_core
git commit -m "feat(nova_core): add core module with schedule definitions"
```

---

### Task 5: 实现 NovaApp 封装

**Files:**
- Create: `crates/nova_core/src/app.rs`
- Modify: `crates/nova_core/src/lib.rs`

- [ ] **Step 1: 创建 nova_core/src/app.rs**

```rust
//! Nova App 封装

use bevy::prelude::*;

/// Nova 应用构建器
pub struct Nova;

impl Nova {
    /// 创建新的 Nova 应用
    pub fn app() -> NovaApp {
        NovaApp::new()
    }
}

/// Nova 应用实例
pub struct NovaApp {
    app: App,
}

impl NovaApp {
    /// 创建新的应用实例
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    /// 添加插件
    pub fn add_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.app.add_plugins(plugin);
        self
    }

    /// 添加插件组
    pub fn add_plugins<M>(mut self, plugins: impl bevy::app::Plugins<M>) -> Self {
        self.app.add_plugins(plugins);
        self
    }

    /// 添加系统到指定调度阶段
    pub fn add_system<M>(
        mut self,
        schedule: impl bevy::ecs::schedule::ScheduleLabel,
        system: impl IntoSystemConfigs<M>,
    ) -> Self {
        self.app.add_systems(schedule, system);
        self
    }

    /// 插入资源
    pub fn insert_resource<R: Resource>(mut self, resource: R) -> Self {
        self.app.insert_resource(resource);
        self
    }

    /// 注册事件类型
    pub fn add_event<E: Event>(mut self) -> Self {
        self.app.add_event::<E>();
        self
    }

    /// 运行应用
    pub fn run(mut self) {
        self.app.run();
    }

    /// 获取底层 Bevy App 的可变引用（高级用法）
    pub fn bevy_app_mut(&mut self) -> &mut App {
        &mut self.app
    }
}

impl Default for NovaApp {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 2: 更新 nova_core/src/lib.rs**

```rust
//! Nova Engine 核心模块
//!
//! 提供 ECS 封装、App 生命周期和插件系统。

pub mod app;
pub mod schedule;

pub mod prelude {
    pub use crate::app::{Nova, NovaApp};
    pub use crate::schedule::*;
    pub use bevy::prelude::{
        App, Bundle, Commands, Component, Deref, DerefMut, Entity, Event, EventReader,
        EventWriter, In, IntoSystem, IntoSystemConfigs, Local, Or, Plugin, Query, Res,
        ResMut, Resource, StartupSet, SystemSet, With, Without, World,
    };
    pub use bevy::math::{Vec2, Vec3, Vec4, Quat, Mat4};
    pub use bevy::transform::components::Transform;
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_core`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_core
git commit -m "feat(nova_core): implement NovaApp wrapper"
```

---

### Task 6: 实现插件系统

**Files:**
- Create: `crates/nova_core/src/plugin.rs`
- Modify: `crates/nova_core/src/lib.rs`
- Modify: `crates/nova_core/src/prelude.rs`

- [ ] **Step 1: 创建 nova_core/src/plugin.rs**

```rust
//! Nova 插件系统

use bevy::prelude::*;
use crate::app::NovaApp;

/// Nova 插件 trait
pub trait NovaPlugin: Send + Sync + 'static {
    /// 构建插件，注册系统和资源
    fn build(&self, app: &mut NovaApp);

    /// 插件名称（用于调试）
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// 将 NovaPlugin 转换为 Bevy Plugin 的适配器
pub struct NovaPluginAdapter<P: NovaPlugin> {
    plugin: P,
}

impl<P: NovaPlugin> NovaPluginAdapter<P> {
    pub fn new(plugin: P) -> Self {
        Self { plugin }
    }
}

impl<P: NovaPlugin> Plugin for NovaPluginAdapter<P> {
    fn build(&self, app: &mut App) {
        let mut nova_app = NovaApp::from_bevy_app(app);
        self.plugin.build(&mut nova_app);
    }
}
```

- [ ] **Step 2: 更新 nova_core/src/app.rs 添加 from_bevy_app**

在 `NovaApp` impl 块中添加：

```rust
    /// 从 Bevy App 创建（内部使用）
    pub(crate) fn from_bevy_app(app: &mut App) -> NovaAppRef<'_> {
        NovaAppRef { app }
    }
```

并添加新结构体：

```rust
/// Nova 应用引用（用于插件构建）
pub struct NovaAppRef<'a> {
    app: &'a mut App,
}

impl<'a> NovaAppRef<'a> {
    /// 添加系统
    pub fn add_system<M>(
        &mut self,
        schedule: impl bevy::ecs::schedule::ScheduleLabel,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.app.add_systems(schedule, system);
        self
    }

    /// 插入资源
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.app.insert_resource(resource);
        self
    }

    /// 添加事件
    pub fn add_event<E: Event>(&mut self) -> &mut Self {
        self.app.add_event::<E>();
        self
    }

    /// 添加 Bevy 插件
    pub fn add_bevy_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.app.add_plugins(plugin);
        self
    }
}
```

- [ ] **Step 3: 更新 nova_core/src/lib.rs**

```rust
//! Nova Engine 核心模块
//!
//! 提供 ECS 封装、App 生命周期和插件系统。

pub mod app;
pub mod plugin;
pub mod schedule;

pub mod prelude {
    pub use crate::app::{Nova, NovaApp, NovaAppRef};
    pub use crate::plugin::NovaPlugin;
    pub use crate::schedule::*;
    pub use bevy::prelude::{
        App, Bundle, Commands, Component, Deref, DerefMut, Entity, Event, EventReader,
        EventWriter, In, IntoSystem, IntoSystemConfigs, Local, Or, Plugin, Query, Res,
        ResMut, Resource, StartupSet, SystemSet, With, Without, World,
    };
    pub use bevy::math::{Vec2, Vec3, Vec4, Quat, Mat4};
    pub use bevy::transform::components::Transform;
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_core`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_core
git commit -m "feat(nova_core): implement plugin system"
```

---

## Chunk 3: Nova Render

### Task 7: 创建 nova_render crate 基础结构

**Files:**
- Create: `crates/nova_render/Cargo.toml`
- Create: `crates/nova_render/src/lib.rs`
- Create: `crates/nova_render/src/plugin.rs`

- [ ] **Step 1: 创建 nova_render/Cargo.toml**

```toml
[package]
name = "nova_render"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_render/src/plugin.rs**

```rust
//! 渲染插件

use bevy::prelude::*;

/// Nova 渲染插件
pub struct NovaRenderPlugin;

impl Plugin for NovaRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Nova Engine".into(),
                        canvas: Some("#bevy".into()),
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::WARN,
                    ..default()
                }),
        );
    }
}
```

- [ ] **Step 3: 创建 nova_render/src/lib.rs**

```rust
//! Nova Engine 渲染模块
//!
//! 提供 3D 渲染、相机、光照和材质系统。

pub mod plugin;

pub mod prelude {
    pub use crate::plugin::NovaRenderPlugin;
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_render`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_render
git commit -m "feat(nova_render): add render plugin with window setup"
```

---

### Task 8: 实现相机组件

**Files:**
- Create: `crates/nova_render/src/camera.rs`
- Modify: `crates/nova_render/src/lib.rs`

- [ ] **Step 1: 创建 nova_render/src/camera.rs**

```rust
//! 相机组件

use bevy::prelude::*;

/// 3D 透视相机
#[derive(Bundle, Default)]
pub struct Camera3d {
    pub camera: Camera3dBundle,
}

impl Camera3d {
    /// 创建默认相机
    pub fn new() -> Self {
        Self {
            camera: Camera3dBundle {
                transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
        }
    }

    /// 设置相机位置
    pub fn position(mut self, pos: Vec3) -> Self {
        self.camera.transform.translation = pos;
        self
    }

    /// 设置相机朝向目标点
    pub fn looking_at(mut self, target: Vec3) -> Self {
        self.camera.transform = self.camera.transform.looking_at(target, Vec3::Y);
        self
    }

    /// 设置 FOV（弧度）
    pub fn fov(mut self, fov: f32) -> Self {
        if let Projection::Perspective(ref mut persp) = self.camera.projection {
            persp.fov = fov;
        }
        self
    }
}
```

- [ ] **Step 2: 更新 nova_render/src/lib.rs**

```rust
//! Nova Engine 渲染模块
//!
//! 提供 3D 渲染、相机、光照和材质系统。

pub mod camera;
pub mod plugin;

pub mod prelude {
    pub use crate::camera::Camera3d;
    pub use crate::plugin::NovaRenderPlugin;
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_render`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_render
git commit -m "feat(nova_render): implement Camera3d component"
```

---

### Task 9: 实现光照组件

**Files:**
- Create: `crates/nova_render/src/light.rs`
- Modify: `crates/nova_render/src/lib.rs`

- [ ] **Step 1: 创建 nova_render/src/light.rs**

```rust
//! 光照组件

use bevy::prelude::*;

/// 方向光
#[derive(Bundle)]
pub struct DirectionalLight {
    pub light: DirectionalLightBundle,
}

impl DirectionalLight {
    /// 创建类似太阳的方向光
    pub fn sun() -> Self {
        Self {
            light: DirectionalLightBundle {
                directional_light: bevy::pbr::DirectionalLight {
                    illuminance: 10000.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    -std::f32::consts::FRAC_PI_4,
                    std::f32::consts::FRAC_PI_4,
                    0.0,
                )),
                ..default()
            },
        }
    }

    /// 设置光照强度
    pub fn illuminance(mut self, value: f32) -> Self {
        self.light.directional_light.illuminance = value;
        self
    }

    /// 设置是否启用阴影
    pub fn shadows(mut self, enabled: bool) -> Self {
        self.light.directional_light.shadows_enabled = enabled;
        self
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self::sun()
    }
}

/// 点光源
#[derive(Bundle)]
pub struct PointLight {
    pub light: PointLightBundle,
}

impl PointLight {
    /// 创建点光源
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self {
            light: PointLightBundle {
                point_light: bevy::pbr::PointLight {
                    intensity,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    }

    /// 设置光照颜色
    pub fn color(mut self, color: Color) -> Self {
        self.light.point_light.color = color;
        self
    }

    /// 设置光照范围
    pub fn range(mut self, range: f32) -> Self {
        self.light.point_light.range = range;
        self
    }
}

/// 环境光
#[derive(Resource)]
pub struct AmbientLight {
    pub color: Color,
    pub brightness: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            brightness: 0.1,
        }
    }
}
```

- [ ] **Step 2: 更新 nova_render/src/lib.rs**

```rust
//! Nova Engine 渲染模块
//!
//! 提供 3D 渲染、相机、光照和材质系统。

pub mod camera;
pub mod light;
pub mod plugin;

pub mod prelude {
    pub use crate::camera::Camera3d;
    pub use crate::light::{AmbientLight, DirectionalLight, PointLight};
    pub use crate::plugin::NovaRenderPlugin;
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_render`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_render
git commit -m "feat(nova_render): implement light components"
```

---

### Task 10: 实现网格和材质组件

**Files:**
- Create: `crates/nova_render/src/mesh.rs`
- Create: `crates/nova_render/src/material.rs`
- Modify: `crates/nova_render/src/lib.rs`

- [ ] **Step 1: 创建 nova_render/src/mesh.rs**

```rust
//! 网格组件

use bevy::prelude::*;

/// 网格句柄包装
#[derive(Component, Clone)]
pub struct Mesh(pub Handle<bevy::render::mesh::Mesh>);

impl Mesh {
    /// 创建立方体网格
    pub fn cube(size: f32) -> MeshRequest {
        MeshRequest::Cube(size)
    }

    /// 创建球体网格
    pub fn sphere(radius: f32) -> MeshRequest {
        MeshRequest::Sphere { radius, segments: 32 }
    }

    /// 创建平面网格
    pub fn plane(size: f32) -> MeshRequest {
        MeshRequest::Plane(size)
    }

    /// 创建胶囊体
    pub fn capsule(radius: f32, height: f32) -> MeshRequest {
        MeshRequest::Capsule { radius, height }
    }
}

/// 网格创建请求（延迟到有 Meshes 资源时创建）
#[derive(Clone)]
pub enum MeshRequest {
    Cube(f32),
    Sphere { radius: f32, segments: u32 },
    Plane(f32),
    Capsule { radius: f32, height: f32 },
}

impl MeshRequest {
    /// 转换为 Bevy Mesh
    pub fn to_bevy_mesh(&self) -> bevy::render::mesh::Mesh {
        use bevy::render::mesh::Mesh as BevyMesh;
        match self {
            MeshRequest::Cube(size) => BevyMesh::from(Cuboid::new(*size, *size, *size)),
            MeshRequest::Sphere { radius, segments } => {
                BevyMesh::from(Sphere::new(*radius).mesh().ico(*segments as usize).unwrap())
            }
            MeshRequest::Plane(size) => BevyMesh::from(Plane3d::new(Vec3::Y, Vec2::splat(*size / 2.0))),
            MeshRequest::Capsule { radius, height } => {
                BevyMesh::from(Capsule3d::new(*radius, *height))
            }
        }
    }
}
```

- [ ] **Step 2: 创建 nova_render/src/material.rs**

```rust
//! 材质组件

use bevy::prelude::*;

/// PBR 材质
#[derive(Clone)]
pub struct Material {
    pub base_color: Color,
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: Color,
}

impl Material {
    /// 创建新材质
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置基础颜色
    pub fn color(mut self, color: Color) -> Self {
        self.base_color = color;
        self
    }

    /// 设置金属度 (0.0 - 1.0)
    pub fn metallic(mut self, value: f32) -> Self {
        self.metallic = value;
        self
    }

    /// 设置粗糙度 (0.0 - 1.0)
    pub fn roughness(mut self, value: f32) -> Self {
        self.roughness = value;
        self
    }

    /// 设置自发光颜色
    pub fn emissive(mut self, color: Color) -> Self {
        self.emissive = color;
        self
    }

    /// 转换为 Bevy StandardMaterial
    pub fn to_bevy_material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            metallic: self.metallic,
            perceptual_roughness: self.roughness,
            emissive: self.emissive.into(),
            ..default()
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            metallic: 0.0,
            roughness: 0.5,
            emissive: Color::BLACK,
        }
    }
}
```

- [ ] **Step 3: 更新 nova_render/src/lib.rs**

```rust
//! Nova Engine 渲染模块
//!
//! 提供 3D 渲染、相机、光照和材质系统。

pub mod camera;
pub mod light;
pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::camera::Camera3d;
    pub use crate::light::{AmbientLight, DirectionalLight, PointLight};
    pub use crate::material::Material;
    pub use crate::mesh::{Mesh, MeshRequest};
    pub use crate::plugin::NovaRenderPlugin;
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_render`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_render
git commit -m "feat(nova_render): implement mesh and material components"
```

---

## Chunk 4: Nova Physics

### Task 11: 创建 nova_physics crate

**Files:**
- Create: `crates/nova_physics/Cargo.toml`
- Create: `crates/nova_physics/src/lib.rs`
- Create: `crates/nova_physics/src/plugin.rs`

- [ ] **Step 1: 创建 nova_physics/Cargo.toml**

```toml
[package]
name = "nova_physics"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
bevy_rapier3d = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_physics/src/plugin.rs**

```rust
//! 物理插件

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Nova 物理插件
pub struct NovaPhysicsPlugin;

impl Plugin for NovaPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default().disabled());
    }
}

/// 启用物理调试渲染
pub fn enable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = true;
}

/// 禁用物理调试渲染
pub fn disable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = false;
}
```

- [ ] **Step 3: 创建 nova_physics/src/lib.rs**

```rust
//! Nova Engine 物理模块
//!
//! 提供碰撞检测、刚体和物理模拟。

pub mod plugin;

pub mod prelude {
    pub use crate::plugin::{NovaPhysicsPlugin, enable_physics_debug, disable_physics_debug};
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_physics`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_physics
git commit -m "feat(nova_physics): add physics plugin with Rapier"
```

---

### Task 12: 实现刚体组件

**Files:**
- Create: `crates/nova_physics/src/rigid_body.rs`
- Modify: `crates/nova_physics/src/lib.rs`

- [ ] **Step 1: 创建 nova_physics/src/rigid_body.rs**

```rust
//! 刚体组件

pub use bevy_rapier3d::prelude::RigidBody;

/// 刚体类型便捷访问
pub struct RigidBodyType;

impl RigidBodyType {
    /// 动态刚体 - 受物理模拟影响
    pub const DYNAMIC: RigidBody = RigidBody::Dynamic;

    /// 静态刚体 - 不移动，但参与碰撞
    pub const STATIC: RigidBody = RigidBody::Fixed;

    /// 运动学刚体 - 可手动控制移动
    pub const KINEMATIC: RigidBody = RigidBody::KinematicPositionBased;
}

/// 刚体速度
pub use bevy_rapier3d::prelude::Velocity;

/// 外力
pub use bevy_rapier3d::prelude::ExternalForce;

/// 外部冲量
pub use bevy_rapier3d::prelude::ExternalImpulse;

/// 重力缩放
pub use bevy_rapier3d::prelude::GravityScale;

/// 阻尼
pub use bevy_rapier3d::prelude::Damping;
```

- [ ] **Step 2: 更新 nova_physics/src/lib.rs**

```rust
//! Nova Engine 物理模块
//!
//! 提供碰撞检测、刚体和物理模拟。

pub mod plugin;
pub mod rigid_body;

pub mod prelude {
    pub use crate::plugin::{NovaPhysicsPlugin, enable_physics_debug, disable_physics_debug};
    pub use crate::rigid_body::{
        Damping, ExternalForce, ExternalImpulse, GravityScale, RigidBody, RigidBodyType, Velocity,
    };
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_physics`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_physics
git commit -m "feat(nova_physics): implement rigid body components"
```

---

### Task 13: 实现碰撞器组件

**Files:**
- Create: `crates/nova_physics/src/collider.rs`
- Modify: `crates/nova_physics/src/lib.rs`

- [ ] **Step 1: 创建 nova_physics/src/collider.rs**

```rust
//! 碰撞器组件

pub use bevy_rapier3d::prelude::Collider;

/// 碰撞器形状构建器
pub struct ColliderShape;

impl ColliderShape {
    /// 立方体碰撞器
    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Collider {
        Collider::cuboid(hx, hy, hz)
    }

    /// 球体碰撞器
    pub fn sphere(radius: f32) -> Collider {
        Collider::ball(radius)
    }

    /// 胶囊体碰撞器
    pub fn capsule(half_height: f32, radius: f32) -> Collider {
        Collider::capsule_y(half_height, radius)
    }

    /// 圆柱体碰撞器
    pub fn cylinder(half_height: f32, radius: f32) -> Collider {
        Collider::cylinder(half_height, radius)
    }

    /// 地面平面碰撞器
    pub fn ground() -> Collider {
        Collider::halfspace(bevy::math::Vec3::Y).unwrap()
    }
}

/// 碰撞组
pub use bevy_rapier3d::prelude::CollisionGroups;

/// 传感器（触发器）
pub use bevy_rapier3d::prelude::Sensor;

/// 摩擦系数
pub use bevy_rapier3d::prelude::Friction;

/// 恢复系数（弹性）
pub use bevy_rapier3d::prelude::Restitution;
```

- [ ] **Step 2: 更新 nova_physics/src/lib.rs**

```rust
//! Nova Engine 物理模块
//!
//! 提供碰撞检测、刚体和物理模拟。

pub mod collider;
pub mod plugin;
pub mod rigid_body;

pub mod prelude {
    pub use crate::collider::{Collider, ColliderShape, CollisionGroups, Friction, Restitution, Sensor};
    pub use crate::plugin::{NovaPhysicsPlugin, enable_physics_debug, disable_physics_debug};
    pub use crate::rigid_body::{
        Damping, ExternalForce, ExternalImpulse, GravityScale, RigidBody, RigidBodyType, Velocity,
    };
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_physics`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_physics
git commit -m "feat(nova_physics): implement collider components"
```

---

### Task 14: 实现碰撞事件

**Files:**
- Create: `crates/nova_physics/src/events.rs`
- Modify: `crates/nova_physics/src/lib.rs`

- [ ] **Step 1: 创建 nova_physics/src/events.rs**

```rust
//! 碰撞事件

pub use bevy_rapier3d::prelude::{CollisionEvent, ContactForceEvent};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// 碰撞开始事件
#[derive(Event)]
pub struct CollisionStarted {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

/// 碰撞结束事件
#[derive(Event)]
pub struct CollisionEnded {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

/// 将 Rapier 事件转换为 Nova 事件的系统
pub fn process_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut started_writer: EventWriter<CollisionStarted>,
    mut ended_writer: EventWriter<CollisionEnded>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                started_writer.send(CollisionStarted {
                    entity_a: *a,
                    entity_b: *b,
                });
            }
            CollisionEvent::Stopped(a, b, _) => {
                ended_writer.send(CollisionEnded {
                    entity_a: *a,
                    entity_b: *b,
                });
            }
        }
    }
}

/// 射线检测结果
pub use bevy_rapier3d::prelude::RayIntersection;

/// 射线检测上下文
pub use bevy_rapier3d::prelude::RapierContext;
```

- [ ] **Step 2: 更新 nova_physics/src/plugin.rs 注册事件**

```rust
//! 物理插件

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::events::{CollisionStarted, CollisionEnded, process_collision_events};

/// Nova 物理插件
pub struct NovaPhysicsPlugin;

impl Plugin for NovaPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default().disabled())
            .add_event::<CollisionStarted>()
            .add_event::<CollisionEnded>()
            .add_systems(Update, process_collision_events);
    }
}

/// 启用物理调试渲染
pub fn enable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = true;
}

/// 禁用物理调试渲染
pub fn disable_physics_debug(mut debug_render: ResMut<DebugRenderContext>) {
    debug_render.enabled = false;
}
```

- [ ] **Step 3: 更新 nova_physics/src/lib.rs**

```rust
//! Nova Engine 物理模块
//!
//! 提供碰撞检测、刚体和物理模拟。

pub mod collider;
pub mod events;
pub mod plugin;
pub mod rigid_body;

pub mod prelude {
    pub use crate::collider::{Collider, ColliderShape, CollisionGroups, Friction, Restitution, Sensor};
    pub use crate::events::{CollisionStarted, CollisionEnded, RapierContext, RayIntersection};
    pub use crate::plugin::{NovaPhysicsPlugin, enable_physics_debug, disable_physics_debug};
    pub use crate::rigid_body::{
        Damping, ExternalForce, ExternalImpulse, GravityScale, RigidBody, RigidBodyType, Velocity,
    };
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_physics`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_physics
git commit -m "feat(nova_physics): implement collision events"
```

---

## Chunk 5: Nova UI

### Task 15: 创建 nova_ui crate

**Files:**
- Create: `crates/nova_ui/Cargo.toml`
- Create: `crates/nova_ui/src/lib.rs`
- Create: `crates/nova_ui/src/plugin.rs`

- [ ] **Step 1: 创建 nova_ui/Cargo.toml**

```toml
[package]
name = "nova_ui"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
bevy_egui = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_ui/src/plugin.rs**

```rust
//! UI 插件

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// Nova UI 插件
pub struct NovaUiPlugin;

impl Plugin for NovaUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
    }
}
```

- [ ] **Step 3: 创建 nova_ui/src/lib.rs**

```rust
//! Nova Engine UI 模块
//!
//! 提供即时模式 UI 系统。

pub mod plugin;

pub mod prelude {
    pub use crate::plugin::NovaUiPlugin;
    pub use bevy_egui::{egui, EguiContexts};
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_ui`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_ui
git commit -m "feat(nova_ui): add UI plugin with egui"
```

---

### Task 16: 实现 UI 上下文和组件

**Files:**
- Create: `crates/nova_ui/src/context.rs`
- Create: `crates/nova_ui/src/widgets.rs`
- Modify: `crates/nova_ui/src/lib.rs`

- [ ] **Step 1: 创建 nova_ui/src/context.rs**

```rust
//! UI 上下文

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Nova UI 上下文参数
pub type UiContext<'w, 's> = EguiContexts<'w, 's>;

/// UI 面板位置
#[derive(Clone, Copy, Debug)]
pub enum PanelPosition {
    Left,
    Right,
    Top,
    Bottom,
}

/// 创建侧边面板
pub fn side_panel(
    ctx: &egui::Context,
    position: PanelPosition,
    id: impl Into<egui::Id>,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    match position {
        PanelPosition::Left => {
            egui::SidePanel::left(id).show(ctx, add_contents);
        }
        PanelPosition::Right => {
            egui::SidePanel::right(id).show(ctx, add_contents);
        }
        PanelPosition::Top => {
            egui::TopBottomPanel::top(id).show(ctx, add_contents);
        }
        PanelPosition::Bottom => {
            egui::TopBottomPanel::bottom(id).show(ctx, add_contents);
        }
    }
}

/// 创建居中窗口
pub fn window(
    ctx: &egui::Context,
    title: impl Into<egui::WidgetText>,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Window::new(title)
        .collapsible(true)
        .resizable(true)
        .show(ctx, add_contents);
}
```

- [ ] **Step 2: 创建 nova_ui/src/widgets.rs**

```rust
//! 游戏 UI 组件

use bevy_egui::egui;

/// 绘制血条
pub fn health_bar(ui: &mut egui::Ui, current: f32, max: f32) {
    let ratio = (current / max).clamp(0.0, 1.0);
    let color = if ratio > 0.5 {
        egui::Color32::GREEN
    } else if ratio > 0.25 {
        egui::Color32::YELLOW
    } else {
        egui::Color32::RED
    };

    let desired_size = egui::vec2(200.0, 20.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // 背景
        painter.rect_filled(rect, 4.0, egui::Color32::DARK_GRAY);

        // 血条
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(rect.width() * ratio, rect.height()),
        );
        painter.rect_filled(fill_rect, 4.0, color);

        // 边框
        painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::WHITE));

        // 文本
        let text = format!("{:.0}/{:.0}", current, max);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }
}

/// 绘制准星
pub fn crosshair(ui: &mut egui::Ui) {
    let size = 20.0;
    let thickness = 2.0;
    let gap = 4.0;
    let color = egui::Color32::WHITE;

    let center = ui.available_rect_before_wrap().center();
    let painter = ui.painter();

    // 水平线
    painter.line_segment(
        [
            egui::pos2(center.x - size, center.y),
            egui::pos2(center.x - gap, center.y),
        ],
        egui::Stroke::new(thickness, color),
    );
    painter.line_segment(
        [
            egui::pos2(center.x + gap, center.y),
            egui::pos2(center.x + size, center.y),
        ],
        egui::Stroke::new(thickness, color),
    );

    // 垂直线
    painter.line_segment(
        [
            egui::pos2(center.x, center.y - size),
            egui::pos2(center.x, center.y - gap),
        ],
        egui::Stroke::new(thickness, color),
    );
    painter.line_segment(
        [
            egui::pos2(center.x, center.y + gap),
            egui::pos2(center.x, center.y + size),
        ],
        egui::Stroke::new(thickness, color),
    );
}

/// FPS 显示
pub fn fps_counter(ui: &mut egui::Ui, fps: f32) {
    ui.label(format!("FPS: {:.1}", fps));
}
```

- [ ] **Step 3: 更新 nova_ui/src/lib.rs**

```rust
//! Nova Engine UI 模块
//!
//! 提供即时模式 UI 系统。

pub mod context;
pub mod plugin;
pub mod widgets;

pub mod prelude {
    pub use crate::context::{side_panel, window, PanelPosition, UiContext};
    pub use crate::plugin::NovaUiPlugin;
    pub use crate::widgets::{crosshair, fps_counter, health_bar};
    pub use bevy_egui::{egui, EguiContexts};
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_ui`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_ui
git commit -m "feat(nova_ui): implement UI context and game widgets"
```

---

## Chunk 6: Nova Animation

### Task 17: 创建 nova_animation crate

**Files:**
- Create: `crates/nova_animation/Cargo.toml`
- Create: `crates/nova_animation/src/lib.rs`
- Create: `crates/nova_animation/src/plugin.rs`

- [ ] **Step 1: 创建 nova_animation/Cargo.toml**

```toml
[package]
name = "nova_animation"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
nova_core = { workspace = true }
bevy = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_animation/src/plugin.rs**

```rust
//! 动画插件

use bevy::prelude::*;

/// Nova 动画插件
pub struct NovaAnimationPlugin;

impl Plugin for NovaAnimationPlugin {
    fn build(&self, _app: &mut App) {
        // Bevy 已内置动画支持，这里可添加额外功能
    }
}
```

- [ ] **Step 3: 创建 nova_animation/src/lib.rs**

```rust
//! Nova Engine 动画模块
//!
//! 提供骨骼动画和动画控制。

pub mod plugin;

pub mod prelude {
    pub use crate::plugin::NovaAnimationPlugin;
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_animation`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_animation
git commit -m "feat(nova_animation): add animation plugin"
```

---

### Task 18: 实现动画播放器

**Files:**
- Create: `crates/nova_animation/src/clip.rs`
- Create: `crates/nova_animation/src/player.rs`
- Modify: `crates/nova_animation/src/lib.rs`

- [ ] **Step 1: 创建 nova_animation/src/clip.rs**

```rust
//! 动画片段

use bevy::prelude::*;

/// 动画片段引用
pub use bevy::animation::AnimationClip;

/// 动画图
pub use bevy::animation::AnimationGraph;

/// 动画图句柄
pub use bevy::animation::AnimationGraphHandle;

/// 动画节点索引
pub use bevy::animation::AnimationNodeIndex;
```

- [ ] **Step 2: 创建 nova_animation/src/player.rs**

```rust
//! 动画播放器

use bevy::prelude::*;
use bevy::animation::{AnimationPlayer, AnimationTransitions};

/// Nova 动画播放器组件
#[derive(Component)]
pub struct NovaAnimationPlayer {
    /// 当前播放的动画索引
    current_animation: Option<bevy::animation::AnimationNodeIndex>,
    /// 是否循环播放
    looping: bool,
    /// 播放速度
    speed: f32,
}

impl Default for NovaAnimationPlayer {
    fn default() -> Self {
        Self {
            current_animation: None,
            looping: true,
            speed: 1.0,
        }
    }
}

impl NovaAnimationPlayer {
    /// 创建新的动画播放器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否循环
    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// 设置播放速度
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// 获取当前动画索引
    pub fn current(&self) -> Option<bevy::animation::AnimationNodeIndex> {
        self.current_animation
    }

    /// 设置当前动画
    pub fn set_current(&mut self, index: bevy::animation::AnimationNodeIndex) {
        self.current_animation = Some(index);
    }

    /// 获取播放速度
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// 是否循环
    pub fn is_looping(&self) -> bool {
        self.looping
    }
}

/// 播放动画的辅助函数
pub fn play_animation(
    player: &mut AnimationPlayer,
    transitions: &mut AnimationTransitions,
    animation_index: bevy::animation::AnimationNodeIndex,
    transition_duration: std::time::Duration,
) {
    transitions
        .play(player, animation_index, transition_duration)
        .repeat();
}

/// 停止动画
pub fn stop_animation(player: &mut AnimationPlayer) {
    player.pause();
}

/// 暂停/恢复动画
pub fn toggle_pause(player: &mut AnimationPlayer) {
    if player.is_paused() {
        player.resume();
    } else {
        player.pause();
    }
}
```

- [ ] **Step 3: 更新 nova_animation/src/lib.rs**

```rust
//! Nova Engine 动画模块
//!
//! 提供骨骼动画和动画控制。

pub mod clip;
pub mod player;
pub mod plugin;

pub mod prelude {
    pub use crate::clip::{AnimationClip, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex};
    pub use crate::player::{
        play_animation, stop_animation, toggle_pause, NovaAnimationPlayer,
    };
    pub use crate::plugin::NovaAnimationPlugin;
    pub use bevy::animation::{AnimationPlayer, AnimationTransitions};
}
```

- [ ] **Step 4: 验证编译**

Run: `cargo check -p nova_animation`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add crates/nova_animation
git commit -m "feat(nova_animation): implement animation player"
```

---

## Chunk 7: Nova Engine 入口和示例

### Task 19: 创建 nova_engine 统一入口

**Files:**
- Create: `crates/nova_engine/Cargo.toml`
- Create: `crates/nova_engine/src/lib.rs`

- [ ] **Step 1: 创建 nova_engine/Cargo.toml**

```toml
[package]
name = "nova_engine"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
nova_core = { workspace = true }
nova_render = { workspace = true }
nova_physics = { workspace = true }
nova_ui = { workspace = true }
nova_animation = { workspace = true }
bevy = { workspace = true }
log = { workspace = true }
```

- [ ] **Step 2: 创建 nova_engine/src/lib.rs**

```rust
//! # Nova Engine
//!
//! A Web 3D game engine built with Rust and WebAssembly.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use nova_engine::prelude::*;
//!
//! fn main() {
//!     Nova::app()
//!         .add_plugin(NovaDefaults)
//!         .add_system(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut cmd: Commands) {
//!     cmd.spawn(Camera3d::new());
//!     cmd.spawn(DirectionalLight::sun());
//! }
//! ```

pub use nova_core as core;
pub use nova_render as render;
pub use nova_physics as physics;
pub use nova_ui as ui;
pub use nova_animation as animation;

use bevy::prelude::*;

/// 默认插件集
pub struct NovaDefaults;

impl Plugin for NovaDefaults {
    fn build(&self, app: &mut App) {
        app.add_plugins(nova_render::prelude::NovaRenderPlugin)
            .add_plugins(nova_physics::prelude::NovaPhysicsPlugin)
            .add_plugins(nova_ui::prelude::NovaUiPlugin)
            .add_plugins(nova_animation::prelude::NovaAnimationPlugin);
    }
}

/// Nova Engine Prelude - 导入常用类型
pub mod prelude {
    // Core
    pub use nova_core::prelude::*;

    // Render
    pub use nova_render::prelude::*;

    // Physics
    pub use nova_physics::prelude::*;

    // UI
    pub use nova_ui::prelude::*;

    // Animation
    pub use nova_animation::prelude::*;

    // Engine
    pub use crate::NovaDefaults;

    // Re-export common Bevy types
    pub use bevy::prelude::{
        Assets, AssetServer, Handle, Color, KeyCode, MouseButton,
        ButtonInput, Time, Name, Parent, Children,
    };
    pub use bevy::input::keyboard::KeyCode as Key;
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check -p nova_engine`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add crates/nova_engine
git commit -m "feat(nova_engine): add unified engine entry point"
```

---

### Task 20: 创建基础示例项目

**Files:**
- Create: `examples/basic_demo/Cargo.toml`
- Create: `examples/basic_demo/src/main.rs`
- Create: `examples/basic_demo/index.html`
- Create: `examples/basic_demo/Trunk.toml`
- Create: `examples/basic_demo/assets/.gitkeep`

- [ ] **Step 1: 创建 examples/basic_demo/Cargo.toml**

```toml
[package]
name = "basic_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
nova_engine = { workspace = true }
bevy = { workspace = true }
```

- [ ] **Step 2: 创建 examples/basic_demo/src/main.rs**

```rust
//! Nova Engine 基础示例
//!
//! 展示基础的 3D 场景渲染和物理交互。

use nova_engine::prelude::*;

fn main() {
    Nova::app()
        .add_plugin(NovaDefaults)
        .add_system(Startup, setup)
        .add_system(Update, handle_input)
        .add_system(Update, draw_ui)
        .run();
}

/// 场景设置
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::render::mesh::Mesh>>,
    mut materials: ResMut<Assets<bevy::pbr::StandardMaterial>>,
) {
    // 相机
    commands.spawn(Camera3d::new().position(Vec3::new(0.0, 8.0, 15.0)).looking_at(Vec3::ZERO));

    // 光照
    commands.spawn(DirectionalLight::sun());

    // 地面
    commands.spawn((
        bevy::pbr::PbrBundle {
            mesh: meshes.add(Mesh::plane(20.0).to_bevy_mesh()),
            material: materials.add(Material::new().color(Color::srgb(0.3, 0.5, 0.3)).to_bevy_material()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
    ));

    // 动态立方体
    for i in 0..5 {
        let color = Color::hsl(i as f32 * 60.0, 0.7, 0.5);
        commands.spawn((
            bevy::pbr::PbrBundle {
                mesh: meshes.add(Mesh::cube(1.0).to_bevy_mesh()),
                material: materials.add(Material::new().color(color).to_bevy_material()),
                transform: Transform::from_xyz(i as f32 * 1.5 - 3.0, 5.0 + i as f32 * 2.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
        ));
    }
}

/// 输入处理
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::render::mesh::Mesh>>,
    mut materials: ResMut<Assets<bevy::pbr::StandardMaterial>>,
) {
    // 按空格键生成新的球体
    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn((
            bevy::pbr::PbrBundle {
                mesh: meshes.add(Mesh::sphere(0.5).to_bevy_mesh()),
                material: materials.add(Material::new().color(Color::srgb(1.0, 0.3, 0.3)).to_bevy_material()),
                transform: Transform::from_xyz(0.0, 10.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.5),
        ));
    }
}

/// 绘制 UI
fn draw_ui(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::Area::new(egui::Id::new("instructions"))
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            ui.label("Nova Engine Demo");
            ui.separator();
            ui.label("Press SPACE to spawn a ball");
        });
}
```

- [ ] **Step 3: 创建 examples/basic_demo/index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nova Engine - Basic Demo</title>
    <style>
        html, body {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            background: #1a1a2e;
        }
        canvas {
            display: block;
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <canvas id="bevy"></canvas>
    <link data-trunk rel="rust" data-wasm-opt="z" />
</body>
</html>
```

- [ ] **Step 4: 创建 examples/basic_demo/Trunk.toml**

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
ignore = ["dist", "target"]

[serve]
address = "127.0.0.1"
port = 8080
open = true
```

- [ ] **Step 5: 创建 assets 目录**

Run: `mkdir -p examples/basic_demo/assets && touch examples/basic_demo/assets/.gitkeep`

- [ ] **Step 6: 验证编译**

Run: `cargo check -p basic_demo`
Expected: 编译成功

- [ ] **Step 7: Commit**

```bash
git add examples/basic_demo
git commit -m "feat: add basic demo example"
```

---

### Task 21: 添加 CI 配置

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: 创建 .github/workflows/ci.yml**

```yaml
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Check
        run: cargo check --all-targets

      - name: Check WASM
        run: cargo check --target wasm32-unknown-unknown -p nova_engine -p basic_demo

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets -- -D warnings
```

- [ ] **Step 2: Commit**

```bash
git add .github
git commit -m "ci: add GitHub Actions workflow"
```

---

### Task 22: 最终验证

- [ ] **Step 1: 完整构建检查**

Run: `cargo check --all-targets`
Expected: 全部编译成功

- [ ] **Step 2: WASM 构建检查**

Run: `cargo check --target wasm32-unknown-unknown -p basic_demo`
Expected: WASM 目标编译成功

- [ ] **Step 3: 运行 demo（本地测试）**

Run: `cd examples/basic_demo && trunk serve`
Expected: 浏览器打开 http://localhost:8080，显示 3D 场景

- [ ] **Step 4: 最终提交**

```bash
git add -A
git commit -m "feat: complete Nova Engine MVP implementation"
```

---

## Summary

完成以上 22 个任务后，Nova Engine MVP 将具备：

1. **核心系统** - NovaApp、插件系统、调度阶段
2. **渲染系统** - 相机、光照、网格、材质
3. **物理系统** - 刚体、碰撞器、碰撞事件
4. **UI 系统** - egui 集成、游戏 UI 组件
5. **动画系统** - 动画播放器、动画控制
6. **示例项目** - 可运行的 3D demo
7. **CI/CD** - GitHub Actions 自动化

预计总耗时：4-6 小时
