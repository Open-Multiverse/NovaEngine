//! 阵型模式 - 计算每个槽位的相对偏移

use bevy::prelude::*;

/// 阵型模式
#[derive(Clone, Debug)]
pub enum FormationPattern {
    /// 方阵 - 步兵常用
    Square { rows: u32, cols: u32 },
    /// 楔形 - 冲锋
    Wedge { depth: u32 },
    /// 横线 - 远程
    Line,
    /// 圆形 - 防御
    Circle { radius: f32 },
    /// 自定义
    Custom { slots: Vec<Vec3> },
}

impl FormationPattern {
    /// 计算第 index 个槽位的相对偏移（基于 spacing）
    pub fn slot_offset(&self, index: usize, spacing: f32) -> Vec3 {
        match self {
            FormationPattern::Square { rows: _, cols } => {
                let col = (index as u32) % cols;
                let row = (index as u32) / cols;
                Vec3::new(
                    col as f32 * spacing - ((*cols - 1) as f32 * spacing / 2.0),
                    0.0,
                    row as f32 * spacing,
                )
            }
            FormationPattern::Wedge { depth } => {
                let row = (index as u32) % (*depth + 1);
                let col_offset = index as i32 - (row * (row + 1) / 2) as i32;
                Vec3::new(col_offset as f32 * spacing, 0.0, row as f32 * spacing)
            }
            FormationPattern::Line => Vec3::new(index as f32 * spacing, 0.0, 0.0),
            FormationPattern::Circle { radius } => {
                let angle = (index as f32 / 8.0) * std::f32::consts::TAU;
                Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
            }
            FormationPattern::Custom { slots } => {
                slots.get(index).copied().unwrap_or(Vec3::ZERO)
            }
        }
    }

    /// 计算该阵型最大支持多少单位
    pub fn capacity(&self) -> Option<usize> {
        match self {
            FormationPattern::Square { rows, cols } => Some((*rows * *cols) as usize),
            FormationPattern::Custom { slots } => Some(slots.len()),
            _ => None, // 无限
        }
    }
}
