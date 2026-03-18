//! 高度图数据结构

use serde::{Deserialize, Serialize};

/// 高度图数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeightMap {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
    /// 高度数据（0.0 ~ 1.0）
    data: Vec<f32>,
}

impl HeightMap {
    /// 创建空高度图
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0.5; (width * height) as usize],
        }
    }

    /// 从数据创建
    pub fn from_data(width: u32, height: u32, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Self {
            width,
            height,
            data,
        }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 检查坐标是否在范围内
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// 获取指定位置的高度值
    pub fn get(&self, x: u32, y: u32) -> f32 {
        if self.in_bounds(x, y) {
            self.data[(y * self.width + x) as usize]
        } else {
            0.0
        }
    }

    /// 设置指定位置的高度值
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        if self.in_bounds(x, y) {
            self.data[(y * self.width + x) as usize] = value.clamp(0.0, 1.0);
        }
    }

    /// 获取插值高度（双线性插值）
    pub fn get_interpolated(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x.fract();
        let fy = y.fract();

        let v00 = self.get(x0, y0);
        let v10 = self.get(x1, y0);
        let v01 = self.get(x0, y1);
        let v11 = self.get(x1, y1);

        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;

        v0 * (1.0 - fy) + v1 * fy
    }

    /// 归一化高度值到 0.0 ~ 1.0
    pub fn normalize(&mut self) {
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;

        if range > 0.0 {
            for v in &mut self.data {
                *v = (*v - min) / range;
            }
        }
    }

    /// 迭代所有高度值
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, f32)> + '_ {
        self.data.iter().enumerate().map(move |(idx, &h)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            (x, y, h)
        })
    }
}
