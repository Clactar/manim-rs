//! Style to SVG attributes conversion.
//!
//! This module converts manim-rs style types into SVG attribute key-value pairs.

use crate::core::Color;
use crate::renderer::{FontWeight, PathFillRule, PathStyle, TextAlignment, TextStyle};

/// Converts a [`PathStyle`] to SVG attributes.
///
/// Returns a vector of attribute name-value pairs suitable for inclusion in an SVG path element.
///
/// # Examples
///
/// ```ignore
/// // Internal function used by SvgRenderer
/// use manim_rs::core::Color;
/// use manim_rs::renderer::PathStyle;
///
/// let style = PathStyle::stroke(Color::RED, 2.5);
/// // path_style_to_svg_attrs is used internally by the SVG backend
/// ```
pub fn path_style_to_svg_attrs(style: &PathStyle) -> Vec<(&'static str, String)> {
    let mut attrs = Vec::with_capacity(8);

    // Stroke
    if let Some(stroke_color) = &style.stroke_color {
        attrs.push(("stroke", color_to_svg(stroke_color)));
        attrs.push(("stroke-width", format!("{}", style.stroke_width)));

        // Apply opacity to stroke if needed
        if style.opacity < 1.0 {
            let stroke_opacity = stroke_color.a * style.opacity;
            if stroke_opacity < 1.0 {
                attrs.push(("stroke-opacity", format!("{:.3}", stroke_opacity)));
            }
        } else if stroke_color.a < 1.0 {
            attrs.push(("stroke-opacity", format!("{:.3}", stroke_color.a)));
        }
    } else {
        attrs.push(("stroke", "none".to_string()));
    }

    // Fill
    if let Some(fill_color) = &style.fill_color {
        attrs.push(("fill", color_to_svg(fill_color)));

        // Apply opacity to fill if needed
        if style.opacity < 1.0 {
            let fill_opacity = fill_color.a * style.opacity;
            if fill_opacity < 1.0 {
                attrs.push(("fill-opacity", format!("{:.3}", fill_opacity)));
            }
        } else if fill_color.a < 1.0 {
            attrs.push(("fill-opacity", format!("{:.3}", fill_color.a)));
        }

        // Fill rule
        let fill_rule = match style.fill_rule {
            PathFillRule::NonZero => "nonzero",
            PathFillRule::EvenOdd => "evenodd",
        };
        attrs.push(("fill-rule", fill_rule.to_string()));
    } else {
        attrs.push(("fill", "none".to_string()));
    }

    attrs
}

/// Converts a [`TextStyle`] to SVG attributes.
///
/// Returns a vector of attribute name-value pairs suitable for inclusion in an SVG text element.
pub fn text_style_to_svg_attrs(style: &TextStyle) -> Vec<(&'static str, String)> {
    let mut attrs = Vec::with_capacity(6);

    attrs.push(("fill", color_to_svg(&style.color)));

    if style.color.a < 1.0 || style.opacity < 1.0 {
        let opacity = style.color.a * style.opacity;
        if opacity < 1.0 {
            attrs.push(("opacity", format!("{:.3}", opacity)));
        }
    }

    attrs.push(("font-size", format!("{}", style.font_size)));
    attrs.push(("font-family", style.font_family.clone()));

    let weight = match style.font_weight {
        FontWeight::Normal => "normal",
        FontWeight::Bold => "bold",
    };
    attrs.push(("font-weight", weight.to_string()));

    let anchor = match style.alignment {
        TextAlignment::Left => "start",
        TextAlignment::Center => "middle",
        TextAlignment::Right => "end",
    };
    attrs.push(("text-anchor", anchor.to_string()));

    attrs
}

/// Converts a [`Color`] to an SVG color string.
///
/// Returns a hex color string in the format `#RRGGBB`.
/// Alpha channel is handled separately via opacity attributes.
///
/// # Examples
///
/// ```ignore
/// // Internal function used by SvgRenderer
/// use manim_rs::core::Color;
///
/// let color = Color::rgba(1.0, 0.5, 0.0, 1.0);
/// // color_to_svg is used internally by the SVG backend
/// ```
pub fn color_to_svg(color: &Color) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8
    )
}

