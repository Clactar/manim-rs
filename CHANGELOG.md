# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Phase 2: Rendering Backends (2025-10-19)

- **SVG Backend** (`svg` feature, enabled by default)

  - `SvgRenderer` implementing the `Renderer` trait
  - Hand-crafted SVG XML generation with zero external dependencies
  - Support for paths, text, and all rendering styles
  - Centered coordinate system with Y-axis pointing up
  - Example: `svg_basic.rs` demonstrating SVG rendering
  - 33 unit tests + 7 integration tests

- **Raster Backend** (`raster` feature, optional)

  - `RasterRenderer` implementing the `Renderer` trait via tiny-skia
  - CPU-based rendering with anti-aliasing and SIMD optimizations
  - PNG export support via `save_png()` method
  - High-quality anti-aliased output suitable for final renders
  - Example: `raster_basic.rs` demonstrating raster rendering
  - 20 unit tests + 6 integration tests

- **Rendering Infrastructure**
  - Feature flags for optional backend compilation
  - Path to SVG 'd' attribute conversion
  - Path to tiny-skia path conversion
  - Style to SVG attributes conversion
  - Style to tiny-skia Paint/Stroke conversion
  - Complete API documentation for both backends

#### Phase 1.2 & Phase 2.1 (Previously Added)

- Rendering system with backend-agnostic traits (`Renderer`, `PathProvider`)
- Path primitives: `Path`, `PathCommand`, `PathCursor` with SmallVec optimization
- Style configuration: `PathStyle` and `TextStyle` with builder patterns
- Support for various path commands: MoveTo, LineTo, QuadraticTo, CubicTo, Close
- Bounding box computation with caching for paths
- Transform application to paths
- Comprehensive examples: `path_demo.rs` demonstrating all path features
- Performance benchmarks for path operations (path_ops.rs)
- Integration tests for rendering system (renderer_tests.rs)
- Extended math types: `BoundingBox`, `QuadraticBezier`, `CubicBezier`
- Angle types: `Degrees` and `Radians` with conversions and trig functions
- Additional Vector2D methods: `min_components`, `max_components`
- Comprehensive unit tests (167 passing tests, 95 doctests)
- Complete documentation with examples for all public APIs

### Changed

- Updated prelude to include `Transform`
- Enhanced core module exports with new types
- Optimized Path storage with SmallVec for stack allocation of small shapes
- Separated backends into feature-gated modules
- Updated Cargo.toml with optional dependencies (tiny-skia, image)

### Fixed

- All clippy warnings resolved
- All doctests passing (95 doc tests)
- Code formatting consistent with rustfmt
- Cargo.toml edition corrected to "2021"

## [0.1.0] - 2025-10-19

### Added

- Initial release with core types and project foundation
