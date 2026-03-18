//! UI 系统

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::*;
use crate::selection::SelectionBox;

/// UI 系统插件
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (game_ui, selection_box_ui));
    }
}

/// 游戏主 UI
fn game_ui(
    mut contexts: EguiContexts,
    selected_units: Query<(&Health, &Attack), With<Selected>>,
    player_units: Query<&Team, With<Unit>>,
) {
    // 选中单位信息面板
    egui::Window::new("单位信息")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            let selected_count = selected_units.iter().count();

            if selected_count == 0 {
                ui.label("未选中单位");
            } else if selected_count == 1 {
                // 单个单位详细信息
                if let Some((health, attack)) = selected_units.iter().next() {
                    ui.horizontal(|ui| {
                        ui.label("生命值:");
                        let progress = health.percentage();
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .text(format!("{:.0}/{:.0}", health.current, health.max)),
                        );
                    });
                    ui.label(format!("攻击力: {:.0}", attack.damage));
                    ui.label(format!("攻击范围: {:.1}", attack.range));
                }
            } else {
                // 多个单位简略信息
                ui.label(format!("已选中 {} 个单位", selected_count));

                let total_health: f32 = selected_units.iter().map(|(h, _)| h.current).sum();
                let max_health: f32 = selected_units.iter().map(|(h, _)| h.max).sum();
                ui.horizontal(|ui| {
                    ui.label("总生命值:");
                    ui.add(
                        egui::ProgressBar::new(total_health / max_health)
                            .text(format!("{:.0}/{:.0}", total_health, max_health)),
                    );
                });
            }
        });

    // 操作提示
    egui::Window::new("操作提示")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("左键: 选中/框选  |  右键: 移动/攻击  |  WASD: 移动相机  |  滚轮: 缩放");
        });

    // 单位统计
    let mut player_count = 0;
    let mut enemy_count = 0;
    for team in player_units.iter() {
        match team {
            Team::Player => player_count += 1,
            Team::Enemy => enemy_count += 1,
        }
    }

    egui::Window::new("统计")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.colored_label(
                egui::Color32::from_rgb(50, 150, 255),
                format!("我方: {}", player_count),
            );
            ui.colored_label(
                egui::Color32::from_rgb(255, 80, 50),
                format!("敌方: {}", enemy_count),
            );
        });
}

/// 选择框渲染
fn selection_box_ui(mut contexts: EguiContexts, selection_box: Res<SelectionBox>) {
    if !selection_box.active {
        return;
    }

    let painter = contexts.ctx_mut().layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("selection_box"),
    ));

    let min = egui::pos2(
        selection_box.start.x.min(selection_box.end.x),
        selection_box.start.y.min(selection_box.end.y),
    );
    let max = egui::pos2(
        selection_box.start.x.max(selection_box.end.x),
        selection_box.start.y.max(selection_box.end.y),
    );

    let rect = egui::Rect::from_min_max(min, max);

    // 绘制半透明填充
    painter.rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 255, 0, 30),
    );

    // 绘制边框
    painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::GREEN));
}
