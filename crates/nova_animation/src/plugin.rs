//! Nova 动画插件

use bevy::prelude::*;

use crate::tween::update_position_tweens;

/// Nova 动画插件
pub struct NovaAnimationPlugin;

impl Plugin for NovaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position_tweens);
    }
}
