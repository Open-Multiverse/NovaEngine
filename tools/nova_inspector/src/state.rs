//! Inspector 状态管理

use bevy::prelude::*;

/// 调试器标签页
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    Entities,
    Performance,
    Resources,
    Scene,
}

impl InspectorTab {
    pub fn all() -> &'static [Self] {
        &[
            Self::Entities,
            Self::Performance,
            Self::Resources,
            Self::Scene,
        ]
    }
}

/// 调试器状态
#[derive(Resource)]
pub struct InspectorState {
    /// 是否打开
    pub is_open: bool,
    /// 当前标签页
    pub active_tab: InspectorTab,
    /// 选中的实体
    pub selected_entity: Option<Entity>,
    /// 实体过滤器
    pub entity_filter: String,
    /// 性能数据
    pub performance: PerformanceData,
    /// 折叠状态
    pub collapsed: CollapsedState,
}

impl Default for InspectorState {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tab: InspectorTab::Entities,
            selected_entity: None,
            entity_filter: String::new(),
            performance: PerformanceData::default(),
            collapsed: CollapsedState::default(),
        }
    }
}

/// 性能监控数据
#[derive(Default)]
pub struct PerformanceData {
    pub fps: f64,
    pub frame_time: f64,
    pub entity_count: usize,
    pub fps_history: Vec<f64>,
    pub last_update: f32,
}

/// 折叠面板状态
#[derive(Default)]
pub struct CollapsedState {
    pub components: bool,
    pub children: bool,
    pub resources: bool,
}
