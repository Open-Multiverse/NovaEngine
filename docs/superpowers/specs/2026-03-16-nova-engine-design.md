# Nova Engine 设计文档

> 基于 Bevy 的 Web 3D 游戏引擎，面向开源社区

## 概述

Nova Engine 是一个功能完整的 Web 3D 游戏引擎，使用 Rust + WebAssembly 构建，以 WebGPU 作为图形后端。引擎基于 Bevy 作为底层运行时，提供独立稳定的 API 层，隔离底层依赖变动。

### 项目目标

- **面向实际开发**：可用于生产级 Web 3D 游戏开发
- **开源社区友好**：完善文档、模块化设计、宽松许可证
- **API 稳定**：独立 API 层，不受 Bevy 版本变动影响

### 技术选型

| 项目 | 选择 |
|------|------|
| 语言 | Rust + WebAssembly |
| 图形 API | WebGPU |
| ECS 运行时 | Bevy |
| 物理引擎 | Rapier 3D |
| UI 框架 | egui |
| 音频引擎 | kira |

## 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                      Nova Engine API                        │
│  (稳定的公共接口，用户代码只接触这一层)                          │
├─────────────────────────────────────────────────────────────┤
│                     Nova Adapter Layer                      │
│  (将 Nova API 映射到 Bevy，隔离 Bevy 版本变动)                  │
├──────────┬──────────┬──────────┬──────────┬────────────────┤
│ Renderer │ Physics  │ Animation│   UI     │   Audio        │
│ (wgpu)   │ (Rapier) │ (Bevy)   │ (egui)   │   (kira)       │
├──────────┴──────────┴──────────┴──────────┴────────────────┤
│                    Bevy ECS Runtime                         │
├─────────────────────────────────────────────────────────────┤
│                  WebAssembly + WebGPU                       │
└─────────────────────────────────────────────────────────────┘
```

### 架构方案：独立 API + Bevy 后端

设计独立的 Nova API，内部使用 Bevy 作为实现：

- **API 稳定性**：用户代码只依赖 `nova_engine` crate，不直接依赖 Bevy
- **语义版本控制**：Nova API 遵循 semver，Bevy 升级在适配层处理
- **渐进暴露**：高级用户可选择性访问底层 Bevy 功能

## 项目结构

```
nova_engine/
├── crates/
│   ├── nova_core/          # 核心类型、ECS 封装、App 生命周期
│   ├── nova_render/        # 渲染系统（场景图、材质、光照、相机）
│   ├── nova_physics/       # 物理系统（碰撞、刚体、触发器）
│   ├── nova_animation/     # 动画系统（骨骼、混合、状态机）
│   ├── nova_ui/            # UI 系统（egui 封装、游戏 UI 组件）
│   ├── nova_audio/         # 音频系统（3D 音效、背景音乐）
│   ├── nova_script/        # 脚本系统（Lua/JS 绑定）
│   ├── nova_net/           # 网络多人（同步、服务器架构）
│   ├── nova_ai/            # AI 系统（寻路、行为树）
│   ├── nova_vfx/           # 粒子特效系统
│   ├── nova_assets/        # 资产管线（glTF、纹理、热重载）
│   └── nova_editor/        # 场景编辑器（后续版本）
├── nova_engine/            # 统一入口 crate（re-export 所有模块）
├── examples/               # 示例项目
├── docs/                   # 文档、教程
├── tools/                  # 开发者工具（CLI、调试面板）
└── web/                    # Web 构建配置、HTML 模板
```

## API 设计

### 用户代码示例

```rust
use nova_engine::prelude::*;

fn main() {
    Nova::app()
        .add_plugin(NovaDefaults)      // 默认插件集（渲染、物理、UI、动画）
        .add_system(Startup, setup)     // 启动系统
        .add_system(Update, game_loop)  // 每帧更新
        .run();
}

