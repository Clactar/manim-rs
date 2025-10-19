# Manim-rs Architecture

This document explains the architectural decisions and dependency hierarchy of manim-rs.

## Core Principle: Dependency Hierarchy

Manim-rs is built in layers, where each layer only depends on layers below it:

```
┌─────────────────────────────────────┐
│         Export & Output             │  Phase 6
│    (video, images, interactive)     │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│       Scene Management              │  Phase 5
│  (orchestration, camera, timeline)  │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│       Animation System              │  Phase 4
│   (timing, easing, interpolation)   │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Mobjects (Math Objects)        │  Phase 3
│   (shapes, text, LaTeX, groups)     │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│      Rendering Foundation           │  Phase 2
│  (traits, backends: SVG, GPU, etc)  │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│    Core Mathematical Types          │  Phase 1 ✅
│   (Vector2D, Color, Transform)      │
└─────────────────────────────────────┘
```

## Why This Order?

### 1. Core Math First (Phase 1) ✅
Everything in an animation library needs:
- **Positions**: `Vector2D`
- **Colors**: `Color`
- **Transformations**: `Transform`

These are pure data types with no external dependencies. They're done!

### 2. Rendering Second (Phase 2) 🔄 Next
Before we can create shapes, we need to know **how to draw them**.

**Key Insight from Manim**: The rendering layer is abstract. Manim uses:
- SVG for vector graphics
- Cairo/OpenGL for rasterization
- Different backends for different use cases

We define:
1. **Renderer trait** - Abstract interface
2. **Path representation** - How shapes are described
3. **SVG backend** - First concrete implementation

This lets us test shapes by rendering them to SVG files.

### 3. Mobjects Third (Phase 3)
Mobjects are **what we render**. They use:
- **Core math** for positions and transforms
- **Renderer** to draw themselves

A Circle needs to:
1. Store its radius (math)
2. Convert radius → bezier path (geometry)
3. Call renderer to draw the path (rendering)

Dependencies: ✅ Core, ✅ Renderer

### 4. Animations Fourth (Phase 4)
Animations **modify mobjects over time**. They need:
- **Mobjects** to animate
- **Interpolation** to transition smoothly
- **Timeline** to coordinate timing

A FadeIn animation:
1. Takes a mobject
2. Changes its opacity from 0 → 1
3. Uses easing function for smooth transition

Dependencies: ✅ Core, ✅ Mobjects

### 5. Scenes Fifth (Phase 5)
Scenes **orchestrate everything**:
- Add mobjects to the scene
- Play animations in sequence
- Render frames at specified FPS
- Manage camera view

Dependencies: ✅ All previous phases

### 6. Export Last (Phase 6)
Export takes **completed scenes** and outputs them:
- Individual frames → images
- Frame sequences → video (via FFmpeg)
- Live preview window

Dependencies: ✅ Scenes must work first

## Module Structure

```
manim-rs/
├── src/
│   ├── core/                    # Phase 1 ✅
│   │   ├── vector.rs           # Vector2D
│   │   ├── color.rs            # Color
│   │   ├── transform.rs        # Transform
│   │   ├── bounding_box.rs     # BoundingBox (Phase 2)
│   │   └── error.rs            # Error types
│   │
│   ├── renderer/                # Phase 2
│   │   ├── mod.rs              # Renderer trait
│   │   ├── path.rs             # Path representation
│   │   └── style.rs            # Rendering styles
│   │
│   ├── backends/                # Phase 2
│   │   ├── svg/                # SVG backend
│   │   ├── raster/             # Raster backend (later)
│   │   └── gpu/                # GPU backend (future)
│   │
│   ├── mobject/                 # Phase 3
│   │   ├── mod.rs              # Mobject trait
│   │   ├── vmobject.rs         # Vector-based mobject
│   │   ├── group.rs            # Mobject groups
│   │   ├── geometry/           # Geometric shapes
│   │   │   ├── circle.rs
│   │   │   ├── rectangle.rs
│   │   │   ├── line.rs
│   │   │   └── ...
│   │   ├── text/               # Text rendering
│   │   └── tex/                # LaTeX (later)
│   │
│   ├── animation/               # Phase 4
│   │   ├── mod.rs              # Animation trait
│   │   ├── timeline.rs         # Timeline management
│   │   ├── easing.rs           # Easing functions
│   │   ├── creation.rs         # Create, FadeIn, etc.
│   │   ├── transform.rs        # Transform, Rotate, etc.
│   │   └── movement.rs         # Move, Shift, etc.
│   │
│   ├── scene/                   # Phase 5
│   │   ├── mod.rs              # Scene trait
│   │   ├── camera.rs           # Camera system
│   │   └── config.rs           # Scene configuration
│   │
│   └── export/                  # Phase 6
│       ├── image.rs            # Image export
│       ├── video.rs            # Video export
│       └── preview.rs          # Live preview
│
├── examples/                    # Examples for each phase
├── benches/                     # Performance benchmarks
└── tests/                       # Integration tests
```

