//! 动画系统性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 模拟动画系统的核心计算

/// 缓动函数 - 线性
fn ease_linear(t: f32) -> f32 {
    t
}

/// 缓动函数 - 二次缓入
fn ease_quad_in(t: f32) -> f32 {
    t * t
}

/// 缓动函数 - 二次缓出
fn ease_quad_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// 缓动函数 - 二次缓入缓出
fn ease_quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// 线性插值
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Vec3 线性插值
fn vec3_lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
        lerp(a[2], b[2], t),
    ]
}

fn benchmark_easing_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("easing");

    group.bench_function("linear", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(ease_linear(t));
            }
        })
    });

    group.bench_function("quad_in", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(ease_quad_in(t));
            }
        })
    });

    group.bench_function("quad_out", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(ease_quad_out(t));
            }
        })
    });

    group.bench_function("quad_in_out", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(ease_quad_in_out(t));
            }
        })
    });

    group.finish();
}

fn benchmark_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpolation");

    let start = [0.0f32, 0.0, 0.0];
    let end = [10.0f32, 5.0, -3.0];

    group.bench_function("vec3_lerp_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(vec3_lerp(start, end, t));
            }
        })
    });

    group.bench_function("vec3_lerp_with_easing_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                let eased_t = ease_quad_in_out(t);
                black_box(vec3_lerp(start, end, eased_t));
            }
        })
    });

    group.finish();
}

fn benchmark_animation_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_update");

    // 模拟多个动画对象的更新
    struct AnimState {
        start: [f32; 3],
        end: [f32; 3],
        elapsed: f32,
        duration: f32,
    }

    let mut states: Vec<AnimState> = (0..1000)
        .map(|i| AnimState {
            start: [0.0, 0.0, 0.0],
            end: [i as f32, i as f32 * 0.5, 0.0],
            elapsed: 0.0,
            duration: 2.0,
        })
        .collect();

    group.bench_function("update_1000_animations", |b| {
        b.iter(|| {
            let delta = 0.016f32; // ~60 FPS
            for state in &mut states {
                state.elapsed += delta;
                let t = (state.elapsed / state.duration).clamp(0.0, 1.0);
                let eased_t = ease_quad_in_out(t);
                black_box(vec3_lerp(state.start, state.end, eased_t));
            }
            // 重置
            for state in &mut states {
                state.elapsed = 0.0;
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_easing_functions,
    benchmark_interpolation,
    benchmark_animation_update
);
criterion_main!(benches);
