# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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
- Comprehensive unit tests (114 passing tests, 84 doctests)
- Complete documentation with examples for all public APIs

### Changed

- Updated prelude to include `Transform`
- Enhanced core module exports with new types
- Optimized Path storage with SmallVec for stack allocation of small shapes

### Fixed

- All clippy warnings resolved
- All doctests passing (84 doc tests)
- Code formatting consistent with rustfmt
- Cargo.toml edition corrected to "2021"

## [0.1.0] - 2025-10-19

### Added

- Initial release with core types and project foundation
