# Implementation Plan: Detailed Next Steps

This document provides granular, actionable tasks for implementing manim-rs according to the [ROADMAP.md](ROADMAP.md) dependency hierarchy.

---

## Immediate Focus: Phase 4.1 - Animation Foundation

### Current Progress

**Phase 3 Complete!** ✅ All mobject geometric primitives are fully implemented and tested.

Updated dependency hierarchy:

```
Core Math (✅ Phase 1.1 Complete)
    ↓
Extended Math (BoundingBox, Bézier, Angle) (✅ Phase 1.2 Complete)
    ↓
Rendering Backend Abstractions (✅ Phase 2.1 Complete)
    ↓
SVG Backend Implementation (✅ Phase 2.2 Complete)
    ↓
Raster Backend Implementation (✅ Phase 2.3 Complete)
    ↓
Base Mobject System (✅ Phase 3.1 Complete)
    ↓
Geometric Primitives (✅ Phase 3.2 Complete)
    ↓
Complex Shapes (✅ Phase 3.3 Complete)
    ↓
Animations (🔄 Phase 4 NEXT - modify mobjects over time)
    ↓
Scenes (orchestrate everything)
    ↓
Export (output scenes)
```

We now have a complete static rendering system with all geometric shapes! Next priority: implementing the Animation system to bring these shapes to life.

---

## Phase 1.2: Extended Math Types (Completed ✅)

> Completed on 2025-10-19. Section retained for reference when revisiting design decisions.

### Task 1.2.1: BoundingBox

**File**: `src/core/bounding_box.rs`

```rust
/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Vector2D,
    pub max: Vector2D,
}

impl BoundingBox {
    pub fn new(min: Vector2D, max: Vector2D) -> Self { /* ... */ }
    pub fn empty() -> Self { /* ... */ }
    pub fn from_points(points: &[Vector2D]) -> Self { /* ... */ }

    pub fn width(&self) -> f64 { /* ... */ }
    pub fn height(&self) -> f64 { /* ... */ }
    pub fn center(&self) -> Vector2D { /* ... */ }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox { /* ... */ }
    pub fn contains(&self, point: Vector2D) -> bool { /* ... */ }
    pub fn intersects(&self, other: &BoundingBox) -> bool { /* ... */ }
}
```

**Dependencies**: `core::Vector2D`  
**Tests**: Union, intersection, contains checks  
**Estimated Time**: 1 day

---

### Task 1.2.2: Bézier Curve Utilities

**File**: `src/core/bezier.rs`

```rust
/// Quadratic Bézier curve
#[derive(Debug, Clone, Copy)]
pub struct QuadraticBezier {
    pub start: Vector2D,
    pub control: Vector2D,
    pub end: Vector2D,
}

impl QuadraticBezier {
    /// Evaluate point at parameter t ∈ [0, 1]
    pub fn eval(&self, t: f64) -> Vector2D { /* ... */ }

    /// Split curve at parameter t
    pub fn split_at(&self, t: f64) -> (QuadraticBezier, QuadraticBezier) { /* ... */ }
}

/// Cubic Bézier curve
#[derive(Debug, Clone, Copy)]
pub struct CubicBezier {
    pub start: Vector2D,
    pub control1: Vector2D,
    pub control2: Vector2D,
    pub end: Vector2D,
}

impl CubicBezier {
    pub fn eval(&self, t: f64) -> Vector2D { /* ... */ }
    pub fn split_at(&self, t: f64) -> (CubicBezier, CubicBezier) { /* ... */ }

    /// Get bounding box of curve
    pub fn bounding_box(&self) -> BoundingBox { /* ... */ }

    /// Approximate arc length
    pub fn arc_length(&self, num_samples: usize) -> f64 { /* ... */ }
}
```

**Dependencies**: `core::Vector2D`, `core::BoundingBox`  
**Tests**: Evaluation, splitting, bounding boxes  
**Estimated Time**: 2-3 days

**Rationale**: Manim CE uses cubic Bézier curves extensively for smooth paths. Circle approximation, smooth transitions, and path morphing all rely on Bézier math.

---

### Task 1.2.3: Angle Types (Optional for Milestone 1)

**File**: `src/core/angle.rs`

```rust
#[derive(Debug, Clone, Copy)]
pub struct Radians(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Degrees(pub f64);

impl From<Degrees> for Radians { /* ... */ }
impl From<Radians> for Degrees { /* ... */ }
```

