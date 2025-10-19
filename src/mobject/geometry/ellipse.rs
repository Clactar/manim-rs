//! Ellipse mobject.
//!
//! Implements an ellipse using 4 cubic Bézier curves, similar to Circle.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// Magic number for approximating a circle/ellipse with cubic Bézier curves.
const BEZIER_MAGIC: f64 = 0.551_915_024_493_510_6;

/// An ellipse mobject.
///
/// When width equals height, this becomes a circle.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::mobject::geometry::Ellipse;
///
/// let ellipse = Ellipse::new(3.0, 2.0);
///
/// let ellipse = Ellipse::builder()
///     .width(4.0)
///     .height(2.0)
///     .fill_color(Color::BLUE)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Ellipse {
    vmobject: VMobject,
    width: f64,
    height: f64,
}

impl Ellipse {
    /// Creates a new ellipse with the given width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Ellipse;
    ///
    /// let ellipse = Ellipse::new(4.0, 2.0);
    /// ```
    pub fn new(width: f64, height: f64) -> Self {
        let path = Self::create_ellipse_path(width, height);
        Self {
            vmobject: VMobject::new(path),
            width,
            height,
        }
    }

    /// Returns a builder for constructing an ellipse.
    pub fn builder() -> EllipseBuilder {
        EllipseBuilder::new()
    }

    /// Returns the width of the ellipse.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the height of the ellipse.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets the width and height of the ellipse.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
        let path = Self::create_ellipse_path(width, height);
        *self.vmobject.path_mut() = path;
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, stroke_width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, stroke_width);
        self
    }

    /// Sets the fill color.
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.vmobject.set_fill(color);
        self
    }

    /// Creates an ellipse path using 4 cubic Bézier curves.
    fn create_ellipse_path(width: f64, height: f64) -> Path {
        let mut path = Path::new();
        let rx = width / 2.0;
        let ry = height / 2.0;
        let magic_x = rx * BEZIER_MAGIC;
        let magic_y = ry * BEZIER_MAGIC;

        // Start at rightmost point
        path.move_to(Vector2D::new(rx, 0.0));

        // Top-right quadrant
        path.cubic_to(
            Vector2D::new(rx, magic_y),
            Vector2D::new(magic_x, ry),
            Vector2D::new(0.0, ry),
        );

        // Top-left quadrant
        path.cubic_to(
            Vector2D::new(-magic_x, ry),
            Vector2D::new(-rx, magic_y),
            Vector2D::new(-rx, 0.0),
        );

        // Bottom-left quadrant
        path.cubic_to(
            Vector2D::new(-rx, -magic_y),
            Vector2D::new(-magic_x, -ry),
            Vector2D::new(0.0, -ry),
        );

        // Bottom-right quadrant
        path.cubic_to(
            Vector2D::new(magic_x, -ry),
            Vector2D::new(rx, -magic_y),
            Vector2D::new(rx, 0.0),
        );

        path.close();
        path
    }
}

impl Mobject for Ellipse {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.vmobject.render(renderer)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.vmobject.bounding_box()
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.vmobject.apply_transform(transform);
    }

    fn position(&self) -> Vector2D {
        self.vmobject.position()
    }

    fn set_position(&mut self, pos: Vector2D) {
        self.vmobject.set_position(pos);
    }

    fn opacity(&self) -> f64 {
        self.vmobject.opacity()
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.vmobject.set_opacity(opacity);
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

/// Builder for constructing ellipses.
#[derive(Clone, Debug)]
pub struct EllipseBuilder {
    width: f64,
    height: f64,
    center: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
}

impl EllipseBuilder {
    pub fn new() -> Self {
        Self {
            width: 2.0,
            height: 1.0,
            center: Vector2D::ZERO,
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
        }
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn center(mut self, center: Vector2D) -> Self {
        self.center = center;
        self
    }

    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_color = None;
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> Ellipse {
        let mut ellipse = Ellipse::new(self.width, self.height);

        if let Some(color) = self.stroke_color {
            ellipse.set_stroke(color, self.stroke_width);
        } else {
            ellipse.vmobject.clear_stroke();
        }

        if let Some(color) = self.fill_color {
            ellipse.set_fill(color);
        }

        ellipse.set_opacity(self.opacity);

        if self.center != Vector2D::ZERO {
            ellipse.set_position(self.center);
        }

        ellipse
    }
}

impl Default for EllipseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipse_new() {
        let ellipse = Ellipse::new(4.0, 2.0);
        assert_eq!(ellipse.width(), 4.0);
        assert_eq!(ellipse.height(), 2.0);
    }

    #[test]
    fn test_ellipse_reduces_to_circle() {
        let ellipse = Ellipse::new(2.0, 2.0);
        assert_eq!(ellipse.width(), ellipse.height());
    }

    #[test]
    fn test_ellipse_set_size() {
        let mut ellipse = Ellipse::new(2.0, 1.0);
        ellipse.set_size(5.0, 3.0);
        assert_eq!(ellipse.width(), 5.0);
        assert_eq!(ellipse.height(), 3.0);
    }

    #[test]
    fn test_ellipse_builder() {
        let ellipse = Ellipse::builder()
            .width(6.0)
            .height(4.0)
            .fill_color(Color::BLUE)
            .build();

        assert_eq!(ellipse.width(), 6.0);
        assert_eq!(ellipse.height(), 4.0);
    }
}

