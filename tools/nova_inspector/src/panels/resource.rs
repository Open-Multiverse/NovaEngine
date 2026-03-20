//! 资源面板

use bevy::prelude::*;
use bevy_egui::egui;

pub fn render_resource_panel(ui: &mut egui::Ui, world: &World) {
    ui.heading("Resources");
    ui.separator();

    // 获取所有资源类型
    let components = world.components();
    let mut resource_types: Vec<&str> = Vec::new();

    // 遍历所有已注册的资源类型
    for id in components.iter_resources() {
        if let Some(info) = components.get_info(id) {
            resource_types.push(info.name());
        }
    }

    resource_types.sort();

    // 显示资源列表
    egui::ScrollArea::vertical()
        .max_height(500.0)
        .show(ui, |ui| {
            for name in resource_types {
                ui.label(name);
            }
        });

    ui.separator();

    // 资产信息
    if let Some(asset_server) = world.get_resource::<AssetServer>() {
        ui.collapsing("Asset Server", |ui| {
            ui.label("Asset server is available");
            // 这里可以扩展为显示加载的资产列表
        });
    }
}
