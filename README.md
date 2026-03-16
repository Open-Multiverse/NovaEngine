# Nova Engine

A Web 3D game engine built with Rust and WebAssembly.

## Features

- **WebGPU Rendering** - Modern graphics API for high-performance 3D
- **Physics** - Powered by Rapier3D
- **UI** - Immediate mode UI with egui
- **Animation** - Skeletal animation support

## Quick Start

```rust
use nova_engine::prelude::*;

fn main() {
    Nova::app()
        .add_plugin(NovaDefaults)
        .add_system(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera3d::default());
    cmd.spawn(DirectionalLight::sun());
    cmd.spawn((Mesh::cube(1.0), Material::default()));
}
```

## Building for Web

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve examples/basic_demo
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

