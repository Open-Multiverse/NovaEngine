//! 动画状态机 - 管理角色动画状态切换

use std::collections::HashMap;

use bevy::prelude::*;

use crate::tween::LoopMode;

/// 动画状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    Attack,
    Hit,
    Die,
    Custom(u32),
}

/// 单个状态的配置
#[derive(Clone, Debug)]
pub struct AnimationStateConfig {
    /// 动画片段名称（对应 AnimationPlayer 中的片段）
    pub clip_name: String,
    pub loop_mode: LoopMode,
    pub speed: f32,
}

/// 状态转换触发条件
#[derive(Clone, Debug)]
pub enum TransitionCondition {
    /// 立即切换
    Immediate,
    /// 当前动画播放完毕后切换
    OnClipEnd,
    /// 超时后切换（秒）
    AfterSeconds(f32),
}

/// 状态转换规则
#[derive(Clone, Debug)]
pub struct AnimationTransition {
    pub from: AnimationState,
    pub to: AnimationState,
    pub condition: TransitionCondition,
    /// 混合时间（秒）
    pub blend_duration: f32,
}

/// 动画状态机组件
#[derive(Component, Clone, Debug)]
pub struct AnimationStateMachine {
    pub current_state: AnimationState,
    pub states: HashMap<AnimationState, AnimationStateConfig>,
    pub transitions: Vec<AnimationTransition>,
    /// 当前状态已持续时间
    pub state_time: f32,
}

impl AnimationStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: AnimationState::Idle,
            states: HashMap::new(),
            transitions: vec![],
            state_time: 0.0,
        }
    }

    pub fn with_state(mut self, state: AnimationState, config: AnimationStateConfig) -> Self {
        self.states.insert(state, config);
        self
    }

    pub fn with_transition(mut self, transition: AnimationTransition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// 强制切换到新状态
    pub fn transition_to(&mut self, new_state: AnimationState) {
        if self.current_state != new_state {
            self.current_state = new_state;
            self.state_time = 0.0;
        }
    }

    /// 获取当前状态的配置
    pub fn current_config(&self) -> Option<&AnimationStateConfig> {
        self.states.get(&self.current_state)
    }
}

impl Default for AnimationStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// 动画状态机计时系统
pub fn animation_state_machine_system(
    time: Res<Time>,
    mut query: Query<&mut AnimationStateMachine>,
) {
    for mut asm in query.iter_mut() {
        asm.state_time += time.delta_secs();
    }
}
