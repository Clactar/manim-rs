//! Line mobject.
//!
//! Provides line segments with various styling options.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// A line segment mobject.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{Color, Vector2D};
/// use manim_rs::mobject::geometry::Line;
///
/// let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0));
///
/// let line = Line::builder()
///     .start(Vector2D::new(-1.0, 0.0))
///     .end(Vector2D::new(1.0, 0.0))
///     .stroke_color(Color::BLUE)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Line {
    vmobject: VMobject,
    start: Vector2D,
    end: Vector2D,
}

impl Line {
    /// Creates a new line from start to end point.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::geometry::Line;
    ///
    /// let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0));
    /// ```
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        let path = Self::create_line_path(start, end);
        Self {
            vmobject: VMobject::new(path),
            start,
            end,
        }
    }

    /// Returns a builder for constructing a line.
    pub fn builder() -> LineBuilder {
        LineBuilder::new()
    }

    /// Returns the start point of the line.
    pub fn start(&self) -> Vector2D {
        self.start
    }

    /// Returns the end point of the line.
    pub fn end(&self) -> Vector2D {
        self.end
    }

    /// Sets the start and end points of the line.
    pub fn set_points(&mut self, start: Vector2D, end: Vector2D) {
        self.start = start;
        self.end = end;
        let path = Self::create_line_path(start, end);
        *self.vmobject.path_mut() = path;
    }

    /// Returns the length of the line.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::geometry::Line;
    ///
    /// let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(3.0, 4.0));
    /// assert!((line.length() - 5.0).abs() < 1e-10);
    /// ```
    pub fn length(&self) -> f64 {
        (self.end - self.start).magnitude()
    }

    /// Returns the angle of the line in radians.
    ///
    /// The angle is measured from the positive x-axis.
    pub fn angle(&self) -> f64 {
        let delta = self.end - self.start;
        delta.y.atan2(delta.x)
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, width);
        self
    }

    /// Creates a line path from start to end.
    fn create_line_path(start: Vector2D, end: Vector2D) -> Path {
        let mut path = Path::new();
        path.move_to(start).line_to(end);
        path
    }
}

impl Mobject for Line {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.vmobject.render(renderer)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.vmobject.bounding_box()
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.vmobject.apply_transform(transform);
        self.start = transform.apply(self.start);
        self.end = transform.apply(self.end);
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

/// Builder for constructing lines.
#[derive(Clone, Debug)]
pub struct LineBuilder {
    start: Vector2D,
    end: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    opacity: f64,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self {
            start: Vector2D::new(-1.0, 0.0),
            end: Vector2D::new(1.0, 0.0),
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            opacity: 1.0,
        }
    }

    pub fn start(mut self, start: Vector2D) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: Vector2D) -> Self {
        self.end = end;
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

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> Line {
        let mut line = Line::new(self.start, self.end);

        if let Some(color) = self.stroke_color {
            line.set_stroke(color, self.stroke_width);
        } else {
            line.vmobject.clear_stroke();
        }

        line.set_opacity(self.opacity);

        line
    }
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_line_new() {
        let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0));
        assert_eq!(line.start(), Vector2D::new(0.0, 0.0));
        assert_eq!(line.end(), Vector2D::new(1.0, 1.0));
    }

    #[test]
    fn test_line_length() {
        let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(3.0, 4.0));
        assert_relative_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_line_angle() {
        let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 0.0));
        assert_relative_eq!(line.angle(), 0.0);

        let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 1.0));
        assert_relative_eq!(line.angle(), std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn test_line_set_points() {
        let mut line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 0.0));
        line.set_points(Vector2D::new(2.0, 2.0), Vector2D::new(5.0, 6.0));

        assert_eq!(line.start(), Vector2D::new(2.0, 2.0));
        assert_eq!(line.end(), Vector2D::new(5.0, 6.0));
    }

    #[test]
    fn test_line_builder() {
        let line = Line::builder()
            .start(Vector2D::new(-1.0, -1.0))
            .end(Vector2D::new(1.0, 1.0))
            .stroke_color(Color::BLUE)
            .stroke_width(3.0)
            .build();

        assert_eq!(line.start(), Vector2D::new(-1.0, -1.0));
        assert_eq!(line.end(), Vector2D::new(1.0, 1.0));
    }
}

