//! Path to tiny-skia path conversion.
//!
//! This module converts manim-rs [`Path`] objects into tiny-skia paths.

use crate::renderer::{Path, PathCommand};

/// Converts a manim-rs Path to a tiny-skia Path.
///
/// Returns `None` if the path is empty or cannot be converted.
///
/// # Examples
///
/// ```ignore
/// // Internal function used by RasterRenderer
/// use manim_rs::core::Vector2D;
/// use manim_rs::renderer::Path;
///
/// let mut path = Path::new();
/// path.move_to(Vector2D::new(0.0, 0.0))
///     .line_to(Vector2D::new(10.0, 10.0));
///
/// // path_to_tiny_skia is used internally by the raster backend
/// ```
pub fn path_to_tiny_skia(path: &Path) -> Option<tiny_skia::Path> {
    let commands = path.commands();
    if commands.is_empty() {
        return None;
    }

    let mut builder = tiny_skia::PathBuilder::new();

    for cmd in commands {
        path_command_to_skia_builder(cmd, &mut builder);
    }

    builder.finish()
}

/// Converts a single path command and appends it to a tiny-skia PathBuilder.
pub fn path_command_to_skia_builder(cmd: &PathCommand, builder: &mut tiny_skia::PathBuilder) {
    match cmd {
        PathCommand::MoveTo(p) => {
            builder.move_to(p.x as f32, p.y as f32);
        }
        PathCommand::LineTo(p) => {
            builder.line_to(p.x as f32, p.y as f32);
        }
        PathCommand::QuadraticTo { control, to } => {
            builder.quad_to(control.x as f32, control.y as f32, to.x as f32, to.y as f32);
        }
        PathCommand::CubicTo {
            control1,
            control2,
            to,
        } => {
            builder.cubic_to(
                control1.x as f32,
                control1.y as f32,
                control2.x as f32,
                control2.y as f32,
                to.x as f32,
                to.y as f32,
            );
        }
        PathCommand::Close => {
            builder.close();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Vector2D;

    #[test]
    fn test_path_to_tiny_skia_empty() {
        let path = Path::new();
        assert!(path_to_tiny_skia(&path).is_none());
    }

    #[test]
    fn test_path_to_tiny_skia_simple() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(10.0, 10.0));

        let skia_path = path_to_tiny_skia(&path);
        assert!(skia_path.is_some());
    }

    #[test]
    fn test_path_to_tiny_skia_closed() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(10.0, 0.0))
            .line_to(Vector2D::new(10.0, 10.0))
            .close();

        let skia_path = path_to_tiny_skia(&path);
        assert!(skia_path.is_some());
    }

    #[test]
    fn test_path_with_curves() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .quadratic_to(Vector2D::new(5.0, 10.0), Vector2D::new(10.0, 0.0))
            .cubic_to(
                Vector2D::new(15.0, 5.0),
                Vector2D::new(20.0, 5.0),
                Vector2D::new(25.0, 0.0),
            );

        let skia_path = path_to_tiny_skia(&path);
        assert!(skia_path.is_some());
    }

    #[test]
    fn test_path_with_multiple_subpaths() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(10.0, 10.0))
            .close()
            .move_to(Vector2D::new(20.0, 20.0))
            .line_to(Vector2D::new(30.0, 30.0))
            .close();

        let skia_path = path_to_tiny_skia(&path);
        assert!(skia_path.is_some());
    }
}
