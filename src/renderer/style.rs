//! Rendering style configuration.
//!
//! This module provides [`PathStyle`] and [`TextStyle`] for configuring how
//! paths and text are rendered by backends.
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::Color;
//! use manim_rs::renderer::{PathStyle, TextStyle};
//!
//! // Create a stroke-only style
//! let stroke_style = PathStyle::stroke(Color::BLUE, 2.0);
//!
//! // Create a fill-only style
//! let fill_style = PathStyle::fill(Color::RED);
//!
//! // Create a text style
//! let text_style = TextStyle::new(Color::WHITE, 48.0);
//! ```

use crate::core::Color;

/// Fill rule for path rendering.
///
/// Determines which areas are considered "inside" a path when filling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFillRule {
    /// Non-zero winding rule (default for most graphics systems).
    ///
    /// A point is inside if a ray from the point crosses a non-zero net number
    /// of path segments.
    NonZero,

    /// Even-odd rule.
    ///
    /// A point is inside if a ray from the point crosses an odd number of path
    /// segments.
    EvenOdd,
}

impl Default for PathFillRule {
    fn default() -> Self {
        Self::NonZero
    }
}

/// Style configuration for path rendering.
///
/// Controls stroke, fill, opacity, and fill rules for vector paths.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::renderer::PathStyle;
///
/// // Default style (white stroke, no fill)
/// let style = PathStyle::default();
///
/// // Stroke only
/// let stroke = PathStyle::stroke(Color::BLUE, 3.0);
///
/// // Fill only
/// let fill = PathStyle::fill(Color::RED);
///
/// // Both stroke and fill
/// let both = PathStyle::default()
///     .with_stroke(Color::BLACK, 1.0)
///     .with_fill(Color::from_hex("#FF5733").unwrap())
///     .with_opacity(0.8);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PathStyle {
    /// Stroke color (None means no stroke)
    pub stroke_color: Option<Color>,

    /// Stroke width in user units
    pub stroke_width: f64,

    /// Fill color (None means no fill)
    pub fill_color: Option<Color>,

    /// Fill rule for determining inside/outside
    pub fill_rule: PathFillRule,

    /// Overall opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f64,
}

impl PathStyle {
    /// Creates a stroke-only style.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::PathStyle;
    ///
    /// let style = PathStyle::stroke(Color::BLUE, 2.0);
    /// ```
    pub fn stroke(color: Color, width: f64) -> Self {
        Self {
            stroke_color: Some(color),
            stroke_width: width,
            fill_color: None,
            fill_rule: PathFillRule::default(),
            opacity: 1.0,
        }
    }

    /// Creates a fill-only style.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::PathStyle;
    ///
    /// let style = PathStyle::fill(Color::RED);
    /// ```
    pub fn fill(color: Color) -> Self {
        Self {
            stroke_color: None,
            stroke_width: 0.0,
            fill_color: Some(color),
            fill_rule: PathFillRule::default(),
            opacity: 1.0,
        }
    }

    /// Sets the stroke color and width.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::PathStyle;
    ///
    /// let style = PathStyle::default()
    ///     .with_stroke(Color::BLACK, 2.0);
    /// ```
    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    /// Sets the fill color.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::PathStyle;
    ///
    /// let style = PathStyle::default()
    ///     .with_fill(Color::RED);
    /// ```
    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Sets the fill rule.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::renderer::{PathFillRule, PathStyle};
    ///
    /// let style = PathStyle::default()
    ///     .with_fill_rule(PathFillRule::EvenOdd);
    /// ```
    pub fn with_fill_rule(mut self, rule: PathFillRule) -> Self {
        self.fill_rule = rule;
        self
    }

    /// Sets the opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::renderer::PathStyle;
    ///
    /// let style = PathStyle::default()
    ///     .with_opacity(0.5);
    /// ```
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Default for PathStyle {
    /// Creates the default style: white stroke, no fill, full opacity.
    fn default() -> Self {
        Self {
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            fill_rule: PathFillRule::default(),
            opacity: 1.0,
        }
    }
}

/// Font weight for text rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    /// Normal weight (400)
    Normal,

