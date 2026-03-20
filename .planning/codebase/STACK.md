# Technology Stack

**Analysis Date:** 2026-03-20

## Languages

**Primary:**
- Rust (edition 2021, MSRV 1.80) - All engine crates, examples, and tools
- HTML/CSS - Web shell (`index.html`, example `index.html` files)

**Secondary:**
- Bash - Build/setup scripts (`scripts/implement.sh`)

## Runtime

**Environment:**
- WebAssembly (`wasm32-unknown-unknown`) - Production deployment target
- Native (Linux/macOS) - Development and CI testing

**Package Manager:**
- Cargo (Rust workspace, resolver = "2")
- Lockfile: `Cargo.lock` present and committed

## Frameworks

**Core ECS / Game Engine:**
- `bevy` 0.15 - Entity-Component-System runtime, rendering pipeline, asset management, windowing
  - Enabled features: `bevy_asset`, `bevy_render`, `bevy_core_pipeline`, `bevy_pbr`, `bevy_gltf`, `bevy_winit`, `bevy_state`, `webgpu`, `tonemapping_luts`, `ktx2`, `zstd`
  - `default-features = false` to minimize WASM binary size

**UI:**
- `bevy_egui` 0.31 - Immediate-mode GUI overlay integrated with Bevy (used in `nova_ui`, `nova_inspector`, `rts_demo`)

**Testing / Benchmarking:**
- `criterion` 0.5 - Micro-benchmarking harness (dev-dependency in `nova_animation`, `nova_assets`, `nova_map`, `nova_character`, `nova_test`)
- `wasm-pack` (CLI, installed in CI) - Runs browser-based WASM tests via headless Firefox

**Build / Dev:**
- `trunk` - WASM bundler and dev server (`Trunk.toml`; serves on `127.0.0.1:8080`, dist → `dist/`, entry `index.html`)
- `wasm-opt "z"` - Applied by Trunk during HTML build for size optimization

## Key Dependencies

**Critical:**
- `bevy_rapier3d` 0.28 - 3D physics simulation (used in `nova_physics`, `rts_demo`)
- `wasm-bindgen` 0.2 - Rust↔JS bridge for WASM target (workspace-level; used in `nova_test`, `rts_demo`, `nova_engine` WASM target)
- `getrandom` 0.3 (`wasm_js` feature) - WASM-compatible random number source, declared in `nova_engine` under `cfg(target_arch = "wasm32")`

**Infrastructure:**
- `serde` 1.0 (`derive`) - Serialization for assets, map data, character data (`nova_core`, `nova_map`, `nova_character`)
- `serde_json` 1.0 - JSON I/O for game data files
- `noise` 0.9 - Procedural noise generation for map system (`nova_map`)
- `log` 0.4 - Logging facade used across all crates

**WASM-specific (nova_test):**
- `wasm-bindgen-test` 0.3 - In-browser test runner
- `web-sys` 0.3 (`console`, `Window`, `Performance`, `HtmlCanvasElement`) - DOM/Web API access in tests

## Configuration

**Build:**
- `Trunk.toml` - Root-level Trunk config (dist output, watch ignores, serve address/port)
- `rust-toolchain.toml` - Pins Rust channel to `stable`, adds `wasm32-unknown-unknown` target
- `Cargo.toml` (workspace root) - Centralizes all dependency versions via `[workspace.dependencies]`

**Environment:**
- No `.env` files detected; no runtime environment variables required for development
- CI sets `CARGO_TERM_COLOR=always` and `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true`

**Feature Flags (notable):**
- `nova_test`: `headless` (default) enables `bevy_render` + `bevy_pbr`; `wasm` for WASM test path
- `nova_inspector`: `all-panels` (default) composes `entity-panel`, `performance-panel`, `resource-panel`, `scene-panel`

## Platform Requirements

**Development:**
- Rust stable toolchain with `wasm32-unknown-unknown` target
- Linux CI requires system packages: `libasound2-dev`, `libudev-dev` (audio + udev for Bevy)
- `trunk` CLI for serving WASM demos
- `wasm-pack` CLI for running browser tests

**Production:**
- Static file hosting capable of serving `.wasm` + `.js` bundles
- Browser with **WebGPU** support (Chrome 113+, Firefox Nightly behind flag)
- No server-side runtime; fully client-side WASM application

---

*Stack analysis: 2026-03-20*
