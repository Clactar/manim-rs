//! SVG element types.
//!
//! This module defines internal types for representing SVG elements.

use crate::core::Vector2D;

/// An SVG element that can be rendered.
#[derive(Debug, Clone)]
pub(crate) enum SvgElement {
    /// A background rectangle
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        fill: String,
    },
    /// A path element
    Path {
        d: String,
        attrs: Vec<(String, String)>,
    },
    /// A text element
    Text {
        content: String,
        position: Vector2D,
        attrs: Vec<(String, String)>,
    },
}

impl SvgElement {
    /// Converts the element to an SVG string.
    pub(crate) fn to_svg_string(&self, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);

        match self {
            SvgElement::Rect {
                x,
                y,
                width,
                height,
                fill,
            } => {
                format!(
                    "{}<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
                    indent_str, x, y, width, height, fill
                )
            }
            SvgElement::Path { d, attrs } => {
                let mut result = format!("{}<path d=\"{}\"", indent_str, d);
                for (key, value) in attrs {
                    result.push_str(&format!(" {}=\"{}\"", key, value));
                }
                result.push_str(" />");
                result
            }
            SvgElement::Text {
                content,
                position,
                attrs,
            } => {
                let mut result = format!(
                    "{}<text x=\"{}\" y=\"{}\"",
                    indent_str, position.x, position.y
                );
                for (key, value) in attrs {
                    result.push_str(&format!(" {}=\"{}\"", key, value));
                }
                result.push('>');
                result.push_str(content);
                result.push_str("</text>");
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_element() {
        let rect = SvgElement::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
            fill: "#FF0000".to_string(),
        };

        let svg = rect.to_svg_string(1);
        assert!(svg.contains("<rect"));
        assert!(svg.contains("width=\"100\""));
        assert!(svg.contains("height=\"50\""));
        assert!(svg.contains("fill=\"#FF0000\""));
    }

    #[test]
    fn test_path_element() {
        let path = SvgElement::Path {
            d: "M 0 0 L 10 10".to_string(),
            attrs: vec![
                ("stroke".to_string(), "#0000FF".to_string()),
                ("stroke-width".to_string(), "2".to_string()),
            ],
        };

        let svg = path.to_svg_string(1);
        assert!(svg.contains("<path"));
        assert!(svg.contains("d=\"M 0 0 L 10 10\""));
        assert!(svg.contains("stroke=\"#0000FF\""));
        assert!(svg.contains("stroke-width=\"2\""));
    }

    #[test]
    fn test_text_element() {
        let text = SvgElement::Text {
            content: "Hello, World!".to_string(),
            position: Vector2D::new(100.0, 200.0),
            attrs: vec![
                ("fill".to_string(), "#FFFFFF".to_string()),
                ("font-size".to_string(), "48".to_string()),
            ],
        };

        let svg = text.to_svg_string(1);
        assert!(svg.contains("<text"));
        assert!(svg.contains("x=\"100\""));
        assert!(svg.contains("y=\"200\""));
        assert!(svg.contains("Hello, World!"));
        assert!(svg.contains("fill=\"#FFFFFF\""));
        assert!(svg.contains("</text>"));
    }

    #[test]
    fn test_element_indentation() {
        let rect = SvgElement::Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            fill: "#000000".to_string(),
        };

        let svg0 = rect.to_svg_string(0);
        assert!(!svg0.starts_with(' '));

        let svg1 = rect.to_svg_string(1);
        assert!(svg1.starts_with("  "));

        let svg2 = rect.to_svg_string(2);
        assert!(svg2.starts_with("    "));
    }
}
