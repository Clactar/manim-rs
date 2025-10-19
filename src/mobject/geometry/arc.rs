//! Arc mobject.
//!
//! Provides circular arcs using Bézier curve approximation.

use std::f64::consts::PI;

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// Magic number for approximating a circle/arc with cubic Bézier curves.
/// (Currently unused - arc uses a different approximation method)
#[allow(dead_code)]
const BEZIER_MAGIC: f64 = 0.551_915_024_493_510_6;

/// An arc mobject.
///
/// Represents a portion of a circle defined by a radius and angle range.
///
/// # Examples
///
/// ```
/// use std::f64::consts::PI;
/// use manim_rs::mobject::geometry::Arc;
///
/// // Quarter circle
/// let arc = Arc::new(1.0, 0.0, PI / 2.0);
///
/// // Using builder
/// let arc = Arc::builder()
///     .radius(2.0)
///     .start_angle(0.0)
///     .end_angle(PI)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Arc {
    vmobject: VMobject,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl Arc {
    /// Creates a new arc with the given radius and angle range.
    ///
    /// Angles are in radians, measured counterclockwise from the positive x-axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use manim_rs::mobject::geometry::Arc;
    ///
    /// let arc = Arc::new(1.0, 0.0, PI / 2.0); // Quarter circle
    /// ```
    pub fn new(radius: f64, start_angle: f64, end_angle: f64) -> Self {
        let path = Self::create_arc_path(radius, start_angle, end_angle);
        Self {
            vmobject: VMobject::new(path),
            radius,
            start_angle,
            end_angle,
        }
    }

    /// Returns a builder for constructing an arc.
    pub fn builder() -> ArcBuilder {
        ArcBuilder::new()
    }

    /// Returns the radius of the arc.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the start angle of the arc in radians.
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Returns the end angle of the arc in radians.
    pub fn end_angle(&self) -> f64 {
        self.end_angle
    }

    /// Returns the angular extent of the arc in radians.
    pub fn angle(&self) -> f64 {
        self.end_angle - self.start_angle
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, width);
        self
    }

    /// Creates an arc path using cubic Bézier curves.
    ///
    /// The arc is approximated by dividing it into segments, each handled by a cubic Bézier.
    /// For angles up to π/2, one Bézier curve is sufficient. For larger angles, multiple curves are used.
    fn create_arc_path(radius: f64, start_angle: f64, end_angle: f64) -> Path {
        let mut path = Path::new();
        let mut angle = end_angle - start_angle;

        // Normalize angle to [0, 2π)
        while angle < 0.0 {
            angle += 2.0 * PI;
        }
        while angle >= 2.0 * PI {
            angle -= 2.0 * PI;
        }

        // Starting point
        let start_x = radius * start_angle.cos();
        let start_y = radius * start_angle.sin();
        path.move_to(Vector2D::new(start_x, start_y));

        // Divide arc into segments of at most π/2 each
        let num_segments = ((angle / (PI / 2.0)).ceil() as usize).max(1);
        let segment_angle = angle / num_segments as f64;

        for i in 0..num_segments {
            let seg_start = start_angle + i as f64 * segment_angle;
            let seg_end = seg_start + segment_angle;

            // Calculate control points for this segment
            let (cp1, cp2, end) =
                Self::bezier_arc_segment(radius, seg_start, seg_end, segment_angle);

            path.cubic_to(cp1, cp2, end);
        }

        path
    }

    /// Calculates control points for a single arc segment using Bézier approximation.
    fn bezier_arc_segment(
        radius: f64,
        start: f64,
        end: f64,
        angle: f64,
    ) -> (Vector2D, Vector2D, Vector2D) {
        // Calculate the control point offset
        let alpha = angle.sin() * (((1.0 + angle.cos()).sqrt() - 1.0) / 3.0).sqrt();

        let cos_start = start.cos();
        let sin_start = start.sin();
        let cos_end = end.cos();
        let sin_end = end.sin();

        let cp1 = Vector2D::new(
            radius * (cos_start - sin_start * alpha),
            radius * (sin_start + cos_start * alpha),
        );

        let cp2 = Vector2D::new(
            radius * (cos_end + sin_end * alpha),
            radius * (sin_end - cos_end * alpha),
        );

        let end_point = Vector2D::new(radius * cos_end, radius * sin_end);

        (cp1, cp2, end_point)
    }
}

impl Mobject for Arc {
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

/// Builder for constructing arcs.
#[derive(Clone, Debug)]
pub struct ArcBuilder {
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    center: Vector2D,
    stroke_color: Option<Color>,
    stroke_width: f64,
    opacity: f64,
}

impl ArcBuilder {
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            start_angle: 0.0,
            end_angle: PI,
            center: Vector2D::ZERO,
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            opacity: 1.0,
        }
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
        self
    }

    pub fn end_angle(mut self, angle: f64) -> Self {
        self.end_angle = angle;
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

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> Arc {
        let mut arc = Arc::new(self.radius, self.start_angle, self.end_angle);

        if let Some(color) = self.stroke_color {
            arc.set_stroke(color, self.stroke_width);
        } else {
            arc.vmobject.clear_stroke();
        }

        arc.set_opacity(self.opacity);

        if self.center != Vector2D::ZERO {
            arc.set_position(self.center);
        }

        arc
    }
}

impl Default for ArcBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_arc_new() {
        let arc = Arc::new(1.0, 0.0, PI / 2.0);
        assert_eq!(arc.radius(), 1.0);
        assert_eq!(arc.start_angle(), 0.0);
        assert_relative_eq!(arc.end_angle(), PI / 2.0);
    }

    #[test]
    fn test_arc_angle() {
        let arc = Arc::new(1.0, 0.0, PI / 2.0);
        assert_relative_eq!(arc.angle(), PI / 2.0);

        let arc = Arc::new(1.0, PI / 4.0, 3.0 * PI / 4.0);
        assert_relative_eq!(arc.angle(), PI / 2.0);
    }

    #[test]
    fn test_arc_full_circle() {
        let arc = Arc::new(1.0, 0.0, 2.0 * PI);
        assert_relative_eq!(arc.angle(), 2.0 * PI);
    }

    #[test]
    fn test_arc_builder() {
        let arc = Arc::builder()
            .radius(2.0)
            .start_angle(0.0)
            .end_angle(PI)
            .stroke_color(Color::BLUE)
            .build();

        assert_eq!(arc.radius(), 2.0);
        assert_relative_eq!(arc.end_angle(), PI);
    }

    #[test]
    fn test_arc_quarter_circle() {
        let arc = Arc::new(1.0, 0.0, PI / 2.0);
        assert_relative_eq!(arc.angle(), PI / 2.0);
    }

    #[test]
    fn test_arc_three_quarters() {
        let arc = Arc::new(1.0, 0.0, 3.0 * PI / 2.0);
        assert_relative_eq!(arc.angle(), 3.0 * PI / 2.0);
    }
}
