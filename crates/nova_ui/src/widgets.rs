//! 基础 UI 组件
//!
//! 提供常用的 UI 组件和辅助函数

use bevy::prelude::*;
use bevy_egui::egui;

/// 调试面板组件
pub struct DebugPanel;

impl DebugPanel {
    /// 显示基础调试信息
    pub fn show(
        ui: &mut egui::Ui,
        time: &Time,
        diagnostics: Option<&bevy::diagnostic::DiagnosticsStore>,
    ) {
        ui.heading("调试信息");
        ui.separator();

        // 时间信息
        ui.label(format!("运行时间: {:.1}s", time.elapsed_secs()));
        ui.label(format!("帧时间: {:.2}ms", time.delta_secs() * 1000.0));

        // FPS 信息
        if let Some(diagnostics) = diagnostics {
            if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    ui.label(format!("FPS: {:.0}", value));
                }
            }
        }
    }
}

/// FPS 显示组件
pub struct FpsDisplay;

impl FpsDisplay {
    /// 在屏幕角落显示 FPS
    pub fn show_corner(ctx: &egui::Context, diagnostics: &bevy::diagnostic::DiagnosticsStore) {
        egui::Area::new(egui::Id::new("fps_display"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180))
                    .inner_margin(egui::Margin::same(8.0))
                    .rounding(egui::Rounding::same(4.0))
                    .show(ui, |ui| {
                        if let Some(fps) =
                            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                        {
                            if let Some(value) = fps.smoothed() {
                                ui.colored_label(
                                    if value >= 55.0 {
                                        egui::Color32::GREEN
                                    } else if value >= 30.0 {
                                        egui::Color32::YELLOW
                                    } else {
                                        egui::Color32::RED
                                    },
                                    format!("FPS: {:.0}", value),
                                );
                            }
                        }
                    });
            });
    }
}

/// 属性编辑器
pub struct PropertyEditor;

impl PropertyEditor {
    /// 编辑 Vec3
    pub fn vec3(ui: &mut egui::Ui, label: &str, value: &mut Vec3) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label(label);
            changed |= ui
                .add(egui::DragValue::new(&mut value.x).prefix("X: ").speed(0.1))
                .changed();
            changed |= ui
                .add(egui::DragValue::new(&mut value.y).prefix("Y: ").speed(0.1))
                .changed();
            changed |= ui
                .add(egui::DragValue::new(&mut value.z).prefix("Z: ").speed(0.1))
                .changed();
        });
        changed
    }

    /// 编辑 Transform
    pub fn transform(ui: &mut egui::Ui, transform: &mut Transform) -> bool {
        let mut changed = false;

        ui.collapsing("位置", |ui| {
            changed |= Self::vec3(ui, "", &mut transform.translation);
        });

        ui.collapsing("旋转", |ui| {
            let (mut x, mut y, mut z) = transform.rotation.to_euler(EulerRot::XYZ);
            x = x.to_degrees();
            y = y.to_degrees();
            z = z.to_degrees();

            ui.horizontal(|ui| {
                let x_changed = ui
                    .add(egui::DragValue::new(&mut x).prefix("X: ").speed(1.0))
                    .changed();
                let y_changed = ui
                    .add(egui::DragValue::new(&mut y).prefix("Y: ").speed(1.0))
                    .changed();
                let z_changed = ui
                    .add(egui::DragValue::new(&mut z).prefix("Z: ").speed(1.0))
                    .changed();

                if x_changed || y_changed || z_changed {
                    transform.rotation = Quat::from_euler(
                        EulerRot::XYZ,
                        x.to_radians(),
                        y.to_radians(),
                        z.to_radians(),
                    );
                    changed = true;
                }
            });
        });

        ui.collapsing("缩放", |ui| {
            changed |= Self::vec3(ui, "", &mut transform.scale);
        });

        changed
    }

    /// 编辑颜色
    pub fn color(ui: &mut egui::Ui, label: &str, color: &mut Color) -> bool {
        let rgba = color.to_srgba();
        let mut arr = [rgba.red, rgba.green, rgba.blue, rgba.alpha];

        ui.horizontal(|ui| {
            ui.label(label);
            if ui.color_edit_button_rgba_unmultiplied(&mut arr).changed() {
                *color = Color::srgba(arr[0], arr[1], arr[2], arr[3]);
                return true;
            }
            false
        })
        .inner
    }

    /// 编辑浮点数滑块
    pub fn slider(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut f32,
        range: std::ops::RangeInclusive<f32>,
    ) -> bool {
        ui.add(egui::Slider::new(value, range).text(label))
            .changed()
    }
}

/// 按钮样式
pub struct NovaButton;

impl NovaButton {
    /// 主要按钮
    pub fn primary(ui: &mut egui::Ui, text: &str) -> egui::Response {
        ui.add(
            egui::Button::new(text)
                .fill(egui::Color32::from_rgb(66, 135, 245))
                .min_size(egui::vec2(80.0, 28.0)),
        )
    }

    /// 次要按钮
    pub fn secondary(ui: &mut egui::Ui, text: &str) -> egui::Response {
        ui.add(
            egui::Button::new(text)
                .fill(egui::Color32::from_rgb(80, 80, 80))
                .min_size(egui::vec2(80.0, 28.0)),
        )
    }

    /// 危险操作按钮
    pub fn danger(ui: &mut egui::Ui, text: &str) -> egui::Response {
        ui.add(
            egui::Button::new(text)
                .fill(egui::Color32::from_rgb(200, 60, 60))
                .min_size(egui::vec2(80.0, 28.0)),
        )
    }
}
