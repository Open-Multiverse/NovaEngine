//! WASM 测试支持
//!
//! 提供 WebAssembly 环境下的测试工具

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

/// WASM 测试配置
#[cfg(target_arch = "wasm32")]
pub struct WasmTestConfig {
    /// 浏览器超时时间（毫秒）
    pub timeout_ms: u32,
}

#[cfg(target_arch = "wasm32")]
impl Default for WasmTestConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000, // 30秒
        }
    }
}

/// WASM 日志输出
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// 在浏览器控制台打印消息
#[cfg(target_arch = "wasm32")]
pub fn console_log(msg: &str) {
    log(msg);
}

/// 检查是否在 WASM 环境中
pub const IS_WASM: bool = cfg!(target_arch = "wasm32");

/// WASM 测试断言宏
#[macro_export]
macro_rules! wasm_assert {
    ($cond:expr) => {{
        if cfg!(target_arch = "wasm32") {
            wasm_bindgen_test::wasm_bindgen_test!($cond);
        } else {
            assert!($cond);
        }
    }};
}

/// WASM 测试辅助函数
#[cfg(target_arch = "wasm32")]
pub mod wasm_utils {
    use super::*;
    
    /// 异步等待指定毫秒
    pub async fn sleep(ms: u32) {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        ms as i32,
                    )
                    .unwrap();
            });
        
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    }
    
    /// 获取当前内存使用情况
    pub fn get_memory_usage() -> Option<usize> {
        let performance = web_sys::window()?.performance()?;
        // 注意：这需要 memory API 支持
        None
    }
}

/// 浏览器兼容性检查
pub struct BrowserCompatibility;

impl BrowserCompatibility {
    /// 检查是否支持 WebGPU
    #[cfg(target_arch = "wasm32")]
    pub fn supports_webgpu() -> bool {
        web_sys::window()
            .and_then(|w| w.navigator().gpu())
            .is_some()
    }
    
    /// 检查是否支持 WebGL2
    #[cfg(target_arch = "wasm32")]
    pub fn supports_webgl2() -> bool {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("canvas").ok())
            .and_then(|c| {
                c.dyn_into::<web_sys::HtmlCanvasElement>()
                    .ok()
                    .and_then(|c| c.get_context("webgl2").ok().flatten())
            })
            .is_some()
    }
    
    /// 在非 WASM 环境中始终返回 true
    #[cfg(not(target_arch = "wasm32"))]
    pub fn supports_webgpu() -> bool {
        true
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn supports_webgl2() -> bool {
        true
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    
    #[wasm_bindgen_test]
    fn test_wasm_environment() {
        console_log("Running in WASM browser environment");
        assert!(IS_WASM);
    }
    
    #[wasm_bindgen_test]
    fn test_browser_compatibility() {
        let webgpu = BrowserCompatibility::supports_webgpu();
        let webgl2 = BrowserCompatibility::supports_webgl2();
        
        console_log(&format!("WebGPU support: {}", webgpu));
        console_log(&format!("WebGL2 support: {}", webgl2));
        
        // 至少应该支持一种渲染后端
        assert!(webgpu || webgl2, "Browser must support WebGPU or WebGL2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_wasm() {
        // 在原生测试中，IS_WASM 应该为 false
        #[cfg(not(target_arch = "wasm32"))]
        assert!(!IS_WASM);
    }
}
