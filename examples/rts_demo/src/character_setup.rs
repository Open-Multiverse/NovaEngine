//! 角色生成辅助 - 使用 nova_character 定义的角色类型

use bevy::prelude::*;

use nova_ai::{
    behavior::BehaviorTree,
    emotion::Emotion,
    perception::{PerceivedEntities, Perception},
    personality::Personality,
};
use nova_animation::ProceduralIdle;
use nova_character::{
    attributes::{Attributes, Health},
    character::{Character, CharacterType, Faction},
    feedback::{HealthBar, StatusIndicator},
    state::{AttackCooldown, CharacterState},
};

/// 生成玩家角色
pub fn spawn_player_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    unit_id: u64,
    phase: f32,
) -> Entity {
    let mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 1.0),
        ..default()
    });

    let attrs = Attributes::default();
    let perception_range = attrs.vision_range;
    let attack_speed = attrs.attack_speed;

    commands
        .spawn((
            Character::new(unit_id, "士兵", CharacterType::Infantry),
            Faction::Player,
            attrs,
            CharacterState::Idle,
            AttackCooldown::new(attack_speed),
            Perception::new(perception_range),
            PerceivedEntities::default(),
            Personality::soldier(),
            Emotion::default(),
            BehaviorTree::standard_soldier(),
            HealthBar::default(),
            StatusIndicator::default(),
            ProceduralIdle::new_with_phase(phase),
        ))
        .insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position),
        ))
        .id()
}

/// 生成敌方角色
pub fn spawn_enemy_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    unit_id: u64,
    phase: f32,
) -> Entity {
    let mesh = meshes.add(Capsule3d::new(0.3, 0.8));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.3, 0.2),
        ..default()
    });

    let attrs = Attributes {
        health: Health::new(80.0),
        attack: 8.0,
        attack_speed: 1.2,
        vision_range: 8.0,
        ..default()
    };
    let perception_range = attrs.vision_range;
    let attack_speed = attrs.attack_speed;

    commands
        .spawn((
            Character::new(unit_id, "敌兵", CharacterType::Infantry),
            Faction::Enemy,
            attrs,
            CharacterState::Idle,
            AttackCooldown::new(attack_speed),
            Perception::new(perception_range),
            PerceivedEntities::default(),
            Personality::soldier(),
            Emotion::default(),
            BehaviorTree::standard_soldier(),
            HealthBar::default(),
            StatusIndicator::default(),
            ProceduralIdle::new_with_phase(phase),
        ))
        .insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position),
        ))
        .id()
}
