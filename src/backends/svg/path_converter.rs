//! Path to SVG path data conversion.
//!
//! This module converts manim-rs [`Path`] objects into SVG path `d` attribute strings.

use crate::renderer::{Path, PathCommand};

/// Converts a path to an SVG path `d` attribute string.
///
/// # Examples
///
/// ```ignore
/// // Internal function used by SvgRenderer
/// use manim_rs::core::Vector2D;
/// use manim_rs::renderer::Path;
///
/// let mut path = Path::new();
/// path.move_to(Vector2D::new(0.0, 0.0))
///     .line_to(Vector2D::new(10.0, 10.0));
///
/// // path_to_svg_d is used internally by the SVG backend
/// ```
pub fn path_to_svg_d(path: &Path) -> String {
    let commands = path.commands();
    if commands.is_empty() {
        return String::new();
    }

    // Estimate capacity: ~15 chars per command average
    let mut result = String::with_capacity(commands.len() * 15);

    for (i, cmd) in commands.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&path_command_to_svg(cmd));
    }

    result
}

/// Converts a single path command to SVG syntax.
///
/// # Examples
///
/// ```ignore
/// // Internal function, not exported
/// use manim_rs::core::Vector2D;
/// use manim_rs::renderer::PathCommand;
///
/// let cmd = PathCommand::MoveTo(Vector2D::new(10.0, 20.0));
/// // path_command_to_svg is internal, use path_to_svg_d instead
/// ```
pub fn path_command_to_svg(cmd: &PathCommand) -> String {
    match cmd {
        PathCommand::MoveTo(p) => format!("M {} {}", format_coord(p.x), format_coord(p.y)),
        PathCommand::LineTo(p) => format!("L {} {}", format_coord(p.x), format_coord(p.y)),
        PathCommand::QuadraticTo { control, to } => format!(
            "Q {} {} {} {}",
            format_coord(control.x),
            format_coord(control.y),
            format_coord(to.x),
            format_coord(to.y)
        ),
        PathCommand::CubicTo {
            control1,
            control2,
            to,
        } => format!(
            "C {} {} {} {} {} {}",
            format_coord(control1.x),
            format_coord(control1.y),
            format_coord(control2.x),
            format_coord(control2.y),
            format_coord(to.x),
            format_coord(to.y)
        ),
        PathCommand::Close => "Z".to_string(),
    }
}

/// Formats a coordinate value for SVG output.
///
/// Rounds to 2 decimal places to reduce file size while maintaining visual accuracy.
#[inline]
fn format_coord(value: f64) -> String {
    // Round to 2 decimal places
    let rounded = (value * 100.0).round() / 100.0;

    // Remove trailing zeros and decimal point if integer
    if rounded.fract().abs() < f64::EPSILON {
        format!("{}", rounded as i32)
    } else {
        format!("{:.2}", rounded).trim_end_matches('0').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Vector2D;

    #[test]
    fn test_format_coord_integer() {
        assert_eq!(format_coord(10.0), "10");
        assert_eq!(format_coord(-5.0), "-5");
        assert_eq!(format_coord(0.0), "0");
    }

    #[test]
    fn test_format_coord_decimal() {
        assert_eq!(format_coord(10.5), "10.5");
        assert_eq!(format_coord(3.15), "3.15");
        assert_eq!(format_coord(-2.7), "-2.7");
    }

    #[test]
    fn test_format_coord_removes_trailing_zeros() {
        assert_eq!(format_coord(10.10), "10.1");
        assert_eq!(format_coord(5.00), "5");
    }

    #[test]
    fn test_move_to_svg() {
        let cmd = PathCommand::MoveTo(Vector2D::new(10.0, 20.0));
        assert_eq!(path_command_to_svg(&cmd), "M 10 20");
    }

    #[test]
    fn test_line_to_svg() {
        let cmd = PathCommand::LineTo(Vector2D::new(30.5, 40.25));
        assert_eq!(path_command_to_svg(&cmd), "L 30.5 40.25");
    }

    #[test]
    fn test_quadratic_to_svg() {
        let cmd = PathCommand::QuadraticTo {
            control: Vector2D::new(10.0, 20.0),
            to: Vector2D::new(30.0, 40.0),
        };
        assert_eq!(path_command_to_svg(&cmd), "Q 10 20 30 40");
    }

    #[test]
    fn test_cubic_to_svg() {
        let cmd = PathCommand::CubicTo {
            control1: Vector2D::new(1.0, 2.0),
            control2: Vector2D::new(3.0, 4.0),
            to: Vector2D::new(5.0, 6.0),
        };
        assert_eq!(path_command_to_svg(&cmd), "C 1 2 3 4 5 6");
    }

    #[test]
    fn test_close_svg() {
        let cmd = PathCommand::Close;
        assert_eq!(path_command_to_svg(&cmd), "Z");
    }

    #[test]
    fn test_path_to_svg_d_empty() {
        let path = Path::new();
        assert_eq!(path_to_svg_d(&path), "");
    }

    #[test]
    fn test_path_to_svg_d_simple() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(10.0, 10.0));

        let d = path_to_svg_d(&path);
        assert_eq!(d, "M 0 0 L 10 10");
    }

    #[test]
    fn test_path_to_svg_d_triangle() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 0.0))
            .line_to(Vector2D::new(0.5, 1.0))
            .close();

        let d = path_to_svg_d(&path);
        assert_eq!(d, "M 0 0 L 1 0 L 0.5 1 Z");
    }

    #[test]
    fn test_path_to_svg_d_with_curves() {
        let mut path = Path::new();
        path.move_to(Vector2D::new(0.0, 0.0)).cubic_to(
            Vector2D::new(1.0, 2.0),
            Vector2D::new(3.0, 4.0),
            Vector2D::new(5.0, 0.0),
        );

        let d = path_to_svg_d(&path);
        assert_eq!(d, "M 0 0 C 1 2 3 4 5 0");
    }
}
