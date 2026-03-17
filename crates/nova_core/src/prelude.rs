//! Nova Core Prelude - 常用类型重导出

pub use crate::app::NovaApp;
pub use crate::components::{EntityName, GameTime, Static, Visible};
pub use crate::input::{
    helpers as input_helpers, InputAction, InputActions, InputAxes, InputAxis, InputState,
    NovaInputPlugin,
};
pub use crate::plugin::{NovaCorePlugin, NovaDefaultPlugins, NovaMinimalPlugins};
pub use crate::schedule::{NovaSystemSet, Schedules};

// 重导出 Bevy prelude
pub use bevy::prelude::*;
