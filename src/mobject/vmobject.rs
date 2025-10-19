//! Vector-based mobject implementation.
//!
//! [`VMobject`] is the foundational implementation for most geometric shapes in
//! manim-rs. It wraps a [`Path`] and styling information, providing the core
//! implementation of the [`Mobject`] trait for path-based objects.

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::Mobject;
use crate::renderer::{Path, PathStyle, Renderer};

/// A mobject based on vector paths.
///
/// [`VMobject`] is the building block for most geometric shapes. It combines a
/// [`Path`] with styling properties (stroke, fill, opacity) and position tracking.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{Color, Vector2D};
/// use manim_rs::mobject::VMobject;
/// use manim_rs::renderer::Path;
///
/// // Create a simple square
/// let mut path = Path::new();
/// path.move_to(Vector2D::new(-1.0, -1.0))
///     .line_to(Vector2D::new(1.0, -1.0))
///     .line_to(Vector2D::new(1.0, 1.0))
///     .line_to(Vector2D::new(-1.0, 1.0))
///     .close();
///
/// let mut vmobject = VMobject::new(path);
/// vmobject.set_stroke(Color::BLUE, 2.0)
///         .set_fill(Color::RED);
/// ```
#[derive(Clone, Debug)]
pub struct VMobject {
    path: Path,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
    position: Vector2D,
}

impl VMobject {
    /// Creates a new VMobject from a path.
    ///
    /// The VMobject starts with default styling: white stroke, no fill, full opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{Mobject, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let vmobject = VMobject::new(Path::new());
    /// assert_eq!(vmobject.opacity(), 1.0);
    /// ```
    pub fn new(path: Path) -> Self {
        Self {
            path,
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
            position: Vector2D::ZERO,
        }
    }

    /// Creates a VMobject from a list of points connected by lines.
    ///
    /// This is a convenience method for creating simple polylines. The path
    /// is automatically closed if the first and last points differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::VMobject;
    ///
    /// let points = vec![
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    /// ];
    /// let vmobject = VMobject::from_points(&points);
    /// ```
    pub fn from_points(points: &[Vector2D]) -> Self {
        let mut path = Path::new();
        if let Some(first) = points.first() {
            path.move_to(*first);
            for point in points.iter().skip(1) {
                path.line_to(*point);
            }
        }
        Self::new(path)
    }

    /// Sets the stroke color and width.
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut vmobject = VMobject::new(Path::new());
    /// vmobject.set_stroke(Color::BLUE, 3.0)
    ///         .set_fill(Color::RED);
    /// ```
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    /// Removes the stroke.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut vmobject = VMobject::new(Path::new());
    /// vmobject.clear_stroke();
    /// ```
    pub fn clear_stroke(&mut self) -> &mut Self {
        self.stroke_color = None;
        self
    }

    /// Sets the fill color.
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut vmobject = VMobject::new(Path::new());
    /// vmobject.set_fill(Color::from_hex("#FF5733").unwrap());
    /// ```
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.fill_color = Some(color);
        self
    }

    /// Removes the fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut vmobject = VMobject::new(Path::new());
    /// vmobject.clear_fill();
    /// ```
    pub fn clear_fill(&mut self) -> &mut Self {
        self.fill_color = None;
        self
    }

    /// Returns a reference to the underlying path.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let vmobject = VMobject::new(Path::new());
    /// let path = vmobject.path();
    /// assert!(path.is_empty());
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a mutable reference to the underlying path.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::VMobject;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut vmobject = VMobject::new(Path::new());
    /// vmobject.path_mut().move_to(Vector2D::new(1.0, 2.0));
    /// ```
    pub fn path_mut(&mut self) -> &mut Path {
        &mut self.path
    }

    /// Returns the stroke color, if any.
    pub fn stroke_color(&self) -> Option<Color> {
        self.stroke_color
    }

    /// Returns the stroke width.
    pub fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    /// Returns the fill color, if any.
    pub fn fill_color(&self) -> Option<Color> {
        self.fill_color
    }
}