/// Escapes special XML characters in text content.
pub fn escape_xml(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            '<' => "&lt;".chars().collect::<Vec<_>>(),
            '>' => "&gt;".chars().collect::<Vec<_>>(),
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '"' => "&quot;".chars().collect::<Vec<_>>(),
            '\'' => "&apos;".chars().collect::<Vec<_>>(),
            c => vec![c],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_svg() {
        let color = Color::rgba(1.0, 0.5, 0.0, 1.0);
        assert_eq!(color_to_svg(&color), "#FF7F00");

        let black = Color::BLACK;
        assert_eq!(color_to_svg(&black), "#000000");

        let white = Color::WHITE;
        assert_eq!(color_to_svg(&white), "#FFFFFF");
    }

    #[test]
    fn test_color_to_svg_hex_input() {
        let color = Color::from_hex("#FF5733").unwrap();
        assert_eq!(color_to_svg(&color), "#FF5733");
    }

    #[test]
    fn test_path_style_stroke_only() {
        let style = PathStyle::stroke(Color::from_hex("#FF0000").unwrap(), 2.5);
        let attrs = path_style_to_svg_attrs(&style);

        assert!(attrs.iter().any(|(k, v)| k == &"stroke" && v == "#FF0000"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"stroke-width" && v == "2.5"));
        assert!(attrs.iter().any(|(k, v)| k == &"fill" && v == "none"));
    }

    #[test]
    fn test_path_style_fill_only() {
        let style = PathStyle::fill(Color::from_hex("#00FF00").unwrap());
        let attrs = path_style_to_svg_attrs(&style);

        assert!(attrs.iter().any(|(k, v)| k == &"fill" && v == "#00FF00"));
        assert!(attrs.iter().any(|(k, v)| k == &"stroke" && v == "none"));
    }

    #[test]
    fn test_path_style_both_stroke_and_fill() {
        let style = PathStyle::default()
            .with_stroke(Color::BLACK, 1.0)
            .with_fill(Color::RED);

        let attrs = path_style_to_svg_attrs(&style);

        assert!(attrs.iter().any(|(k, v)| k == &"stroke" && v == "#000000"));
        assert!(attrs.iter().any(|(k, v)| k == &"fill" && v == "#FF0000"));
    }

    #[test]
    fn test_path_style_with_opacity() {
        let style = PathStyle::stroke(Color::BLUE, 2.0).with_opacity(0.5);
        let attrs = path_style_to_svg_attrs(&style);

        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"stroke-opacity" && v.starts_with("0.5")));
    }

    #[test]
    fn test_path_style_with_fill_rule() {
        let style = PathStyle::fill(Color::RED).with_fill_rule(PathFillRule::EvenOdd);
        let attrs = path_style_to_svg_attrs(&style);

        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"fill-rule" && v == "evenodd"));
    }

    #[test]
    fn test_text_style_to_svg_attrs() {
        let style = TextStyle::new(Color::WHITE, 48.0)
            .with_font_family("Arial")
            .with_weight(FontWeight::Bold)
            .with_alignment(TextAlignment::Center);

        let attrs = text_style_to_svg_attrs(&style);

        assert!(attrs.iter().any(|(k, v)| k == &"fill" && v == "#FFFFFF"));
        assert!(attrs.iter().any(|(k, v)| k == &"font-size" && v == "48"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"font-family" && v == "Arial"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"font-weight" && v == "bold"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"text-anchor" && v == "middle"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("Hello"), "Hello");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("A & B"), "A &amp; B");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(escape_xml("'single'"), "&apos;single&apos;");
    }

    #[test]
    fn test_escape_xml_complex() {
        let input = "<div class=\"test\" data='value'>Content & more</div>";
        let escaped = escape_xml(input);
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&quot;"));
        assert!(escaped.contains("&apos;"));
        assert!(escaped.contains("&amp;"));
    }
}
