//! 动画片段定义
//!
//! 提供动画片段的数据结构和管理

use bevy::prelude::*;

/// 动画关键帧
#[derive(Debug, Clone)]
pub struct Keyframe<T: Clone> {
    /// 时间点（秒）
    pub time: f32,
    /// 关键帧值
    pub value: T,
}

impl<T: Clone> Keyframe<T> {
    pub fn new(time: f32, value: T) -> Self {
        Self { time, value }
    }
}

/// 动画轨道 - 存储特定属性的关键帧序列
#[derive(Debug, Clone)]
pub struct AnimationTrack<T: Clone> {
    /// 关键帧列表（按时间排序）
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Clone> Default for AnimationTrack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> AnimationTrack<T> {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    /// 添加关键帧
    pub fn add_keyframe(&mut self, time: f32, value: T) {
        self.keyframes.push(Keyframe::new(time, value));
        // 按时间排序
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// 获取轨道时长
    pub fn duration(&self) -> f32 {
        self.keyframes.last().map(|k| k.time).unwrap_or(0.0)
    }

    /// 获取关键帧数量
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// 获取指定时间点的两个相邻关键帧
    pub fn get_surrounding_keyframes(
        &self,
        time: f32,
    ) -> Option<(&Keyframe<T>, &Keyframe<T>, f32)> {
        if self.keyframes.len() < 2 {
            return None;
        }

        for i in 0..self.keyframes.len() - 1 {
            let curr = &self.keyframes[i];
            let next = &self.keyframes[i + 1];

            if time >= curr.time && time <= next.time {
                let t = (time - curr.time) / (next.time - curr.time);
                return Some((curr, next, t));
            }
        }

        None
    }
}

/// 位置动画轨道
pub type PositionTrack = AnimationTrack<Vec3>;

/// 旋转动画轨道
pub type RotationTrack = AnimationTrack<Quat>;

/// 缩放动画轨道
pub type ScaleTrack = AnimationTrack<Vec3>;

/// 动画片段 - 包含多个轨道
#[derive(Debug, Clone, Default)]
pub struct AnimationClip {
    /// 片段名称
    pub name: String,
    /// 位置轨道
    pub position: Option<PositionTrack>,
    /// 旋转轨道
    pub rotation: Option<RotationTrack>,
    /// 缩放轨道
    pub scale: Option<ScaleTrack>,
    /// 是否循环
    pub looping: bool,
}

impl AnimationClip {
    /// 创建新的动画片段
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    /// 设置位置轨道
    pub fn with_position(mut self, track: PositionTrack) -> Self {
        self.position = Some(track);
        self
    }

    /// 设置旋转轨道
    pub fn with_rotation(mut self, track: RotationTrack) -> Self {
        self.rotation = Some(track);
        self
    }

    /// 设置缩放轨道
    pub fn with_scale(mut self, track: ScaleTrack) -> Self {
        self.scale = Some(track);
        self
    }

    /// 设置循环
    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    /// 获取片段总时长
    pub fn duration(&self) -> f32 {
        let mut max_duration = 0.0f32;

        if let Some(ref track) = self.position {
            max_duration = max_duration.max(track.duration());
        }
        if let Some(ref track) = self.rotation {
            max_duration = max_duration.max(track.duration());
        }
        if let Some(ref track) = self.scale {
            max_duration = max_duration.max(track.duration());
        }

        max_duration
    }

    /// 在指定时间采样 Transform
    pub fn sample(&self, time: f32, base_transform: &Transform) -> Transform {
        let mut result = *base_transform;

        // 采样位置
        if let Some(ref track) = self.position {
            if let Some((k1, k2, t)) = track.get_surrounding_keyframes(time) {
                result.translation = k1.value.lerp(k2.value, t);
            }
        }

        // 采样旋转
        if let Some(ref track) = self.rotation {
            if let Some((k1, k2, t)) = track.get_surrounding_keyframes(time) {
                result.rotation = k1.value.slerp(k2.value, t);
            }
        }

        // 采样缩放
        if let Some(ref track) = self.scale {
            if let Some((k1, k2, t)) = track.get_surrounding_keyframes(time) {
                result.scale = k1.value.lerp(k2.value, t);
            }
        }

        result
    }
}

/// 动画片段资源
#[derive(Resource, Default)]
pub struct AnimationClips {
    clips: Vec<AnimationClip>,
}

impl AnimationClips {
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加动画片段
    pub fn add(&mut self, clip: AnimationClip) -> usize {
        let index = self.clips.len();
        self.clips.push(clip);
        index
    }

    /// 获取动画片段
    pub fn get(&self, index: usize) -> Option<&AnimationClip> {
        self.clips.get(index)
    }

    /// 按名称查找
    pub fn find_by_name(&self, name: &str) -> Option<(usize, &AnimationClip)> {
        self.clips.iter().enumerate().find(|(_, c)| c.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_new() {
        let kf = Keyframe::new(1.5, Vec3::ONE);
        assert_eq!(kf.time, 1.5);
        assert_eq!(kf.value, Vec3::ONE);
    }

    #[test]
    fn test_animation_track_new() {
        let track: AnimationTrack<Vec3> = AnimationTrack::new();
        assert_eq!(track.keyframe_count(), 0);
        assert_eq!(track.duration(), 0.0);
    }

    #[test]
    fn test_animation_track_add_keyframe() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(1.0, Vec3::X);
        track.add_keyframe(0.0, Vec3::ZERO);
        track.add_keyframe(2.0, Vec3::Y);

        assert_eq!(track.keyframe_count(), 3);
        assert_eq!(track.duration(), 2.0);
    }

    #[test]
    fn test_animation_track_keyframes_sorted() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(2.0, Vec3::Y);
        track.add_keyframe(0.0, Vec3::ZERO);
        track.add_keyframe(1.0, Vec3::X);

        // 关键帧应该按时间排序
        let (k1, k2, _) = track.get_surrounding_keyframes(0.5).unwrap();
        assert_eq!(k1.time, 0.0);
        assert_eq!(k2.time, 1.0);
    }

    #[test]
    fn test_animation_track_get_surrounding_keyframes() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(0.0, Vec3::ZERO);
        track.add_keyframe(2.0, Vec3::ONE);

        // 中间时间点
        let result = track.get_surrounding_keyframes(1.0);
        assert!(result.is_some());
        let (k1, k2, t) = result.unwrap();
        assert_eq!(k1.value, Vec3::ZERO);
        assert_eq!(k2.value, Vec3::ONE);
        assert!((t - 0.5).abs() < 0.001);

        // 边界时间点
        let result = track.get_surrounding_keyframes(0.0);
        assert!(result.is_some());

        let result = track.get_surrounding_keyframes(2.0);
        assert!(result.is_some());

        // 超出范围
        let result = track.get_surrounding_keyframes(3.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_animation_track_single_keyframe() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(1.0, Vec3::ONE);

        // 只有一个关键帧，无法获取 surrounding
        assert!(track.get_surrounding_keyframes(0.5).is_none());
    }

    #[test]
    fn test_animation_clip_new() {
        let clip = AnimationClip::new("test_clip");
        assert_eq!(clip.name, "test_clip");
        assert!(clip.position.is_none());
        assert!(clip.rotation.is_none());
        assert!(clip.scale.is_none());
        assert!(!clip.looping);
    }

    #[test]
    fn test_animation_clip_duration() {
        let mut pos_track = PositionTrack::new();
        pos_track.add_keyframe(0.0, Vec3::ZERO);
        pos_track.add_keyframe(2.0, Vec3::ONE);

        let mut scale_track = ScaleTrack::new();
        scale_track.add_keyframe(0.0, Vec3::ONE);
        scale_track.add_keyframe(3.0, Vec3::splat(2.0));

        let clip = AnimationClip::new("test")
            .with_position(pos_track)
            .with_scale(scale_track);

        // 时长应该是所有轨道中最长的
        assert_eq!(clip.duration(), 3.0);
    }

    #[test]
    fn test_animation_clip_looping() {
        let clip = AnimationClip::new("loop_test").looping();
        assert!(clip.looping);
    }

    #[test]
    fn test_animation_clip_sample() {
        let mut pos_track = PositionTrack::new();
        pos_track.add_keyframe(0.0, Vec3::ZERO);
        pos_track.add_keyframe(1.0, Vec3::new(10.0, 0.0, 0.0));

        let clip = AnimationClip::new("move").with_position(pos_track);

        let base = Transform::default();
        let sampled = clip.sample(0.5, &base);

        // 在 0.5 秒时，位置应该在中间
        assert!((sampled.translation.x - 5.0).abs() < 0.001);
        assert!((sampled.translation.y - 0.0).abs() < 0.001);
        assert!((sampled.translation.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_clips_add_and_get() {
        let mut clips = AnimationClips::new();
        let clip = AnimationClip::new("clip1");
        let index = clips.add(clip);

        assert_eq!(index, 0);
        assert!(clips.get(0).is_some());
        assert!(clips.get(1).is_none());
    }

    #[test]
    fn test_animation_clips_find_by_name() {
        let mut clips = AnimationClips::new();
        clips.add(AnimationClip::new("idle"));
        clips.add(AnimationClip::new("walk"));
        clips.add(AnimationClip::new("run"));

        let result = clips.find_by_name("walk");
        assert!(result.is_some());
        let (index, clip) = result.unwrap();
        assert_eq!(index, 1);
        assert_eq!(clip.name, "walk");

        // 不存在的名称
        assert!(clips.find_by_name("jump").is_none());
    }
}
