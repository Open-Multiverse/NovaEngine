//! 音频插件

use bevy::prelude::*;

use crate::source::{AudioSettings, AudioState};

/// Nova 音频插件
pub struct NovaAudioPlugin;

impl Plugin for NovaAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSettings>()
            .init_resource::<AudioState>()
            .add_event::<AudioEvent>()
            .add_systems(Update, process_audio_events);
    }
}

/// 音频事件
#[derive(Event, Debug, Clone)]
pub enum AudioEvent {
    /// 播放音效
    PlaySound { name: String, volume: f32 },
    /// 播放背景音乐
    PlayMusic { name: String, looping: bool },
    /// 停止背景音乐
    StopMusic,
    /// 暂停所有音频
    PauseAll,
    /// 恢复所有音频
    ResumeAll,
    /// 设置主音量
    SetMasterVolume(f32),
}

/// 处理音频事件
fn process_audio_events(
    mut events: EventReader<AudioEvent>,
    mut settings: ResMut<AudioSettings>,
    mut state: ResMut<AudioState>,
) {
    for event in events.read() {
        match event {
            AudioEvent::PlaySound { name, volume } => {
                state.last_played_sound = Some(name.clone());
                state.last_sound_volume = *volume;
            }
            AudioEvent::PlayMusic { name, looping } => {
                state.current_music = Some(name.clone());
                state.music_looping = *looping;
                state.music_playing = true;
            }
            AudioEvent::StopMusic => {
                state.music_playing = false;
            }
            AudioEvent::PauseAll => {
                state.paused = true;
            }
            AudioEvent::ResumeAll => {
                state.paused = false;
            }
            AudioEvent::SetMasterVolume(vol) => {
                settings.master_volume = vol.clamp(0.0, 1.0);
            }
        }
    }
}
