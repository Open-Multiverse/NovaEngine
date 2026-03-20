//! AI 黑板组件——用于行为树节点之间共享运行时状态

use std::{any::Any, collections::HashMap};
use bevy::prelude::*;

/// AI 黑板：键值存储，用于行为树节点读写运行时状态
///
/// 使用类型擦除（Box<dyn Any>）存储任意 Send + Sync 值。
/// 不实现 Clone（通过 Query<&mut Blackboard> 访问，无需克隆）。
#[derive(Component, Default)]
pub struct Blackboard {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl std::fmt::Debug for Blackboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blackboard {{ {} keys }}", self.data.len())
    }
}

impl Blackboard {
    /// 写入任意类型的值
    pub fn set<T: Any + Send + Sync>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }

    /// 读取指定类型的值，类型不匹配时返回 None
    pub fn get<T: Any + 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref()
    }

    /// 删除指定键
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    /// 检查键是否存在
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}
