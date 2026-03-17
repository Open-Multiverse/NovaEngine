//! 碰撞事件系统
//!
//! 提供简化的碰撞事件处理接口

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// 碰撞开始事件
#[derive(Event, Debug, Clone)]
pub struct CollisionStarted {
    /// 碰撞的实体 A
    pub entity_a: Entity,
    /// 碰撞的实体 B
    pub entity_b: Entity,
}

/// 碰撞结束事件
#[derive(Event, Debug, Clone)]
pub struct CollisionEnded {
    /// 碰撞的实体 A
    pub entity_a: Entity,
    /// 碰撞的实体 B
    pub entity_b: Entity,
}

/// 触发器进入事件
#[derive(Event, Debug, Clone)]
pub struct TriggerEntered {
    /// 触发器实体
    pub trigger: Entity,
    /// 进入触发器的实体
    pub entity: Entity,
}

/// 触发器离开事件
#[derive(Event, Debug, Clone)]
pub struct TriggerExited {
    /// 触发器实体
    pub trigger: Entity,
    /// 离开触发器的实体
    pub entity: Entity,
}

/// 碰撞事件插件
pub struct NovaCollisionEventsPlugin;

impl Plugin for NovaCollisionEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionStarted>()
            .add_event::<CollisionEnded>()
            .add_event::<TriggerEntered>()
            .add_event::<TriggerExited>()
            .add_systems(Update, process_collision_events);
    }
}

/// 处理 Rapier 碰撞事件并转换为 Nova 事件
fn process_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut collision_started: EventWriter<CollisionStarted>,
    mut collision_ended: EventWriter<CollisionEnded>,
    sensors: Query<&Sensor>,
    mut trigger_entered: EventWriter<TriggerEntered>,
    mut trigger_exited: EventWriter<TriggerExited>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _flags) => {
                let is_sensor_a = sensors.get(*e1).is_ok();
                let is_sensor_b = sensors.get(*e2).is_ok();

                if is_sensor_a || is_sensor_b {
                    // 触发器事件
                    let (trigger, entity) = if is_sensor_a { (*e1, *e2) } else { (*e2, *e1) };
                    trigger_entered.send(TriggerEntered { trigger, entity });
                } else {
                    // 普通碰撞事件
                    collision_started.send(CollisionStarted {
                        entity_a: *e1,
                        entity_b: *e2,
                    });
                }
            }
            CollisionEvent::Stopped(e1, e2, _flags) => {
                let is_sensor_a = sensors.get(*e1).is_ok();
                let is_sensor_b = sensors.get(*e2).is_ok();

                if is_sensor_a || is_sensor_b {
                    let (trigger, entity) = if is_sensor_a { (*e1, *e2) } else { (*e2, *e1) };
                    trigger_exited.send(TriggerExited { trigger, entity });
                } else {
                    collision_ended.send(CollisionEnded {
                        entity_a: *e1,
                        entity_b: *e2,
                    });
                }
            }
        }
    }
}

/// 碰撞检测辅助函数
pub fn check_collision_between(
    entity_a: Entity,
    entity_b: Entity,
    event: &CollisionStarted,
) -> bool {
    (event.entity_a == entity_a && event.entity_b == entity_b)
        || (event.entity_a == entity_b && event.entity_b == entity_a)
}

/// 检查实体是否参与了碰撞
pub fn entity_in_collision(entity: Entity, event: &CollisionStarted) -> Option<Entity> {
    if event.entity_a == entity {
        Some(event.entity_b)
    } else if event.entity_b == entity {
        Some(event.entity_a)
    } else {
        None
    }
}
