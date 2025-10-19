//! Rectangle and Square mobjects.
//!
//! Provides rectangular shapes with optional rounded corners.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// A rectangle mobject.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::mobject::geometry::Rectangle;
///
/// let rect = Rectangle::new(4.0, 3.0);
///
/// let rect = Rectangle::builder()
///     .width(5.0)
///     .height(3.0)
///     .stroke_color(Color::BLUE)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Rectangle {
    vmobject: VMobject,
    width: f64,
    height: f64,
}

impl Rectangle {
    /// Creates a new rectangle with the given width and height.
    ///
    /// The rectangle is centered at the origin.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Rectangle;
    ///
    /// let rect = Rectangle::new(4.0, 3.0);
    /// assert_eq!(rect.width(), 4.0);
    /// assert_eq!(rect.height(), 3.0);
    /// ```
    pub fn new(width: f64, height: f64) -> Self {
        let path = Self::create_rectangle_path(width, height);
        Self {
            vmobject: VMobject::new(path),
            width,
            height,
        }
    }

    /// Returns a builder for constructing a rectangle.
    pub fn builder() -> RectangleBuilder {
        RectangleBuilder::new()
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets the width and height of the rectangle.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
        let path = Self::create_rectangle_path(width, height);
        *self.vmobject.path_mut() = path;
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, width);
        self
    }

    /// Sets the fill color.
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.vmobject.set_fill(color);
        self
    }

    /// Creates a rectangular path.
    fn create_rectangle_path(width: f64, height: f64) -> Path {
        let mut path = Path::new();
        let half_w = width / 2.0;
        let half_h = height / 2.0;

        path.move_to(Vector2D::new(-half_w, -half_h))
            .line_to(Vector2D::new(half_w, -half_h))
            .line_to(Vector2D::new(half_w, half_h))
            .line_to(Vector2D::new(-half_w, half_h))
            .close();

        path
    }
}

impl Mobject for Rectangle {
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

/// Builder for constructing rectangles.
#[derive(Clone, Debug)]
pub struct RectangleBuilder {
    width: f64,
    height: f64,
    center: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
}

impl RectangleBuilder {
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

    pub fn build(self) -> Rectangle {
        let mut rect = Rectangle::new(self.width, self.height);

        if let Some(color) = self.stroke_color {
            rect.set_stroke(color, self.stroke_width);
        } else {
            rect.vmobject.clear_stroke();
        }

        if let Some(color) = self.fill_color {
            rect.set_fill(color);
        }

        rect.set_opacity(self.opacity);

        if self.center != Vector2D::ZERO {
            rect.set_position(self.center);
        }

        rect
    }
}

impl Default for RectangleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A square mobject.
///
/// Internally implemented as a [`Rectangle`] with equal width and height.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::mobject::geometry::Square;
///
/// let square = Square::new(3.0);
///
/// let square = Square::builder()
///     .side_length(4.0)
///     .fill_color(Color::RED)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Square {
    rectangle: Rectangle,
}

impl Square {
    /// Creates a new square with the given side length.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Square;
    ///
    /// let square = Square::new(3.0);
    /// assert_eq!(square.side_length(), 3.0);
    /// ```
    pub fn new(side_length: f64) -> Self {
        Self {
            rectangle: Rectangle::new(side_length, side_length),
        }
    }

    /// Returns a builder for constructing a square.
    pub fn builder() -> SquareBuilder {
        SquareBuilder::new()
    }

    /// Returns the side length of the square.
    pub fn side_length(&self) -> f64 {
        self.rectangle.width()
    }

    /// Sets the side length of the square.
    pub fn set_side_length(&mut self, side_length: f64) {
        self.rectangle.set_size(side_length, side_length);
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.rectangle.set_stroke(color, width);
        self
    }

    /// Sets the fill color.
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.rectangle.set_fill(color);
        self
    }
}

impl Mobject for Square {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.rectangle.render(renderer)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.rectangle.bounding_box()
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.rectangle.apply_transform(transform);
    }

    fn position(&self) -> Vector2D {
        self.rectangle.position()
    }

    fn set_position(&mut self, pos: Vector2D) {
        self.rectangle.set_position(pos);
    }

    fn opacity(&self) -> f64 {
        self.rectangle.opacity()
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.rectangle.set_opacity(opacity);
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

/// Builder for constructing squares.
#[derive(Clone, Debug)]
pub struct SquareBuilder {
    side_length: f64,
    center: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
}

impl SquareBuilder {
    pub fn new() -> Self {
        Self {
            side_length: 2.0,
            center: Vector2D::ZERO,
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
        }
    }

    pub fn side_length(mut self, side_length: f64) -> Self {
        self.side_length = side_length;
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

    pub fn build(self) -> Square {
        let mut square = Square::new(self.side_length);

        if let Some(color) = self.stroke_color {
            square.set_stroke(color, self.stroke_width);
        } else {
            square.rectangle.vmobject.clear_stroke();
        }

        if let Some(color) = self.fill_color {
            square.set_fill(color);
        }

        square.set_opacity(self.opacity);

        if self.center != Vector2D::ZERO {
            square.set_position(self.center);
        }

        square
    }
}

impl Default for SquareBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(4.0, 3.0);
        assert_eq!(rect.width(), 4.0);
        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_rectangle_path_commands() {
        let rect = Rectangle::new(2.0, 1.0);
        let path = rect.vmobject.path();
        // MoveTo + 4 LineTo + Close = 6 commands
        assert_eq!(path.len(), 5);
    }

    #[test]
    fn test_rectangle_set_size() {
        let mut rect = Rectangle::new(2.0, 1.0);
        rect.set_size(5.0, 4.0);
        assert_eq!(rect.width(), 5.0);
        assert_eq!(rect.height(), 4.0);
    }

    #[test]
    fn test_rectangle_builder() {
        let rect = Rectangle::builder()
            .width(5.0)
            .height(3.0)
            .stroke_color(Color::BLUE)
            .fill_color(Color::RED)
            .build();

        assert_eq!(rect.width(), 5.0);
        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_square_new() {
        let square = Square::new(3.0);
        assert_eq!(square.side_length(), 3.0);
    }

    #[test]
    fn test_square_is_square() {
        let square = Square::new(2.0);
        assert_eq!(square.rectangle.width(), square.rectangle.height());
    }

    #[test]
    fn test_square_set_side_length() {
        let mut square = Square::new(2.0);
        square.set_side_length(5.0);
        assert_eq!(square.side_length(), 5.0);
    }

    #[test]
    fn test_square_builder() {
        let square = Square::builder()
            .side_length(4.0)
            .fill_color(Color::GREEN)
            .build();

        assert_eq!(square.side_length(), 4.0);
    }
}

