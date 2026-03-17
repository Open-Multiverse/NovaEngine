//! 资源插件

use bevy::prelude::*;

use crate::loader::{AssetLoadState, AssetRegistry};

/// Nova 资源插件
pub struct NovaAssetsPlugin;

impl Plugin for NovaAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetRegistry>()
            .init_resource::<AssetLoadState>()
            .add_event::<AssetLoadedEvent>()
            .add_event::<AssetFailedEvent>();
    }
}

/// 资源加载完成事件
#[derive(Event, Debug, Clone)]
pub struct AssetLoadedEvent {
    /// 资源路径
    pub path: String,
    /// 资源类型
    pub asset_type: String,
}

/// 资源加载失败事件
#[derive(Event, Debug, Clone)]
pub struct AssetFailedEvent {
    /// 资源路径
    pub path: String,
    /// 错误信息
    pub error: String,
}
