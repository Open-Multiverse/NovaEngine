//! 集成测试 - AI 系统

use bevy::prelude::*;
use nova_ai::{AiAgent, BehaviorTree, Blackboard};
use nova_ai::perception::PerceivedEntities;
use nova_character::attributes::Attributes;
use nova_test::TestApp;
use nova_test::assert_has_component;

#[test]
fn test_ai_agent_creation() {
    let mut app = TestApp::new();

    app.world_mut().spawn((
        AiAgent::new(),
        BehaviorTree::default(),
        Blackboard::default(),
    ));

    app.run_frames(5);

    assert_has_component!(app, AiAgent);
    assert_has_component!(app, BehaviorTree);
}

#[test]
fn test_behavior_tree_execution() {
    let mut app = TestApp::new();

    // behavior_tree_system 查询需要 Transform、Attributes、PerceivedEntities
    let bt = BehaviorTree::sequence().child(BehaviorTree::action(|blackboard| {
        blackboard.set("test", true);
        true
    }));

    app.world_mut().spawn((
        AiAgent::new(),
        bt,
        Blackboard::default(),
        Transform::default(),
        Attributes::default(),
        PerceivedEntities::default(),
    ));

    // 注册 AI 系统使行为树可以被执行
    app.add_plugin(nova_ai::NovaAiPlugin);

    app.run_frames(10);

    // 验证行为树被执行，blackboard 中的 "test" 值为 true
    let blackboard = app.world().query::<&Blackboard>().single(app.world());

    assert!(blackboard.get::<bool>("test").unwrap_or(false));
}