fn setup(mut cmd: Commands, assets: Assets) {
    // 加载场景
    cmd.spawn_scene(assets.load("levels/demo.gltf"));

    // 创建相机
    cmd.spawn(Camera3d::default().looking_at(Vec3::ZERO));

    // 添加光源
    cmd.spawn(DirectionalLight::sun());

    // 创建带物理的物体
    cmd.spawn((
        Mesh::cube(1.0),
        Material::default(),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}

fn game_loop(input: Input, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        if input.pressed(Key::W) {
            transform.translate_forward(0.1);
        }
    }
}
```

### API 设计原则

- **零样板代码**：最小化启动一个 3D 场景只需几行
- **组合优于继承**：使用 ECS 组件组合功能
- **合理默认值**：常用配置有良好默认，高级用户可覆盖
- **类型安全**：充分利用 Rust 类型系统防止错误

## MVP 功能规格

### 渲染系统 (nova_render)

| 功能 | 描述 |
|------|------|
| PBR 材质 | 基于物理的渲染，金属度/粗糙度工作流 |
| 光照 | 方向光、点光源、聚光灯、环境光 |
| 阴影 | 级联阴影贴图 (CSM) |
| 天空盒 | HDR 环境贴图、程序化天空 |
| 相机 | 透视/正交、后处理栈 |
| glTF 加载 | 完整支持 glTF 2.0 |

### 物理系统 (nova_physics)

| 功能 | 描述 |
|------|------|
| 刚体 | 动态、静态、运动学 |
| 碰撞器 | 盒子、球体、胶囊、网格 |
| 碰撞检测 | 事件回调、触发器 |
| 射线检测 | Raycast、形状投射 |
| 关节 | 固定、铰链、弹簧 |

### UI 系统 (nova_ui)

| 功能 | 描述 |
|------|------|
| 即时模式 UI | 基于 egui 封装 |
| 游戏 HUD | 血条、小地图、准星 |
| 菜单系统 | 主菜单、暂停、设置 |
| 响应式布局 | 适配不同分辨率 |

### 动画系统 (nova_animation)

| 功能 | 描述 |
|------|------|
| 骨骼动画 | glTF 动画导入 |
| 动画播放 | 播放、暂停、循环、速度控制 |
| 动画混合 | 淡入淡出过渡 |
| 动画事件 | 关键帧回调 |

## 完整功能路线图

MVP 之后逐步实现的功能：

| 模块 | 功能 |
|------|------|
| nova_audio | 3D 空间音效、背景音乐、音量控制 |
| nova_script | Lua/JavaScript 脚本绑定 |
| nova_net | 多人游戏同步、客户端-服务器架构 |
| nova_ai | A* 寻路、行为树、有限状态机 |
| nova_vfx | 粒子系统、火焰、烟雾、魔法效果 |
| nova_assets | 资产管线、热重载、资产打包 |
| nova_editor | 可视化场景编辑器 |

## 开源社区支持

### 文档体系

| 类型 | 内容 |
|------|------|
| API 文档 | 自动生成的 rustdoc，每个公共 API 都有示例 |
| 入门教程 | "5分钟创建第一个游戏"系列 |
| 概念指南 | ECS 入门、渲染管线、物理系统原理 |
| 示例项目 | 从简单到复杂的 5-10 个完整示例 |

### 模块化与插件

```rust
// 用户可选择性引入模块
use nova_engine::{core, render, physics}; // 只用需要的

// 第三方插件接口
impl NovaPlugin for MyPlugin {
    fn build(&self, app: &mut NovaApp) {
        app.add_system(Update, my_system);
    }
}
```

### 开发者工具

| 工具 | 描述 |
|------|------|
| 调试面板 | FPS、内存、实体数量、渲染统计 |
| 检查器 | 运行时查看/修改实体组件 |
| 性能分析 | 火焰图、系统耗时分析 |
| 热重载 | 资产修改自动刷新（开发模式） |

### 社区基础设施

| 项目 | 内容 |
|------|------|
| 许可证 | MIT OR Apache-2.0（双许可） |
| 贡献指南 | CONTRIBUTING.md、代码规范、PR 流程 |
| Issue 模板 | Bug 报告、功能请求、问题咨询 |
| CI/CD | GitHub Actions 自动测试、文档部署 |
| 社区 | Discord 服务器、GitHub Discussions |

## 技术依赖

### 核心依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| bevy | 0.15+ | ECS、渲染、资产管理 |
| wgpu | (via bevy) | WebGPU 图形后端 |
| rapier3d | 0.22+ | 3D 物理引擎 |
| egui | 0.30+ | 即时模式 UI |
| kira | 0.9+ | 音频引擎 |
| glam | (via bevy) | 数学库 |

### Web 构建工具链

```bash
# 安装工具链
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

# 开发模式（热重载）
trunk serve

# 生产构建
trunk build --release
```

### 浏览器兼容性

| 浏览器 | WebGPU 支持 |
|--------|-------------|
| Chrome 113+ | 完全支持 |
| Edge 113+ | 完全支持 |
| Firefox | Nightly 开启 flag |
| Safari 18+ | 支持 |

## 配置文件清单

```
nova_engine/
├── Cargo.toml              # workspace 配置
├── rust-toolchain.toml     # Rust 版本锁定
├── .cargo/config.toml      # WASM 构建配置
├── Trunk.toml              # Web 构建配置
├── index.html              # Web 入口模板
└── .github/
    └── workflows/
        ├── ci.yml          # 持续集成
        └── docs.yml        # 文档部署
```

## 成功标准

MVP 版本完成标准：

1. **可运行**：能在 Chrome/Edge 中加载并渲染 3D 场景
2. **物理交互**：物体有碰撞和重力
3. **UI 显示**：能渲染基础 HUD 元素
4. **动画播放**：能播放 glTF 骨骼动画
5. **示例完整**：至少一个可玩的简单 3D 游戏 demo
6. **文档齐全**：API 文档 + 入门教程

---

*文档版本: 1.0*
*创建日期: 2026-03-16*
