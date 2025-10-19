# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Extended math types: `BoundingBox`, `QuadraticBezier`, `CubicBezier`
- Angle types: `Degrees` and `Radians` with conversions and trig functions
- Additional Vector2D methods: `min_components`, `max_components`
- Comprehensive unit tests for all new types (70 total tests)
- Complete documentation with examples for all public APIs

### Changed

- Updated prelude to include `Transform`
- Enhanced core module exports with new types

### Fixed

- All clippy warnings resolved
- All doctests passing
- Code formatting consistent with rustfmt

## [0.1.0] - 2025-10-19

### Added

- Initial release with core types and project foundation
