//! Path representation for 2D vector graphics.
//!
//! This module provides [`Path`] and [`PathCommand`] for building and manipulating
//! 2D vector paths. Paths are optimized for typical shapes using [`smallvec`] to
//! avoid heap allocation for common cases.
//!
//! # Performance
//!
//! - Small paths (≤16 commands) are stack-allocated via `SmallVec`
//! - Bounding boxes are cached and only recomputed when the path is modified
//! - Hot-path methods are inlined for optimal performance
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::Vector2D;
//! use manim_rs::renderer::Path;
//!
//! let mut path = Path::new();
//! path.move_to(Vector2D::new(0.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 1.0))
//!     .line_to(Vector2D::new(0.0, 1.0))
//!     .close();
//!
//! let bounds = path.bounding_box();
//! assert_eq!(bounds.width(), 1.0);
//! assert_eq!(bounds.height(), 1.0);
//! ```

use smallvec::SmallVec;

use crate::core::{BoundingBox, Transform, Vector2D};

/// A command in a 2D vector path.
///
/// Paths are composed of sequences of these commands. Each command either moves
/// the pen position or draws a line/curve to a new position.
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Move the pen to a position without drawing.
    MoveTo(Vector2D),

    /// Draw a straight line to a position.
    LineTo(Vector2D),

    /// Draw a quadratic Bézier curve.
    QuadraticTo {
        /// Control point
        control: Vector2D,
        /// End point
        to: Vector2D,
    },

    /// Draw a cubic Bézier curve.
    CubicTo {
        /// First control point
        control1: Vector2D,
        /// Second control point
        control2: Vector2D,
        /// End point
        to: Vector2D,
    },

    /// Close the current subpath by drawing a line to the start.
    Close,
}

/// Internal storage optimized for typical shapes.
///
/// Circles use ~13 commands (4 cubic beziers), so we use 16 as the inline size.
/// This means circles, squares, triangles, and most simple shapes are stack-allocated.
type PathCommands = SmallVec<[PathCommand; 16]>;

/// A 2D vector path composed of drawing commands.
///
/// Paths are built using a fluent API with methods like [`move_to`](Path::move_to),
/// [`line_to`](Path::line_to), etc. Bounding boxes are cached for performance.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
/// use manim_rs::renderer::Path;
///
/// // Build a triangle
/// let mut path = Path::new();
/// path.move_to(Vector2D::new(0.0, 0.0))
///     .line_to(Vector2D::new(1.0, 0.0))
///     .line_to(Vector2D::new(0.5, 1.0))
///     .close();
/// ```
#[derive(Debug, Clone)]
pub struct Path {
    commands: PathCommands,
    cached_bounds: Option<BoundingBox>,
}

