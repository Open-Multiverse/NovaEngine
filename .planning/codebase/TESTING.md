# Testing Patterns

**Analysis Date:** 2026-03-20

## Test Framework

**Runner:**
- Rust's built-in `#[test]` framework (cargo test)
- No separate test runner config file; tests invoked via `cargo test`

**Benchmark Framework:**
- `criterion` version 0.5
- Benchmarks declared with `harness = false` in `Cargo.toml`

**WASM Test Framework:**
- `wasm-bindgen-test` version 0.3
- Configured with `wasm_bindgen_test_configure!(run_in_browser)` in `crates/nova_test/src/wasm.rs`

**Assertion Library:**
- Standard `assert!`, `assert_eq!`, `assert_ne!` macros
- Project-specific assertion macros and trait in `crates/nova_test/src/assertions.rs`

**Run Commands:**
```bash
cargo test                                           # Run all tests
cargo test --package nova_core                       # Run a specific crate
cargo test -p nova_test                              # Run nova_test crate
cargo bench                                          # Run all benchmarks
cargo bench -p nova_animation                        # Run animation benchmarks
cargo bench -p nova_test                             # Run ECS benchmarks
wasm-pack test --headless --firefox                  # Run WASM browser tests
```

## Test File Organization

**Location:**
- Unit tests: co-located with source in `#[cfg(test)] mod tests` blocks at the bottom of each `.rs` file
- Integration tests: `tests/integration/` at workspace root, organized by subsystem

**Naming:**
- Test functions: `test_<thing_being_tested>_<scenario>` pattern
  - `test_animation_player_pause_resume`
  - `test_game_time_scaled_delta`
  - `test_scene_bounds_with_transforms`
- Benchmark functions: `bench_<operation>` or `benchmark_<group>`
  - `bench_entity_spawn`, `benchmark_easing_functions`

**Structure:**
```
game_engine/
├── crates/
│   ├── nova_animation/
│   │   ├── src/
│   │   │   ├── player.rs          # #[cfg(test)] mod tests at bottom
│   │   │   ├── clip.rs            # #[cfg(test)] mod tests at bottom
│   │   │   └── tween.rs           # #[cfg(test)] mod tests at bottom
│   │   └── benches/
│   │       └── animation_bench.rs  # criterion benchmarks
│   ├── nova_test/
│   │   ├── src/
│   │   │   ├── app_runner.rs       # TestApp + inline unit tests
│   │   │   ├── assertions.rs       # assertion helpers + inline tests
│   │   │   ├── render.rs           # RenderTest + inline tests
│   │   │   └── wasm.rs             # WASM test utilities
│   │   └── benches/
│   │       └── ecs_benchmarks.rs   # ECS performance benchmarks
│   └── nova_assets/
│       └── benches/
│           └── assets_bench.rs     # asset system benchmarks
└── tests/
    └── integration/
        ├── mod.rs
        ├── character_tests.rs      # character system integration tests
        ├── ai_tests.rs             # AI system integration tests
        └── map_tests.rs            # map system integration tests
```

## Test Suite Structure

**Unit test module pattern:**
```rust
// Bottom of any source file, e.g. crates/nova_core/src/components.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_time_scaled_delta() {
        let mut time = GameTime::new();
        time.delta = 0.016; // ~60 FPS
        time.scale = 2.0;

        let scaled = time.scaled_delta();
        assert!((scaled - 0.032).abs() < 0.0001);
    }
}
```

**Integration test pattern (uses `TestApp`):**
```rust
// tests/integration/map_tests.rs
use nova_map::{MapPlugin, TileMap};
use nova_test::TestApp;

#[test]
fn test_map_dimensions() {
    let mut app = TestApp::new().add_plugin(MapPlugin);

    app.world_mut().spawn(TileMap::new(50, 75));
    app.run_frames(3);

    let tilemap = app.world().query::<&TileMap>().single(app.world());
    assert_eq!(tilemap.width(), 50);
    assert_eq!(tilemap.height(), 75);
}
```