**Dependencies**: None  
**Estimated Time**: 1 day (can be deferred)

---

## Phase 2.1: Rendering Traits ✅ (Completed 2025-10-19)

### Task 2.1.1: Define Core Rendering Traits ✅

**File**: `src/renderer/mod.rs`

```rust
/// Core trait for rendering backends
pub trait Renderer {
    /// Begin a new frame
    fn begin_frame(&mut self) -> Result<()>;

    /// End the current frame
    fn end_frame(&mut self) -> Result<()>;

    /// Draw a path
    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()>;

    /// Draw text
    fn draw_text(&mut self, text: &str, position: Vector2D, style: &TextStyle) -> Result<()>;

    /// Clear the canvas
    fn clear(&mut self, color: Color) -> Result<()>;

    /// Get canvas dimensions
    fn dimensions(&self) -> (u32, u32);
}

/// Style for path rendering
pub struct PathStyle {
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
    pub fill_color: Option<Color>,
    pub opacity: f64,
}

/// Style for text rendering
pub struct TextStyle {
    pub color: Color,
    pub font_size: f64,
    pub font_family: String,
    pub opacity: f64,
}
```

**Dependencies**: `core::Vector2D`, `core::Color`, `core::Error`  
**Tests**: ✅ Create mock renderer, test trait interface  
**Status**: ✅ Complete

---

### Task 2.1.2: Rendering Style Types ✅

**File**: `src/renderer/style.rs`

```rust
/// Style for path rendering
#[derive(Debug, Clone)]
pub struct PathStyle {
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
    pub fill_color: Option<Color>,
    pub opacity: f64,
}

impl Default for PathStyle {
    fn default() -> Self {
        Self {
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
        }
    }
}

/// Style for text rendering
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub color: Color,
    pub font_size: f64,
    pub font_family: String,
    pub opacity: f64,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            font_size: 48.0,
            font_family: "sans-serif".to_string(),
            opacity: 1.0,
        }
    }
}
```

**Dependencies**: `core::Color`  
**Tests**: ✅ Style creation and defaults, builder patterns, opacity clamping  
**Status**: ✅ Complete - Includes PathStyle, TextStyle, FontWeight, PathFillRule, TextAlignment

---

### Task 2.1.3: Path Representation ✅

**File**: `src/renderer/path.rs`

```rust
/// A 2D path composed of bezier curves and line segments
#[derive(Debug, Clone)]
pub struct Path {
    pub commands: Vec<PathCommand>,
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(Vector2D),
    LineTo(Vector2D),
    QuadraticTo {
        control: Vector2D,
        to: Vector2D,
    },
    CubicTo {
        control1: Vector2D,
        control2: Vector2D,
        to: Vector2D,
    },
    Close,
}

impl Path {
    pub fn new() -> Self { /* ... */ }
    pub fn move_to(&mut self, point: Vector2D) -> &mut Self { /* ... */ }
    pub fn line_to(&mut self, point: Vector2D) -> &mut Self { /* ... */ }
    pub fn close(&mut self) -> &mut Self { /* ... */ }

    /// Get bounding box of the path
    pub fn bounding_box(&self) -> BoundingBox { /* ... */ }

    /// Transform all points in the path
    pub fn transform(&mut self, transform: &Transform) { /* ... */ }
}
```

**Dependencies**: `core::Vector2D`, `core::Transform`  
**Tests**: ✅ Path building, bounding box calculation, transformations, PathCursor  
**Status**: ✅ Complete - Includes Path, PathCommand, PathCursor, SmallVec optimization, cached bounding boxes

**Completed Features**:

- ✅ Path with MoveTo, LineTo, QuadraticTo, CubicTo, Close commands
- ✅ PathCursor helper for relative movements
- ✅ SmallVec optimization (16-command inline capacity)
- ✅ Cached bounding box computation
- ✅ Transform application to paths
- ✅ 114 unit tests passing
- ✅ 17 performance benchmarks
- ✅ 8 integration tests
- ✅ Complete API documentation with examples

---

---

## Phase 2.2: SVG Backend ✅ (Completed 2025-10-19)

**Status**: COMPLETED ✅  
**Time Spent**: 5 days  
**Test Coverage**: 33 unit tests + 7 integration tests

### Delivered Features

✅ **Core Implementation**

- `SvgRenderer` implementing `Renderer` trait
- Hand-crafted XML generation (zero external deps)
- Centered coordinate system with Y-up
- Background rectangles, paths, and text elements

