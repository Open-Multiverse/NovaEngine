//! Nova 调度阶段定义
//!
//! 提供 Nova 引擎的调度阶段常量，简化 Bevy 调度系统的使用

pub use bevy::prelude::{
    First, FixedFirst, FixedLast, FixedPostUpdate, FixedPreUpdate, FixedUpdate, Last, PostStartup,
    PostUpdate, PreStartup, PreUpdate, Startup, Update,
};

/// Nova 调度阶段常量
///
/// 提供类型安全的调度阶段访问
pub struct Schedules;

impl Schedules {
    /// 游戏启动前运行一次
    pub const PRE_STARTUP: PreStartup = PreStartup;

    /// 游戏启动时运行一次
    pub const STARTUP: Startup = Startup;

    /// 游戏启动后运行一次
    pub const POST_STARTUP: PostStartup = PostStartup;

    /// 帧开始时最先运行
    pub const FIRST: First = First;

    /// 每帧更新前运行
    pub const PRE_UPDATE: PreUpdate = PreUpdate;

    /// 每帧主更新
    pub const UPDATE: Update = Update;

    /// 每帧更新后运行
    pub const POST_UPDATE: PostUpdate = PostUpdate;

    /// 帧结束时最后运行
    pub const LAST: Last = Last;

    /// 固定时间步更新（物理等）
    pub const FIXED_UPDATE: FixedUpdate = FixedUpdate;
}

/// Nova 系统集定义
///
/// 用于组织和排序系统
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy::prelude::SystemSet)]
pub enum NovaSystemSet {
    /// 输入处理
    Input,
    /// 游戏逻辑
    Logic,
    /// 物理模拟
    Physics,
    /// 动画更新
    Animation,
    /// 渲染准备
    PreRender,
    /// UI 更新
    Ui,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedules_constants() {
        // 验证常量可以正常访问
        let _ = Schedules::STARTUP;
        let _ = Schedules::UPDATE;
        let _ = Schedules::FIXED_UPDATE;
    }

    #[test]
    fn test_nova_system_set_equality() {
        assert_eq!(NovaSystemSet::Input, NovaSystemSet::Input);
        assert_ne!(NovaSystemSet::Input, NovaSystemSet::Logic);
    }

    #[test]
    fn test_nova_system_set_debug() {
        let set = NovaSystemSet::Physics;
        let debug_str = format!("{:?}", set);
        assert!(debug_str.contains("Physics"));
    }
}
