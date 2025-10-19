//! Arrow mobject.
//!
//! Provides arrows as a composite of a line and an arrowhead tip.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, MobjectGroup};
use crate::renderer::Renderer;

use super::{Line, Polygon};

/// An arrow mobject.
///
/// An arrow consists of a line segment with an optional tip (arrowhead) at one or both ends.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
/// use manim_rs::mobject::geometry::Arrow;
///
/// let arrow = Arrow::new(Vector2D::ZERO, Vector2D::new(1.0, 0.0));
///
/// let arrow = Arrow::builder()
///     .start(Vector2D::new(-1.0, 0.0))
///     .end(Vector2D::new(1.0, 0.0))
///     .build();
/// ```
#[derive(Debug)]
pub struct Arrow {
    group: MobjectGroup,
    start: Vector2D,
    end: Vector2D,
    tip_length: f64,
    tip_width: f64,
}

impl Clone for Arrow {
    fn clone(&self) -> Self {
        Self {
            group: self.group.clone(),
            start: self.start,
            end: self.end,
            tip_length: self.tip_length,
            tip_width: self.tip_width,
        }
    }
}

impl Arrow {
    /// Creates a new arrow from start to end point with default tip.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::geometry::Arrow;
    ///
    /// let arrow = Arrow::new(Vector2D::ZERO, Vector2D::new(1.0, 0.0));
    /// ```
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        let tip_length = 0.35;
        let tip_width = 0.35;

        let mut group = MobjectGroup::new();
        Self::build_arrow(&mut group, start, end, tip_length, tip_width);

        Self {
            group,
            start,
            end,
            tip_length,
            tip_width,
        }
    }

    /// Returns a builder for constructing an arrow.
    pub fn builder() -> ArrowBuilder {
        ArrowBuilder::new()
    }

    /// Returns the start point of the arrow.
    pub fn start(&self) -> Vector2D {
        self.start
    }

    /// Returns the end point of the arrow.
    pub fn end(&self) -> Vector2D {
        self.end
    }

    /// Returns the length of the arrow tip.
    pub fn tip_length(&self) -> f64 {
        self.tip_length
    }

    /// Returns the width of the arrow tip.
    pub fn tip_width(&self) -> f64 {
        self.tip_width
    }

    /// Builds the arrow geometry by creating the line and tip.
    fn build_arrow(
        group: &mut MobjectGroup,
        start: Vector2D,
        end: Vector2D,
        tip_length: f64,
        tip_width: f64,
    ) {
        // Calculate direction and length
        let direction = end - start;
        let length = direction.magnitude();

        if length < tip_length {
            // If arrow is too short, just draw a line
            let line = Line::new(start, end);
            group.add(Box::new(line));
            return;
        }

        // Shorten the line to make room for the tip
        let line_end = start + direction * ((length - tip_length) / length);
        let line = Line::new(start, line_end);

        // Create the arrowhead tip
        let tip_base_center = end - direction * (tip_length / length);

        let perpendicular = Vector2D::new(-direction.y, direction.x).normalize().unwrap_or(Vector2D::new(0.0, 1.0));

        let tip_vertices = vec![
            end, // Point of the arrow
            tip_base_center + perpendicular * (tip_width / 2.0),
            tip_base_center - perpendicular * (tip_width / 2.0),
        ];

        let mut tip = Polygon::new(tip_vertices);
        tip.set_fill(Color::WHITE); // Default fill

        group.add(Box::new(line));
        group.add(Box::new(tip));
    }
}

impl Mobject for Arrow {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.group.render(renderer)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.group.bounding_box()
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.group.apply_transform(transform);
        self.start = transform.apply(self.start);
        self.end = transform.apply(self.end);
    }

    fn position(&self) -> Vector2D {
        self.group.position()
    }

    fn set_position(&mut self, pos: Vector2D) {
        self.group.set_position(pos);
    }

    fn opacity(&self) -> f64 {
        self.group.opacity()
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.group.set_opacity(opacity);
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

/// Builder for constructing arrows.
#[derive(Clone, Debug)]
pub struct ArrowBuilder {
    start: Vector2D,
    end: Vector2D,
    tip_length: f64,
    tip_width: f64,
    stroke_color: Option<Color>,
    stroke_width: f64,
    opacity: f64,
}

impl ArrowBuilder {
    pub fn new() -> Self {
        Self {
            start: Vector2D::new(-1.0, 0.0),
            end: Vector2D::new(1.0, 0.0),
            tip_length: 0.35,
            tip_width: 0.35,
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

    pub fn tip_length(mut self, length: f64) -> Self {
        self.tip_length = length;
        self
    }

    pub fn tip_width(mut self, width: f64) -> Self {
        self.tip_width = width;
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

    pub fn build(self) -> Arrow {
        let mut group = MobjectGroup::new();
        let direction = self.end - self.start;
        let length = direction.magnitude();

        let color = self.stroke_color.unwrap_or(Color::WHITE);

        if length >= self.tip_length {
            let line_end = self.start + direction * ((length - self.tip_length) / length);
            let mut line = Line::new(self.start, line_end);
            line.set_stroke(color, self.stroke_width);
            line.set_opacity(self.opacity);

            let tip_base_center = self.end - direction * (self.tip_length / length);
            let perpendicular = Vector2D::new(-direction.y, direction.x).normalize().unwrap_or(Vector2D::new(0.0, 1.0));

            let tip_vertices = vec![
                self.end,
                tip_base_center + perpendicular * (self.tip_width / 2.0),
                tip_base_center - perpendicular * (self.tip_width / 2.0),
            ];

            let mut tip = Polygon::new(tip_vertices);
            tip.set_fill(color);
            tip.set_opacity(self.opacity);

            group.add(Box::new(line));
            group.add(Box::new(tip));
        } else {
            let mut line = Line::new(self.start, self.end);
            line.set_stroke(color, self.stroke_width);
            line.set_opacity(self.opacity);
            group.add(Box::new(line));
        }

        Arrow {
            group,
            start: self.start,
            end: self.end,
            tip_length: self.tip_length,
            tip_width: self.tip_width,
        }
    }
}

impl Default for ArrowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_new() {
        let arrow = Arrow::new(Vector2D::ZERO, Vector2D::new(1.0, 0.0));
        assert_eq!(arrow.start(), Vector2D::ZERO);
        assert_eq!(arrow.end(), Vector2D::new(1.0, 0.0));
    }

    #[test]
    fn test_arrow_builder() {
        let arrow = Arrow::builder()
            .start(Vector2D::new(-1.0, -1.0))
            .end(Vector2D::new(1.0, 1.0))
            .stroke_color(Color::BLUE)
            .build();

        assert_eq!(arrow.start(), Vector2D::new(-1.0, -1.0));
        assert_eq!(arrow.end(), Vector2D::new(1.0, 1.0));
    }

    #[test]
    fn test_arrow_custom_tip_size() {
        let arrow = Arrow::builder()
            .start(Vector2D::ZERO)
            .end(Vector2D::new(2.0, 0.0))
            .tip_length(0.5)
            .tip_width(0.4)
            .build();

        assert_eq!(arrow.tip_length(), 0.5);
        assert_eq!(arrow.tip_width(), 0.4);
    }

    #[test]
    fn test_arrow_short() {
        // Very short arrow should still work
        let arrow = Arrow::new(Vector2D::ZERO, Vector2D::new(0.1, 0.0));
        assert_eq!(arrow.start(), Vector2D::ZERO);
    }
}

