//! Circle mobject.
//!
//! Implements a circle using 4 cubic Bézier curves for accurate approximation.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// Magic number for approximating a circle with cubic Bézier curves.
///
/// This constant (≈0.5519150244935105707435627) represents the optimal control
/// point distance for approximating a quarter circle with a cubic Bézier curve.
/// Using 4 such curves produces a nearly perfect circle.
///
/// Source: http://spencermortensen.com/articles/bezier-circle/
const BEZIER_CIRCLE_MAGIC: f64 = 0.551_915_024_493_510_6;

/// A circle mobject.
///
/// [`Circle`] represents a perfect circle using 4 cubic Bézier curves.
/// It supports both full circles and arcs through optional start/end angles.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::mobject::geometry::Circle;
///
/// // Create a simple circle
/// let circle = Circle::new(2.0);
///
/// // Create with builder pattern
/// let circle = Circle::builder()
///     .radius(3.0)
///     .stroke_color(Color::BLUE)
///     .fill_color(Color::from_hex("#87CEEB").unwrap())
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Circle {
    vmobject: VMobject,
    radius: f64,
}

impl Circle {
    /// Creates a new circle with the given radius.
    ///
    /// The circle is centered at the origin with default styling
    /// (white stroke, no fill).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::new(2.0);
    /// assert_eq!(circle.radius(), 2.0);
    /// ```
    pub fn new(radius: f64) -> Self {
        let path = Self::create_circle_path(radius);
        Self {
            vmobject: VMobject::new(path),
            radius,
        }
    }

    /// Returns a builder for constructing a circle with custom properties.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .radius(3.0)
    ///     .stroke_color(Color::BLUE)
    ///     .stroke_width(2.5)
    ///     .build();
    /// ```
    pub fn builder() -> CircleBuilder {
        CircleBuilder::new()
    }

    /// Returns the radius of the circle.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::new(3.0);
    /// assert_eq!(circle.radius(), 3.0);
    /// ```
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Sets the radius of the circle.
    ///
    /// This regenerates the underlying path.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let mut circle = Circle::new(2.0);
    /// circle.set_radius(5.0);
    /// assert_eq!(circle.radius(), 5.0);
    /// ```
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        let path = Self::create_circle_path(radius);
        *self.vmobject.path_mut() = path;
    }

    /// Sets the stroke color and width.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let mut circle = Circle::new(2.0);
    /// circle.set_stroke(Color::BLUE, 3.0);
    /// ```
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, width);
        self
    }

    /// Sets the fill color.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let mut circle = Circle::new(2.0);
    /// circle.set_fill(Color::RED);
    /// ```
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.vmobject.set_fill(color);
        self
    }

    /// Creates a path representing a circle using 4 cubic Bézier curves.
    ///
    /// This is the standard technique for representing circles in vector graphics.
    /// Each quadrant is approximated by one cubic Bézier curve.
    fn create_circle_path(radius: f64) -> Path {
        let mut path = Path::new();
        let magic = radius * BEZIER_CIRCLE_MAGIC;

        // Start at rightmost point (3 o'clock position)
        path.move_to(Vector2D::new(radius, 0.0));

        // Top-right quadrant (3 o'clock → 12 o'clock)
        path.cubic_to(
            Vector2D::new(radius, magic),
            Vector2D::new(magic, radius),
            Vector2D::new(0.0, radius),
        );

        // Top-left quadrant (12 o'clock → 9 o'clock)
        path.cubic_to(
            Vector2D::new(-magic, radius),
            Vector2D::new(-radius, magic),
            Vector2D::new(-radius, 0.0),
        );

        // Bottom-left quadrant (9 o'clock → 6 o'clock)
        path.cubic_to(
            Vector2D::new(-radius, -magic),
            Vector2D::new(-magic, -radius),
            Vector2D::new(0.0, -radius),
        );

        // Bottom-right quadrant (6 o'clock → 3 o'clock)
        path.cubic_to(
            Vector2D::new(magic, -radius),
            Vector2D::new(radius, -magic),
            Vector2D::new(radius, 0.0),
        );

        path.close();
        path
    }
}

impl Mobject for Circle {
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

/// Builder for constructing circles with custom properties.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{Color, Vector2D};
/// use manim_rs::mobject::geometry::Circle;
///
/// let circle = Circle::builder()
///     .radius(5.0)
///     .center(Vector2D::new(1.0, 2.0))
///     .stroke_color(Color::BLUE)
///     .stroke_width(2.0)
///     .fill_color(Color::from_hex("#FFB6C1").unwrap())
///     .opacity(0.8)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct CircleBuilder {
    radius: f64,
    center: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
}

impl CircleBuilder {
    /// Creates a new circle builder with default values.
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            center: Vector2D::ZERO,
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
        }
    }

    /// Sets the radius of the circle.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .radius(5.0)
    ///     .build();
    /// ```
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the center position of the circle.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .center(Vector2D::new(3.0, 4.0))
    ///     .build();
    /// ```
    pub fn center(mut self, center: Vector2D) -> Self {
        self.center = center;
        self
    }

    /// Sets the stroke color.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .stroke_color(Color::BLUE)
    ///     .build();
    /// ```
    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Sets the stroke width.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .stroke_width(3.0)
    ///     .build();
    /// ```
    pub fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = width;
        self
    }

    /// Removes the stroke.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .no_stroke()
    ///     .fill_color(Color::RED)
    ///     .build();
    /// ```
    pub fn no_stroke(mut self) -> Self {
        self.stroke_color = None;
        self
    }

    /// Sets the fill color.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .fill_color(Color::RED)
    ///     .build();
    /// ```
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Sets the opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .opacity(0.5)
    ///     .build();
    /// ```
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    /// Builds the circle with the configured properties.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Circle;
    ///
    /// let circle = Circle::builder()
    ///     .radius(3.0)
    ///     .build();
    /// ```
    pub fn build(self) -> Circle {
        let mut circle = Circle::new(self.radius);

        // Apply stroke
        if let Some(color) = self.stroke_color {
            circle.set_stroke(color, self.stroke_width);
        } else {
            circle.vmobject.clear_stroke();
        }

        // Apply fill
        if let Some(color) = self.fill_color {
            circle.set_fill(color);
        }

        // Apply opacity
        circle.set_opacity(self.opacity);

        // Apply position
        if self.center != Vector2D::ZERO {
            circle.set_position(self.center);
        }

        circle
    }
}

