# manim-rs 🦀

A Rust-first exploration of Manim-style mathematical animation primitives. The current focus is building a rock-solid core math layer that future rendering and animation systems can rely on.

## ✨ Current Capabilities

- 📐 Core math types: `Vector2D`, `Color`, `Transform`, `Angle`, `BoundingBox`, `QuadraticBezier`, `CubicBezier`
- 📚 Full documentation: every public API ships with runnable examples
- ✅ 70 unit tests + 56 doctests covering edge cases and error paths
- ⏱️ Criterion benchmark suite for vector operations
- 🧭 Clear roadmap for scenes, animation, and rendering backends

## 🎯 Project Status

**Phase 2 In Progress** — Core math types are production-ready. The focus now shifts to rendering traits, SVG backend work, and foundational mobjects.

## 📦 Installation

Add this to your `Cargo.toml` (crate publishing planned for a future release):

```toml
[dependencies]
manim-rs = { git = "https://github.com/Clactar/manim-rs", branch = "main" }
```

Or use cargo add:

```bash
cargo add --git https://github.com/Clactar/manim-rs.git
```

## 🚀 Quick Start

```rust
use manim_rs::core::{BoundingBox, Color, QuadraticBezier, Vector2D};

fn main() {
    let start = Vector2D::new(0.0, 0.0);
    let control = Vector2D::new(0.5, 1.0);
    let end = Vector2D::new(1.0, 0.0);

    let curve = QuadraticBezier::new(start, control, end);
    let bbox = BoundingBox::from_points([start, control, end]);

    println!("Curve midpoint: {:?}", curve.evaluate(0.5));
    println!("Bounding box dimensions: {:?}", bbox.dimensions());

    let highlight = Color::from_hex("#FF8800").unwrap();
    println!("Highlight color: {}", highlight.to_hex());
}
```

## 📖 Examples

Real demos live in the [examples](examples/) directory:

- `vector_demo.rs` — Vector math operations, normalization, interpolation
- `color_demo.rs` — Color creation, conversion, and interpolation utilities
- `basic/`, `intermediate/`, `advanced/`, `showcase/` — Reserved for upcoming scene-based demos

Run an example:

```bash
cargo run --example vector_demo
```

## 🏗️ Architecture

```
manim-rs/
├── src/
│   ├── core/          # Fundamental math types (implemented)
│   ├── scene/         # (WIP) scene graph and object management
│   ├── animation/     # (WIP) animation primitives and timing
│   ├── mobject/       # (WIP) mathematical objects (shapes, text, equations)
│   ├── renderer/      # (WIP) backend-agnostic rendering traits
│   ├── backends/      # (WIP) concrete rendering backends
│   └── utils/         # (WIP) shared utilities
├── examples/          # Runnable examples and demos
├── benches/           # Criterion benchmarks
└── tests/             # Future integration tests
```

## 🎨 Design Principles

1. **Performance First** — Optimize the math core before adding rendering overhead
2. **Composability** — Small, focused APIs designed to work together
3. **Type Safety** — Distinct types (e.g., `Degrees` vs `Radians`) prevent misuse
4. **Zero-Copy** — Minimize allocations and data movement
5. **Documentation** — Every public API includes runnable examples

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
cargo bench --no-run

# Check code quality
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## 📊 Performance

Phase 1.2 ships with Criterion benchmarks for vector normalization, dot products, and interpolation. More benchmarks will arrive as new modules land.

```bash
cargo bench --no-run
```

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) and [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed planning.

**Current Status**: Phase 1.2 Complete ✅ → Phase 2 (Rendering Foundations) Up Next 🔄

### Completed

- [x] Project setup and core architecture
- [x] Core math types: `Vector2D`, `Color`, `Transform`, `Angle`, `BoundingBox`, Bézier curves
- [x] Documentation coverage for all public APIs
- [x] Testing infrastructure with 70 unit tests + 56 doctests
- [x] Criterion benchmark suite for vector operations

### Next Milestones

- [ ] **Milestone 1** (Rendering Foundations)
  - Rendering traits and SVG backend(s)
  - Basic geometric primitives (Circle, Rectangle, Line)
- [ ] **Milestone 2** (Animation Basics)
  - Animation system with easing functions
  - FadeIn, Transform, Move primitives
- [ ] **Milestone 3** (Typography)
  - Text rendering pipeline
  - Mathematical equation support (LaTeX/MathML)
- [ ] **Milestone 4** (Video Export)
  - Frame sequence generation
  - FFmpeg integration for MP4/WebM output
- [ ] **Future**: GPU rendering, 3D scenes, interactive previews

## 📄 License

Dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## 🙏 Acknowledgments

- Inspired by [3Blue1Brown's Manim](https://github.com/3b1b/manim)
- Built with amazing Rust crates: `nalgebra`, `lyon`, `tiny-skia`, and more

## 📬 Contact

- GitHub Issues: [Report bugs or request features](https://github.com/Clactar/manim-rs/issues)
- Discussions: [Ask questions and share ideas](https://github.com/Clactar/manim-rs/discussions)

---

Made with ❤️ and 🦀
