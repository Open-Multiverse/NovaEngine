//! 角色移动系统
//!
//! 实现 CharacterState::Moving { target } 驱动的 Transform 更新，
//! 集成 A* 寻路（PathFollow）和直线降级（无 TileMap 时）。

use bevy::prelude::*;
use nova_map::{
    pathfinding::{PathFollow, Pathfinder},
    tilemap::TileMap,
};

use crate::{attributes::Attributes, state::CharacterState};

/// 记录上一帧是否处于 Moving 状态
///
/// 使用 bool newtype 避免克隆含 Entity/Vec3 字段的 CharacterState 枚举。
/// 必须随 CharacterBundle 一起 spawn（默认 was_moving: false）。
#[derive(Component, Debug, Default, Reflect)]
pub struct PreviousCharacterState {
    pub was_moving: bool,
}

/// 状态历史更新系统（必须在 movement_system 之前运行）
///
/// 将本帧 CharacterState 的 is_moving() 写入 PreviousCharacterState，
/// 供下一步 movement_system 判断是否"首次进入 Moving"。
pub fn update_previous_state_system(
    mut query: Query<(&CharacterState, &mut PreviousCharacterState)>,
) {
    for (state, mut prev) in query.iter_mut() {
        prev.was_moving = state.is_moving();
    }
}

/// 角色移动系统
///
/// 逻辑流：
/// 1. 只处理 CharacterState::Moving { target } 的实体
/// 2. 若上一帧不是 Moving（首次进入）→ 触发 A* 寻路，结果写入 PathFollow
///    - 有 TileMap：调用 Pathfinder::find_path；路径为空则直接切换 Idle
///    - 无 TileMap：静默降级，直接线性移动到 target（不插入 PathFollow）
///    - world_to_tile 返回 None 或 find_path 返回 None：同样降级直线移动
/// 3. 有 PathFollow 组件（路径跟随模式）：
///    - current_target() → tile_to_world → 计算 diff/distance
///    - distance > 0.5：更新 Transform.translation
///    - distance <= 0.5：调用 advance()
///    - finished == true：insert Idle，commands.remove::<PathFollow>（下帧生效）
/// 4. 无 PathFollow（直线降级模式）：
///    - 从 CharacterState::Moving { target } 读取 target
///    - 标准 diff/distance 移动
///    - distance <= 0.5：直接 *state = CharacterState::Idle
pub fn movement_system(
    time: Res<Time>,
    tilemap: Option<Res<TileMap>>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut CharacterState,
        &Attributes,
        Option<&mut PathFollow>,
        Option<&PreviousCharacterState>,
    )>,
) {
    for (entity, mut transform, mut state, attrs, path_follow, prev_state) in query.iter_mut() {
        // 只处理 Moving 状态
        let target = match *state {
            CharacterState::Moving { target } => target,
            _ => continue,
        };

        // 判断是否首次进入 Moving（上一帧不是 Moving）
        let was_moving = prev_state.map(|p| p.was_moving).unwrap_or(false);

        if !was_moving && path_follow.is_none() {
            // 首次进入 Moving —— 触发一次寻路
            if let Some(ref tm) = tilemap {
                if let (Some(start_tile), Some(goal_tile)) = (
                    tm.world_to_tile(transform.translation),
                    tm.world_to_tile(target),
                ) {
                    match Pathfinder::find_path(tm, start_tile, goal_tile) {
                        Some(result) if !result.path.is_empty() => {
                            commands.entity(entity).insert(PathFollow::new(result.path));
                        }
                        Some(_) => {
                            // start == goal，路径为空，直接 Idle
                            *state = CharacterState::Idle;
                        }
                        None => {
                            // 目标不可达，降级直线移动（不插入 PathFollow）
                        }
                    }
                }
                // world_to_tile 返回 None（超出地图范围），降级直线移动
            }
            // 无 TileMap：静默降级，直接进入直线移动分支
        }

        // 路径跟随模式（有 PathFollow 组件）
        if let Some(mut pf) = path_follow {
            if pf.finished {
                // PathFollow 已完成（Commands 延迟删除前的守卫）
                continue;
            }

            if let Some(tile) = pf.current_target() {
                let target_world = tilemap
                    .as_deref()
                    .map(|tm| tm.tile_to_world(tile.0, tile.1))
                    .unwrap_or(Vec3::new(tile.0 as f32, 0.0, tile.1 as f32));

                let diff = target_world - transform.translation;
                let distance = diff.length();

                if distance > 0.5 {
                    let step = attrs.move_speed * time.delta_secs();
                    transform.translation += diff.normalize() * step.min(distance);
                } else {
                    pf.advance();
                    if pf.finished {
                        *state = CharacterState::Idle;
                        commands.entity(entity).remove::<PathFollow>();
                    }
                }
            }
        } else {
            // 直线降级模式（无 PathFollow）
            let diff = target - transform.translation;
            let distance = diff.length();

            if distance > 0.5 {
                let step = attrs.move_speed * time.delta_secs();
                transform.translation += diff.normalize() * step.min(distance);
            } else {
                *state = CharacterState::Idle;
            }
        }
    }
}
