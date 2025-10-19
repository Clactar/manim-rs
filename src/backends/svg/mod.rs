//! SVG rendering backend.
//!
//! This module provides an SVG-based implementation of the [`Renderer`] trait.
//! SVG output is ideal for vector graphics that can be scaled without quality loss
//! and easily embedded in web pages or edited in vector graphics software.
//!
//! # Examples
//!
//! ```
//! use manim_rs::backends::SvgRenderer;
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::renderer::{Path, PathStyle, Renderer};
//!
//! # fn main() -> manim_rs::core::Result<()> {
//! let mut renderer = SvgRenderer::new(1920, 1080);
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
//! // Save to file
//! renderer.save("output.svg")?;
//! # Ok(())
//! # }
//! ```
//!
//! # Performance
//!
//! - Zero-copy design: paths and styles are borrowed, not cloned
//! - Efficient string building with pre-allocated capacity
//! - Minimal overhead compared to direct SVG generation
//!
//! # Coordinate System
//!
//! The SVG renderer uses a centered coordinate system where (0, 0) is at the center
//! of the canvas, with positive Y pointing up (opposite to standard SVG coordinates).
//! This matches the mathematical convention used in Manim.

use std::fs;
use std::io::Write;

use crate::core::{Color, Result, Vector2D};
use crate::renderer::{Path, PathStyle, Renderer, TextStyle};

mod elements;
mod path_converter;
mod style_converter;

pub use path_converter::path_to_svg_d;
pub use style_converter::{color_to_svg, path_style_to_svg_attrs, text_style_to_svg_attrs};

use elements::SvgElement;
use style_converter::escape_xml;

/// SVG rendering backend.
///
/// Renders scenes to SVG (Scalable Vector Graphics) format. SVG is ideal for
/// mathematical visualizations as it maintains perfect clarity at any zoom level.
///
/// # Examples
///
/// ```
/// use manim_rs::backends::SvgRenderer;
/// use manim_rs::core::Color;
/// use manim_rs::renderer::Renderer;
///
/// let mut renderer = SvgRenderer::new(1920, 1080);
/// renderer.clear(Color::BLACK).unwrap();
///
/// // Render your scene...
///
/// let svg_string = renderer.to_svg_string();
/// assert!(svg_string.contains("<svg"));
/// ```
#[derive(Debug, Clone)]
pub struct SvgRenderer {
    width: u32,
    height: u32,
    background: Color,
    elements: Vec<SvgElement>,
}

impl SvgRenderer {
    /// Creates a new SVG renderer with the specified dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::backends::SvgRenderer;
    /// use manim_rs::renderer::Renderer;
    ///
    /// let renderer = SvgRenderer::new(1920, 1080);
    /// assert_eq!(renderer.dimensions(), (1920, 1080));
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            background: Color::BLACK,
            elements: Vec::new(),
        }
    }

    /// Converts the renderer's content to an SVG string.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::backends::SvgRenderer;
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::Renderer;
    ///
    /// let mut renderer = SvgRenderer::new(800, 600);
    /// renderer.clear(Color::WHITE).unwrap();
    ///
    /// let svg = renderer.to_svg_string();
    /// assert!(svg.contains("<svg"));
    /// assert!(svg.contains("width=\"800\""));
    /// assert!(svg.contains("height=\"600\""));
    /// ```
    pub fn to_svg_string(&self) -> String {
        let mut result = String::with_capacity(1024 + self.elements.len() * 128);

        // SVG header with centered coordinate system
        result.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        result.push_str("<svg ");
        result.push_str(&format!("width=\"{}\" ", self.width));
        result.push_str(&format!("height=\"{}\" ", self.height));

        // ViewBox: center coordinate system at (0, 0)
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;
        result.push_str(&format!(
            "viewBox=\"{} {} {} {}\" ",
            -half_width, -half_height, self.width, self.height
        ));

        result.push_str("xmlns=\"http://www.w3.org/2000/svg\" ");
        result.push_str("version=\"1.1\">\n");

        // Add a group for coordinate system transformation (flip Y axis)
        result.push_str("  <g transform=\"scale(1, -1)\">\n");

        // Render all elements
        for element in &self.elements {
            result.push_str(&element.to_svg_string(2));
            result.push('\n');
        }

        result.push_str("  </g>\n");
        result.push_str("</svg>\n");

        result
    }

    /// Saves the SVG to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use manim_rs::backends::SvgRenderer;
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::Renderer;
    ///
    /// # fn main() -> manim_rs::core::Result<()> {
    /// let mut renderer = SvgRenderer::new(1920, 1080);
    /// renderer.clear(Color::BLACK)?;
    ///
    /// renderer.save("output/scene.svg")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save(&self, path: &str) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        file.write_all(self.to_svg_string().as_bytes())?;
        Ok(())
    }
}