impl Path {
    /// Creates a new empty path.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::renderer::Path;
    ///
    /// let path = Path::new();
    /// assert!(path.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            commands: SmallVec::new(),
            cached_bounds: None,
        }
    }

    /// Creates a new path with preallocated capacity.
    ///
    /// Use this when you know the approximate number of commands to avoid
    /// reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::renderer::Path;
    ///
    /// let path = Path::with_capacity(100);
    /// assert_eq!(path.len(), 0);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: SmallVec::with_capacity(capacity),
            cached_bounds: None,
        }
    }

    /// Returns the number of commands in the path.
    #[inline]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns `true` if the path contains no commands.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Returns an immutable slice of the path's commands.
    #[inline]
    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }

    /// Moves the pen to a position without drawing.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(1.0, 2.0));
    /// ```
    #[inline]
    pub fn move_to(&mut self, point: Vector2D) -> &mut Self {
        self.commands.push(PathCommand::MoveTo(point));
        self.cached_bounds = None;
        self
    }

    /// Draws a straight line to a position.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .line_to(Vector2D::new(1.0, 1.0));
    /// ```
    #[inline]
    pub fn line_to(&mut self, point: Vector2D) -> &mut Self {
        self.commands.push(PathCommand::LineTo(point));
        self.cached_bounds = None;
        self
    }

    /// Draws a quadratic Bézier curve.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .quadratic_to(Vector2D::new(0.5, 1.0), Vector2D::new(1.0, 0.0));
    /// ```
    #[inline]
    pub fn quadratic_to(&mut self, control: Vector2D, to: Vector2D) -> &mut Self {
        self.commands.push(PathCommand::QuadraticTo { control, to });
        self.cached_bounds = None;
        self
    }

    /// Draws a cubic Bézier curve.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .cubic_to(
    ///         Vector2D::new(0.33, 1.0),
    ///         Vector2D::new(0.66, 1.0),
    ///         Vector2D::new(1.0, 0.0)
    ///     );
    /// ```
    #[inline]
    pub fn cubic_to(&mut self, control1: Vector2D, control2: Vector2D, to: Vector2D) -> &mut Self {
        self.commands.push(PathCommand::CubicTo {
            control1,
            control2,
            to,
        });
        self.cached_bounds = None;
        self
    }

    /// Closes the current subpath.
    ///
    /// This draws a straight line back to the most recent `MoveTo` command.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .line_to(Vector2D::new(1.0, 0.0))
    ///     .line_to(Vector2D::new(0.5, 1.0))
    ///     .close(); // Completes the triangle
    /// ```
    #[inline]
    pub fn close(&mut self) -> &mut Self {
        self.commands.push(PathCommand::Close);
        self
    }

    /// Returns the bounding box of the path.
    ///
    /// The bounding box is cached, so repeated calls are cheap. The cache is
    /// invalidated whenever the path is modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .line_to(Vector2D::new(2.0, 3.0));
    ///
    /// let bounds = path.bounding_box();
    /// assert_eq!(bounds.width(), 2.0);
    /// assert_eq!(bounds.height(), 3.0);
    /// ```
    pub fn bounding_box(&self) -> BoundingBox {
        if let Some(bounds) = self.cached_bounds {
            return bounds;
        }

        // Collect all points from commands
        let mut points = Vec::new();
        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(p) | PathCommand::LineTo(p) => {
                    points.push(*p);
                }
                PathCommand::QuadraticTo { control, to } => {
                    points.push(*control);
                    points.push(*to);
                }
                PathCommand::CubicTo {
                    control1,
                    control2,
                    to,
                } => {
                    points.push(*control1);
                    points.push(*control2);
                    points.push(*to);
                }
                PathCommand::Close => {}
            }
        }

        if points.is_empty() {
            BoundingBox::zero()
        } else {
            BoundingBox::from_points(points).unwrap_or_else(BoundingBox::zero)
        }
    }

    /// Applies a transformation to all points in the path.
    ///
    /// This modifies the path in-place and invalidates the cached bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{Transform, Vector2D};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut path = Path::new();
    /// path.move_to(Vector2D::new(0.0, 0.0))
    ///     .line_to(Vector2D::new(1.0, 0.0));
    ///
    /// let transform = Transform::translate(2.0, 3.0);
    /// path.apply_transform(&transform);
    /// ```
    pub fn apply_transform(&mut self, transform: &Transform) {
        for cmd in &mut self.commands {
            match cmd {
                PathCommand::MoveTo(p) | PathCommand::LineTo(p) => {
                    *p = transform.apply(*p);
                }
                PathCommand::QuadraticTo { control, to } => {
                    *control = transform.apply(*control);
                    *to = transform.apply(*to);
                }
                PathCommand::CubicTo {
                    control1,
                    control2,
                    to,
                } => {
                    *control1 = transform.apply(*control1);
                    *control2 = transform.apply(*control2);
                    *to = transform.apply(*to);
                }
                PathCommand::Close => {}
            }
        }
        self.cached_bounds = None;
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.commands == other.commands
    }
}

/// A helper for building paths with cursor tracking.
///
/// [`PathCursor`] maintains the current pen position, making it easier to build
/// paths with relative movements.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
/// use manim_rs::renderer::PathCursor;
///
/// let mut cursor = PathCursor::new();
/// cursor.move_to(Vector2D::new(0.0, 0.0))
///       .line_to(Vector2D::new(1.0, 0.0))
///       .relative_line_to(Vector2D::new(0.0, 1.0)); // Goes to (1.0, 1.0)
///
/// let path = cursor.into_path();
/// ```
#[derive(Debug, Clone)]
pub struct PathCursor {
    path: Path,
    current: Vector2D,
}

