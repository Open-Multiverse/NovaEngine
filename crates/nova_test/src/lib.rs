//! Nova Test - 测试工具库
//!
//! 提供游戏引擎测试的基础设施：
//! - `TestApp` - 自动化测试用的 App 构建器
//! - `TestWorld` - 世界状态断言
//! - `RenderTest` - 渲染测试支持
//! - `WASM` - WebAssembly 测试工具

pub mod app_runner;
pub mod assertions;
pub mod render;
pub mod wasm;

pub use app_runner::TestApp;
pub use assertions::*;

/// 测试工具集合
pub mod prelude {
    pub use super::TestApp;
    pub use super::assertions::*;
}