**Benchmark pattern:**
```rust
// crates/nova_test/benches/ecs_benchmarks.rs
fn bench_entity_spawn(c: &mut Criterion) {
    c.bench_function("entity_spawn_1000", |b| {
        b.iter(|| {
            let mut world = World::new();
            for i in 0..1000 {
                world.spawn((Transform::from_xyz(i as f32, 0.0, 0.0), GlobalTransform::default()));
            }
            black_box(world.entities().len());
        });
    });
}
criterion_group!(benches, bench_entity_spawn, bench_query_iter, bench_component_operations);
criterion_main!(benches);
```

## TestApp: Core Integration Test Utility

`TestApp` (defined in `crates/nova_test/src/app_runner.rs`) is the primary tool for integration tests.

```rust
// Create minimal Bevy app with MinimalPlugins + AssetPlugin
let mut app = TestApp::new();

// Register a plugin
app.add_plugin(MapPlugin);

// Add systems
app.add_system(my_system);
app.add_startup_system(setup);

// Advance simulation
app.run_frames(10);                         // run exactly N frames
app.run_until(|world| condition(world), 100); // run until condition or panic
app.run_for(Duration::from_secs(1));         // run for a duration

// Inspect world
let world = app.world();
let resource = app.resource::<MyResource>();
let world_mut = app.world_mut();
```

`TestApp` uses `MinimalPlugins` (no window, no rendering) to keep tests headless and fast.

## Mocking

**Framework:** None — no mock library (mockall, etc.) is used.

**Patterns:**
- Bevy ECS provides natural isolation: spawn test-specific components and resources directly into `World`
- Atomic counters used for side-effect verification:
  ```rust
  // crates/nova_test/src/app_runner.rs
  static COUNTER: AtomicUsize = AtomicUsize::new(0);
  let mut app = TestApp::new();
  app.add_system(|| { COUNTER.fetch_add(1, Ordering::SeqCst); });
  app.run_frames(3);
  assert_eq!(COUNTER.load(Ordering::SeqCst), 3);
  ```
- Test-only components/resources defined inline inside `#[cfg(test)] mod tests`:
  ```rust
  #[derive(Component, PartialEq, Debug)]
  struct TestComponent(i32);

  #[derive(Resource, PartialEq, Debug)]
  struct TestResource(i32);
  ```

**What to Mock:**
- Use `world.insert_resource(MyResource { ... })` to inject specific resource state
- Use `world.spawn(TestComponent(42))` to inject entities with known state

**What NOT to Mock:**
- Do not mock Bevy ECS internals; use `TestApp` with `MinimalPlugins` instead
- Do not mock `Time`; use `run_frames()` / `run_for()` to advance time

## Fixtures and Factories

**Test Data:**
- Constructed inline within each test — no shared fixture files detected
- Builder pattern used for complex setup:
  ```rust
  // tests/integration/character_tests.rs
  let stats = CharacterStats::new("Test Character")
      .with_health(100.0)
      .with_attack(20.0)
      .with_defense(10.0);
  app.world_mut().spawn(CharacterBundle { stats, ..default() });
  ```
- `Default` trait used to fill remaining fields: `CharacterBundle::default()`, `..default()`

**Location:**
- No dedicated fixtures directory; data is constructed per-test inline

## Custom Assertion Macros

Defined in `crates/nova_test/src/assertions.rs`:

```rust
// Assert entity count in world
assert_entity_count!(app, 3);

// Assert at least one entity has a component
assert_has_component!(app, CharacterStats);

// Assert a resource equals an expected value
assert_resource_eq!(app, MyResource, MyResource::default());
```

**`WorldAssertions` trait** (also from `crates/nova_test/src/assertions.rs`):
```rust
world
    .assert_entity_count(1)
    .assert_has_component::<TestComponent>()
    .assert_component_count::<TestComponent>(1)
    .assert_resource_exists::<TestResource>()
    .assert_resource(&TestResource(100));
```

