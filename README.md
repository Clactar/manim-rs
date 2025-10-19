# manim-rs 🦀

A Rust-first exploration of Manim-style mathematical animation primitives. The current focus is building a rock-solid core math layer that future rendering and animation systems can rely on.

## ✨ Current Capabilities

- 📐 **Core Math**: `Vector2D`, `Color`, `Transform`, `Angle`, `BoundingBox`, Bézier curves
- 🎨 **Rendering**: SVG and Raster (PNG) backends with feature flags
- 🖼️ **Output**: Export static shapes to SVG or high-quality PNG images
- 📚 **Documentation**: Every public API has runnable examples
- ✅ **Testing**: 188 unit/integration tests + 101 doc tests (all passing)
- ⚡ **Performance**: SmallVec optimization, SIMD via tiny-skia, zero-copy design
- 🧭 **Roadmap**: Clear path to animations, mobjects, and video export

## 🎯 Project Status

**Phase 2 Complete! ✅** — Core math and rendering backends are production-ready. Both SVG and PNG export work. Next: Mobject system (reusable geometric shapes).

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

**Render shapes to SVG:**

```rust
use manim_rs::backends::SvgRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::renderer::{Path, PathStyle, Renderer};

fn main() -> manim_rs::core::Result<()> {
    let mut renderer = SvgRenderer::new(800, 600);
    
    // Create a circle using bezier curves
    let mut circle = Path::new();
    let r = 100.0;
    let k = 0.552; // Magic number for circle approximation
    
    circle.move_to(Vector2D::new(r, 0.0))
        .cubic_to(Vector2D::new(r, r*k), Vector2D::new(r*k, r), Vector2D::new(0.0, r))
        .cubic_to(Vector2D::new(-r*k, r), Vector2D::new(-r, r*k), Vector2D::new(-r, 0.0))
        .cubic_to(Vector2D::new(-r, -r*k), Vector2D::new(-r*k, -r), Vector2D::new(0.0, -r))
        .cubic_to(Vector2D::new(r*k, -r), Vector2D::new(r, -r*k), Vector2D::new(r, 0.0))
        .close();
    
    let style = PathStyle::stroke(Color::BLUE, 2.0)
        .with_fill(Color::from_hex("#87CEEB")?);
    
    renderer.begin_frame()?;
    renderer.clear(Color::WHITE)?;
    renderer.draw_path(&circle, &style)?;
    renderer.end_frame()?;
    
    renderer.save("output.svg")?;
    Ok(())
}
```

**Or render to PNG with the raster backend:**

```rust
use manim_rs::backends::RasterRenderer;
// ... same path creation ...
renderer.save_png("output.png")?;
```

## 📖 Examples

Real demos live in the [examples](examples/) directory:

- `svg_basic.rs` — Render circle, square, triangle to SVG
- `raster_basic.rs` — Render shapes to PNG with anti-aliasing
- `path_demo.rs` — Path building and manipulation
- `vector_demo.rs` — Vector math operations
- `color_demo.rs` — Color utilities

Run an example:

```bash
cargo run --example svg_basic --features svg
cargo run --example raster_basic --features raster
```

## 🏗️ Architecture

```
manim-rs/
├── src/
│   ├── core/          # ✅ Math types (Vector2D, Color, Transform, Bézier, etc.)
│   ├── renderer/      # ✅ Backend-agnostic rendering traits (Renderer, Path, Style)
│   ├── backends/      # ✅ SVG and Raster (tiny-skia) implementations
│   ├── scene/         # 🔄 Scene graph and object management (placeholder)
│   ├── animation/     # ⏳ Animation primitives and timing (Phase 4)
│   ├── mobject/       # ⏳ Geometric shapes and text (Phase 3, next)
│   └── utils/         # 🔄 Shared utilities
├── examples/          # ✅ svg_basic, raster_basic, path_demo, vector_demo, color_demo
├── benches/           # ✅ Criterion benchmarks (vector_ops, path_ops)
└── tests/             # ✅ Integration tests (svg_backend, raster_backend, renderer)
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

Comprehensive Criterion benchmarks track performance:
- Vector operations (normalization, dot/cross products, interpolation)
- Path operations (bounding box, transforms, cloning)
- Small path optimization (16-command inline capacity via SmallVec)

```bash
cargo bench
```

View results: `target/criterion/report/index.html`

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) and [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed planning.

**Current Status**: Phase 2 Complete ✅ → Phase 3 (Mobjects) Up Next 🔄

### Completed ✅

- [x] **Phase 1**: Core math types (Vector2D, Color, Transform, Angle, BoundingBox, Bézier)
- [x] **Phase 2.1**: Rendering traits (Renderer, Path, PathStyle, PathProvider)
- [x] **Phase 2.2**: SVG backend with hand-crafted XML generation
- [x] **Phase 2.3**: Raster backend with tiny-skia integration
- [x] 188 unit/integration tests + 101 doc tests (all passing)
- [x] Zero clippy warnings (strict mode)
- [x] Complete API documentation with examples
- [x] Working examples for SVG and PNG output

### Next Milestones

- [ ] **Phase 3.1** (Mobject System) — Next up! 🔄
  - Mobject trait for drawable objects
  - VMobject for vector-based shapes
  - Transform and style management
- [ ] **Phase 3.2** (Geometric Primitives)
  - Circle, Rectangle, Square, Line, Polygon
  - Arrow with customizable tips
- [ ] **Phase 4** (Animation System)
  - Timeline and easing functions
  - FadeIn, Transform, Move, Rotate animations
- [ ] **Phase 5** (Scene Management)
  - Scene orchestration and camera system
- [ ] **Phase 6** (Video Export)
  - FFmpeg integration for MP4/WebM
- [ ] **Future**: GPU acceleration, 3D support, interactive previews

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
