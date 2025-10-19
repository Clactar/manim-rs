//! BezierPath mobject.
//!
//! Provides a wrapper for arbitrary Bézier curves.

use crate::core::{BoundingBox, Color, CubicBezier, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// A mobject for arbitrary Bézier curve paths.
///
/// [`BezierPath`] provides a flexible way to create complex shapes from Bézier curves
/// or to import existing paths.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{CubicBezier, Vector2D};
/// use manim_rs::mobject::BezierPath;
/// use manim_rs::renderer::Path;
///
/// // From existing path
/// let mut path = Path::new();
/// path.move_to(Vector2D::ZERO);
/// path.line_to(Vector2D::new(1.0, 1.0));
/// let bezier = BezierPath::from_path(path);
///
/// // From Bézier curves
/// let curve = CubicBezier::new(
///     Vector2D::ZERO,
///     Vector2D::new(0.5, 1.0),
///     Vector2D::new(1.5, 1.0),
///     Vector2D::new(2.0, 0.0),
/// );
/// let bezier = BezierPath::from_bezier_curves(vec![curve]);
/// ```
#[derive(Clone, Debug)]
pub struct BezierPath {
    vmobject: VMobject,
}

impl BezierPath {
    /// Creates a BezierPath from an existing path.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::BezierPath;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::ZERO);
    /// path.line_to(Vector2D::new(1.0, 1.0));
    /// let bezier = BezierPath::from_path(path);
    /// ```
    pub fn from_path(path: Path) -> Self {
        Self {
            vmobject: VMobject::new(path),
        }
    }

    /// Creates a BezierPath from a vector of cubic Bézier curves.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{CubicBezier, Vector2D};
    /// use manim_rs::mobject::BezierPath;
    ///
    /// let curve = CubicBezier::new(
    ///     Vector2D::ZERO,
    ///     Vector2D::new(0.5, 1.0),
    ///     Vector2D::new(1.5, 1.0),
    ///     Vector2D::new(2.0, 0.0),
    /// );
    /// let bezier = BezierPath::from_bezier_curves(vec![curve]);
    /// ```
    pub fn from_bezier_curves(curves: Vec<CubicBezier>) -> Self {
        let mut path = Path::new();

        if let Some(first) = curves.first() {
            path.move_to(first.p0);

            for curve in curves {
                path.cubic_to(curve.p1, curve.p2, curve.p3);
            }
        }

        Self {
            vmobject: VMobject::new(path),
        }
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

    /// Returns a mutable reference to the underlying VMobject.
    pub fn vmobject_mut(&mut self) -> &mut VMobject {
        &mut self.vmobject
    }
}

impl Mobject for BezierPath {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_path_from_path() {
        let mut path = Path::new();
        path.move_to(Vector2D::ZERO);
        path.line_to(Vector2D::new(1.0, 1.0));

        let bezier = BezierPath::from_path(path);
        assert_eq!(bezier.opacity(), 1.0);
    }

    #[test]
    fn test_bezier_path_from_curves() {
        let curve = CubicBezier::new(
            Vector2D::ZERO,
            Vector2D::new(0.5, 1.0),
            Vector2D::new(1.5, 1.0),
            Vector2D::new(2.0, 0.0),
        );

        let bezier = BezierPath::from_bezier_curves(vec![curve]);
        assert_eq!(bezier.opacity(), 1.0);
    }

    #[test]
    fn test_bezier_path_empty_curves() {
        let bezier = BezierPath::from_bezier_curves(vec![]);
        assert_eq!(bezier.opacity(), 1.0);
    }

    #[test]
    fn test_bezier_path_set_stroke() {
        let mut path = Path::new();
        path.move_to(Vector2D::ZERO);
        let mut bezier = BezierPath::from_path(path);

        bezier.set_stroke(Color::RED, 3.0);
        assert_eq!(bezier.vmobject.stroke_color(), Some(Color::RED));
    }
}
