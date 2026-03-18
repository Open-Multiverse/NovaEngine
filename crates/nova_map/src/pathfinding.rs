//! A* 寻路算法

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use bevy::prelude::*;

use crate::tilemap::TileMap;

/// 寻路结果
#[derive(Clone, Debug)]
pub struct PathResult {
    /// 路径点序列（不包含起点）
    pub path: Vec<(u32, u32)>,
    /// 总代价
    pub cost: f32,
}

/// 寻路节点（用于优先队列）
#[derive(Clone, Debug)]
struct PathNode {
    pos: (u32, u32),
    f_score: f32, // g + h
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反向排序（最小堆）
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 寻路器
pub struct Pathfinder;

impl Pathfinder {
    /// A* 寻路
    pub fn find_path(
        tilemap: &TileMap,
        start: (u32, u32),
        goal: (u32, u32),
    ) -> Option<PathResult> {
        if start == goal {
            return Some(PathResult {
                path: vec![],
                cost: 0.0,
            });
        }

        // 检查目标是否可达
        if !tilemap.get(goal.0, goal.1).map(|t| t.walkable()).unwrap_or(false) {
            return None;
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
        let mut g_score: HashMap<(u32, u32), f32> = HashMap::new();

        g_score.insert(start, 0.0);
        open_set.push(PathNode {
            pos: start,
            f_score: Self::heuristic(start, goal),
        });

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                // 重建路径
                let path = Self::reconstruct_path(&came_from, goal);
                let cost = g_score[&goal];
                return Some(PathResult { path, cost });
            }

            let current_g = g_score[&current.pos];

            // 遍历邻居（8方向）
            for neighbor in tilemap.neighbors8(current.pos.0, current.pos.1) {
                // 检查是否可通行
                let Some(tile) = tilemap.get(neighbor.0, neighbor.1) else {
                    continue;
                };
                let Some(move_cost) = tile.move_cost() else {
                    continue;
                };

                // 对角线移动代价更高
                let dx = (neighbor.0 as i32 - current.pos.0 as i32).abs();
                let dy = (neighbor.1 as i32 - current.pos.1 as i32).abs();
                let distance = if dx + dy == 2 { 1.414 } else { 1.0 };

                let tentative_g = current_g + move_cost * distance;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                    came_from.insert(neighbor, current.pos);
                    g_score.insert(neighbor, tentative_g);

                    let f_score = tentative_g + Self::heuristic(neighbor, goal);
                    open_set.push(PathNode {
                        pos: neighbor,
                        f_score,
                    });
                }
            }
        }

        None // 无法到达
    }

    /// 启发函数（欧几里得距离）
    fn heuristic(a: (u32, u32), b: (u32, u32)) -> f32 {
        let dx = (a.0 as f32 - b.0 as f32).abs();
        let dy = (a.1 as f32 - b.1 as f32).abs();
        (dx * dx + dy * dy).sqrt()
    }

    /// 重建路径
    fn reconstruct_path(
        came_from: &HashMap<(u32, u32), (u32, u32)>,
        goal: (u32, u32),
    ) -> Vec<(u32, u32)> {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.pop(); // 移除起点
        path.reverse();
        path
    }
}

/// 路径跟随组件
#[derive(Component, Clone, Debug, Reflect)]
pub struct PathFollow {
    /// 路径点序列
    pub path: Vec<(u32, u32)>,
    /// 当前目标点索引
    pub current_index: usize,
    /// 是否到达终点
    pub finished: bool,
}

impl PathFollow {
    pub fn new(path: Vec<(u32, u32)>) -> Self {
        Self {
            path,
            current_index: 0,
            finished: false,
        }
    }

    /// 获取当前目标瓦片
    pub fn current_target(&self) -> Option<(u32, u32)> {
        self.path.get(self.current_index).copied()
    }

    /// 前进到下一个目标点
    pub fn advance(&mut self) {
        self.current_index += 1;
        if self.current_index >= self.path.len() {
            self.finished = true;
        }
    }
}