**Floating point assertions:**
```rust
// crates/nova_test/src/assertions.rs
assert_approx_eq(a, b, epsilon);              // f32 approximate equality
assert_vec3_approx_eq(a, b, epsilon);         // Vec3 approximate equality
assert_transform_approx_eq(&a, &b, epsilon);  // Transform approximate equality
```

## Coverage

**Requirements:** None enforced — no coverage threshold configuration detected

**View Coverage:**
```bash
# Using cargo-tarpaulin (must be installed separately)
cargo tarpaulin --out Html
```

## Test Types

**Unit Tests:**
- Scope: Individual structs, methods, and pure functions
- Approach: Co-located `#[cfg(test)] mod tests`, test struct behavior without Bevy ECS
- Files with unit tests: `crates/nova_animation/src/player.rs`, `crates/nova_animation/src/clip.rs`, `crates/nova_animation/src/tween.rs`, `crates/nova_core/src/components.rs`, `crates/nova_core/src/schedule.rs`, `crates/nova_test/src/app_runner.rs`, `crates/nova_test/src/assertions.rs`, `crates/nova_test/src/render.rs`, `crates/nova_assets/src/handle.rs`, `crates/nova_assets/src/loader.rs`, `crates/nova_render/src/mesh.rs`, `crates/nova_render/src/camera.rs`, `crates/nova_render/src/performance/lod.rs`, `crates/nova_render/src/performance/spatial_grid.rs`, `crates/nova_render/src/performance/instancing.rs`, `crates/nova_render/src/performance/frustum_culling.rs`, `crates/nova_audio/src/source.rs`, `crates/nova_core/src/scene.rs`

**Integration Tests:**
- Scope: Multi-crate interactions through `TestApp`; tests that span plugin + component + system
- Location: `tests/integration/`
- Files: `character_tests.rs`, `ai_tests.rs`, `map_tests.rs`
- Use `nova_test::TestApp` and `assert_has_component!` macros

**Benchmark Tests:**
- Framework: `criterion` 0.5
- Location: `crates/*/benches/`
- Files: `crates/nova_test/benches/ecs_benchmarks.rs`, `crates/nova_animation/benches/animation_bench.rs`, `crates/nova_assets/benches/assets_bench.rs`

**E2E Tests:** Not used — WASM browser tests in `crates/nova_test/src/wasm.rs` are the closest equivalent, targeting browser rendering pipeline validation

**WASM Tests:**
- Framework: `wasm-bindgen-test`
- Gate: `#[cfg(all(test, target_arch = "wasm32"))]`
- Use `#[wasm_bindgen_test]` attribute instead of `#[test]`
- Configuration: `wasm_bindgen_test_configure!(run_in_browser)` in `crates/nova_test/src/wasm.rs`

## Common Patterns

**Async Testing (WASM):**
```rust
// crates/nova_test/src/wasm.rs
#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_browser_compatibility() {
        let webgpu = BrowserCompatibility::supports_webgpu();
        assert!(webgpu || BrowserCompatibility::supports_webgl2());
    }
}
```

**Error Path Testing:**
```rust
// crates/nova_test/src/render.rs
#[test]
fn test_scene_validation_empty() {
    let world = World::new();
    let result = SceneTester::validate_scene(&world);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.contains("No camera")));
    assert!(errors.iter().any(|e| e.contains("No lights")));
}
```

**Frame-count Based Testing:**
```rust
// Standard integration test shape
let mut app = TestApp::new().add_plugin(SomePlugin);
app.world_mut().spawn(SomeBundle::default());
app.run_frames(5);   // allow systems to execute
// then query world state
```

**Panic Boundary Testing:**
- `TestApp::run_frames` panics if `max_frames` exceeded
- `TestApp::run_until` panics if condition not met within timeout
- Use these to assert systems converge within expected frame counts

---

*Testing analysis: 2026-03-20*
