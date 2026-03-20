//! 性能面板

use crate::state::InspectorState;
use bevy::prelude::*;
use bevy_egui::egui;

pub fn render_performance_panel(ui: &mut egui::Ui, _world: &World, state: &mut InspectorState) {
    let perf = &state.performance;

    // 关键指标卡片
    ui.horizontal(|ui| {
        // FPS 卡片
        let fps_color = if perf.fps >= 55.0 {
            egui::Color32::GREEN
        } else if perf.fps >= 30.0 {
            egui::Color32::YELLOW
        } else {
            egui::Color32::RED
        };

        ui.vertical(|ui| {
            ui.label("FPS");
            ui.colored_label(fps_color, format!("{:.1}", perf.fps));
        });

        ui.separator();

        // 帧时间卡片
        ui.vertical(|ui| {
            ui.label("Frame Time");
            ui.label(format!("{:.2} ms", perf.frame_time * 1000.0));
        });

        ui.separator();

        // 实体数量卡片
        ui.vertical(|ui| {
            ui.label("Entities");
            ui.label(format!("{}", perf.entity_count));
        });
    });

    ui.separator();

    // FPS 历史图表
    ui.label("FPS History (60s)");

    let plot = egui::plot::Plot::new("fps_history")
        .view_aspect(3.0)
        .include_y(0.0)
        .include_y(70.0);

    plot.show(ui, |plot_ui| {
        let points: Vec<[f64; 2]> = perf
            .fps_history
            .iter()
            .enumerate()
            .map(|(i, &fps)| [i as f64, fps])
            .collect();

        if !points.is_empty() {
            plot_ui.line(
                egui::plot::Line::new(egui::plot::PlotPoints::new(points))
                    .color(egui::Color32::from_rgb(0, 255, 0))
                    .name("FPS"),
            );
        }
    });

    ui.separator();

    // 内存使用（如果可用）
    ui.collapsing("Memory Usage", |ui| {
        // 注意：Bevy 目前没有直接提供内存诊断
        // 这里可以扩展为自定义资源监控
        ui.label("Memory monitoring requires custom implementation");
    });

    // 系统执行时间
    ui.collapsing("System Performance", |ui| {
        ui.label("System timing requires bevy::diagnostic::LogDiagnosticsPlugin");
    });
}
