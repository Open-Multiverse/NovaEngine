//! 性能基准测试 - ECS 系统

use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// 基准测试：实体创建性能
fn bench_entity_spawn(c: &mut Criterion) {
    c.bench_function("entity_spawn_1000", |b| {
        b.iter(|| {
            let mut world = World::new();

            for i in 0..1000 {
                world.spawn((
                    Transform::from_xyz(i as f32, 0.0, 0.0),
                    GlobalTransform::default(),
                ));
            }

            black_box(world.entities().len());
        });
    });
}

/// 基准测试：查询性能
fn bench_query_iter(c: &mut Criterion) {
    c.bench_function("query_iter_10000", |b| {
        let mut world = World::new();

        // 创建测试实体
        for i in 0..10000 {
            world.spawn((
                Transform::from_xyz(i as f32, 0.0, 0.0),
                GlobalTransform::default(),
                i as i32,
            ));
        }

        b.iter(|| {
            let mut count = 0;
            for (_, _, _) in world
                .query::<(&Transform, &GlobalTransform, &i32)>()
                .iter(&world)
            {
                count += 1;
            }
            black_box(count);
        });
    });
}

/// 基准测试：组件添加/移除
fn bench_component_operations(c: &mut Criterion) {
    c.bench_function("component_add_remove", |b| {
        let mut world = World::new();
        let entity = world.spawn_empty().id();

        b.iter(|| {
            world.entity_mut(entity).insert(Transform::default());
            world.entity_mut(entity).remove::<Transform>();
        });
    });
}

criterion_group!(
    benches,
    bench_entity_spawn,
    bench_query_iter,
    bench_component_operations
);
criterion_main!(benches);
