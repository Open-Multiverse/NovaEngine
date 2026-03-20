# External Integrations

**Analysis Date:** 2026-03-20

## APIs & External Services

No external HTTP APIs or third-party SaaS services are integrated. The engine is a self-contained, fully client-side WASM application.

**Browser Web APIs (via `web-sys` in `nova_test`):**
- `Window` / `Performance` - Timing measurements in WASM tests (`crates/nova_test/src/wasm.rs`)
- `HtmlCanvasElement` - Canvas access for rendering tests (`crates/nova_test/src/wasm.rs`)
- `console` - Log output from WASM context (`crates/nova_test/src/wasm.rs`)

## Data Storage

**Databases:**
- None - No database dependency detected

**File Storage:**
- Local filesystem via Bevy's asset system (`bevy_asset` feature)
- Asset directory: `examples/basic_demo/assets/`
- Supported asset formats via enabled Bevy features: glTF (`bevy_gltf`), KTX2 textures (`ktx2`), Zstd-compressed assets (`zstd`)

**Caching:**
- None - No external cache service

## Authentication & Identity

**Auth Provider:**
- None - No authentication layer; engine has no user accounts or sessions

## Monitoring & Observability

**Error Tracking:**
- None - No Sentry, Datadog, or equivalent

**Logs:**
- `log` 0.4 facade across all crates; concrete backend provided by Bevy at runtime (routes to browser console in WASM, stdout in native)
- Runtime debug inspector: `tools/nova_inspector` (standalone binary using `bevy_egui` panels for entity inspection and performance monitoring)

## CI/CD & Deployment

**Hosting:**
- Not configured for a specific hosting provider; produces static assets in `dist/` via `trunk build`

**CI Pipeline:**
- GitHub Actions (`.github/workflows/ci.yml`) with five jobs:
  - `check` - `cargo check --all-targets` on ubuntu-latest
  - `fmt` - `cargo fmt --all -- --check`
  - `clippy` - `cargo clippy --all-targets -- -D warnings`
  - `test` - `cargo test --workspace --lib` + `cargo test --test integration`
  - `wasm` - `cargo check --target wasm32-unknown-unknown -p nova_engine` + `wasm-pack test --headless --firefox crates/nova_test` (continue-on-error)
  - `benchmark` - `cargo bench -- --noplot` (main branch pushes only, continue-on-error)
- Caching: `Swatinem/rust-cache@v2` used in check, clippy, test, wasm, benchmark jobs
- Triggers: push/PR to `main` and `develop` branches

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## Environment Configuration

**Required env vars:**
- None required for development or runtime
- CI-only vars set in workflow:
  - `CARGO_TERM_COLOR=always`
  - `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true`

**Secrets location:**
- No secrets files detected; no `.env` files present in the repository

## Physics Integration

**Bevy Rapier 3D (`bevy_rapier3d` 0.28):**
- Used in `crates/nova_physics/` and `examples/rts_demo/`
- Pure Rust; no external service calls; computes physics locally in WASM or native runtime

## Noise / Procedural Generation

**`noise` 0.9:**
- Used in `crates/nova_map/` for procedural terrain and map generation
- Pure Rust; no external service

---

*Integration audit: 2026-03-20*
