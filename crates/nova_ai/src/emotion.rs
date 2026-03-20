//! 情绪系统 - 动态变化，影响行为

use crate::personality::Personality;
use bevy::prelude::*;

/// 情绪类型
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
pub enum EmotionType {
    /// 平静 - 正常行为
    #[default]
    Calm,
    /// 愤怒 - 攻击加成，防御降低
    Angry,
    /// 恐惧 - 倾向逃跑
    Fearful,
    /// 狂暴 - 无视命令，疯狂攻击
    Berserk,
}

/// 情绪组件
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct Emotion {
    pub current: EmotionType,
    /// 情绪强度 0-1
    pub intensity: f32,
    /// 情绪持续计时器（归零后恢复平静）
    pub duration: f32,
}

impl Emotion {
    /// 受伤时的情绪反应
    pub fn on_damage(&mut self, damage_percent: f32, personality: &Personality) {
        if damage_percent > 0.3 {
            if personality.courage < 0.3 {
                self.current = EmotionType::Fearful;
                self.intensity = 0.8;
                self.duration = 5.0;
            } else if personality.aggression > 0.7 {
                self.current = EmotionType::Angry;
                self.intensity = 0.6;
                self.duration = 4.0;
            }
        }
    }

    /// 盟友死亡时的情绪反应
    pub fn on_ally_death(&mut self, personality: &Personality) {
        if personality.aggression > 0.8 {
            self.current = EmotionType::Berserk;
            self.intensity = 1.0;
            self.duration = 8.0;
        } else if personality.courage < 0.4 {
            self.current = EmotionType::Fearful;
            self.intensity = 0.9;
            self.duration = 6.0;
        }
    }

    /// 情绪冷却（逐渐恢复平静）
    pub fn tick(&mut self, delta: f32) {
        if self.duration > 0.0 {
            self.duration -= delta;
            if self.duration <= 0.0 {
                self.current = EmotionType::Calm;
                self.intensity = 0.0;
            }
        }
    }
}

/// 情绪冷却系统
pub fn emotion_tick_system(time: Res<Time>, mut query: Query<&mut Emotion>) {
    for mut emotion in query.iter_mut() {
        emotion.tick(time.delta_secs());
    }
}
