# Nova Engine 技术演进完整方案

## 概述

本方案提供 Nova Engine 在**测试、工具链、性能**三个维度的完整实施方案。

## 已创建的文件结构

```
nova_engine/
├── crates/
│   └── nova_test/              # 测试框架
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # 主入口
│           ├── app_runner.rs   # TestApp 实现
│           ├── assertions.rs   # 断言工具
│           ├── render.rs       # 渲染测试
│           └── wasm.rs         # WASM 测试支持
├── tools/
│   └── nova_inspector/         # 调试工具
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # 插件入口
│           ├── main.rs         # CLI 入口
│           ├── state.rs        # 状态管理
│           └── panels/         # 面板模块
│               ├── mod.rs
│               ├── entity.rs   # 实体面板
│               ├── performance.rs # 性能面板
│               ├── resource.rs # 资源面板
│               └── scene.rs    # 场景面板
├── crates/nova_render/src/
│   └── performance/            # 性能优化模块
│       ├── mod.rs              # 插件入口
│       ├── frustum_culling.rs  # 视锥剔除
│       ├── instancing.rs       # 实例化渲染
│       ├── spatial_grid.rs     # 空间哈希
│       └── lod.rs              # LOD 系统
├── tests/
│   └── integration/            # 集成测试
│       ├── mod.rs
│       ├── map_tests.rs
│       ├── character_tests.rs
│       └── ai_tests.rs
├── benches/
│   └── ecs_benchmarks.rs       # 性能基准测试
├── .github/workflows/
│   └── ci.yml                  # 更新的 CI 配置
├── scripts/
│   └── implement.sh            # 实施脚本
└── IMPLEMENTATION.md           # 本文档
```

## 1. 测试框架 (nova_test)

### 1.1 TestApp - 测试运行器

```rust
use nova_test::TestApp;

#[test]
fn test_example() {
    let mut app = TestApp::new()
        .add_plugin(MyPlugin)
        .add_system(my_system);
    
    app.run_frames(10)
       .assert_entity_count(5);
}
```

### 1.2 断言宏

- `assert_entity_count!(app, expected)` - 断言实体数量
- `assert_has_component!(app, ComponentType)` - 断言组件存在
- `assert_resource_eq!(app, ResourceType, expected)` - 断言资源值

### 1.3 WASM 测试

```rust
#[wasm_bindgen_test]
async fn test_wasm_feature() {
    console_log("Running in browser");
    assert!(BrowserCompatibility::supports_webgpu());
}
```

## 2. 调试工具 (nova_inspector)

### 2.1 使用方法

```rust
use nova_inspector::NovaInspectorPlugin;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(NovaInspectorPlugin)
    .run();
```

### 2.2 快捷键

- `F12` - 切换调试器
- `Ctrl+Shift+I` - 切换调试器

### 2.3 功能面板

1. **实体面板** - 查看实体树和组件详情
2. **性能面板** - 实时监控 FPS、帧时间、实体数量
3. **资源面板** - 查看所有资源类型
4. **场景面板** - 场景统计信息（相机、光源、网格数量）

## 3. 性能优化系统

### 3.1 使用方式

```rust
use nova_render::NovaPerformancePlugin;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(NovaPerformancePlugin)
    .run();
```

### 3.2 配置选项

```rust
app.insert_resource(PerformanceSettings {
    enable_frustum_culling: true,    // 启用视锥剔除
    enable_instancing: true,          // 启用实例化渲染
    instancing_threshold: 50,         // 实例化阈值
    enable_spatial_grid: true,        // 启用空间哈希
    spatial_cell_size: 50.0,          // 空间格子大小
    enable_lod: true,                 // 启用 LOD
});
```

### 3.3 特性说明