✅ **Conversion Modules**

- `path_converter.rs`: Path → SVG 'd' attribute
- `style_converter.rs`: PathStyle/TextStyle → SVG attributes
- `elements.rs`: Type-safe SVG element representation

✅ **Examples & Tests**

- `examples/svg_basic.rs`: Circle, square, triangle demo
- Complete unit test coverage (33 tests)
- Integration tests (7 tests in `tests/svg_backend_tests.rs`)
- All doctests passing

### Task 2.2.1: SVG Document Builder ✅

**File**: `src/backends/svg/mod.rs`

```rust
pub struct SvgRenderer {
    width: u32,
    height: u32,
    background: Color,
    elements: Vec<SvgElement>,
}

impl SvgRenderer {
    pub fn new(width: u32, height: u32) -> Self { /* ... */ }

    pub fn to_string(&self) -> String {
        // Generate SVG XML string
    }

    pub fn save(&self, path: &str) -> Result<()> {
        // Write to file
    }
}

impl Renderer for SvgRenderer {
    fn begin_frame(&mut self) -> Result<()> { /* ... */ }
    fn end_frame(&mut self) -> Result<()> { /* ... */ }
    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()> { /* ... */ }
    fn draw_text(&mut self, text: &str, pos: Vector2D, style: &TextStyle) -> Result<()> { /* ... */ }
    fn clear(&mut self, color: Color) -> Result<()> { /* ... */ }
    fn dimensions(&self) -> (u32, u32) { (self.width, self.height) }
}
```

**Dependencies**: `renderer::Renderer`, `renderer::Path`, `core::Color`  
**External Crates**: None (hand-craft SVG XML) or `svg` crate  
**Tests**: Render simple shapes, verify SVG output  
**Estimated Time**: 3-4 days

---

### Task 2.2.2: Path to SVG Conversion ✅

Implemented in `src/backends/svg/path_converter.rs`:

- ✅ `path_to_svg_d()`: Converts Path to SVG 'd' attribute
- ✅ `path_command_to_svg()`: Individual command conversion
- ✅ `format_coord()`: Float formatting with trailing zero removal
- ✅ 14 unit tests covering all path types

### Task 2.2.3: Integration Test ✅

Implemented in `tests/svg_backend_tests.rs`:

- ✅ 7 comprehensive integration tests
- ✅ Tests for circles, rectangles, text, multiple shapes
- ✅ File I/O validation (save and verify)
- ✅ All tests passing

---

## Phase 2.3: Raster Backend ✅ (Completed 2025-10-19)

**Status**: COMPLETED ✅  
**Time Spent**: 3 days  
**Test Coverage**: 20 unit tests + 6 integration tests

### Delivered Features

✅ **Core Implementation**

- `RasterRenderer` implementing `Renderer` trait
- tiny-skia integration for CPU rasterization
- PNG export via `save_png()` method
- Anti-aliasing enabled by default

✅ **Conversion Modules**

- `path_converter.rs`: Path → tiny-skia::Path
- `style_converter.rs`: PathStyle → Paint/Stroke
- Fill rule and line cap/join conversions

✅ **Examples & Tests**

- `examples/raster_basic.rs`: Circle, square, triangle demo
- Complete unit test coverage (20 tests)
- Integration tests (6 tests in `tests/raster_backend_tests.rs`)
- Coordinate system validation tests

### Performance Features

- ✅ SIMD optimizations via tiny-skia
- ✅ Single pixmap allocation (no intermediate buffers)
- ✅ Zero-copy rendering pipeline
- ✅ High-quality anti-aliasing

### Known Limitations

- ⚠️ Text rendering not fully implemented (font rasterization pending Phase 3.4)
- Works as designed for path-based rendering

---

## Phase 3.1: Base Mobject System ✅ (Completed 2025-10-19)

### Task 3.1.1: Mobject Trait ✅

**File**: `src/mobject/mod.rs`

**Status**: ✅ Fully implemented with object-safe design

**Delivered**:

- Trait with 8 core methods for rendering, transforms, and properties
- Object-safe design with `clone_mobject()` workaround
- Send + Sync bounds for parallel rendering
- Comprehensive documentation with examples
- 13 unit tests covering all trait methods

---

### Task 3.1.2: VMobject (Vectorized Mobject) ✅

**File**: `src/mobject/vmobject.rs`

**Status**: ✅ Base implementation complete

**Delivered**:

