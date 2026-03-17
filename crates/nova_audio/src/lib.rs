//! Nova Audio - 音频系统
//!
//! 提供游戏音频功能：
//! - 背景音乐播放
//! - 音效播放
//! - 空间音频（3D 声音定位）
//! - 音量控制（主音量、音乐、音效）
//!
//! # 快速开始
//!
//! ```ignore
//! use nova_audio::prelude::*;
//!
//! // 发送音频事件
//! fn play_sound(mut events: EventWriter<AudioEvent>) {
//!     events.send(AudioEvent::PlaySound {
//!         name: "explosion.ogg".to_string(),
//!         volume: 1.0,
//!     });
//! }
//!
//! // 播放背景音乐
//! fn play_music(mut events: EventWriter<AudioEvent>) {
//!     events.send(AudioEvent::PlayMusic {
//!         name: "theme.ogg".to_string(),
//!         looping: true,
//!     });
//! }
//!
//! // 设置音量
//! fn set_volume(mut events: EventWriter<AudioEvent>) {
//!     events.send(AudioEvent::SetMasterVolume(0.8));
//! }
//! ```
//!
//! # 空间音频
//!
//! ```ignore
//! use nova_audio::prelude::*;
//!
//! // 创建空间音频源
//! commands.spawn((
//!     AudioSource::new("engine.ogg")
//!         .looping()
//!         .spatial(SpatialAudioSettings {
//!             min_distance: 1.0,
//!             max_distance: 50.0,
//!             rolloff_factor: 1.0,
//!         }),
//!     Transform::from_xyz(10.0, 0.0, 0.0),
//! ));
//!
//! // 添加音频监听器（通常在相机上）
//! commands.spawn((Camera3d::default(), AudioListener));
//! ```
//!
//! # 模块说明
//!
//! - [`source`] - 音频源组件和设置
//! - [`plugin`] - 音频事件和系统

pub mod plugin;
pub mod prelude;
pub mod source;

pub use plugin::NovaAudioPlugin;
