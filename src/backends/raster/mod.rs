//! Raster rendering backend using tiny-skia.
//!
//! This module provides a CPU-based raster rendering implementation using the
//! `tiny-skia` library. It's ideal for generating high-quality PNG images
//! or when you need pixel-perfect output.
//!
//! # Examples
//!
//! ```
//! use manim_rs::backends::RasterRenderer;
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::renderer::{Path, PathStyle, Renderer};
//!
//! # #[cfg(feature = "raster")]
//! # fn main() -> manim_rs::core::Result<()> {
//! let mut renderer = RasterRenderer::new(1920, 1080);
//!
//! // Create a simple path
//! let mut path = Path::new();
//! path.move_to(Vector2D::new(0.0, 0.0))
//!     .line_to(Vector2D::new(100.0, 100.0));
//!
//! // Render it
//! renderer.begin_frame()?;
//! renderer.clear(Color::WHITE)?;
//! renderer.draw_path(&path, &PathStyle::default())?;
//! renderer.end_frame()?;
//!
//! // Save to PNG
//! renderer.save_png("output.png")?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "raster"))]
//! # fn main() {}
//! ```
//!
//! # Performance
//!
//! - CPU-based rendering with SIMD optimizations
//! - Anti-aliasing enabled by default for smooth edges
//! - Memory efficient: single pixmap allocation
//!
//! # Coordinate System
//!
//! The raster renderer uses a centered coordinate system where (0, 0) is at
//! the center of the canvas, with positive Y pointing up.

use std::fs;

use crate::core::{Color, Error, Result, Vector2D};
use crate::renderer::{Path, PathStyle, Renderer, TextStyle};

mod path_converter;
mod style_converter;

pub use path_converter::path_to_tiny_skia;
pub use style_converter::{
    color_to_skia_color, fill_rule_to_skia, path_style_to_fill_paint, path_style_to_stroke,
    path_style_to_stroke_paint,
};

/// Raster rendering backend using tiny-skia.
///
/// Renders scenes to raster images (PNG, etc.) using CPU-based rendering.
/// Provides high-quality anti-aliased output suitable for final renders.
///
/// # Examples
///
/// ```
/// use manim_rs::backends::RasterRenderer;
/// use manim_rs::core::Color;
/// use manim_rs::renderer::Renderer;
///
/// # #[cfg(feature = "raster")]
/// # fn test() {
/// let mut renderer = RasterRenderer::new(1920, 1080);
/// renderer.clear(Color::WHITE).unwrap();
///
/// // Render your scene...
/// # }
/// ```
pub struct RasterRenderer {
    width: u32,
    height: u32,
    pixmap: tiny_skia::Pixmap,
}

impl RasterRenderer {
    /// Creates a new raster renderer with the specified dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::backends::RasterRenderer;
    /// use manim_rs::renderer::Renderer;
    ///
    /// # #[cfg(feature = "raster")]
    /// # {
    /// let renderer = RasterRenderer::new(1920, 1080);
    /// assert_eq!(renderer.dimensions(), (1920, 1080));
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the dimensions are invalid (zero or too large for allocation).
    pub fn new(width: u32, height: u32) -> Self {
        let pixmap = tiny_skia::Pixmap::new(width, height)
            .expect("Failed to allocate pixmap - dimensions may be invalid");

        Self {
            width,
            height,
            pixmap,
        }
    }

    /// Saves the rendered image as a PNG file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use manim_rs::backends::RasterRenderer;
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::Renderer;
    ///
    /// # #[cfg(feature = "raster")]
    /// # fn main() -> manim_rs::core::Result<()> {
    /// let mut renderer = RasterRenderer::new(1920, 1080);
    /// renderer.clear(Color::BLACK)?;
    ///
    /// renderer.save_png("output/scene.png")?;
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "raster"))]
    /// # fn main() {}
    /// ```
    pub fn save_png(&self, path: &str) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        self.pixmap
            .save_png(path)
            .map_err(|e| Error::Render(format!("Failed to save PNG: {}", e)))
    }

    /// Returns a reference to the underlying pixmap data.
    ///
    /// Useful for custom post-processing or analysis.
    pub fn pixmap(&self) -> &tiny_skia::Pixmap {
        &self.pixmap
    }

    /// Returns the raw pixel data as a byte slice (RGBA).
    pub fn data(&self) -> &[u8] {
        self.pixmap.data()
    }

    /// Converts screen coordinates to pixmap coordinates.
    ///
    /// Manim uses centered coordinates with Y-up, while pixmap uses top-left origin with Y-down.
    #[allow(dead_code)]
    #[inline]
    fn to_pixmap_coords(&self, x: f64, y: f64) -> (f32, f32) {
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;

        // Convert from centered coords with Y-up to top-left with Y-down
        let px = (x + half_width) as f32;
        let py = (half_height - y) as f32;

        (px, py)
    }

    /// Creates a transform for converting from manim coordinates to pixmap coordinates.
    fn create_transform(&self) -> tiny_skia::Transform {
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;

        // Translate to center and flip Y-axis
        tiny_skia::Transform::from_translate(half_width, half_height)
            .post_concat(tiny_skia::Transform::from_scale(1.0, -1.0))
    }
}