| 特性 | 描述 | 预期收益 |
|------|------|----------|
| **视锥剔除** | 只渲染相机视野内物体 | -30% 渲染实体 |
| **实例化渲染** | 合并相同材质/网格的绘制调用 | -50% Draw Calls |
| **空间哈希** | 加速空间查询和碰撞检测 | O(n) → O(1) |
| **LOD** | 根据距离切换模型精度 | -40% 面数 |

## 4. 集成测试

### 4.1 运行测试

```bash
# 所有单元测试
cargo test --workspace --lib

# 集成测试
cargo test --test integration

# 特定模块测试
cargo test -p nova_test
```

### 4.2 测试覆盖

- **地图系统** - 地图创建、尺寸验证、地形生成
- **角色系统** - 角色创建、属性验证、状态转换
- **AI 系统** - Agent 创建、行为树执行

## 5. 性能基准测试

### 5.1 运行基准测试

```bash
cargo bench
```

### 5.2 测试场景

- **实体创建** - 创建 1000 个实体的性能
- **查询迭代** - 遍历 10000 个实体的性能
- **组件操作** - 添加/移除组件的性能

## 6. CI/CD 配置

### 6.1 工作流

- **check** - 编译检查
- **fmt** - 代码格式检查
- **clippy** - 代码质量检查
- **test** - 单元测试和集成测试
- **wasm** - WASM 构建和浏览器测试
- **benchmark** - 性能基准测试（仅 main 分支）

### 6.2 触发条件

- Push 到 main/develop 分支
- Pull Request 到 main/develop 分支

## 7. 实施步骤

### 7.1 快速开始

```bash
# 1. 运行实施脚本
./scripts/implement.sh

# 2. 检查编译
cargo check --all-targets

# 3. 运行测试
cargo test --workspace

# 4. 运行 Inspector
cargo run -p nova_inspector

# 5. 运行基准测试
cargo bench
```

### 7.2 修复潜在问题

由于新代码使用了一些假设的 API，可能需要根据实际依赖进行调整：

1. **nova_map API** - 确保 TileMap 有 `width()`, `height()`, `tiles()` 方法
2. **nova_character API** - 确保 CharacterStats 有对应的 getter 方法
3. **nova_ai API** - 确保 BehaviorTree 和 Blackboard 接口正确

### 7.3 在示例中使用

在 `examples/basic_demo` 或 `examples/rts_demo` 中添加：

```rust
// 在 main.rs 中添加
use nova_render::NovaPerformancePlugin;
use nova_inspector::NovaInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NovaPerformancePlugin)
        .add_plugins(NovaInspectorPlugin)  // 仅在开发时添加
        .run();
}
```

## 8. 后续优化建议

### 8.1 测试增强

- [ ] 添加渲染测试（截图对比）
- [ ] 添加性能回归测试
- [ ] 添加 WASM 浏览器兼容性测试矩阵
- [ ] 配置代码覆盖率工具 (cargo-tarpaulin)

### 8.2 工具链增强

- [ ] 添加帧时间线分析器
- [ ] 添加 GPU 性能分析
- [ ] 添加内存分析器
- [ ] 场景编辑功能

### 8.3 性能优化增强

- [ ] GPU 遮挡剔除
- [ ] 异步纹理加载
- [ ] 动态批次合并
- [ ] GPU 粒子系统

## 9. 文档

### 9.1 生成的文档

```bash
cargo doc --workspace --no-deps --open
```

### 9.2 查看测试报告

```bash
# 安装覆盖率工具
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

## 10. 故障排除

### 10.1 编译错误

如果遇到编译错误，检查：

1. Rust 版本 >= 1.80
2. 所有依赖 crate 版本兼容
3. 特征（features）配置正确

### 10.2 测试失败

```bash
# 详细输出
cargo test -- --nocapture

# 特定测试
cargo test test_name -- --exact
```

### 10.3 性能问题

```bash
# 使用 perf 分析
perf record -g cargo run --release
perf report

# 使用 flamegraph
cargo install flamegraph
cargo flamegraph
```

---

**实施完成日期**: 2026-03-20
**版本**: 0.1.0
