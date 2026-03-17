# Nova Engine

基于 Rust 和 WebAssembly 构建的 Web 3D 游戏引擎。

## 特性

- **WebGPU 渲染** - 使用现代图形 API 实现高性能 3D 渲染
- **物理引擎** - 基于 Rapier3D 的物理模拟
- **UI 系统** - 使用 egui 实现即时模式 UI
- **动画系统** - 支持骨骼动画和 Tween 动画
- **音频系统** - 支持空间音频和音量控制
- **资源管理** - 异步资源加载和状态追踪
- **场景序列化** - JSON 格式场景保存/加载

## 快速开始

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

## 项目结构

```
nova_engine/
├── crates/
│   ├── nova_core/       # 核心类型、ECS 封装、App 生命周期
│   ├── nova_render/     # 渲染系统（相机、网格、材质、光照）
│   ├── nova_physics/    # 物理系统（刚体、碰撞器）
│   ├── nova_ui/         # UI 系统（egui 集成）
│   ├── nova_animation/  # 动画系统（Tween、关键帧）
│   ├── nova_audio/      # 音频系统（空间音频）
│   ├── nova_assets/     # 资源管理系统
│   └── nova_engine/     # 统一入口 crate
├── examples/            # 示例项目
│   ├── basic_demo/      # 基础演示
│   ├── physics_demo/    # 物理演示
│   ├── animation_demo/  # 动画演示
│   └── ui_demo/         # UI 演示
└── docs/                # 文档
```

## Web 构建

```bash
# 安装依赖
rustup target add wasm32-unknown-unknown
cargo install trunk

# 运行示例
trunk serve examples/basic_demo
```

## 开发命令

```bash
# 检查编译
cargo check --all-targets

# WASM 构建检查
cargo check --target wasm32-unknown-unknown -p nova_engine

# 运行测试
cargo test --all

# 运行基准测试
cargo bench

# 格式化代码
cargo fmt --all

# Lint 检查
cargo clippy --all-targets
```

## 技术栈

| 组件 | 技术 |
|------|------|
| 语言 | Rust + WebAssembly |
| 图形 API | WebGPU |
| ECS 运行时 | Bevy 0.15 |
| 物理引擎 | Rapier 3D |
| UI 框架 | egui |
| 构建工具 | Trunk, wasm-bindgen |

## 许可证

本项目采用双许可证：
- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../LICENSE-MIT))

您可以选择任一许可证使用本项目。
