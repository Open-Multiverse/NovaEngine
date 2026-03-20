//! Nova Inspector - 运行时调试工具
//!
//! 提供实时的游戏世界检查功能：
//! - 实体树浏览
//! - 组件详情查看
//! - 性能监控面板
//! - 资源状态检查
//!
//! # 使用方式
//!
//! ```rust
//! use bevy::prelude::*;
//! use nova_inspector::NovaInspectorPlugin;
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(NovaInspectorPlugin)
//!     .run();
//! ```

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod panels;
mod state;

pub use panels::*;
pub use state::*;

/// 调试器插件
pub struct NovaInspectorPlugin;

impl Plugin for NovaInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<InspectorState>()
            .add_systems(Startup, setup_inspector)
            .add_systems(Update, (
                render_inspector_ui,
                update_performance_data,
                handle_input,
            ));
    }
}

fn setup_inspector(mut state: ResMut<InspectorState>) {
    // 初始化状态
    state.is_open = true;
    state.active_tab = InspectorTab::Entities;
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<InspectorState>,
) {
    // F12 切换调试器
    if keyboard.just_pressed(KeyCode::F12) {
        state.is_open = !state.is_open;
    }
    
    // Ctrl+Shift+I 切换调试器
    if keyboard.pressed(KeyCode::ControlLeft) 
        && keyboard.pressed(KeyCode::ShiftLeft)
        && keyboard.just_pressed(KeyCode::KeyI) {
        state.is_open = !state.is_open;
    }
}

fn render_inspector_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<InspectorState>,
    world: &World,
) {
    if !state.is_open {
        return;
    }

    let ctx = contexts.ctx_mut();
    
    egui::Window::new("🔧 Nova Inspector")
        .default_size([400.0, 600.0])
        .show(ctx, |ui| {
            // 标签页选择
            ui.horizontal(|ui| {
                for tab in InspectorTab::all() {
                    let is_active = state.active_tab == tab;
                    let label = match tab {
                        InspectorTab::Entities => "📋 Entities",
                        InspectorTab::Performance => "📊 Performance",
                        InspectorTab::Resources => "📦 Resources",
                        InspectorTab::Scene => "🎬 Scene",
                    };
                    
                    if ui.selectable_label(is_active, label).clicked() {
                        state.active_tab = tab;
                    }
                }
            });
            
            ui.separator();
            
            // 渲染对应面板
            match state.active_tab {
                InspectorTab::Entities => {
                    panels::entity::render_entity_panel(ui, world, &mut state);
                }
                InspectorTab::Performance => {
                    panels::performance::render_performance_panel(ui, world, &mut state);
                }
                InspectorTab::Resources => {
                    panels::resource::render_resource_panel(ui, world);
                }
                InspectorTab::Scene => {
                    panels::scene::render_scene_panel(ui, world);
                }
            }
        });
}

fn update_performance_data(
    mut state: ResMut<InspectorState>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
) {
    // 每秒更新一次性能数据
    state.performance.last_update += time.delta_secs();
    
    if state.performance.last_update >= 1.0 {
        state.performance.last_update = 0.0;
        
        // 更新 FPS
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            state.performance.fps = fps.smoothed().unwrap_or(0.0);
            state.performance.fps_history.push(state.performance.fps);
            if state.performance.fps_history.len() > 60 {
                state.performance.fps_history.remove(0);
            }
        }
        
        // 更新帧时间
        if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            state.performance.frame_time = frame_time.smoothed().unwrap_or(0.0);
        }
        
        // 更新实体数量
        if let Some(entity_count) = diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
            state.performance.entity_count = entity_count.value().unwrap_or(0.0) as usize;
        }
    }
}

/// 主入口（CLI 模式）
#[cfg(feature = "cli")]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NovaInspectorPlugin)
        .run();
}
