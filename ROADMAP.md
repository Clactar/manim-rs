# Manim-rs Development Roadmap

This document outlines the implementation plan for manim-rs, organized by dependency hierarchy. Each phase builds upon the previous ones.

## Current Status: Phase 2 (In Progress 🔄)

---

## Phase 1: Core Mathematical Foundations ✅

**Status**: COMPLETED  
**Dependencies**: None

### 1.1 Basic Math Types ✅

- [x] `Vector2D` - 2D vectors with operations
- [x] `Color` - RGBA color representation with hex support
- [x] `Transform` - 2D transformation matrices
- [x] Error handling system

### 1.2 Extended Math Types ✅

- [x] `BoundingBox` - Axis-aligned bounding boxes for spatial queries
- [x] `QuadraticBezier` / `CubicBezier` - Bézier curve utilities
- [x] Angle types (`Radians`, `Degrees`)
- [ ] `Vector3D` - 3D vectors for future 3D support (defer to Phase 7)
- [ ] `Matrix` - General matrix operations (defer to Phase 7)
- [ ] `Quaternion` - 3D rotations (defer to Phase 7)

**Why First**: Everything else depends on these primitives. Vectors define positions, colors define appearance, transforms define motion. BoundingBox and Bézier curves are needed before rendering can work properly.

---

## Phase 2: Rendering Foundation

**Status**: In Progress 🔄  
**Dependencies**: Phase 1.1 ✅, Phase 1.2 ✅  
**Estimated Complexity**: High

### 2.1 Rendering Traits & Abstractions

- [ ] `Renderer` trait - Backend-agnostic rendering interface
- [ ] `PathStyle` / `TextStyle` - Rendering style types
- [ ] `Path` - 2D path representation with Bézier commands
- [ ] `RenderContext` - Shared rendering state (optional for Milestone 1)
- [ ] `Layer` system for depth control (defer to Phase 5)

### 2.2 SVG Backend (Priority 1)

- [ ] SVG path generation
- [ ] SVG element rendering (shapes, text)
- [ ] SVG file output
- [ ] SVG animation support (SMIL)

### 2.3 Raster Backend (Priority 2)

- [ ] Integration with `tiny-skia` for CPU rasterization
- [ ] PNG/JPEG image export
- [ ] Frame buffer management

### 2.4 GPU Backend (Future - Optional)

- [ ] `wgpu` integration for hardware acceleration
- [ ] Shader system
- [ ] GPU-accelerated transformations

**Why Second**: We need to be able to draw before we can animate. The renderer is independent of scene logic and can be tested with simple shapes.

---

## Phase 3: Mathematical Objects (Mobjects)

**Status**: Not Started  
**Dependencies**: Phase 1, Phase 2.1-2.2  
**Estimated Complexity**: Medium-High

### 3.1 Base Mobject System

- [ ] `Mobject` trait - Common interface for all drawable objects
- [ ] `MobjectProperties` - Shared properties (position, color, opacity)
- [ ] `MobjectGroup` - Container for multiple mobjects
- [ ] Hierarchy and parent-child relationships

### 3.2 Geometric Primitives

- [ ] `Point` - Single point
- [ ] `Line` - Line segment
- [ ] `Circle` - Circle/arc
- [ ] `Rectangle` / `Square`
- [ ] `Polygon` - Regular and irregular polygons
- [ ] `Arc` - Circular arcs
- [ ] `Arrow` - Arrows with customizable tips
- [ ] `Ellipse`

### 3.3 Complex Shapes

- [ ] `BezierPath` - Arbitrary bezier curves
- [ ] `VMobject` - Vectorized mobject (from paths)
- [ ] `CurvesAsSubmobjects` - Complex compound shapes
- [ ] SVG import and parsing

### 3.4 Text Rendering

- [ ] Font loading and management
- [ ] `Text` mobject with font/size/weight
- [ ] `MarkupText` - Rich text with formatting
- [ ] Glyph path extraction
- [ ] Text measurement and layout

### 3.5 Mathematical Notation (Later Priority)

- [ ] LaTeX integration (via external process)
- [ ] `MathTex` - Mathematical equations
- [ ] `Tex` - General LaTeX text
- [ ] Symbol library for common math symbols

**Why Third**: Mobjects are what we actually render. They need the rendering backend but are independent of animation logic. We can test them by rendering static frames.

**Implementation Strategy**:

1. Start with geometric primitives (easiest to test)
2. Add text support (requires font handling)
3. Add LaTeX support last (most complex, external dependency)

---

## Phase 4: Animation System

**Status**: Not Started  
**Dependencies**: Phase 1, Phase 3.1-3.2  
**Estimated Complexity**: High

### 4.1 Time and Interpolation

- [ ] `Timeline` - Manages animation timing
- [ ] Easing functions library:
  - Linear, smooth (ease in/out)
  - Cubic, exponential, elastic
  - Custom easing function support
