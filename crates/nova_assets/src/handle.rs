//! 资源句柄

use bevy::prelude::*;
use std::marker::PhantomData;

/// 资源句柄包装器
///
/// 提供类型安全的资源句柄访问
#[derive(Debug)]
pub struct AssetRef<T: Asset> {
    /// 内部句柄
    handle: Handle<T>,
    /// 资源路径
    path: String,
}

impl<T: Asset> AssetRef<T> {
    /// 创建新的资源引用
    pub fn new(handle: Handle<T>, path: impl Into<String>) -> Self {
        Self {
            handle,
            path: path.into(),
        }
    }

    /// 获取内部句柄
    pub fn handle(&self) -> &Handle<T> {
        &self.handle
    }

    /// 获取资源路径
    pub fn path(&self) -> &str {
        &self.path
    }

    /// 克隆句柄
    pub fn clone_handle(&self) -> Handle<T> {
        self.handle.clone()
    }
}

impl<T: Asset> Clone for AssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            path: self.path.clone(),
        }
    }
}

/// 资源句柄集合
#[derive(Debug, Default)]
pub struct AssetHandles<T: Asset> {
    handles: Vec<AssetRef<T>>,
    _marker: PhantomData<T>,
}

impl<T: Asset> AssetHandles<T> {
    /// 创建新的句柄集合
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// 添加资源引用
    pub fn add(&mut self, asset_ref: AssetRef<T>) {
        self.handles.push(asset_ref);
    }

    /// 获取资源引用数量
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }

    /// 按路径查找
    pub fn find_by_path(&self, path: &str) -> Option<&AssetRef<T>> {
        self.handles.iter().find(|h| h.path == path)
    }

    /// 获取所有句柄
    pub fn iter(&self) -> impl Iterator<Item = &AssetRef<T>> {
        self.handles.iter()
    }
}

/// 预加载资源标记组件
#[derive(Component, Debug)]
pub struct PreloadAsset {
    /// 资源路径
    pub path: String,
    /// 资源类型名称
    pub asset_type: String,
}

impl PreloadAsset {
    /// 创建预加载标记
    pub fn new(path: impl Into<String>, asset_type: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            asset_type: asset_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::image::Image;

    #[test]
    fn test_asset_ref() {
        let handle = Handle::<Image>::default();
        let asset_ref = AssetRef::new(handle.clone(), "test.png");

        assert_eq!(asset_ref.path(), "test.png");
    }

    #[test]
    fn test_asset_handles_collection() {
        let mut handles = AssetHandles::<Image>::new();
        assert!(handles.is_empty());

        handles.add(AssetRef::new(Handle::default(), "a.png"));
        handles.add(AssetRef::new(Handle::default(), "b.png"));

        assert_eq!(handles.len(), 2);
        assert!(!handles.is_empty());
    }

    #[test]
    fn test_asset_handles_find_by_path() {
        let mut handles = AssetHandles::<Image>::new();
        handles.add(AssetRef::new(Handle::default(), "textures/player.png"));
        handles.add(AssetRef::new(Handle::default(), "textures/enemy.png"));

        let found = handles.find_by_path("textures/player.png");
        assert!(found.is_some());
        assert_eq!(found.unwrap().path(), "textures/player.png");

        let not_found = handles.find_by_path("textures/missing.png");
        assert!(not_found.is_none());
    }
}
