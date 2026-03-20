//! 集成测试 - AI 系统

use nova_ai::{AiAgent, BehaviorTree, Blackboard};
use nova_test::TestApp;

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

    let bt = BehaviorTree::sequence().child(BehaviorTree::action(|blackboard| {
        blackboard.set("test", true);
        true
    }));

    app.world_mut()
        .spawn((AiAgent::new(), bt, Blackboard::default()));

    app.run_frames(10);

    // 验证行为树被执行
    let blackboard = app.world().query::<&Blackboard>().single(app.world());

    assert!(blackboard.get::<bool>("test").unwrap_or(false));
}
