# Coding Conventions

**Analysis Date:** 2026-03-20

## Naming Patterns

**Files:**
- `snake_case` for all source files: `app_runner.rs`, `state_machine.rs`, `spatial_grid.rs`
- `lib.rs` as crate root, `prelude.rs` for re-exports, `plugin.rs` for Bevy plugin impl
- `mod.rs` for submodule grouping (e.g., `src/panels/mod.rs`, `src/performance/`)

**Functions:**
- `snake_case` for all functions and methods: `find_path`, `add_keyframe`, `run_frames`
- System functions named after their action: `update_animation_players`, `update_game_time`, `behavior_tree_system`
- Constructor pattern: `.new()` for creating instances, `.default()` for zero-value defaults
- Builder setters return `Self` for chaining: `with_title`, `with_ease`, `with_friction`
- Boolean query methods prefixed with `is_` or `has_`: `is_playing`, `is_paused`, `is_registered`
- Conversion functions: `to_collider()`, `to_world_center()`, `from_world()`

**Variables and Fields:**
- `snake_case` throughout: `frame_count`, `cell_size`, `half_height`, `clip_index`
- Field doc comments in Chinese for struct fields describing game concepts

**Types:**
- `PascalCase` for structs, enums, traits: `AnimationPlayer`, `PlaybackState`, `WorldAssertions`
- Enum variants in `PascalCase`: `PlaybackState::Playing`, `NovaEaseFunction::QuadInOut`
- Crate names use `snake_case` with `nova_` prefix: `nova_core`, `nova_render`, `nova_animation`
- Plugin types suffixed `Plugin`: `NovaCorePlugin`, `MapPlugin`, `NovaInputPlugin`
- Builder types suffixed `Builder`: `MaterialBuilder`, `SimpleAnimationBuilder`, `RenderTestBuilder`
- Event types suffixed `Event`: `AnimationFinished`, `LoadSceneEvent`, `SceneLoadedEvent`

**Constants:**
- `SCREAMING_SNAKE_CASE` for associated constants: `CollisionLayer::DEFAULT`, `CollisionLayer::PLAYER`

## Code Style

**Formatting:**
- `cargo fmt --all` via standard `rustfmt` (no custom `rustfmt.toml` detected)
- Standard Rust 2021 edition formatting rules

**Linting:**
- `cargo clippy --all-targets` enforced
- `#[allow(clippy::type_complexity)]` used selectively in `crates/nova_ai/src/decision.rs` for complex Bevy query types

## Import Organization

**Order (consistent across codebase):**
1. `use bevy::prelude::*;` — always first, glob import from Bevy
2. Standard library imports: `use std::collections::HashMap;`
3. Internal crate imports: `use crate::clip::{AnimationClip, AnimationClips};`
4. Cross-crate imports: `use nova_character::attributes::Attributes;`

**Prelude Pattern:**
- Every crate exposes a `prelude` module (`src/prelude.rs`) re-exporting the primary public API
- `nova_core::prelude` additionally re-exports `bevy::prelude::*`
- Users import via `use nova_core::prelude::*;` or `use nova_engine::prelude::*;`

**Path Aliases:**
- None detected; full paths used throughout

## Builder Pattern

Widely used across the codebase. Two forms:

**Consuming builder** (returns `Self`):
```rust
// crates/nova_render/src/material.rs
MaterialBuilder::new()
    .color(Color::WHITE)
    .roughness(0.5)
    .metallic(0.0)
    .build()
```

**Mutating builder** (returns `&mut Self`):
```rust
// crates/nova_animation/src/player.rs
player.play(0)
      .set_speed(2.0)
      .seek(3.5);
```

## Error Handling

**Patterns:**
- `Option<T>` preferred over `Result<T, E>` for "may not exist" cases
- Pattern matching with `let Some(x) = ... else { continue; }` in systems: see `crates/nova_animation/src/player.rs` lines 142–146
- `Result<(), Vec<String>>` used for validation that may produce multiple errors: `SceneTester::validate_scene`
- `.unwrap()` is acceptable in tests and benchmarks, but not in production system code
- `.clamp(0.0, 1.0)` used for value normalization instead of panicking

**In Bevy Systems:**
```rust
// crates/nova_animation/src/player.rs
let Some(clip_index) = player.clip_index else {
    continue;
};
let Some(clip) = clips.get(clip_index) else {
    continue;
};
```

## Logging

**Framework:** `log` crate (version 0.4), initialized via Bevy's `LogPlugin`

**Patterns:**
- `log::info!()` for render/test lifecycle events: `crates/nova_test/src/render.rs` line 81
- `bevy::log::Level::INFO` set as default in `NovaCorePlugin`
- WASM: `console_log()` wrapper in `crates/nova_test/src/wasm.rs` for browser console output

## Comments

**Module-level docs (`//!`):**
- Required for every `lib.rs` and `mod.rs`
- Written in Chinese
- Must list key sub-modules or types exported
- Example structure from `crates/nova_core/src/lib.rs`:
  ```
  //! Nova Core - 核心类型与 ECS 封装
  //!
  //! 提供 Nova Engine 的核心功能：
  //! - App 生命周期管理
  //! ...
  //! # 快速开始
  //! ```ignore
  ```

**Item-level docs (`///`):**
- Required for all public structs, enums, traits, and non-trivial methods
- Written in Chinese
- Include `# 示例` sections with ```` ```rust ```` or ```` ```ignore ```` code blocks for complex APIs

**Inline comments (`//`):**
- Used for explaining non-obvious logic, written in Chinese
- Used for `TODO:` markers: `// TODO: 实现截图捕获逻辑`

## Function Design

**Size:** Functions are kept focused; large systems delegate to private helpers (e.g., `evaluate_condition` and `execute_action` extracted from `evaluate_node` in `crates/nova_ai/src/decision.rs`)

**Parameters:** Bevy system functions receive injected parameters via Bevy's parameter extraction; non-system functions use minimal explicit parameters

**Generic Bounds:** Used for `impl Into<String>` on string parameters throughout: `pub fn new(name: impl Into<String>) -> Self`

**Return Values:**
- Builder methods return `Self` or `&mut Self`
- Fallible lookups return `Option<T>`
- Validation returns `Result<(), Vec<String>>`

## Module Design

**Exports:**
- Public API re-exported from `lib.rs`: `pub use app::NovaApp;`
- `prelude.rs` aggregates all user-facing types
- Internal helpers kept private (`fn evaluate_condition`, `fn execute_action`)

**Barrel Files:**
- `prelude.rs` acts as barrel per crate
- Cross-crate access through workspace dependency paths

## Derive Macros

Standard set applied to data types:
```rust
// Components
#[derive(Component, Debug, Clone, Copy, Default)]

// Resources
#[derive(Resource, Debug, Default)]

// Events
#[derive(Event, Debug)]

// Data structures
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

// Serializable scene data
#[derive(serde::Serialize, serde::Deserialize)]
```

---

*Convention analysis: 2026-03-20*
