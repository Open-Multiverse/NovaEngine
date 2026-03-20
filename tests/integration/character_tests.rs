//! 集成测试 - 角色系统

use nova_character::{CharacterBundle, CharacterState, CharacterStats};
use nova_test::TestApp;

#[test]
fn test_character_creation() {
    let mut app = TestApp::new();

    app.world_mut().spawn(CharacterBundle::default());

    app.run_frames(5);

    // 验证角色实体存在
    assert_has_component!(app, CharacterStats);
    assert_has_component!(app, CharacterState);
}

#[test]
fn test_character_stats() {
    let mut app = TestApp::new();

    let stats = CharacterStats::new("Test Character")
        .with_health(100.0)
        .with_attack(20.0)
        .with_defense(10.0);

    app.world_mut()
        .spawn(CharacterBundle { stats, ..default() });

    app.run_frames(3);

    let character_stats = app.world().query::<&CharacterStats>().single(app.world());

    assert_eq!(character_stats.name(), "Test Character");
    assert_eq!(character_stats.max_health(), 100.0);
    assert_eq!(character_stats.attack(), 20.0);
    assert_eq!(character_stats.defense(), 10.0);
}

#[test]
fn test_character_state_transitions() {
    let mut app = TestApp::new();

    let entity = app.world_mut().spawn(CharacterBundle::default()).id();

    app.run_frames(5);

    // 检查初始状态
    {
        let state = app.world().get::<CharacterState>(entity).unwrap();
        assert!(matches!(state.current(), CharacterState::Idle));
    }

    // TODO: 测试状态转换（需要添加状态转换系统）
}
