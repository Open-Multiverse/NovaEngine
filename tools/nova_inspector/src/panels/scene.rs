//! 场景面板

use bevy::prelude::*;
use bevy_egui::egui;

pub fn render_scene_panel(ui: &mut egui::Ui, world: &World) {
    ui.heading("Scene Info");
    ui.separator();

    // 相机统计
    let camera_count = world.query::<&Camera>().iter(world).count();
    ui.label(format!("Cameras: {}", camera_count));

    // 光源统计
    let point_lights = world.query::<&PointLight>().iter(world).count();
    let dir_lights = world.query::<&DirectionalLight>().iter(world).count();
    let spot_lights = world.query::<&SpotLight>().iter(world).count();

    ui.label(format!(
        "Lights: {} point, {} directional, {} spot",
        point_lights, dir_lights, spot_lights
    ));

    // 网格统计
    let mesh_count = world.query::<&Handle<Mesh>>().iter(world).count();
    ui.label(format!("Meshes: {}", mesh_count));

    // 材质统计
    let material_count = world
        .query::<&Handle<StandardMaterial>>()
        .iter(world)
        .count();
    ui.label(format!("Materials: {}", material_count));

    ui.separator();

    // 场景边界
    ui.collapsing("Scene Bounds", |ui| {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        let mut has_meshes = false;

        for transform in world.query::<&Transform>().iter(world) {
            let pos = transform.translation;
            min = min.min(pos);
            max = max.max(pos);
            has_meshes = true;
        }

        if has_meshes {
            let size = max - min;
            let center = (min + max) / 2.0;

            ui.label(format!(
                "Center: [{:.2}, {:.2}, {:.2}]",
                center.x, center.y, center.z
            ));
            ui.label(format!(
                "Size: [{:.2}, {:.2}, {:.2}]",
                size.x, size.y, size.z
            ));
            ui.label(format!("Min: [{:.2}, {:.2}, {:.2}]", min.x, min.y, min.z));
            ui.label(format!("Max: [{:.2}, {:.2}, {:.2}]", max.x, max.y, max.z));
        } else {
            ui.label("No transforms found");
        }
    });

    // 渲染统计
    ui.collapsing("Rendering", |ui| {
        // 这需要访问渲染诊断信息
        ui.label("Rendering statistics require custom instrumentation");
    });
}