impl Default for CircleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_circle_new() {
        let circle = Circle::new(2.0);
        assert_eq!(circle.radius(), 2.0);
    }

    #[test]
    fn test_circle_path_commands() {
        let circle = Circle::new(1.0);
        let path = circle.vmobject.path();

        // Should have: 1 MoveTo + 4 CubicTo + 1 Close = 6 commands
        assert_eq!(path.len(), 6);
    }

    #[test]
    fn test_circle_bounding_box() {
        let circle = Circle::new(2.0);
        let bbox = circle.bounding_box();

        // Bounding box should be slightly larger than 2*radius due to stroke
        assert!(bbox.width() >= 4.0);
        assert!(bbox.height() >= 4.0);
    }

    #[test]
    fn test_circle_set_radius() {
        let mut circle = Circle::new(2.0);
        circle.set_radius(5.0);
        assert_eq!(circle.radius(), 5.0);
    }

    #[test]
    fn test_circle_set_stroke() {
        let mut circle = Circle::new(2.0);
        circle.set_stroke(Color::BLUE, 3.0);

        assert_eq!(circle.vmobject.stroke_color(), Some(Color::BLUE));
        assert_eq!(circle.vmobject.stroke_width(), 3.0);
    }

    #[test]
    fn test_circle_set_fill() {
        let mut circle = Circle::new(2.0);
        circle.set_fill(Color::RED);

        assert_eq!(circle.vmobject.fill_color(), Some(Color::RED));
    }

    #[test]
    fn test_circle_position() {
        let mut circle = Circle::new(2.0);
        circle.set_position(Vector2D::new(3.0, 4.0));

        assert_eq!(circle.position(), Vector2D::new(3.0, 4.0));
    }

    #[test]
    fn test_circle_opacity() {
        let mut circle = Circle::new(2.0);
        circle.set_opacity(0.5);

        assert_relative_eq!(circle.opacity(), 0.5);
    }

    #[test]
    fn test_circle_clone() {
        let mut circle = Circle::new(2.0);
        circle.set_fill(Color::BLUE);

        let cloned = circle.clone();
        assert_eq!(cloned.radius(), 2.0);
        assert_eq!(cloned.vmobject.fill_color(), Some(Color::BLUE));
    }

    #[test]
    fn test_circle_builder_default() {
        let circle = Circle::builder().build();
        assert_eq!(circle.radius(), 1.0);
    }

    #[test]
    fn test_circle_builder_radius() {
        let circle = Circle::builder().radius(5.0).build();
        assert_eq!(circle.radius(), 5.0);
    }

    #[test]
    fn test_circle_builder_center() {
        let circle = Circle::builder().center(Vector2D::new(1.0, 2.0)).build();
        assert_eq!(circle.position(), Vector2D::new(1.0, 2.0));
    }

    #[test]
    fn test_circle_builder_stroke() {
        let circle = Circle::builder()
            .stroke_color(Color::BLUE)
            .stroke_width(3.0)
            .build();
        assert_eq!(circle.vmobject.stroke_color(), Some(Color::BLUE));
        assert_eq!(circle.vmobject.stroke_width(), 3.0);
    }

    #[test]
    fn test_circle_builder_no_stroke() {
        let circle = Circle::builder().no_stroke().fill_color(Color::RED).build();
        assert!(circle.vmobject.stroke_color().is_none());
        assert_eq!(circle.vmobject.fill_color(), Some(Color::RED));
    }

    #[test]
    fn test_circle_builder_fill() {
        let circle = Circle::builder().fill_color(Color::YELLOW).build();
        assert_eq!(circle.vmobject.fill_color(), Some(Color::YELLOW));
    }

    #[test]
    fn test_circle_builder_opacity() {
        let circle = Circle::builder().opacity(0.7).build();
        assert_relative_eq!(circle.opacity(), 0.7);
    }

    #[test]
    fn test_circle_builder_chaining() {
        let circle = Circle::builder()
            .radius(3.0)
            .center(Vector2D::new(1.0, 1.0))
            .stroke_color(Color::BLUE)
            .stroke_width(2.0)
            .fill_color(Color::from_hex("#87CEEB").unwrap())
            .opacity(0.8)
            .build();

        assert_eq!(circle.radius(), 3.0);
        assert_eq!(circle.position(), Vector2D::new(1.0, 1.0));
        assert_eq!(circle.vmobject.stroke_color(), Some(Color::BLUE));
        assert_relative_eq!(circle.opacity(), 0.8);
    }

    #[test]
    fn test_bezier_circle_magic_constant() {
        // Verify the magic constant is approximately correct
        // For a unit circle, control points should be at ±0.5519...
        assert_relative_eq!(BEZIER_CIRCLE_MAGIC, 0.551915, epsilon = 0.0001);
    }

    #[test]
    fn test_circle_transform() {
        let mut circle = Circle::new(2.0);
        let transform = Transform::translate(3.0, 4.0);
        circle.apply_transform(&transform);

        assert_eq!(circle.position(), Vector2D::new(3.0, 4.0));
    }
}
