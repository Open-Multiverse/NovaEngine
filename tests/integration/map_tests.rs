//! 集成测试 - 地图系统

use nova_map::{MapPlugin, TerrainType, TileMap};
use nova_test::TestApp;

#[test]
fn test_map_creation() {
    let mut app = TestApp::new().add_plugin(MapPlugin);

    // 创建地图
    app.world_mut().spawn(TileMap::new(100, 100));

    app.run_frames(5);

    // 验证地图实体存在
    assert_has_component!(app, TileMap);
}

#[test]
fn test_map_dimensions() {
    let mut app = TestApp::new().add_plugin(MapPlugin);

    let width = 50;
    let height = 75;

    app.world_mut().spawn(TileMap::new(width, height));

    app.run_frames(3);

    // 验证地图尺寸
    let tilemap = app.world().query::<&TileMap>().single(app.world());

    assert_eq!(tilemap.width(), width);
    assert_eq!(tilemap.height(), height);
}

#[test]
fn test_terrain_generation() {
    let mut app = TestApp::new().add_plugin(MapPlugin);

    app.world_mut().spawn(TileMap::new(20, 20));

    app.run_frames(10);

    // 验证地图有地形数据
    let tilemap = app.world().query::<&TileMap>().single(app.world());

    // 检查至少有一些不同类型的地形
    let has_variety = tilemap.tiles().iter().any(|t| *t != TerrainType::Plain);

    assert!(has_variety, "Map should have varied terrain types");
}
