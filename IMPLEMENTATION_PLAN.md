# Implementation Plan: Detailed Next Steps

This document provides granular, actionable tasks for implementing manim-rs according to the [ROADMAP.md](ROADMAP.md) dependency hierarchy.

---

## Immediate Focus: Phase 1.2 & Phase 2 - Extended Math + Rendering Foundation

### Why This Order?

Based on analyzing the [Manim Community repository](https://github.com/ManimCommunity/manim), the dependency hierarchy is:

```
Core Math (✅ Phase 1.1 Done)
    ↓
Extended Math (BoundingBox, Bézier, etc.) (Phase 1.2)
    ↓
Rendering Backend
    ↓
Mobjects (use math + rendering)
    ↓
Animations (modify mobjects over time)
    ↓
Scenes (orchestrate everything)
    ↓
Export (output scenes)
```

We **cannot** implement rendering without geometric primitives (bézier, bounding box), and we **cannot** implement mobjects without a renderer. Therefore, **complete foundational math first, then rendering**.

---

## Phase 1.2: Extended Math Types (Week 1)

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

## Phase 2.1: Rendering Traits (Week 2)

### Task 2.1.1: Define Core Rendering Traits

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
**Tests**: Create mock renderer, test trait interface  
**Estimated Time**: 1-2 days

---

### Task 2.1.2: Rendering Style Types

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
**Tests**: Style creation and defaults  
**Estimated Time**: 1 day

---

### Task 2.1.3: Path Representation

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
**Tests**: Path building, bounding box calculation, transformations  
**Estimated Time**: 2-3 days

---

---

## Phase 2.2: SVG Backend (Week 3)

### Task 2.2.1: SVG Document Builder

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

### Task 2.2.2: Path to SVG Conversion

**File**: `src/backends/svg/path.rs`

```rust
pub fn path_to_svg_d(path: &Path) -> String {
    // Convert Path commands to SVG "d" attribute
    // Example: "M 10 10 L 20 20 Q 30 30 40 40 Z"
}

pub fn style_to_svg_attrs(style: &PathStyle) -> Vec<(&str, String)> {
    // Convert PathStyle to SVG attributes
    // [("stroke", "#FF0000"), ("stroke-width", "2"), ("fill", "none")]
}
```

**Dependencies**: `renderer::Path`, `renderer::PathStyle`  
**Tests**: Various path types, style combinations  
**Estimated Time**: 1-2 days

---

### Task 2.2.3: Integration Test

**File**: `tests/svg_rendering.rs`

```rust
#[test]
fn test_render_circle_to_svg() {
    let mut renderer = SvgRenderer::new(800, 600);
    renderer.clear(Color::WHITE).unwrap();

    // Create a circular path
    let mut path = Path::new();
    // ... build circle using bezier curves

    let style = PathStyle {
        stroke_color: Some(Color::BLUE),
        stroke_width: 2.0,
        fill_color: None,
        opacity: 1.0,
    };

    renderer.begin_frame().unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    renderer.save("test_output/circle.svg").unwrap();

    // Verify SVG file exists and contains expected elements
}
```

**Estimated Time**: 1 day

---

## Phase 3.1: Base Mobject System (Week 4-5)

### Task 3.1.1: Mobject Trait

**File**: `src/mobject/mod.rs`

```rust
/// Core trait for all mathematical objects
pub trait Mobject: Send + Sync {
    /// Render the mobject using the given renderer
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()>;

    /// Get the bounding box of the mobject
    fn bounding_box(&self) -> BoundingBox;

    /// Apply a transformation to the mobject
    fn apply_transform(&mut self, transform: &Transform);

    /// Get/set position
    fn position(&self) -> Vector2D;
    fn set_position(&mut self, pos: Vector2D);

    /// Get/set opacity
    fn opacity(&self) -> f64;
    fn set_opacity(&mut self, opacity: f64);

    /// Clone the mobject (workaround for trait object cloning)
    fn clone_mobject(&self) -> Box<dyn Mobject>;
}
```

**Dependencies**: `renderer::Renderer`, `core::Transform`, `core::BoundingBox`  
**Estimated Time**: 2 days

**Note**: Individual color is handled at VMobject level (stroke/fill), not in base trait.

---

### Task 3.1.2: VMobject (Vectorized Mobject)

**Important**: This is the **base implementation** that most shapes will use.

**File**: `src/mobject/vmobject.rs`

```rust
/// A mobject based on vector paths
#[derive(Clone)]
pub struct VMobject {
    path: Path,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
    position: Vector2D,
}

impl VMobject {
    pub fn new(path: Path) -> Self { /* ... */ }

    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self { /* ... */ }
    pub fn set_fill(&mut self, color: Color) -> &mut Self { /* ... */ }
}

impl Mobject for VMobject {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let style = PathStyle {
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            fill_color: self.fill_color,
            opacity: self.opacity,
        };
        renderer.draw_path(&self.path, &style)
    }

    // ... implement other trait methods
}
```

**Dependencies**: `mobject::Mobject`, `renderer::Path`  
**Tests**: Create, render, transform VMobjects  
**Estimated Time**: 2-3 days

---

### Task 3.2.1: Circle Mobject

**File**: `src/mobject/geometry/circle.rs`

```rust
/// A circle mobject
pub struct Circle {
    vmobject: VMobject,
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        let path = Self::create_circle_path(radius);
        Self {
            vmobject: VMobject::new(path),
            radius,
        }
    }

    fn create_circle_path(radius: f64) -> Path {
        // Create path using 4 cubic bezier curves to approximate circle
        // Magic number for bezier: 0.5519150244935105707435627
    }

    pub fn builder() -> CircleBuilder { /* ... */ }
}

impl Mobject for Circle {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.vmobject.render(renderer)
    }
    // Delegate other methods to vmobject
}
```

**Dependencies**: `mobject::VMobject`  
**Tests**: Render circle, verify path accuracy  
**Estimated Time**: 1-2 days

---

### Task 3.2.2: Rectangle and Other Shapes

Similar implementation for:

- `Rectangle` / `Square`
- `Line`
- `Polygon`
- `Arrow`

**Estimated Time**: 1 day each (5 days total)

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

| Phase                    | Tasks    | Estimated Time |
| ------------------------ | -------- | -------------- |
| 1.2 Extended Math        | 3 tasks  | 4-5 days       |
| 2.1 Rendering Traits     | 3 tasks  | 4-6 days       |
| 2.2 SVG Backend          | 3 tasks  | 5-7 days       |
| 3.1 Mobject Base         | 2 tasks  | 4-5 days       |
| 3.2 Shapes               | 6 shapes | 6-8 days       |
| **Total to Milestone 1** |          | **4-5 weeks**  |

---

## Success Criteria for Milestone 1

1. [ ] Render circle to SVG file
2. [ ] Render rectangle to SVG file
3. [ ] Apply transformations (rotate, scale, translate)
4. [ ] Set colors and stroke properties
5. [ ] Multiple shapes in one scene
6. [ ] All tests passing
7. [ ] Documentation complete
8. [ ] Examples working

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
**Next Task**: Task 1.2.1 - BoundingBox Implementation

---

## Key Changes from Original Plan

1. **Added Phase 1.2** - Extended math types (BoundingBox, Bézier) before rendering
2. **Separated PathStyle** - Into its own file (Task 2.1.2) for better organization
3. **Reordered Animation tasks** - Animation trait → Easing → Timeline (dependency-correct)
4. **Simplified Mobject trait** - Removed `color()` methods (handled by VMobject)
5. **Updated time estimates** - Now 4-5 weeks to Milestone 1 (was 3-4)
