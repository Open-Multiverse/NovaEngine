//! 实体面板

use crate::state::InspectorState;
use bevy::prelude::*;
use bevy_egui::egui;

pub fn render_entity_panel(ui: &mut egui::Ui, world: &World, state: &mut InspectorState) {
    // 搜索过滤器
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.text_edit_singleline(&mut state.entity_filter);
        if ui.button("Clear").clicked() {
            state.entity_filter.clear();
        }
    });

    ui.separator();

    egui::ScrollArea::vertical()
        .max_height(400.0)
        .show(ui, |ui| {
            render_entity_tree(ui, world, state);
        });

    // 显示选中实体的组件
    if let Some(entity) = state.selected_entity {
        ui.separator();
        render_entity_details(ui, world, entity);
    }
}

fn render_entity_tree(ui: &mut egui::Ui, world: &World, state: &mut InspectorState) {
    let filter = state.entity_filter.to_lowercase();

    // 收集所有实体
    let mut entities: Vec<Entity> = world.iter_entities().map(|e| e.id()).collect();

    // 按实体 ID 排序
    entities.sort_by_key(|e| e.index());

    for entity in entities {
        // 如果有过滤器，检查实体是否有匹配的组件名称
        if !filter.is_empty() {
            let has_match = world
                .inspect_entity(entity)
                .iter()
                .any(|c| c.name().to_lowercase().contains(&filter));

            if !has_match {
                continue;
            }
        }

        let is_selected = state.selected_entity == Some(entity);

        ui.horizontal(|ui| {
            // 选择按钮
            if ui
                .selectable_label(is_selected, format!("Entity {:?}", entity))
                .clicked()
            {
                state.selected_entity = Some(entity);
            }

            // 显示组件数量
            let component_count = world.inspect_entity(entity).len();
            ui.label(format!("({} components)", component_count));
        });
    }
}

fn render_entity_details(ui: &mut egui::Ui, world: &World, entity: Entity) {
    ui.heading(format!("Entity {:?}", entity));

    // 检查实体是否存在
    if world.get_entity(entity).is_none() {
        ui.colored_label(egui::Color32::RED, "Entity no longer exists!");
        return;
    }

    // 显示组件列表
    ui.collapsing("Components", |ui| {
        for component in world.inspect_entity(entity) {
            let name = component.name();
            ui.label(name);
        }
    });

    // 显示变换信息
    if let Some(transform) = world.get::<Transform>(entity) {
        ui.collapsing("Transform", |ui| {
            ui.label(format!(
                "Position: [{:.2}, {:.2}, {:.2}]",
                transform.translation.x, transform.translation.y, transform.translation.z
            ));
            ui.label(format!(
                "Rotation: [{:.2}, {:.2}, {:.2}, {:.2}]",
                transform.rotation.x,
                transform.rotation.y,
                transform.rotation.z,
                transform.rotation.w
            ));
            ui.label(format!(
                "Scale: [{:.2}, {:.2}, {:.2}]",
                transform.scale.x, transform.scale.y, transform.scale.z
            ));
        });
    }

    // 显示父/子关系
    if let Some(parent) = world.get::<Parent>(entity) {
        ui.label(format!("Parent: {:?}", parent.get()));
    }

    if let Some(children) = world.get::<Children>(entity) {
        ui.collapsing(format!("Children ({})", children.len()), |ui| {
            for child in children.iter() {
                ui.label(format!("- {:?}", child));
            }
        });
    }
}
