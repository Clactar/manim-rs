//! Style to tiny-skia conversion.
//!
//! This module converts manim-rs style types into tiny-skia paint and stroke objects.

use crate::core::Color;
use crate::renderer::{PathFillRule, PathStyle};

/// Converts a Color with opacity to a tiny-skia Color.
///
/// # Examples
///
/// ```ignore
/// // Internal function used by RasterRenderer
/// use manim_rs::core::Color;
///
/// let color = Color::RED;
/// // color_to_skia_color is used internally by the raster backend
/// ```
pub fn color_to_skia_color(color: &Color, opacity: f64) -> tiny_skia::Color {
    let combined_alpha = (color.a * opacity * 255.0) as u8;
    tiny_skia::Color::from_rgba8(
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        combined_alpha,
    )
}

/// Converts a PathStyle to a tiny-skia Paint for filling.
///
/// Returns `None` if the style has no fill color.
pub fn path_style_to_fill_paint(style: &PathStyle) -> Option<tiny_skia::Paint<'static>> {
    let fill_color = style.fill_color.as_ref()?;

    let skia_color = color_to_skia_color(fill_color, style.opacity);

    let paint = tiny_skia::Paint {
        shader: tiny_skia::Shader::SolidColor(skia_color),
        anti_alias: true,
        blend_mode: tiny_skia::BlendMode::SourceOver,
        ..Default::default()
    };

    Some(paint)
}

/// Converts a PathStyle to a tiny-skia Paint for stroking.
///
/// Returns `None` if the style has no stroke color.
pub fn path_style_to_stroke_paint(style: &PathStyle) -> Option<tiny_skia::Paint<'static>> {
    let stroke_color = style.stroke_color.as_ref()?;

    let skia_color = color_to_skia_color(stroke_color, style.opacity);

    let paint = tiny_skia::Paint {
        shader: tiny_skia::Shader::SolidColor(skia_color),
        anti_alias: true,
        blend_mode: tiny_skia::BlendMode::SourceOver,
        ..Default::default()
    };

    Some(paint)
}

/// Converts a PathStyle to a tiny-skia Stroke.
///
/// Returns `None` if the style has no stroke color.
pub fn path_style_to_stroke(style: &PathStyle) -> Option<tiny_skia::Stroke> {
    style.stroke_color.as_ref()?;

    let stroke = tiny_skia::Stroke {
        width: style.stroke_width as f32,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Default::default()
    };

    Some(stroke)
}

/// Converts a PathFillRule to a tiny-skia FillRule.
pub fn fill_rule_to_skia(rule: PathFillRule) -> tiny_skia::FillRule {
    match rule {
        PathFillRule::NonZero => tiny_skia::FillRule::Winding,
        PathFillRule::EvenOdd => tiny_skia::FillRule::EvenOdd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_skia_color() {
        let color = Color::RED;
        let skia_color = color_to_skia_color(&color, 1.0);

        assert!(skia_color.red() > 0.99);
        assert!(skia_color.green() < 0.01);
        assert!(skia_color.blue() < 0.01);
        assert!(skia_color.alpha() > 0.99);
    }

    #[test]
    fn test_color_to_skia_color_with_opacity() {
        let color = Color::BLUE;
        let skia_color = color_to_skia_color(&color, 0.5);

        assert!(skia_color.blue() > 0.99);
        assert!((skia_color.alpha() - 0.5).abs() < 0.02); // ~0.5
    }

    #[test]
    fn test_color_with_alpha_channel() {
        let color = Color::rgba(1.0, 0.0, 0.0, 0.5);
        let skia_color = color_to_skia_color(&color, 1.0);

        assert!(skia_color.red() > 0.99);
        assert!((skia_color.alpha() - 0.5).abs() < 0.02); // ~0.5
    }

    #[test]
    fn test_path_style_to_fill_paint() {
        let style = PathStyle::fill(Color::RED);
        let paint = path_style_to_fill_paint(&style);

        assert!(paint.is_some());
    }

    #[test]
    fn test_path_style_to_fill_paint_no_fill() {
        let style = PathStyle::stroke(Color::RED, 2.0);
        let paint = path_style_to_fill_paint(&style);

        assert!(paint.is_none());
    }

    #[test]
    fn test_path_style_to_stroke_paint() {
        let style = PathStyle::stroke(Color::BLUE, 2.0);
        let paint = path_style_to_stroke_paint(&style);

        assert!(paint.is_some());
    }

    #[test]
    fn test_path_style_to_stroke_paint_no_stroke() {
        let style = PathStyle::fill(Color::BLUE);
        let paint = path_style_to_stroke_paint(&style);

        assert!(paint.is_none());
    }

    #[test]
    fn test_path_style_to_stroke() {
        let style = PathStyle::stroke(Color::GREEN, 3.5);
        let stroke = path_style_to_stroke(&style);

        assert!(stroke.is_some());
        let stroke = stroke.unwrap();
        assert_eq!(stroke.width, 3.5);
    }

    #[test]
    fn test_path_style_to_stroke_no_stroke() {
        let style = PathStyle::fill(Color::GREEN);
        let stroke = path_style_to_stroke(&style);

        assert!(stroke.is_none());
    }

    #[test]
    fn test_fill_rule_conversion() {
        assert_eq!(
            fill_rule_to_skia(PathFillRule::NonZero),
            tiny_skia::FillRule::Winding
        );
        assert_eq!(
            fill_rule_to_skia(PathFillRule::EvenOdd),
            tiny_skia::FillRule::EvenOdd
        );
    }
}