- [ ] `Interpolatable` trait for smooth transitions
- [ ] Rate functions and timing utilities

### 4.2 Core Animation Primitives

- [ ] `Animation` trait - Base for all animations
- [ ] `AnimationGroup` - Parallel/sequential animation groups
- [ ] `Wait` - Pause animation
- [ ] `Succession` - Chain animations

### 4.3 Transform Animations

- [ ] `Transform` - Morph one mobject into another
- [ ] `ReplacementTransform` - Replace with interpolation
- [ ] `TransformFromCopy` - Animate a copy
- [ ] `ClockwiseTransform` / `CounterclockwiseTransform`

### 4.4 Creation/Destruction Animations

- [ ] `Create` / `Uncreate` - Draw/undraw paths
- [ ] `Write` - Animated writing effect
- [ ] `FadeIn` / `FadeOut` - Opacity animations
- [ ] `GrowFromCenter` / `ShrinkToCenter` - Scale animations
- [ ] `DrawBorderThenFill` - Outline then fill

### 4.5 Movement Animations

- [ ] `MoveToTarget` - Move to specified position
- [ ] `ApplyMethod` - Animate method application
- [ ] `Rotate` - Rotation animation
- [ ] `Shift` - Translation animation
- [ ] `Scale` - Scaling animation

### 4.6 Indication Animations

- [ ] `Indicate` - Highlight temporarily
- [ ] `Flash` - Brief flash effect
- [ ] `FocusOn` / `CircleIndicate` - Draw attention
- [ ] `Wiggle` - Shake/wobble effect

**Why Fourth**: Animations modify mobjects over time. They need mobjects to exist and the rendering system to display frames, but they're self-contained logic.

**Implementation Strategy**:

1. Build timing/interpolation foundation first
2. Add simple animations (fade, move) to test the system
3. Add complex animations (transform, morph)
4. Add indication/effect animations last

---

## Phase 5: Scene Management

**Status**: Placeholder exists  
**Dependencies**: Phase 1, Phase 2, Phase 3, Phase 4  
**Estimated Complexity**: Medium

### 5.1 Scene Core

- [x] `SceneConfig` - Scene configuration (resolution, fps, etc.)
- [ ] `Scene` - Main scene orchestration
- [ ] Scene timeline management
- [ ] Frame rate control and timing
- [ ] Background color/image support

### 5.2 Camera System

- [ ] `Camera` - Virtual camera
- [ ] Pan, zoom, rotate camera
- [ ] Camera frame and viewport management
- [ ] `MovingCamera` - Animated camera
- [ ] Multiple camera support

### 5.3 Scene Methods (Manim API compatibility)

- [ ] `self.add()` - Add mobjects to scene
- [ ] `self.remove()` - Remove mobjects
- [ ] `self.play()` - Play animations
- [ ] `self.wait()` - Pause
- [ ] `self.next_section()` - Scene sections

### 5.4 Advanced Scene Features

- [ ] `ThreeDScene` - 3D scene support (future)
- [ ] `VectorScene` - Vector field scenes
- [ ] `GraphScene` - Graphs and plots
- [ ] Section markers for editing

**Why Fifth**: Scenes orchestrate everything else. They need mobjects, animations, and rendering to be complete. This is where the user-facing API lives.

---

## Phase 6: Output and Export

**Status**: Not Started  
**Dependencies**: Phase 2, Phase 5  
**Estimated Complexity**: Medium-High

### 6.1 Image Export

- [ ] Single frame export (PNG, JPEG)
- [ ] High-resolution rendering
- [ ] Transparent backgrounds

### 6.2 Video Export

- [ ] Frame sequence generation
- [ ] FFmpeg integration for video encoding
- [ ] MP4 / WebM output
- [ ] GIF generation
- [ ] Codec selection and quality settings

### 6.3 Interactive Preview

- [ ] Live preview window (optional)
- [ ] Real-time playback
- [ ] Scrubbing timeline
- [ ] Interactive controls

**Why Sixth**: Export is the final step - we need scenes fully working before we can export them. Video encoding is independent of the core animation logic.

---

## Phase 7: Advanced Features

**Status**: Not Started  
**Dependencies**: All previous phases  
**Estimated Complexity**: Varies

### 7.1 Graph and Plot System

- [ ] `Axes` - Coordinate axes
- [ ] `NumberLine` - 1D number line
- [ ] `Graph` - Function plotting
- [ ] `BarChart` / `PieChart` - Data visualization
- [ ] `NumberPlane` - 2D coordinate grid

### 7.2 3D Support

- [ ] 3D vector operations (extend Phase 1.2)
- [ ] 3D transformations and rotations
- [ ] 3D camera with perspective
- [ ] 3D shapes (sphere, cube, surface)
- [ ] Lighting and shading

### 7.3 Physics and Dynamics