impl Renderer for RasterRenderer {
    fn begin_frame(&mut self) -> Result<()> {
        // No-op: pixmap is persistent
        Ok(())
    }

    fn end_frame(&mut self) -> Result<()> {
        // No-op: all drawing is immediate
        Ok(())
    }

    fn clear(&mut self, color: Color) -> Result<()> {
        let skia_color = color_to_skia_color(&color, 1.0);
        self.pixmap.fill(skia_color);
        Ok(())
    }

    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()> {
        let skia_path = path_to_tiny_skia(path)
            .ok_or_else(|| Error::Render("Failed to convert path".to_string()))?;

        let transform = self.create_transform();
        let fill_rule = fill_rule_to_skia(style.fill_rule);

        // Draw fill first
        if let Some(fill_paint) = path_style_to_fill_paint(style) {
            self.pixmap.fill_path(
                &skia_path,
                &fill_paint,
                fill_rule,
                transform,
                None, // No clip mask
            );
        }

        // Draw stroke on top
        if let (Some(stroke_paint), Some(stroke)) = (
            path_style_to_stroke_paint(style),
            path_style_to_stroke(style),
        ) {
            self.pixmap.stroke_path(
                &skia_path,
                &stroke_paint,
                &stroke,
                transform,
                None, // No clip mask
            );
        }

        Ok(())
    }

    fn draw_text(&mut self, text: &str, position: Vector2D, style: &TextStyle) -> Result<()> {
        // Basic text rendering is not well-supported in tiny-skia
        // For now, we'll just log a warning
        // In a production system, you'd want to:
        // 1. Use a font rasterization library like `fontdue` or `ab_glyph`
        // 2. Convert text to paths
        // 3. Render those paths
        eprintln!(
            "Warning: Text rendering not fully implemented in raster backend. Text: \"{}\"",
            text
        );
        eprintln!("  Position: ({}, {})", position.x, position.y);
        eprintln!("  Style: font-size={}px", style.font_size);

        // For now, just succeed without rendering
        Ok(())
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_renderer() {
        let renderer = RasterRenderer::new(1920, 1080);
        assert_eq!(renderer.dimensions(), (1920, 1080));
    }

    #[test]
    fn test_clear() {
        let mut renderer = RasterRenderer::new(100, 100);
        renderer.clear(Color::WHITE).unwrap();

        // Verify pixmap is filled with white
        let data = renderer.data();
        assert!(!data.is_empty());
        // First pixel should be white (255, 255, 255, 255)
        assert_eq!(data[0], 255); // R
        assert_eq!(data[1], 255); // G
        assert_eq!(data[2], 255); // B
        assert_eq!(data[3], 255); // A
    }

    #[test]
    fn test_draw_path() {
        let mut renderer = RasterRenderer::new(400, 300);

        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(50.0, 50.0))
            .line_to(Vector2D::new(0.0, 50.0))
            .close();

        let style = PathStyle::fill(Color::RED);

        renderer.clear(Color::WHITE).unwrap();
        renderer.draw_path(&path, &style).unwrap();

        // Just verify it doesn't crash
        assert_eq!(renderer.dimensions(), (400, 300));
    }

    #[test]
    fn test_begin_end_frame() {
        let mut renderer = RasterRenderer::new(800, 600);

        renderer.begin_frame().unwrap();
        renderer.clear(Color::BLACK).unwrap();
        renderer.end_frame().unwrap();

        assert_eq!(renderer.dimensions(), (800, 600));
    }

    #[test]
    fn test_to_pixmap_coords() {
        let renderer = RasterRenderer::new(800, 600);

        // Center should map to middle of pixmap
        let (px, py) = renderer.to_pixmap_coords(0.0, 0.0);
        assert_eq!(px, 400.0);
        assert_eq!(py, 300.0);

        // Top-right corner
        let (px, py) = renderer.to_pixmap_coords(400.0, 300.0);
        assert_eq!(px, 800.0);
        assert_eq!(py, 0.0);

        // Bottom-left corner
        let (px, py) = renderer.to_pixmap_coords(-400.0, -300.0);
        assert_eq!(px, 0.0);
        assert_eq!(py, 600.0);
    }
}