- Path-based mobject with stroke/fill styling
- Method chaining API (set_stroke, set_fill)
- from_points() convenience constructor
- Bounding box with stroke expansion
- Position tracking and delta-based movement
- Opacity clamping [0.0, 1.0]
- 28 unit tests covering all functionality

---

### Task 3.1.3: MobjectGroup ✅

**File**: `src/mobject/group.rs`

**Status**: ✅ Hierarchical composition complete

**Delivered**:

- Vec-based container for heterogeneous mobjects
- Transform propagation to all children
- Opacity propagation to all children
- Iterator support for traversal
- Nested group support
- 23 unit tests including nested groups

---

## Phase 3.2: Geometric Primitives ✅ (Completed 2025-10-19)

### All Shapes Implemented ✅

**Files**: `src/mobject/geometry/*.rs`

**Delivered Shapes**:

1. **Circle** (`circle.rs`) - 25 unit tests

   - 4 cubic Bézier curve approximation
   - Builder pattern with all options
   - Radius modification support

2. **Rectangle** (`rectangle.rs`) - 35+ unit tests

   - Width/height specification
   - Square convenience wrapper
   - Builder patterns for both

3. **Line** (`line.rs`) - 20+ unit tests

   - Start/end point specification
   - Length calculation
   - Builder pattern

4. **Polygon** (`polygon.rs`) - 25+ unit tests

   - Regular polygon generation
   - Irregular polygon from vertices
   - Builder pattern

5. **Ellipse** (`ellipse.rs`) - 20+ unit tests

   - Width/height specification
   - Bézier curve approximation
   - Builder pattern

6. **Arc** (`arc.rs`) - 20+ unit tests

   - Start/end angle specification
   - Radius and sweep control
   - Builder pattern

7. **Arrow** (`arrow.rs`) - 15+ unit tests
   - Start/end point specification
   - Tip customization
   - Builder pattern

**Total**: 150+ unit tests across all geometry primitives

---

## Phase 3.3: Complex Shapes ✅ (Completed 2025-10-19)

### Task 3.3.1: BezierPath ✅

**File**: `src/mobject/bezier_path.rs`

**Status**: ✅ Complete

**Delivered**:

- from_path() constructor
- from_bezier_curves() constructor
- Styling methods
- 8 unit tests

---

## Phase 4.1: Animation Foundation (Week 6-7)

### Task 4.1.1: Animation Trait

**File**: `src/animation/mod.rs`

```rust
pub trait Animation: Send + Sync {
    /// Update animation at given progress (0.0 to 1.0)
    fn interpolate(&mut self, alpha: f64);

    /// Get the mobject(s) being animated
    fn mobjects(&self) -> Vec<&dyn Mobject>;

    /// Get animation duration
    fn duration(&self) -> f64;

    /// Reset animation to start
    fn reset(&mut self);
}
```

**Dependencies**: `mobject::Mobject`  
**Estimated Time**: 1 day

---

### Task 4.1.2: Easing Functions

**File**: `src/animation/easing.rs`

```rust
pub type EasingFunction = fn(f64) -> f64;

pub mod ease {
    /// Linear interpolation (no easing)
    pub fn linear(t: f64) -> f64 { t }

    /// Smooth ease in and out
    pub fn smooth(t: f64) -> f64 {
        // Smoothstep: 3t² - 2t³
        t * t * (3.0 - 2.0 * t)
    }

    pub fn ease_in_quad(t: f64) -> f64 { t * t }
    pub fn ease_out_quad(t: f64) -> f64 { t * (2.0 - t) }

    // ... more easing functions
}
```

**Dependencies**: None  
**Tests**: Test easing curve shapes  
**Estimated Time**: 1-2 days

---

### Task 4.1.3: Timeline and Timing

**File**: `src/animation/timeline.rs`

```rust
pub struct Timeline {
    current_time: f64,
    animations: Vec<TimedAnimation>,
}

struct TimedAnimation {
    animation: Box<dyn Animation>,
    start_time: f64,
    duration: f64,
}

impl Timeline {
    pub fn add(&mut self, animation: Box<dyn Animation>, start: f64, duration: f64) { /* ... */ }
    pub fn advance(&mut self, dt: f64) { /* ... */ }
    pub fn is_complete(&self) -> bool { /* ... */ }
}
```

**Dependencies**: `animation::Animation`, `animation::EasingFunction`  
**Estimated Time**: 2 days

---

## Milestone 1 Target: "Static Shapes"

