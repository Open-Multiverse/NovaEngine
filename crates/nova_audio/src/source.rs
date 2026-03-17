//! 音频源组件和资源

use bevy::prelude::*;

/// 音频设置资源
#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    /// 主音量 (0.0 - 1.0)
    pub master_volume: f32,
    /// 音乐音量 (0.0 - 1.0)
    pub music_volume: f32,
    /// 音效音量 (0.0 - 1.0)
    pub sfx_volume: f32,
    /// 是否静音
    pub muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.8,
            sfx_volume: 1.0,
            muted: false,
        }
    }
}

impl AudioSettings {
    /// 获取实际音乐音量
    pub fn effective_music_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.master_volume * self.music_volume
        }
    }

    /// 获取实际音效音量
    pub fn effective_sfx_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.master_volume * self.sfx_volume
        }
    }
}

/// 音频状态资源
#[derive(Resource, Debug, Default)]
pub struct AudioState {
    /// 当前播放的音乐
    pub current_music: Option<String>,
    /// 音乐是否循环
    pub music_looping: bool,
    /// 音乐是否正在播放
    pub music_playing: bool,
    /// 是否暂停
    pub paused: bool,
    /// 最后播放的音效
    pub last_played_sound: Option<String>,
    /// 最后音效音量
    pub last_sound_volume: f32,
}

/// 音频源组件 - 用于空间音频
#[derive(Component, Debug)]
pub struct AudioSource {
    /// 音频名称/路径
    pub name: String,
    /// 音量
    pub volume: f32,
    /// 是否循环
    pub looping: bool,
    /// 是否自动播放
    pub autoplay: bool,
    /// 空间音频设置
    pub spatial: Option<SpatialAudioSettings>,
}

impl Default for AudioSource {
    fn default() -> Self {
        Self {
            name: String::new(),
            volume: 1.0,
            looping: false,
            autoplay: false,
            spatial: None,
        }
    }
}

impl AudioSource {
    /// 创建新的音频源
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    /// 设置音量
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    /// 设置循环
    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    /// 设置自动播放
    pub fn autoplay(mut self) -> Self {
        self.autoplay = true;
        self
    }

    /// 设置空间音频
    pub fn spatial(mut self, settings: SpatialAudioSettings) -> Self {
        self.spatial = Some(settings);
        self
    }
}

/// 空间音频设置
#[derive(Debug, Clone)]
pub struct SpatialAudioSettings {
    /// 最小距离（在此距离内音量最大）
    pub min_distance: f32,
    /// 最大距离（超过此距离音量为 0）
    pub max_distance: f32,
    /// 衰减曲线指数
    pub rolloff_factor: f32,
}

impl Default for SpatialAudioSettings {
    fn default() -> Self {
        Self {
            min_distance: 1.0,
            max_distance: 50.0,
            rolloff_factor: 1.0,
        }
    }
}

impl SpatialAudioSettings {
    /// 计算指定距离处的音量衰减
    pub fn calculate_attenuation(&self, distance: f32) -> f32 {
        if distance <= self.min_distance {
            1.0
        } else if distance >= self.max_distance {
            0.0
        } else {
            let normalized = (distance - self.min_distance) / (self.max_distance - self.min_distance);
            (1.0 - normalized).powf(self.rolloff_factor)
        }
    }
}

/// 音频监听器组件（通常附加到相机上）
#[derive(Component, Debug, Default)]
pub struct AudioListener;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_settings_default() {
        let settings = AudioSettings::default();
        assert_eq!(settings.master_volume, 1.0);
        assert_eq!(settings.music_volume, 0.8);
        assert_eq!(settings.sfx_volume, 1.0);
        assert!(!settings.muted);
    }

    #[test]
    fn test_effective_volume() {
        let mut settings = AudioSettings::default();
        assert!((settings.effective_music_volume() - 0.8).abs() < 0.001);
        assert!((settings.effective_sfx_volume() - 1.0).abs() < 0.001);

        settings.muted = true;
        assert_eq!(settings.effective_music_volume(), 0.0);
        assert_eq!(settings.effective_sfx_volume(), 0.0);
    }

    #[test]
    fn test_effective_volume_with_master() {
        let mut settings = AudioSettings::default();
        settings.master_volume = 0.5;
        assert!((settings.effective_music_volume() - 0.4).abs() < 0.001);
        assert!((settings.effective_sfx_volume() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_audio_source_builder() {
        let source = AudioSource::new("music.ogg")
            .with_volume(0.7)
            .looping()
            .autoplay();

        assert_eq!(source.name, "music.ogg");
        assert!((source.volume - 0.7).abs() < 0.001);
        assert!(source.looping);
        assert!(source.autoplay);
    }

    #[test]
    fn test_spatial_audio_attenuation() {
        let spatial = SpatialAudioSettings {
            min_distance: 1.0,
            max_distance: 10.0,
            rolloff_factor: 1.0,
        };

        assert_eq!(spatial.calculate_attenuation(0.5), 1.0);
        assert_eq!(spatial.calculate_attenuation(1.0), 1.0);
        assert!((spatial.calculate_attenuation(5.5) - 0.5).abs() < 0.001);
        assert_eq!(spatial.calculate_attenuation(10.0), 0.0);
        assert_eq!(spatial.calculate_attenuation(15.0), 0.0);
    }
}
