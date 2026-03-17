//! Nova 动画插件

use bevy::prelude::*;

use crate::clip::AnimationClips;
use crate::player::{update_animation_players, AnimationFinished};
use crate::tween::update_position_tweens;

/// Nova 动画插件
pub struct NovaAnimationPlugin;

impl Plugin for NovaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationClips>()
            .add_event::<AnimationFinished>()
            .add_systems(Update, (update_position_tweens, update_animation_players));
    }
}