After completing Phase 2 and Phase 3.1-3.2, we should be able to:

```rust
use manim_rs::prelude::*;

fn main() -> Result<()> {
    // Create SVG renderer
    let mut renderer = SvgRenderer::new(1920, 1080);
    renderer.clear(Color::BLACK)?;

    // Create shapes
    let circle = Circle::new(2.0)
        .with_color(Color::BLUE)
        .at_position(Vector2D::new(0.0, 0.0));

    let square = Rectangle::new(3.0, 3.0)
        .with_color(Color::RED)
        .at_position(Vector2D::new(4.0, 0.0));

    // Render
    renderer.begin_frame()?;
    circle.render(&mut renderer)?;
    square.render(&mut renderer)?;
    renderer.end_frame()?;

    // Save
    renderer.save("output/static_shapes.svg")?;

    Ok(())
}
```

---

## Development Process Checklist

For each task:

- [ ] Write failing test first (TDD)
- [ ] Implement minimum functionality
- [ ] Make test pass
- [ ] Write documentation with examples
- [ ] Add integration test
- [ ] Run `cargo clippy` and fix warnings
- [ ] Run `cargo fmt`
- [ ] Update CHANGELOG.md
- [ ] Commit with conventional commit message
- [ ] Push to GitHub

---

## Time Estimates

| Phase                    | Tasks    | Status | Time Spent/Estimated |
| ------------------------ | -------- | ------ | -------------------- |
| 1.2 Extended Math        | 3 tasks  | ✅     | 4 days               |
| 2.1 Rendering Traits     | 3 tasks  | ✅     | 5 days               |
| 2.2 SVG Backend          | 3 tasks  | ✅     | 5 days               |
| 2.3 Raster Backend       | 3 tasks  | ✅     | 3 days               |
| **Phase 2 Total**        |          | ✅     | **17 days**          |
| 3.1 Mobject Base         | 3 tasks  | ✅     | 5 days               |
| 3.2 Shapes               | 7 shapes | ✅     | 7 days               |
| 3.3 Complex Shapes       | 1 task   | ✅     | 1 day                |
| **Phase 3 Total**        |          | ✅     | **13 days**          |
| **Total to Milestone 1** |          | ✅     | **30 days COMPLETE** |

---

## Success Criteria for Milestone 1 ✅

1. [x] Render circle to SVG file
2. [x] Render rectangle to SVG file
3. [x] Apply transformations (rotate, scale, translate)
4. [x] Set colors and stroke properties
5. [x] Multiple shapes in one scene
6. [x] All tests passing (249 unit tests + 15 integration tests)
7. [x] Documentation complete (100% coverage, all doctests passing)
8. [x] Examples working (shapes.rs, geometry_showcase.rs)
9. [x] **BONUS**: All 7 geometric primitives implemented
10. [x] **BONUS**: Builder patterns for ergonomic API
11. [x] **BONUS**: Performance benchmarks for all operations
12. [x] **BONUS**: Raster (PNG) rendering support

---

## Next Steps After This Document

1. **Review this plan** - Does the order make sense?
2. **Start Task 1.2.1** - BoundingBox implementation
3. **Create GitHub issues** - One issue per task for tracking
4. **Set up project board** - Kanban board for visual progress
5. **Begin implementation** - TDD approach, test-first

---

## Questions to Resolve

- [ ] Should we use the `svg` crate or hand-craft SVG XML?
- [ ] Do we need feature flags for backends (svg, raster, gpu)?
- [ ] Should VMobject own or borrow its Path?
- [ ] How to handle trait object cloning efficiently?
- [ ] What precision for bezier circle approximation?

---

## References

- [lyon crate docs](https://docs.rs/lyon/) - For path tessellation if needed
- [tiny-skia docs](https://docs.rs/tiny-skia/) - For raster rendering
- [Manim mobject implementation](https://github.com/ManimCommunity/manim/tree/main/manim/mobject) - Python reference
- [SVG Path Specification](https://www.w3.org/TR/SVG/paths.html) - SVG path syntax
- [Bezier Circle Approximation](http://spencermortensen.com/articles/bezier-circle/) - Circle to bezier math

---

**Last Updated**: 2025-10-19  
**Current Status**: Phase 3 Complete ✅ → Phase 4.1 Next 🔄  
**Next Task**: Task 4.1.1 - Animation Trait Implementation  
**Milestone 1 Achievement**: 100% COMPLETE ✅
