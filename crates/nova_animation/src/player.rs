//! 动画播放器组件
//!
//! 提供动画播放控制

use bevy::prelude::*;

use crate::clip::{AnimationClip, AnimationClips};

/// 播放状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackState {
    /// 停止
    #[default]
    Stopped,
    /// 播放中
    Playing,
    /// 暂停
    Paused,
}

/// 动画播放器组件
#[derive(Component, Debug)]
pub struct AnimationPlayer {
    /// 当前播放的动画片段索引
    pub clip_index: Option<usize>,
    /// 播放状态
    pub state: PlaybackState,
    /// 当前播放时间
    pub current_time: f32,
    /// 播放速度（1.0 = 正常速度）
    pub speed: f32,
    /// 是否循环播放
    pub looping: bool,
    /// 播放完成后是否自动停止
    pub auto_stop: bool,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            clip_index: None,
            state: PlaybackState::Stopped,
            current_time: 0.0,
            speed: 1.0,
            looping: false,
            auto_stop: true,
        }
    }
}

impl AnimationPlayer {
    /// 创建新的动画播放器
    pub fn new() -> Self {
        Self::default()
    }

    /// 播放指定动画
    pub fn play(&mut self, clip_index: usize) -> &mut Self {
        self.clip_index = Some(clip_index);
        self.state = PlaybackState::Playing;
        self.current_time = 0.0;
        self
    }

    /// 播放动画并设置循环
    pub fn play_looping(&mut self, clip_index: usize) -> &mut Self {
        self.play(clip_index);
        self.looping = true;
        self
    }

    /// 暂停播放
    pub fn pause(&mut self) -> &mut Self {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
        self
    }

    /// 恢复播放
    pub fn resume(&mut self) -> &mut Self {
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
        self
    }

    /// 停止播放
    pub fn stop(&mut self) -> &mut Self {
        self.state = PlaybackState::Stopped;
        self.current_time = 0.0;
        self
    }

    /// 设置播放速度
    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    /// 跳转到指定时间
    pub fn seek(&mut self, time: f32) -> &mut Self {
        self.current_time = time.max(0.0);
        self
    }

    /// 是否正在播放
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing
    }

    /// 是否已暂停
    pub fn is_paused(&self) -> bool {
        self.state == PlaybackState::Paused
    }

    /// 是否已停止
    pub fn is_stopped(&self) -> bool {
        self.state == PlaybackState::Stopped
    }
}

/// 动画播放完成事件
#[derive(Event, Debug)]
pub struct AnimationFinished {
    pub entity: Entity,
    pub clip_index: usize,
}

/// 动画播放器更新系统
pub fn update_animation_players(
    time: Res<Time>,
    clips: Res<AnimationClips>,
    mut query: Query<(Entity, &mut AnimationPlayer, &mut Transform)>,
    mut finished_events: EventWriter<AnimationFinished>,
) {
    for (entity, mut player, mut transform) in &mut query {
        if player.state != PlaybackState::Playing {
            continue;
        }

        let Some(clip_index) = player.clip_index else {
            continue;
        };

        let Some(clip) = clips.get(clip_index) else {
            continue;
        };

        // 更新时间
        player.current_time += time.delta_secs() * player.speed;

        let duration = clip.duration();

        // 检查是否播放完成
        if player.current_time >= duration {
            if player.looping || clip.looping {
                // 循环播放
                player.current_time %= duration;
            } else {
                // 播放完成
                player.current_time = duration;
                if player.auto_stop {
                    player.state = PlaybackState::Stopped;
                }
                finished_events.send(AnimationFinished { entity, clip_index });
            }
        }

        // 采样动画并应用到 Transform
        let sampled = clip.sample(player.current_time, &transform);
        *transform = sampled;
    }
}

/// 简单动画构建器
pub struct SimpleAnimationBuilder {
    name: String,
    keyframes: Vec<(f32, Vec3, Option<Quat>)>,
    looping: bool,
}

impl SimpleAnimationBuilder {
    /// 创建新的构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            keyframes: Vec::new(),
            looping: false,
        }
    }

    /// 添加位置关键帧
    pub fn position_at(mut self, time: f32, position: Vec3) -> Self {
        self.keyframes.push((time, position, None));
        self
    }

    /// 添加位置和旋转关键帧
    pub fn transform_at(mut self, time: f32, position: Vec3, rotation: Quat) -> Self {
        self.keyframes.push((time, position, Some(rotation)));
        self
    }

    /// 设置循环
    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    /// 构建动画片段
    pub fn build(self) -> AnimationClip {
        let mut clip = AnimationClip::new(self.name);
        clip.looping = self.looping;

        let mut position_track = crate::clip::PositionTrack::new();
        let mut rotation_track = crate::clip::RotationTrack::new();
        let mut has_rotation = false;

        for (time, pos, rot) in self.keyframes {
            position_track.add_keyframe(time, pos);
            if let Some(r) = rot {
                rotation_track.add_keyframe(time, r);
                has_rotation = true;
            }
        }

        clip = clip.with_position(position_track);
        if has_rotation {
            clip = clip.with_rotation(rotation_track);
        }

        clip
    }
}