impl PathCursor {
    /// Creates a new cursor at the origin.
    #[inline]
    pub fn new() -> Self {
        Self {
            path: Path::new(),
            current: Vector2D::ZERO,
        }
    }

    /// Returns the current cursor position.
    #[inline]
    pub fn position(&self) -> Vector2D {
        self.current
    }

    /// Moves the cursor to an absolute position.
    #[inline]
    pub fn move_to(&mut self, point: Vector2D) -> &mut Self {
        self.current = point;
        self.path.move_to(point);
        self
    }

    /// Draws a line to an absolute position.
    #[inline]
    pub fn line_to(&mut self, point: Vector2D) -> &mut Self {
        self.current = point;
        self.path.line_to(point);
        self
    }

    /// Draws a line relative to the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::renderer::PathCursor;
    ///
    /// let mut cursor = PathCursor::new();
    /// cursor.move_to(Vector2D::new(1.0, 1.0))
    ///       .relative_line_to(Vector2D::new(2.0, 3.0)); // Goes to (3.0, 4.0)
    ///
    /// assert_eq!(cursor.position(), Vector2D::new(3.0, 4.0));
    /// ```
    #[inline]
    pub fn relative_line_to(&mut self, delta: Vector2D) -> &mut Self {
        self.current = self.current + delta;
        self.path.line_to(self.current);
        self
    }

    /// Draws a quadratic Bézier curve to an absolute position.
    #[inline]
    pub fn quadratic_to(&mut self, control: Vector2D, to: Vector2D) -> &mut Self {
        self.current = to;
        self.path.quadratic_to(control, to);
        self
    }

    /// Draws a cubic Bézier curve to an absolute position.
    #[inline]
    pub fn cubic_to(&mut self, control1: Vector2D, control2: Vector2D, to: Vector2D) -> &mut Self {
        self.current = to;
        self.path.cubic_to(control1, control2, to);
        self
    }

    /// Closes the current subpath.
    #[inline]
    pub fn close(&mut self) -> &mut Self {
        self.path.close();
        self
    }

    /// Consumes the cursor and returns the built path.
    #[inline]
    pub fn into_path(self) -> Path {
        self.path
    }

    /// Returns a reference to the underlying path.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Default for PathCursor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_path_new() {
        let path = Path::new();
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_path_with_capacity() {
        let path = Path::with_capacity(100);
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_path_move_to() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(1.0, 2.0));