## Key Design Patterns

### 1. Trait-Based Polymorphism
```rust
pub trait Mobject {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()>;
    fn bounding_box(&self) -> BoundingBox;
    // ...
}
```
**Why**: Allows different types of mobjects (shapes, text, groups) to be used interchangeably.

### 2. Builder Pattern
```rust
Circle::builder()
    .radius(2.0)
    .color(Color::BLUE)
    .stroke_width(2.0)
    .build()
```
**Why**: Ergonomic API for complex object construction.

### 3. Backend Abstraction
```rust
pub trait Renderer {
    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()>;
}
```
**Why**: Swap rendering backends (SVG, GPU) without changing mobject code.

### 4. Type-State Pattern (Future)
```rust
Animation<Configured> → play() → Animation<Playing>
```
**Why**: Compile-time guarantees about animation state.

## Performance Considerations

### Zero-Cost Abstractions
- Traits compile to static dispatch where possible
- `#[inline]` on hot paths
- Const generics for compile-time optimization

### SIMD Opportunities
- Batch vector operations
- Color blending
- Transform matrices

### Parallel Rendering
- Rayon for multi-threaded frame rendering
- Independent mobjects render in parallel
- Scene graph traversal optimization

## Testing Strategy

### Unit Tests
Each module has tests for its own functionality:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_vector_addition() { /* ... */ }
}
```

### Integration Tests
Cross-module tests in `tests/`:
```rust
#[test]
fn test_render_circle_to_svg() {
    let mut renderer = SvgRenderer::new(800, 600);
    let circle = Circle::new(2.0);
    circle.render(&mut renderer).unwrap();
}
```

### Visual Tests
Compare rendered output against reference images:
```rust
#[test]
fn test_visual_regression() {
    let rendered = render_scene();
    assert_images_match(rendered, "reference/expected.png");
}
```

## API Design Philosophy

### Inspired by Python Manim
```python
# Python Manim
class MyScene(Scene):
    def construct(self):
        circle = Circle()
        self.play(Create(circle))
        self.wait()
```

```rust
// Rust manim-rs (goal)
impl Scene for MyScene {
    fn construct(&mut self) -> Result<()> {
        let circle = Circle::new(1.0);
        self.play(Create::new(circle))?;
        self.wait(1.0)?;
        Ok(())
    }
}
```

### Type Safety
Rust catches errors at compile time:
```rust
// Won't compile: can't animate something that isn't Animatable
let animation = FadeIn::new(not_a_mobject); // ❌ Compile error

// Must handle errors explicitly
scene.play(animation)?; // ✅ Forces error handling
```

## Future Extensions

### 3D Support
Add to Phase 1:
- `Vector3D`
- `Quaternion` for rotations
- 3D transforms

Add to Phase 3:
- `ThreeDObject` trait
- 3D shapes (sphere, cube)

Add to Phase 5:
- `ThreeDCamera` with perspective

### GPU Acceleration
Add to Phase 2:
- `wgpu` backend
- Shader compilation
- GPU buffer management

### Interactivity
Add to Phase 7:
- Event system
- Mouse/keyboard input
- Real-time scene manipulation

## References

- [ManimCommunity/manim](https://github.com/ManimCommunity/manim) - Python implementation
- [ROADMAP.md](../ROADMAP.md) - Full project roadmap
- [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) - Detailed next steps

---

**Last Updated**: 2025-10-19

