# manim-rs 🦀

A high-performance mathematical animation engine inspired by [Manim](https://github.com/3b1b/manim), written in Rust.

[![Crates.io](https://img.shields.io/crates/v/manim-rs.svg)](https://crates.io/crates/manim-rs)
[![Documentation](https://docs.rs/manim-rs/badge.svg)](https://docs.rs/manim-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## ✨ Features

- **🚀 High Performance**: Zero-cost abstractions, SIMD optimizations, and parallel processing
- **🔧 Flexible**: Composable API with trait-based extensibility
- **🎨 Multiple Backends**: SVG, raster, and GPU rendering support
- **📚 Well Documented**: Comprehensive documentation with examples for all public APIs
- **🔒 Type Safe**: Leverage Rust's type system to catch errors at compile time
- **⚡ Modern**: Built with async-ready architecture for efficient rendering pipelines

## 🎯 Project Status

**Early Development** - This project is in active development. APIs are subject to change.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
manim-rs = "0.1"
```

Or use cargo add:

```bash
cargo add manim-rs
```

## 🚀 Quick Start

```rust
use manim_rs::prelude::*;

fn main() -> Result<()> {
    // Create a new scene
    let mut scene = Scene::new(SceneConfig::default());

    // Create a circle
    let circle = Circle::builder()
        .center(Vector2D::ZERO)
        .radius(2.0)
        .color(Color::BLUE)
        .build();

    // Animate the circle
    scene
        .add(circle)
        .animate()
        .fade_in(1.0)
        .then()
        .scale(2.0, 2.0)
        .with_ease(ease::smooth_in_out);

    // Render to SVG
    scene.render("output/circle.svg")?;

    Ok(())
}
```

## 📖 Examples

Check out the [examples](examples/) directory for more demonstrations:

- `basic/` - Simple shapes, colors, and basic animations
- `intermediate/` - Transformations and scene composition
- `advanced/` - Custom objects and GPU rendering
- `showcase/` - Beautiful mathematical visualizations

Run an example:

```bash
cargo run --example hello_circle
```

## 🏗️ Architecture

```
manim-rs/
├── src/
│   ├── core/          # Fundamental types (Vector2D, Color, Transform)
│   ├── scene/         # Scene graph and object management
│   ├── animation/     # Animation primitives and interpolation
│   ├── mobject/       # Mathematical objects (shapes, text, equations)
│   ├── renderer/      # Backend-agnostic rendering traits
│   ├── backends/      # Specific implementations (SVG, GPU)
│   └── utils/         # Common utilities
├── examples/          # Example code and demonstrations
├── benches/           # Performance benchmarks
└── tests/             # Integration tests
```

## 🎨 Design Principles

1. **Performance First**: Optimized for speed without sacrificing ergonomics
2. **Composability**: Small, focused APIs that work well together
3. **Type Safety**: Use Rust's type system to prevent errors
4. **Zero-Copy**: Minimize allocations and data copying
5. **Documentation**: Every public API is documented with examples

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Clactar/manim-rs.git
cd manim-rs

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

## 📊 Performance

Manim-rs is designed for high performance:

- **SIMD Operations**: Vectorized math operations for batch processing
- **Parallel Rendering**: Multi-threaded rendering with Rayon
- **Zero-Cost Abstractions**: No runtime overhead from ergonomic APIs
- **Memory Efficient**: Arena allocation for scene graphs

See [benches/](benches/) for detailed performance benchmarks.

## 🗺️ Roadmap

- [x] Project setup and core architecture
- [ ] Basic geometric primitives (Circle, Rectangle, Line)
- [ ] Core animation system with easing functions
- [ ] SVG rendering backend
- [ ] Text rendering support
- [ ] Mathematical equation rendering (LaTeX integration)
- [ ] Video export (MP4, WebM)
- [ ] GPU-accelerated rendering
- [ ] 3D support
- [ ] Interactive previews

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

- Inspired by [3Blue1Brown's Manim](https://github.com/3b1b/manim)
- Built with amazing Rust crates: nalgebra, lyon, tiny-skia, and more

## 📬 Contact

- GitHub Issues: [Report bugs or request features](https://github.com/Clactar/manim-rs/issues)
- Discussions: [Ask questions and share ideas](https://github.com/Clactar/manim-rs/discussions)

---

Made with ❤️ and 🦀