        assert_eq!(path.len(), 1);
        assert_eq!(
            path.commands()[0],
            PathCommand::MoveTo(Vector2D::new(1.0, 2.0))
        );
    }

    #[test]
    fn test_path_line_to() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        assert_eq!(path.len(), 2);
        assert_eq!(
            path.commands()[1],
            PathCommand::LineTo(Vector2D::new(1.0, 1.0))
        );
    }

    #[test]
    fn test_path_quadratic_to() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .quadratic_to(Vector2D::new(0.5, 1.0), Vector2D::new(1.0, 0.0));

        assert_eq!(path.len(), 2);
        match path.commands()[1] {
            PathCommand::QuadraticTo { control, to } => {
                assert_eq!(control, Vector2D::new(0.5, 1.0));
                assert_eq!(to, Vector2D::new(1.0, 0.0));
            }
            _ => panic!("Expected QuadraticTo command"),
        }
    }

    #[test]
    fn test_path_cubic_to() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0)).cubic_to(
            Vector2D::new(0.33, 1.0),
            Vector2D::new(0.66, 1.0),
            Vector2D::new(1.0, 0.0),
        );

        assert_eq!(path.len(), 2);
        match path.commands()[1] {
            PathCommand::CubicTo {
                control1,
                control2,
                to,
            } => {
                assert_eq!(control1, Vector2D::new(0.33, 1.0));
                assert_eq!(control2, Vector2D::new(0.66, 1.0));
                assert_eq!(to, Vector2D::new(1.0, 0.0));
            }
            _ => panic!("Expected CubicTo command"),
        }
    }

    #[test]
    fn test_path_close() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 0.0))
            .line_to(Vector2D::new(0.5, 1.0))
            .close();

        assert_eq!(path.len(), 4);
        assert_eq!(path.commands()[3], PathCommand::Close);
    }

    #[test]
    fn test_path_bounding_box_empty() {
        let path = Path::new();
        let bounds = path.bounding_box();
        assert_eq!(bounds, BoundingBox::zero());
    }

    #[test]
    fn test_path_bounding_box_simple() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(2.0, 3.0));

        let bounds = path.bounding_box();
        assert_relative_eq!(bounds.width(), 2.0);
        assert_relative_eq!(bounds.height(), 3.0);
    }

    #[test]
    fn test_path_bounding_box_cached() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let bounds1 = path.bounding_box();
        let bounds2 = path.bounding_box();
        assert_eq!(bounds1, bounds2);
    }

    #[test]
    fn test_path_bounding_box_invalidated() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let bounds1 = path.bounding_box();

        path.line_to(Vector2D::new(5.0, 5.0));
        let bounds2 = path.bounding_box();

        assert_ne!(bounds1.width(), bounds2.width());
    }

    #[test]
    fn test_path_transform() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 0.0));

        let transform = Transform::translate(2.0, 3.0);
        path.apply_transform(&transform);

        match path.commands()[0] {
            PathCommand::MoveTo(p) => {
                assert_relative_eq!(p.x, 2.0);
                assert_relative_eq!(p.y, 3.0);
            }
            _ => panic!("Expected MoveTo"),
        }

        match path.commands()[1] {
            PathCommand::LineTo(p) => {
                assert_relative_eq!(p.x, 3.0);
                assert_relative_eq!(p.y, 3.0);
            }
            _ => panic!("Expected LineTo"),
        }
    }

    #[test]
    fn test_path_clone() {
        let mut path1 = Path::new();
        path1
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let path2 = path1.clone();
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_path_equality() {
        let mut path1 = Path::new();
        path1
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let mut path2 = Path::new();
        path2
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        assert_eq!(path1, path2);
    }

    // PathCursor tests
    #[test]
    fn test_cursor_new() {
        let cursor = PathCursor::new();
        assert_eq!(cursor.position(), Vector2D::ZERO);
        assert!(cursor.path().is_empty());
    }

    #[test]
    fn test_cursor_move_to() {
        let mut cursor = PathCursor::new();
        cursor.move_to(Vector2D::new(1.0, 2.0));

        assert_eq!(cursor.position(), Vector2D::new(1.0, 2.0));
        assert_eq!(cursor.path().len(), 1);
    }

    #[test]
    fn test_cursor_line_to() {
        let mut cursor = PathCursor::new();
        cursor
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        assert_eq!(cursor.position(), Vector2D::new(1.0, 1.0));
        assert_eq!(cursor.path().len(), 2);
    }

    #[test]
    fn test_cursor_relative_line_to() {
        let mut cursor = PathCursor::new();
        cursor
            .move_to(Vector2D::new(1.0, 1.0))
            .relative_line_to(Vector2D::new(2.0, 3.0));

        assert_relative_eq!(cursor.position().x, 3.0);
        assert_relative_eq!(cursor.position().y, 4.0);
    }

    #[test]
    fn test_cursor_into_path() {
        let mut cursor = PathCursor::new();
        cursor
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));

        let path = cursor.into_path();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn test_cursor_quadratic() {
        let mut cursor = PathCursor::new();
        cursor
            .move_to(Vector2D::new(0.0, 0.0))
            .quadratic_to(Vector2D::new(0.5, 1.0), Vector2D::new(1.0, 0.0));

        assert_eq!(cursor.position(), Vector2D::new(1.0, 0.0));
    }

    #[test]
    fn test_cursor_cubic() {
        let mut cursor = PathCursor::new();
        cursor.move_to(Vector2D::new(0.0, 0.0)).cubic_to(
            Vector2D::new(0.33, 1.0),
            Vector2D::new(0.66, 1.0),
            Vector2D::new(1.0, 0.0),
        );

        assert_eq!(cursor.position(), Vector2D::new(1.0, 0.0));
    }
}