impl Renderer for SvgRenderer {
    fn begin_frame(&mut self) -> Result<()> {
        // Clear elements for new frame
        self.elements.clear();
        Ok(())
    }

    fn end_frame(&mut self) -> Result<()> {
        // No-op for SVG (everything is already stored)
        Ok(())
    }

    fn clear(&mut self, color: Color) -> Result<()> {
        self.background = color;

        // Add background rectangle
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;

        self.elements.push(SvgElement::Rect {
            x: -half_width,
            y: -half_height,
            width: self.width as f64,
            height: self.height as f64,
            fill: color_to_svg(&color),
        });

        Ok(())
    }

    fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()> {
        let d = path_to_svg_d(path);
        if d.is_empty() {
            return Ok(());
        }

        let svg_attrs = path_style_to_svg_attrs(style);

        // Convert to owned strings for storage
        let attrs: Vec<(String, String)> = svg_attrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        self.elements.push(SvgElement::Path { d, attrs });

        Ok(())
    }

    fn draw_text(&mut self, text: &str, position: Vector2D, style: &TextStyle) -> Result<()> {
        let content = escape_xml(text);
        let svg_attrs = text_style_to_svg_attrs(style);

        // Convert to owned strings for storage
        let attrs: Vec<(String, String)> = svg_attrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        self.elements.push(SvgElement::Text {
            content,
            position,
            attrs,
        });

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
        let renderer = SvgRenderer::new(1920, 1080);
        assert_eq!(renderer.dimensions(), (1920, 1080));
    }

    #[test]
    fn test_clear() {
        let mut renderer = SvgRenderer::new(800, 600);
        renderer.clear(Color::WHITE).unwrap();

        let svg = renderer.to_svg_string();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("width=\"800\""));
        assert!(svg.contains("height=\"600\""));
        assert!(svg.contains("#FFFFFF"));
    }

    #[test]
    fn test_begin_end_frame() {
        let mut renderer = SvgRenderer::new(400, 300);

        renderer.begin_frame().unwrap();
        renderer.clear(Color::BLACK).unwrap();
        renderer.end_frame().unwrap();

        let svg = renderer.to_svg_string();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_draw_path() {
        let mut renderer = SvgRenderer::new(800, 600);

        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(10.0, 10.0));

        let style = PathStyle::stroke(Color::BLUE, 2.0);

        renderer.begin_frame().unwrap();
        renderer.draw_path(&path, &style).unwrap();
        renderer.end_frame().unwrap();

        let svg = renderer.to_svg_string();
        assert!(svg.contains("<path"));
        assert!(svg.contains("M 0 0"));
        assert!(svg.contains("L 10 10"));
    }

    #[test]
    fn test_draw_text() {
        let mut renderer = SvgRenderer::new(1920, 1080);

        let style = TextStyle::new(Color::WHITE, 48.0);

        renderer.begin_frame().unwrap();
        renderer
            .draw_text("Test", Vector2D::new(0.0, 0.0), &style)
            .unwrap();
        renderer.end_frame().unwrap();

        let svg = renderer.to_svg_string();
        assert!(svg.contains("<text"));
        assert!(svg.contains("Test"));
    }

    #[test]
    fn test_multiple_elements() {
        let mut renderer = SvgRenderer::new(800, 600);

        let mut path1 = Path::new();
        path1
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let mut path2 = Path::new();
        path2
            .move_to(Vector2D::new(1.0, 0.0))
            .line_to(Vector2D::new(0.0, 1.0));

        renderer.begin_frame().unwrap();
        renderer.clear(Color::BLACK).unwrap();
        renderer.draw_path(&path1, &PathStyle::default()).unwrap();
        renderer.draw_path(&path2, &PathStyle::default()).unwrap();
        renderer.end_frame().unwrap();

        let svg = renderer.to_svg_string();
        let path_count = svg.matches("<path").count();
        assert!(path_count >= 2);
    }

    #[test]
    fn test_coordinate_system() {
        let renderer = SvgRenderer::new(800, 600);
        let svg = renderer.to_svg_string();

        // Should have viewBox centered at (0, 0)
        assert!(svg.contains("viewBox=\"-400 -300 800 600\""));

        // Should have Y-axis flip transformation
        assert!(svg.contains("scale(1, -1)"));
    }
}
