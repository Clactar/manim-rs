/// An RGBA color representation.
///
/// Colors are stored as normalized floating-point values (0.0 to 1.0)
/// for maximum precision during interpolation and blending.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Color;
///
/// // Create from RGB values (0-255)
/// let red = Color::rgb(255, 0, 0);
/// let blue = Color::rgb(0, 0, 255);
///
/// // Create from normalized values
/// let green = Color::rgba(0.0, 1.0, 0.0, 1.0);
///
/// // Interpolate between colors
/// let purple = red.lerp(blue, 0.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    /// Creates a color from RGBA values (0.0 to 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::rgba(1.0, 0.0, 0.0, 1.0);
    /// ```
    #[inline]
    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a color from RGB byte values (0-255) with full opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// let green = Color::rgb(0, 255, 0);
    /// let blue = Color::rgb(0, 0, 255);
    /// ```
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, 1.0)
    }

    /// Creates a color from a hex string (e.g., "#FF0000" or "FF0000").
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// let blue = Color::from_hex("0000FF").unwrap();
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(Self::rgb(r, g, b))
    }

    /// Converts the color to a hex string (e.g., "#FF0000").
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// assert_eq!(red.to_hex(), "#FF0000");
    /// ```
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    /// Linearly interpolates between this color and another.
    ///
    /// # Arguments
    ///
    /// * `other` - The target color
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// let blue = Color::rgb(0, 0, 255);
    /// let purple = red.lerp(blue, 0.5);
    /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self::rgba(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Returns a color with modified alpha (opacity).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// let transparent_red = red.with_alpha(0.5);
    /// ```
    #[inline]
    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.a = alpha.clamp(0.0, 1.0);
        self
    }

    // Common colors
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = Self::rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = Self::rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgba(1.0, 0.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let red = Color::rgb(255, 0, 0);
        assert_eq!(red.r, 1.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);
        assert_eq!(red.a, 1.0);
    }

    #[test]
    fn test_from_hex() {
        let red = Color::from_hex("#FF0000").unwrap();
        assert!((red.r - 1.0).abs() < 1e-10);
        assert!(red.g.abs() < 1e-10);
        assert!(red.b.abs() < 1e-10);

        let blue = Color::from_hex("0000FF").unwrap();
        assert!(blue.r.abs() < 1e-10);
        assert!(blue.g.abs() < 1e-10);
        assert!((blue.b - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_hex() {
        let red = Color::rgb(255, 0, 0);
        assert_eq!(red.to_hex(), "#FF0000");

        let purple = Color::rgb(128, 0, 128);
        assert_eq!(purple.to_hex(), "#800080");
    }

    #[test]
    fn test_lerp() {
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        let purple = red.lerp(blue, 0.5);

        assert!((purple.r - 0.5).abs() < 1e-10);
        assert!(purple.g.abs() < 1e-10);
        assert!((purple.b - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_with_alpha() {
        let red = Color::rgb(255, 0, 0);
        let transparent_red = red.with_alpha(0.5);
        assert_eq!(transparent_red.a, 0.5);
        assert_eq!(transparent_red.r, 1.0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Color::WHITE.r, 1.0);
        assert_eq!(Color::BLACK.r, 0.0);
        assert_eq!(Color::TRANSPARENT.a, 0.0);
    }
}
