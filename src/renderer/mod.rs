//! Backend-agnostic rendering traits.
//!
//! This module defines the rendering abstractions that all rendering backends
//! must implement. The API is designed for high throughput drawing pipelines
//! and keeps allocations to a minimum by borrowing data wherever possible.
//!
//! # Overview
//!
//! - [`Renderer`] is the core trait implemented by concrete backends (SVG,
//!   raster, GPU, etc.)
//! - [`Path`] describes geometry as a sequence of drawing commands
//! - [`PathStyle`] and [`TextStyle`] configure stroke, fill, and typography
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::renderer::{Path, PathCommand, PathStyle};
//!
//! let mut path = Path::new();
//! path.move_to(Vector2D::new(0.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 1.0))
//!     .close();
//!
//! let style = PathStyle::default();
//!
//! // Renderer backends take `&Path` and `&PathStyle` references to avoid extra
//! // allocations. The concrete backend decides how to rasterize the commands.
//! ```

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};

mod path;
mod style;

pub use path::{Path, PathCommand, PathCursor};
pub use style::{FontWeight, PathFillRule, PathStyle, TextAlignment, TextStyle};

/// Core trait implemented by all rendering backends.
///
/// Backends are expected to minimize allocations and leverage platform
/// capabilities (SIMD, multithreading, GPU) to keep rendering overhead low. The
/// trait is intentionally object-safe so that higher-level APIs can operate on
/// `dyn Renderer` trait objects.
pub trait Renderer {
    /// Begins rendering a new frame.
    ///
    /// Backends can treat this as a signal to reset internal buffers. The
    /// default implementation in [`Renderer`] does nothing; backends should
    /// override when necessary.
    fn begin_frame(&mut self) -> Result<()> {
        Ok(())
    }

    /// Finalizes the current frame.
    ///
    /// Backends may flush pending draw calls or write the frame to disk.
    fn end_frame(&mut self) -> Result<()> {
        Ok(())
    }

    /// Clears the current frame buffer to the provided color.
    fn clear(&mut self, color: Color) -> Result<()>;

    /// Draws a path with the provided style.
    ///
    /// Implementations must honor both stroke and fill properties. The path is
    /// immutable and can be cached by the backend if beneficial.
    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()>;

    /// Draws UTF-8 text at the given position.
    ///
    /// Text rendering details (font loading, shaping, hinting) are delegated to
    /// backends. Higher-level abstractions are expected to pre-process text
    /// (e.g., convert to vector paths) when deterministic output is needed.
    fn draw_text(&mut self, text: &str, position: Vector2D, style: &TextStyle) -> Result<()>;

    /// Returns the current viewport dimensions in pixels.
    fn dimensions(&self) -> (u32, u32);
}

/// Trait for types that can provide a cached path representation.
///
/// This enables higher-level objects (mobjects) to expose precomputed paths to
/// renderers, minimizing redundant allocations and ensuring deterministic
/// geometry.
pub trait PathProvider {
    /// Returns the geometric path for this object.
    fn path(&self) -> &Path;

    /// Returns an axis-aligned bounding box for fast culling.
    fn bounding_box(&self) -> BoundingBox;

    /// Applies a transform to the underlying geometry in-place.
    fn apply_transform(&mut self, transform: &Transform);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRenderer {
        width: u32,
        height: u32,
        cleared_with: Option<Color>,
        last_path: Option<Path>,
        last_style: Option<PathStyle>,
        last_text: Option<(String, Vector2D, TextStyle)>,
    }

    impl TestRenderer {
        fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                cleared_with: None,
                last_path: None,
                last_style: None,
                last_text: None,
            }
        }
    }

    impl Renderer for TestRenderer {
        fn clear(&mut self, color: Color) -> Result<()> {
            self.cleared_with = Some(color);
            Ok(())
        }

        fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()> {
            self.last_path = Some(path.clone());
            self.last_style = Some(style.clone());
            Ok(())
        }

        fn draw_text(&mut self, text: &str, position: Vector2D, style: &TextStyle) -> Result<()> {
            self.last_text = Some((text.to_owned(), position, style.clone()));
            Ok(())
        }

        fn dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }
    }

    #[test]
    fn renderer_trait_allows_basic_operations() {
        let mut renderer = TestRenderer::new(1920, 1080);

        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let style = PathStyle::default();
        renderer.clear(Color::BLACK).unwrap();
        renderer.draw_path(&path, &style).unwrap();
        renderer
            .draw_text("hello", Vector2D::new(10.0, 20.0), &TextStyle::default())
            .unwrap();

        assert_eq!(renderer.dimensions(), (1920, 1080));
        assert!(renderer.cleared_with.is_some());
        assert!(renderer.last_path.is_some());
        assert!(renderer.last_style.is_some());
        assert!(renderer.last_text.is_some());
    }
}
