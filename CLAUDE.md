# Nova Engine 项目指南

## 项目概述

Nova Engine 是一个基于 Bevy 的 Web 3D 游戏引擎，使用 Rust + WebAssembly 构建，以 WebGPU 作为图形后端。

## 语言规范

- **对话语言**：所有对话使用中文
- **文档语言**：所有文档使用中文
- **代码注释**：使用中文
- **Git 提交信息**：使用中文描述

## 技术栈

- **语言**: Rust + WebAssembly
- **图形 API**: WebGPU
- **ECS 运行时**: Bevy 0.15+
- **物理引擎**: Rapier 3D
- **UI 框架**: egui
- **构建工具**: Trunk, wasm-bindgen

## 项目结构

```
nova_engine/
├── crates/
│   ├── nova_core/          # 核心类型、ECS 封装、App 生命周期
│   ├── nova_render/        # 渲染系统
│   ├── nova_physics/       # 物理系统
│   ├── nova_ui/            # UI 系统
│   ├── nova_animation/     # 动画系统
│   └── nova_engine/        # 统一入口 crate
├── examples/               # 示例项目
├── docs/                   # 文档
└── web/                    # Web 构建配置
```

## 开发命令

```bash
# 检查编译
cargo check --all-targets

# WASM 构建检查
cargo check --target wasm32-unknown-unknown -p nova_engine

# 运行示例
cd examples/basic_demo && trunk serve

# 格式化代码
cargo fmt --all

# Lint 检查
cargo clippy --all-targets
```

## Git 提交规范

使用中文描述，格式：

```
<类型>: <简短描述>

<详细说明（可选）>
```

类型：
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具变更
- `ci`: CI/CD 配置

示例：
```
feat(nova_render): 实现相机组件

添加 Camera3d 组件，支持透视和正交投影
```

## 设计文档

- 规格文档：`docs/superpowers/specs/2026-03-16-nova-engine-design.md`
- 实施计划：`docs/superpowers/plans/2026-03-16-nova-engine-mvp.md`

## 许可证

MIT OR Apache-2.0 双许可