impl Mobject for VMobject {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let style = PathStyle {
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            fill_color: self.fill_color,
            fill_rule: crate::renderer::PathFillRule::NonZero,
            opacity: self.opacity,
        };
        renderer.draw_path(&self.path, &style)
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut bbox = self.path.bounding_box();
        // Expand by stroke width to account for strokes extending beyond path
        if self.stroke_color.is_some() && self.stroke_width > 0.0 {
            bbox = bbox.expand_by_margin(self.stroke_width / 2.0);
        }
        bbox
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.path.apply_transform(transform);
        self.position = transform.apply(self.position);
    }

    fn position(&self) -> Vector2D {
        self.position
    }

    fn set_position(&mut self, pos: Vector2D) {
        let delta = pos - self.position;
        let translation = Transform::translate(delta.x, delta.y);
        self.path.apply_transform(&translation);
        self.position = pos;
    }

    fn opacity(&self) -> f64 {
        self.opacity
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::{PathStyle, TextStyle};
    use approx::assert_relative_eq;

    struct TestRenderer {
        last_path: Option<Path>,
        last_style: Option<PathStyle>,
    }

    impl TestRenderer {
        fn new() -> Self {
            Self {
                last_path: None,
                last_style: None,
            }
        }
    }

    impl Renderer for TestRenderer {
        fn clear(&mut self, _color: Color) -> Result<()> {
            Ok(())
        }

        fn draw_path(&mut self, path: &Path, style: &PathStyle) -> Result<()> {
            self.last_path = Some(path.clone());
            self.last_style = Some(style.clone());
            Ok(())
        }

        fn draw_text(
            &mut self,
            _text: &str,
            _position: Vector2D,
            _style: &TextStyle,
        ) -> Result<()> {
            Ok(())
        }

        fn dimensions(&self) -> (u32, u32) {
            (800, 600)
        }
    }

    #[test]
    fn test_vmobject_new() {
        let path = Path::new();
        let vmobject = VMobject::new(path);

        assert_eq!(vmobject.position(), Vector2D::ZERO);
        assert_eq!(vmobject.opacity(), 1.0);
        assert!(vmobject.stroke_color().is_some());
        assert!(vmobject.fill_color().is_none());
        assert_eq!(vmobject.stroke_width(), 2.0);
    }

    #[test]
    fn test_vmobject_from_points() {
        let points = vec![
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 0.0),
            Vector2D::new(1.0, 1.0),
        ];
        let vmobject = VMobject::from_points(&points);

        assert_eq!(vmobject.path().len(), 3); // MoveTo + 2 LineTo
    }

    #[test]
    fn test_vmobject_from_points_empty() {
        let points: Vec<Vector2D> = vec![];
        let vmobject = VMobject::from_points(&points);

        assert!(vmobject.path().is_empty());
    }

    #[test]
    fn test_vmobject_set_stroke() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_stroke(Color::BLUE, 3.0);

        assert_eq!(vmobject.stroke_color(), Some(Color::BLUE));
        assert_eq!(vmobject.stroke_width(), 3.0);
    }

    #[test]
    fn test_vmobject_clear_stroke() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.clear_stroke();

        assert!(vmobject.stroke_color().is_none());
    }

    #[test]
    fn test_vmobject_set_fill() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_fill(Color::RED);

        assert_eq!(vmobject.fill_color(), Some(Color::RED));
    }

    #[test]
    fn test_vmobject_clear_fill() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_fill(Color::RED);
        vmobject.clear_fill();

        assert!(vmobject.fill_color().is_none());
    }

    #[test]
    fn test_vmobject_method_chaining() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject
            .set_stroke(Color::BLUE, 2.0)
            .set_fill(Color::RED)
            .set_opacity(0.5);

        assert_eq!(vmobject.stroke_color(), Some(Color::BLUE));
        assert_eq!(vmobject.fill_color(), Some(Color::RED));
        assert_eq!(vmobject.opacity(), 0.5);
    }

    #[test]
    fn test_vmobject_render_stroke_only() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_stroke(Color::BLUE, 2.0).clear_fill();

        let mut renderer = TestRenderer::new();
        vmobject.render(&mut renderer).unwrap();

        let style = renderer.last_style.unwrap();
        assert_eq!(style.stroke_color, Some(Color::BLUE));
        assert_eq!(style.stroke_width, 2.0);
        assert!(style.fill_color.is_none());
    }

    #[test]
    fn test_vmobject_render_fill_only() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.clear_stroke().set_fill(Color::RED);

        let mut renderer = TestRenderer::new();
        vmobject.render(&mut renderer).unwrap();

        let style = renderer.last_style.unwrap();
        assert!(style.stroke_color.is_none());
        assert_eq!(style.fill_color, Some(Color::RED));
    }

    #[test]
    fn test_vmobject_render_both_stroke_and_fill() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject
            .set_stroke(Color::BLACK, 1.0)
            .set_fill(Color::YELLOW);

        let mut renderer = TestRenderer::new();
        vmobject.render(&mut renderer).unwrap();

        let style = renderer.last_style.unwrap();
        assert_eq!(style.stroke_color, Some(Color::BLACK));
        assert_eq!(style.fill_color, Some(Color::YELLOW));
    }

    #[test]
    fn test_vmobject_opacity() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_opacity(0.5);

        let mut renderer = TestRenderer::new();
        vmobject.render(&mut renderer).unwrap();

        let style = renderer.last_style.unwrap();
        assert_relative_eq!(style.opacity, 0.5);
    }

    #[test]
    fn test_vmobject_opacity_clamping() {
        let mut vmobject = VMobject::new(Path::new());

        vmobject.set_opacity(1.5);
        assert_eq!(vmobject.opacity(), 1.0);

        vmobject.set_opacity(-0.5);
        assert_eq!(vmobject.opacity(), 0.0);
    }

    #[test]
    fn test_vmobject_bounding_box() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(2.0, 3.0));

        let vmobject = VMobject::new(path);
        let bbox = vmobject.bounding_box();

        // Should include stroke expansion
        assert!(bbox.width() >= 2.0);
        assert!(bbox.height() >= 3.0);
    }

    #[test]
    fn test_vmobject_transform() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 0.0));

        let mut vmobject = VMobject::new(path);
        let transform = Transform::translate(2.0, 3.0);
        vmobject.apply_transform(&transform);

        assert_eq!(vmobject.position(), Vector2D::new(2.0, 3.0));
    }

    #[test]
    fn test_vmobject_position() {
        let mut vmobject = VMobject::new(Path::new());
        assert_eq!(vmobject.position(), Vector2D::ZERO);

        vmobject.set_position(Vector2D::new(5.0, 7.0));
        assert_eq!(vmobject.position(), Vector2D::new(5.0, 7.0));
    }

    #[test]
    fn test_vmobject_clone() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject
            .set_stroke(Color::BLUE, 3.0)
            .set_fill(Color::RED)
            .set_opacity(0.8);

        let cloned = vmobject.clone();

        assert_eq!(cloned.stroke_color(), vmobject.stroke_color());
        assert_eq!(cloned.fill_color(), vmobject.fill_color());
        assert_eq!(cloned.opacity(), vmobject.opacity());
    }

    #[test]
    fn test_vmobject_clone_mobject() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_position(Vector2D::new(1.0, 2.0));

        let boxed: Box<dyn Mobject> = Box::new(vmobject);
        let cloned = boxed.clone_mobject();

        assert_eq!(cloned.position(), Vector2D::new(1.0, 2.0));
    }

    #[test]
    fn test_vmobject_path_access() {
        let mut vmobject = VMobject::new(Path::new());
        vmobject.path_mut().move_to(Vector2D::new(1.0, 2.0));

        let path = vmobject.path();
        assert_eq!(path.len(), 1);
    }
}
