//! Nova UI Prelude

pub use crate::context::{apply_ui_theme, UiContextExt, UiState, UiTheme};
pub use crate::widgets::{DebugPanel, FpsDisplay, NovaButton, PropertyEditor};
pub use crate::NovaUiPlugin;

// 重导出常用的 egui 类型
pub use bevy_egui::{egui, EguiContexts};