- [ ] Gravity and forces
- [ ] Particle systems
- [ ] Collision detection
- [ ] Spring/pendulum simulations

### 7.4 Interactive Features

- [ ] Mouse/keyboard input handling
- [ ] Interactive widgets
- [ ] Real-time manipulation
- [ ] Jupyter notebook integration

### 7.5 Performance Optimizations

- [ ] Parallel rendering with Rayon
- [ ] Object culling and frustum culling
- [ ] Level-of-detail (LOD) system
- [ ] Caching and memoization
- [ ] SIMD vectorization for batch operations

**Why Last**: These are enhancements to the core system. Each depends on a solid foundation.

---

## Implementation Guidelines

### Development Order Within Each Phase

1. **Write tests first** (TDD approach where possible)
2. **Implement core functionality** (MVP)
3. **Add documentation and examples**
4. **Optimize and refine**
5. **Integration tests** with previous phases

### Testing Strategy

- **Unit tests**: Each module in isolation
- **Integration tests**: Cross-module interactions
- **Example tests**: Full end-to-end examples
- **Benchmarks**: Performance-critical paths
- **Visual tests**: Compare rendered output

### Documentation Requirements

- Public API docs with examples
- Module-level architecture docs
- User guides for each major feature
- Migration guide from Python Manim (future)

### Code Quality Gates

- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] Documentation builds
- [ ] Examples run successfully
- [ ] Benchmarks show acceptable performance

---

## Milestones

### Milestone 1: "Static Shapes"

**Target**: Static scene with basic shapes rendered to SVG  
**Phases**: 1.1 ✅, 1.2, 2.1-2.2, 3.1-3.2  
**Demo**: Circle, square, line rendered to SVG file  
**Estimated Time**: 4-5 weeks from start of Phase 1.2

### Milestone 2: "Simple Animations"

**Target**: Basic animations (fade, move, rotate)  
**Phases**: 4.1-4.3, 5.1-5.3  
**Demo**: Square morphing into circle with fade

### Milestone 3: "Text and LaTeX"

**Target**: Text rendering and mathematical notation  
**Phases**: 3.4-3.5  
**Demo**: Animated equation with explanation text

### Milestone 4: "Video Export"

**Target**: Export animations to MP4  
**Phases**: 6.1-6.2  
**Demo**: Full animation exported to video file

### Milestone 5: "Feature Parity"

**Target**: Most common Manim features implemented  
**Phases**: 7.1-7.2  
**Demo**: Complex mathematical visualization

### Milestone 6: "Production Ready"

**Target**: Performance, polish, GPU support  
**Phases**: 7.3-7.5  
**Demo**: High-performance complex animations

---

## Success Criteria

### Performance Targets

- Render simple shapes: < 1ms per frame
- Complex scene (50+ objects): < 16ms per frame (60fps)
- 1080p video export: Real-time or better

### API Design Goals

- Intuitive API similar to Python Manim
- Type-safe with compile-time guarantees
- Extensible for custom mobjects and animations
- Zero-cost abstractions where possible

### Community Goals

- Comprehensive documentation
- 20+ working examples
- Active community engagement
- Weekly progress updates

---

## Contributing to the Roadmap

This roadmap is a living document. To contribute:

1. **Add detail** to underspecified items
2. **Suggest reordering** if dependencies are wrong
3. **Add missing features** you'd like to see
4. **Update status** as work progresses
5. **Add benchmarks** and performance targets

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to propose changes.

---

## References

- [ManimCommunity/manim](https://github.com/ManimCommunity/manim) - Original Python implementation
- [3b1b/manim](https://github.com/3b1b/manim) - Grant Sanderson's original version
- [Manim Community Docs](https://docs.manim.community/) - API reference and tutorials

---

**Last Updated**: 2025-10-19
**Current Phase**: Phase 1.2 (Completed) ✅ → Phase 2.1 (Next) 🔄

---

## Roadmap Review Notes (2025-10-19)

### Key Improvements Made

1. **Clarified Phase 1.2**: BoundingBox and Bézier curves must come before rendering
2. **Deferred 3D math**: Vector3D, Matrix, Quaternion moved to Phase 7 (not needed for 2D Manim)
3. **Simplified Phase 2.1**: Removed premature abstractions (RenderContext, Layer system)
4. **Updated dependencies**: Phase 2 now correctly depends on Phase 1.2
5. **Realistic timeline**: Milestone 1 is now 4-5 weeks (was 3-4)

### Critical Path for Milestone 1

```
Phase 1.2 (BoundingBox, Bézier)
  → Phase 2.1 (Renderer trait, Path, PathStyle)
  → Phase 2.2 (SVG Backend)
  → Phase 3.1 (Mobject trait, VMobject)
  → Phase 3.2 (Circle, Rectangle, Line)
  → ✅ Milestone 1: Render static shapes to SVG
```
