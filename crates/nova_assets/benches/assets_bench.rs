//! 资源管理系统性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{HashMap, HashSet};

/// 模拟资源注册表
struct AssetRegistry {
    registered: HashSet<String>,
    groups: HashMap<String, Vec<String>>,
}

impl AssetRegistry {
    fn new() -> Self {
        Self {
            registered: HashSet::new(),
            groups: HashMap::new(),
        }
    }

    fn register(&mut self, path: &str) {
        self.registered.insert(path.to_string());
    }

    fn is_registered(&self, path: &str) -> bool {
        self.registered.contains(path)
    }

    fn create_group(&mut self, name: &str) -> &mut Vec<String> {
        self.groups.entry(name.to_string()).or_default()
    }
}

/// 模拟加载状态
struct LoadState {
    loading: HashSet<String>,
    loaded: HashSet<String>,
}

impl LoadState {
    fn new() -> Self {
        Self {
            loading: HashSet::new(),
            loaded: HashSet::new(),
        }
    }

    fn start_loading(&mut self, path: &str) {
        self.loading.insert(path.to_string());
    }

    fn mark_loaded(&mut self, path: &str) {
        self.loading.remove(path);
        self.loaded.insert(path.to_string());
    }

    fn progress(&self) -> f32 {
        let total = self.loading.len() + self.loaded.len();
        if total == 0 {
            1.0
        } else {
            self.loaded.len() as f32 / total as f32
        }
    }
}

fn benchmark_asset_registration(c: &mut Criterion) {
    let mut group = c.benchmark_group("asset_registration");

    group.bench_function("register_100", |b| {
        b.iter(|| {
            let mut registry = AssetRegistry::new();
            for i in 0..100 {
                registry.register(&format!("assets/texture_{}.png", i));
            }
            black_box(&registry);
        })
    });

    group.bench_function("register_1000", |b| {
        b.iter(|| {
            let mut registry = AssetRegistry::new();
            for i in 0..1000 {
                registry.register(&format!("assets/texture_{}.png", i));
            }
            black_box(&registry);
        })
    });

    group.finish();
}

fn benchmark_asset_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("asset_lookup");

    // 预填充注册表
    let mut registry = AssetRegistry::new();
    for i in 0..1000 {
        registry.register(&format!("assets/texture_{}.png", i));
    }

    group.bench_function("lookup_existing", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(registry.is_registered(&format!("assets/texture_{}.png", i)));
            }
        })
    });

    group.bench_function("lookup_missing", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(registry.is_registered(&format!("assets/missing_{}.png", i)));
            }
        })
    });

    group.finish();
}

fn benchmark_load_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_state");

    group.bench_function("track_100_assets", |b| {
        b.iter(|| {
            let mut state = LoadState::new();

            // 开始加载
            for i in 0..100 {
                state.start_loading(&format!("asset_{}", i));
            }

            // 逐个完成
            for i in 0..100 {
                state.mark_loaded(&format!("asset_{}", i));
                black_box(state.progress());
            }
        })
    });

    group.bench_function("progress_calculation", |b| {
        let mut state = LoadState::new();
        for i in 0..500 {
            state.start_loading(&format!("asset_{}", i));
        }
        for i in 0..250 {
            state.mark_loaded(&format!("asset_{}", i));
        }

        b.iter(|| {
            black_box(state.progress());
        })
    });

    group.finish();
}

fn benchmark_group_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("group_operations");

    group.bench_function("create_and_populate_groups", |b| {
        b.iter(|| {
            let mut registry = AssetRegistry::new();

            for level in 0..10 {
                let group = registry.create_group(&format!("level_{}", level));
                for asset in 0..50 {
                    group.push(format!("level_{}/asset_{}.png", level, asset));
                }
            }

            black_box(&registry);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_asset_registration,
    benchmark_asset_lookup,
    benchmark_load_state,
    benchmark_group_operations
);
criterion_main!(benches);
