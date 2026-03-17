//! 资源加载器

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// 资源注册表
#[derive(Resource, Default)]
pub struct AssetRegistry {
    /// 已注册的资源路径
    registered: HashSet<String>,
    /// 资源组
    groups: HashMap<String, AssetGroup>,
}

impl AssetRegistry {
    /// 注册资源
    pub fn register(&mut self, path: impl Into<String>) {
        self.registered.insert(path.into());
    }

    /// 批量注册资源
    pub fn register_many<I, S>(&mut self, paths: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for path in paths {
            self.registered.insert(path.into());
        }
    }

    /// 检查资源是否已注册
    pub fn is_registered(&self, path: &str) -> bool {
        self.registered.contains(path)
    }

    /// 获取所有已注册资源
    pub fn all_registered(&self) -> impl Iterator<Item = &String> {
        self.registered.iter()
    }

    /// 创建资源组
    pub fn create_group(&mut self, name: impl Into<String>) -> &mut AssetGroup {
        let name = name.into();
        self.groups.entry(name.clone()).or_insert_with(|| AssetGroup::new(&name))
    }

    /// 获取资源组
    pub fn get_group(&self, name: &str) -> Option<&AssetGroup> {
        self.groups.get(name)
    }

    /// 获取可变资源组
    pub fn get_group_mut(&mut self, name: &str) -> Option<&mut AssetGroup> {
        self.groups.get_mut(name)
    }
}

/// 资源组
#[derive(Debug, Clone)]
pub struct AssetGroup {
    /// 组名
    pub name: String,
    /// 组内资源路径
    pub assets: Vec<String>,
    /// 是否预加载
    pub preload: bool,
}

impl AssetGroup {
    /// 创建新的资源组
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            assets: Vec::new(),
            preload: false,
        }
    }

    /// 添加资源到组
    pub fn add(&mut self, path: impl Into<String>) -> &mut Self {
        self.assets.push(path.into());
        self
    }

    /// 批量添加资源
    pub fn add_many<I, S>(&mut self, paths: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for path in paths {
            self.assets.push(path.into());
        }
        self
    }

    /// 设置预加载
    pub fn with_preload(&mut self, preload: bool) -> &mut Self {
        self.preload = preload;
        self
    }

    /// 获取资源数量
    pub fn count(&self) -> usize {
        self.assets.len()
    }
}

/// 资源加载状态
#[derive(Resource, Default)]
pub struct AssetLoadState {
    /// 正在加载的资源
    loading: HashSet<String>,
    /// 已加载的资源
    loaded: HashSet<String>,
    /// 加载失败的资源
    failed: HashMap<String, String>,
}

impl AssetLoadState {
    /// 标记资源开始加载
    pub fn start_loading(&mut self, path: impl Into<String>) {
        self.loading.insert(path.into());
    }

    /// 标记资源加载完成
    pub fn mark_loaded(&mut self, path: &str) {
        self.loading.remove(path);
        self.loaded.insert(path.to_string());
    }

    /// 标记资源加载失败
    pub fn mark_failed(&mut self, path: &str, error: impl Into<String>) {
        self.loading.remove(path);
        self.failed.insert(path.to_string(), error.into());
    }

    /// 检查资源是否正在加载
    pub fn is_loading(&self, path: &str) -> bool {
        self.loading.contains(path)
    }

    /// 检查资源是否已加载
    pub fn is_loaded(&self, path: &str) -> bool {
        self.loaded.contains(path)
    }

    /// 检查资源是否加载失败
    pub fn is_failed(&self, path: &str) -> bool {
        self.failed.contains_key(path)
    }

    /// 获取失败原因
    pub fn get_error(&self, path: &str) -> Option<&String> {
        self.failed.get(path)
    }

    /// 获取正在加载的数量
    pub fn loading_count(&self) -> usize {
        self.loading.len()
    }

    /// 获取已加载的数量
    pub fn loaded_count(&self) -> usize {
        self.loaded.len()
    }

    /// 获取加载失败的数量
    pub fn failed_count(&self) -> usize {
        self.failed.len()
    }

    /// 检查是否所有资源都已加载完成（无正在加载的）
    pub fn all_loaded(&self) -> bool {
        self.loading.is_empty()
    }

    /// 计算加载进度 (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        let total = self.loading.len() + self.loaded.len();
        if total == 0 {
            1.0
        } else {
            self.loaded.len() as f32 / total as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_registry_register() {
        let mut registry = AssetRegistry::default();
        registry.register("textures/player.png");
        registry.register("models/enemy.gltf");

        assert!(registry.is_registered("textures/player.png"));
        assert!(registry.is_registered("models/enemy.gltf"));
        assert!(!registry.is_registered("sounds/music.ogg"));
    }

    #[test]
    fn test_asset_registry_register_many() {
        let mut registry = AssetRegistry::default();
        registry.register_many(["a.png", "b.png", "c.png"]);

        assert!(registry.is_registered("a.png"));
        assert!(registry.is_registered("b.png"));
        assert!(registry.is_registered("c.png"));
    }

    #[test]
    fn test_asset_group() {
        let mut registry = AssetRegistry::default();
        registry.create_group("level1")
            .add("level1/map.json")
            .add("level1/tileset.png")
            .with_preload(true);

        let group = registry.get_group("level1").unwrap();
        assert_eq!(group.name, "level1");
        assert_eq!(group.count(), 2);
        assert!(group.preload);
    }

    #[test]
    fn test_asset_load_state() {
        let mut state = AssetLoadState::default();

        state.start_loading("test.png");
        assert!(state.is_loading("test.png"));
        assert!(!state.is_loaded("test.png"));

        state.mark_loaded("test.png");
        assert!(!state.is_loading("test.png"));
        assert!(state.is_loaded("test.png"));
    }

    #[test]
    fn test_asset_load_state_failed() {
        let mut state = AssetLoadState::default();

        state.start_loading("missing.png");
        state.mark_failed("missing.png", "File not found");

        assert!(!state.is_loading("missing.png"));
        assert!(!state.is_loaded("missing.png"));
        assert!(state.is_failed("missing.png"));
        assert_eq!(state.get_error("missing.png"), Some(&"File not found".to_string()));
    }

    #[test]
    fn test_asset_load_progress() {
        let mut state = AssetLoadState::default();

        state.start_loading("a.png");
        state.start_loading("b.png");
        assert_eq!(state.progress(), 0.0);

        state.mark_loaded("a.png");
        assert!((state.progress() - 0.5).abs() < 0.001);

        state.mark_loaded("b.png");
        assert_eq!(state.progress(), 1.0);
    }
}
