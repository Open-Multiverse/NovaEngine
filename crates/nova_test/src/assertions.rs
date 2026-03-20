//! 断言工具
//!
//! 提供游戏测试中常用的断言宏和函数

use bevy::prelude::*;
use std::fmt::Debug;

/// 断言世界中有指定数量的实体
#[macro_export]
macro_rules! assert_entity_count {
    ($app:expr, $expected:expr) => {{
        let count = $app.world().entities().len();
        assert_eq!(
            count, $expected,
            "Expected {} entities, found {}",
            $expected, count
        );
    }};
}

/// 断言世界中存在具有特定组件的实体
#[macro_export]
macro_rules! assert_has_component {
    ($app:expr, $component:ty) => {{
        let query = $app.world().query::<&$component>();
        let count = query.iter($app.world()).count();
        assert!(
            count > 0,
            "Expected at least one entity with component {}",
            stringify!($component)
        );
    }};
}

/// 断言资源存在且具有特定值
#[macro_export]
macro_rules! assert_resource_eq {
    ($app:expr, $resource:ty, $expected:expr) => {{
        let actual = $app.resource::<$resource>();
        assert_eq!(*actual, $expected);
    }};
}

/// 断言世界状态
pub trait WorldAssertions {
    /// 断言实体数量
    fn assert_entity_count(&self, expected: usize) -> &Self;

    /// 断言具有特定组件的实体数量
    fn assert_component_count<C: Component>(&self, expected: usize) -> &Self;

    /// 断言存在至少一个具有特定组件的实体
    fn assert_has_component<C: Component>(&self) -> &Self;

    /// 断言资源等于预期值
    fn assert_resource<R: Resource + PartialEq + Debug>(&self, expected: &R) -> &Self;

    /// 断言资源存在
    fn assert_resource_exists<R: Resource>(&self) -> &Self;
}

impl WorldAssertions for World {
    fn assert_entity_count(&self, expected: usize) -> &Self {
        let count = self.entities().len();
        assert_eq!(
            count, expected,
            "Expected {} entities, found {}",
            expected, count
        );
        self
    }

    fn assert_component_count<C: Component>(&self, expected: usize) -> &Self {
        let count = self.query::<&C>().iter(self).count();
        assert_eq!(
            count,
            expected,
            "Expected {} entities with component {}, found {}",
            expected,
            std::any::type_name::<C>(),
            count
        );
        self
    }

    fn assert_has_component<C: Component>(&self) -> &Self {
        let count = self.query::<&C>().iter(self).count();
        assert!(
            count > 0,
            "Expected at least one entity with component {}",
            std::any::type_name::<C>()
        );
        self
    }

    fn assert_resource<R: Resource + PartialEq + Debug>(&self, expected: &R) -> &Self {
        let actual = self.resource::<R>();
        assert_eq!(
            actual,
            expected,
            "Resource {} does not match expected value",
            std::any::type_name::<R>()
        );
        self
    }

    fn assert_resource_exists<R: Resource>(&self) -> &Self {
        assert!(
            self.contains_resource::<R>(),
            "Expected resource {} to exist",
            std::any::type_name::<R>()
        );
        self
    }
}

/// 数值近似相等断言
pub fn assert_approx_eq(a: f32, b: f32, epsilon: f32) {
    assert!(
        (a - b).abs() < epsilon,
        "Expected {} to be approximately equal to {} (epsilon: {})",
        a,
        b,
        epsilon
    );
}

/// Vec3 近似相等断言
pub fn assert_vec3_approx_eq(a: Vec3, b: Vec3, epsilon: f32) {
    assert_approx_eq(a.x, b.x, epsilon);
    assert_approx_eq(a.y, b.y, epsilon);
    assert_approx_eq(a.z, b.z, epsilon);
}

/// 变换矩阵近似相等断言
pub fn assert_transform_approx_eq(a: &Transform, b: &Transform, epsilon: f32) {
    assert_vec3_approx_eq(a.translation, b.translation, epsilon);
    assert_approx_eq(a.rotation.x, b.rotation.x, epsilon);
    assert_approx_eq(a.rotation.y, b.rotation.y, epsilon);
    assert_approx_eq(a.rotation.z, b.rotation.z, epsilon);
    assert_approx_eq(a.rotation.w, b.rotation.w, epsilon);
    assert_vec3_approx_eq(a.scale, b.scale, epsilon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, PartialEq, Debug)]
    struct TestComponent(i32);

    #[derive(Resource, PartialEq, Debug)]
    struct TestResource(i32);

    #[test]
    fn test_world_assertions() {
        let mut world = World::new();

        // 测试空世界
        world.assert_entity_count(0);

        // 添加实体
        world.spawn(TestComponent(42));
        world.assert_entity_count(1);
        world.assert_has_component::<TestComponent>();
        world.assert_component_count::<TestComponent>(1);

        // 添加资源
        world.insert_resource(TestResource(100));
        world.assert_resource_exists::<TestResource>();
        world.assert_resource(&TestResource(100));
    }

    #[test]
    fn test_approx_eq() {
        assert_approx_eq(1.0, 1.001, 0.01);
        assert_vec3_approx_eq(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.001, 2.001, 3.001),
            0.01,
        );
    }
}