    /// Bold weight (700)
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Align text to the left
    Left,

    /// Center text
    Center,

    /// Align text to the right
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Left
    }
}

/// Style configuration for text rendering.
///
/// Controls font properties, color, and alignment for text.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
/// use manim_rs::renderer::{FontWeight, TextAlignment, TextStyle};
///
/// let style = TextStyle::new(Color::WHITE, 48.0)
///     .with_font_family("Arial")
///     .with_weight(FontWeight::Bold)
///     .with_alignment(TextAlignment::Center);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// Text color
    pub color: Color,

    /// Font size in points
    pub font_size: f64,

    /// Font family name
    pub font_family: String,

    /// Font weight (normal or bold)
    pub font_weight: FontWeight,

    /// Text alignment
    pub alignment: TextAlignment,

    /// Overall opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f64,
}

impl TextStyle {
    /// Creates a new text style with the given color and font size.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::TextStyle;
    ///
    /// let style = TextStyle::new(Color::WHITE, 48.0);
    /// ```
    pub fn new(color: Color, font_size: f64) -> Self {
        Self {
            color,
            font_size,
            font_family: "sans-serif".to_string(),
            font_weight: FontWeight::default(),
            alignment: TextAlignment::default(),
            opacity: 1.0,
        }
    }

    /// Sets the font family.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::TextStyle;
    ///
    /// let style = TextStyle::new(Color::WHITE, 48.0)
    ///     .with_font_family("Arial");
    /// ```
    pub fn with_font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = family.into();
        self
    }

    /// Sets the font weight.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::{FontWeight, TextStyle};
    ///
    /// let style = TextStyle::new(Color::WHITE, 48.0)
    ///     .with_weight(FontWeight::Bold);
    /// ```
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    /// Sets the text alignment.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::{TextAlignment, TextStyle};
    ///
    /// let style = TextStyle::new(Color::WHITE, 48.0)
    ///     .with_alignment(TextAlignment::Center);
    /// ```
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets the opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    /// use manim_rs::renderer::TextStyle;
    ///
    /// let style = TextStyle::new(Color::WHITE, 48.0)
    ///     .with_opacity(0.8);
    /// ```
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Default for TextStyle {
    /// Creates the default text style: white text, 48pt, sans-serif font.
    fn default() -> Self {
        Self::new(Color::WHITE, 48.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PathFillRule tests
    #[test]
    fn test_fill_rule_default() {
        assert_eq!(PathFillRule::default(), PathFillRule::NonZero);
    }

    // PathStyle tests
    #[test]
    fn test_path_style_default() {
        let style = PathStyle::default();
        assert!(style.stroke_color.is_some());
        assert_eq!(style.stroke_color.unwrap(), Color::WHITE);
        assert_eq!(style.stroke_width, 2.0);
        assert!(style.fill_color.is_none());
        assert_eq!(style.opacity, 1.0);
    }

    #[test]
    fn test_path_style_stroke() {
        let style = PathStyle::stroke(Color::BLUE, 3.0);
        assert_eq!(style.stroke_color, Some(Color::BLUE));
        assert_eq!(style.stroke_width, 3.0);
        assert!(style.fill_color.is_none());
    }

    #[test]
    fn test_path_style_fill() {
        let style = PathStyle::fill(Color::RED);
        assert!(style.stroke_color.is_none());
        assert_eq!(style.fill_color, Some(Color::RED));
    }

    #[test]
    fn test_path_style_with_stroke() {
        let style = PathStyle::default().with_stroke(Color::GREEN, 5.0);
        assert_eq!(style.stroke_color, Some(Color::GREEN));
        assert_eq!(style.stroke_width, 5.0);
    }

    #[test]
    fn test_path_style_with_fill() {
        let style = PathStyle::default().with_fill(Color::YELLOW);
        assert_eq!(style.fill_color, Some(Color::YELLOW));
    }

    #[test]
    fn test_path_style_with_fill_rule() {
        let style = PathStyle::default().with_fill_rule(PathFillRule::EvenOdd);
        assert_eq!(style.fill_rule, PathFillRule::EvenOdd);
    }

    #[test]
    fn test_path_style_with_opacity() {
        let style = PathStyle::default().with_opacity(0.5);
        assert_eq!(style.opacity, 0.5);
    }

    #[test]
    fn test_path_style_opacity_clamped() {
        let style1 = PathStyle::default().with_opacity(-0.5);
        assert_eq!(style1.opacity, 0.0);

        let style2 = PathStyle::default().with_opacity(1.5);
        assert_eq!(style2.opacity, 1.0);
    }

    #[test]
    fn test_path_style_chaining() {
        let style = PathStyle::default()
            .with_stroke(Color::BLACK, 1.0)
            .with_fill(Color::RED)
            .with_opacity(0.8)
            .with_fill_rule(PathFillRule::EvenOdd);

        assert_eq!(style.stroke_color, Some(Color::BLACK));
        assert_eq!(style.stroke_width, 1.0);
        assert_eq!(style.fill_color, Some(Color::RED));
        assert_eq!(style.opacity, 0.8);
        assert_eq!(style.fill_rule, PathFillRule::EvenOdd);
    }

    #[test]
    fn test_path_style_clone() {
        let style1 = PathStyle::stroke(Color::BLUE, 2.0);
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    // FontWeight tests
    #[test]
    fn test_font_weight_default() {
        assert_eq!(FontWeight::default(), FontWeight::Normal);
    }

    // TextAlignment tests
    #[test]
    fn test_text_alignment_default() {
        assert_eq!(TextAlignment::default(), TextAlignment::Left);
    }

    // TextStyle tests
    #[test]
    fn test_text_style_new() {
        let style = TextStyle::new(Color::WHITE, 48.0);
        assert_eq!(style.color, Color::WHITE);
        assert_eq!(style.font_size, 48.0);
        assert_eq!(style.font_family, "sans-serif");
        assert_eq!(style.font_weight, FontWeight::Normal);
        assert_eq!(style.alignment, TextAlignment::Left);
        assert_eq!(style.opacity, 1.0);
    }

    #[test]
    fn test_text_style_default() {
        let style = TextStyle::default();
        assert_eq!(style.color, Color::WHITE);
        assert_eq!(style.font_size, 48.0);
    }

    #[test]
    fn test_text_style_with_font_family() {
        let style = TextStyle::default().with_font_family("Arial");
        assert_eq!(style.font_family, "Arial");
    }

    #[test]
    fn test_text_style_with_weight() {
        let style = TextStyle::default().with_weight(FontWeight::Bold);
        assert_eq!(style.font_weight, FontWeight::Bold);
    }

    #[test]
    fn test_text_style_with_alignment() {
        let style = TextStyle::default().with_alignment(TextAlignment::Center);
        assert_eq!(style.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_text_style_with_opacity() {
        let style = TextStyle::default().with_opacity(0.7);
        assert_eq!(style.opacity, 0.7);
    }

    #[test]
    fn test_text_style_opacity_clamped() {
        let style1 = TextStyle::default().with_opacity(-0.5);
        assert_eq!(style1.opacity, 0.0);

        let style2 = TextStyle::default().with_opacity(2.0);
        assert_eq!(style2.opacity, 1.0);
    }

    #[test]
    fn test_text_style_chaining() {
        let style = TextStyle::new(Color::BLACK, 24.0)
            .with_font_family("Helvetica")
            .with_weight(FontWeight::Bold)
            .with_alignment(TextAlignment::Right)
            .with_opacity(0.9);

        assert_eq!(style.color, Color::BLACK);
        assert_eq!(style.font_size, 24.0);
        assert_eq!(style.font_family, "Helvetica");
        assert_eq!(style.font_weight, FontWeight::Bold);
        assert_eq!(style.alignment, TextAlignment::Right);
        assert_eq!(style.opacity, 0.9);
    }

    #[test]
    fn test_text_style_clone() {
        let style1 = TextStyle::new(Color::BLUE, 36.0);
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }
}
