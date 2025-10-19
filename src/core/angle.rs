//! Angle types for representing rotations and orientations.
//!
//! This module provides type-safe angle representations with automatic
//! conversions between degrees and radians. Angles are fundamental for
//! transformations, rotations, and geometric calculations.
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::{Degrees, Radians};
//!
//! let deg = Degrees(90.0);
//! let rad = Radians(std::f64::consts::PI / 2.0);
//!
//! // Convert between representations
//! assert!((deg.to_radians().0 - rad.0).abs() < 1e-10);
//! assert!((rad.to_degrees().0 - deg.0).abs() < 1e-10);
//!
//! // Normalize angles
//! let normalized = Degrees(450.0).normalized();
//! assert_eq!(normalized.0, 90.0);
//! ```

use std::f64::consts::PI;

/// An angle measured in degrees.
///
/// Degrees are convenient for human-readable angles and are commonly used
/// in user interfaces and configuration files.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Degrees(pub f64);

/// An angle measured in radians.
///
/// Radians are the standard unit for mathematical calculations and are
/// used internally by most trigonometric functions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Radians(pub f64);

impl Degrees {
    /// Creates a new angle in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Degrees;
    ///
    /// let angle = Degrees::new(45.0);
    /// assert_eq!(angle.0, 45.0);
    /// ```
    #[inline]
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    /// Converts degrees to radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Degrees;
    /// use std::f64::consts::PI;
    ///
    /// let deg = Degrees(180.0);
    /// let rad = deg.to_radians();
    /// assert!((rad.0 - PI).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn to_radians(self) -> Radians {
        Radians(self.0 * PI / 180.0)
    }

    /// Normalizes the angle to the range [0, 360).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Degrees;
    ///
    /// let angle = Degrees(450.0);
    /// let normalized = angle.normalized();
    /// assert_eq!(normalized.0, 90.0);
    ///
    /// let negative = Degrees(-90.0);
    /// let normalized_neg = negative.normalized();
    /// assert_eq!(normalized_neg.0, 270.0);
    /// ```
    #[inline]
    pub fn normalized(self) -> Self {
        let mut angle = self.0 % 360.0;
        if angle < 0.0 {
            angle += 360.0;
        }
        Self(angle)
    }

    /// Returns the sine of the angle.
    #[inline]
    pub fn sin(self) -> f64 {
        self.to_radians().0.sin()
    }

    /// Returns the cosine of the angle.
    #[inline]
    pub fn cos(self) -> f64 {
        self.to_radians().0.cos()
    }

    /// Returns the tangent of the angle.
    #[inline]
    pub fn tan(self) -> f64 {
        self.to_radians().0.tan()
    }
}

impl Radians {
    /// Creates a new angle in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let angle = Radians::new(PI / 2.0);
    /// assert_eq!(angle.0, PI / 2.0);
    /// ```
    #[inline]
    pub fn new(radians: f64) -> Self {
        Self(radians)
    }

    /// Converts radians to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rad = Radians(PI);
    /// let deg = rad.to_degrees();
    /// assert!((deg.0 - 180.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn to_degrees(self) -> Degrees {
        Degrees(self.0 * 180.0 / PI)
    }

    /// Normalizes the angle to the range [0, 2π).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let angle = Radians(5.0 * PI);
    /// let normalized = angle.normalized();
    /// assert!((normalized.0 - PI).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn normalized(self) -> Self {
        let mut angle = self.0 % (2.0 * PI);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        Self(angle)
    }

    /// Returns the sine of the angle.
    #[inline]
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// Returns the cosine of the angle.
    #[inline]
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    /// Returns the tangent of the angle.
    #[inline]
    pub fn tan(self) -> f64 {
        self.0.tan()
    }
}

// Conversion traits
impl From<Degrees> for Radians {
    #[inline]
    fn from(degrees: Degrees) -> Self {
        degrees.to_radians()
    }
}

impl From<Radians> for Degrees {
    #[inline]
    fn from(radians: Radians) -> Self {
        radians.to_degrees()
    }
}

// Common angle constants
impl Degrees {
    pub const ZERO: Self = Self(0.0);
    pub const RIGHT: Self = Self(0.0);
    pub const UP: Self = Self(90.0);
    pub const LEFT: Self = Self(180.0);
    pub const DOWN: Self = Self(270.0);
    pub const FULL_CIRCLE: Self = Self(360.0);
}

impl Radians {
    pub const ZERO: Self = Self(0.0);
    pub const RIGHT: Self = Self(0.0);
    pub const UP: Self = Self(PI / 2.0);
    pub const LEFT: Self = Self(PI);
    pub const DOWN: Self = Self(3.0 * PI / 2.0);
    pub const FULL_CIRCLE: Self = Self(2.0 * PI);
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_degrees_creation() {
        let deg = Degrees::new(90.0);
        assert_eq!(deg.0, 90.0);
    }

    #[test]
    fn test_radians_creation() {
        let rad = Radians::new(PI);
        assert_eq!(rad.0, PI);
    }

    #[test]
    fn test_degrees_to_radians() {
        let deg = Degrees(180.0);
        let rad = deg.to_radians();
        assert_relative_eq!(rad.0, PI, epsilon = 1e-10);

        let deg_zero = Degrees(0.0);
        let rad_zero = deg_zero.to_radians();
        assert_relative_eq!(rad_zero.0, 0.0, epsilon = 1e-10);

        let deg_90 = Degrees(90.0);
        let rad_90 = deg_90.to_radians();
        assert_relative_eq!(rad_90.0, PI / 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_radians_to_degrees() {
        let rad = Radians(PI);
        let deg = rad.to_degrees();
        assert_relative_eq!(deg.0, 180.0, epsilon = 1e-10);

        let rad_zero = Radians(0.0);
        let deg_zero = rad_zero.to_degrees();
        assert_relative_eq!(deg_zero.0, 0.0, epsilon = 1e-10);

        let rad_quarter = Radians(PI / 2.0);
        let deg_quarter = rad_quarter.to_degrees();
        assert_relative_eq!(deg_quarter.0, 90.0, epsilon = 1e-10);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_deg = 45.0;
        let deg = Degrees(original_deg);
        let rad = deg.to_radians();
        let back_to_deg = rad.to_degrees();
        assert_relative_eq!(back_to_deg.0, original_deg, epsilon = 1e-10);

        let original_rad = PI / 3.0;
        let rad = Radians(original_rad);
        let deg = rad.to_degrees();
        let back_to_rad = deg.to_radians();
        assert_relative_eq!(back_to_rad.0, original_rad, epsilon = 1e-10);
    }

    #[test]
    fn test_degrees_normalization() {
        assert_eq!(Degrees(450.0).normalized().0, 90.0);
        assert_eq!(Degrees(360.0).normalized().0, 0.0);
        assert_eq!(Degrees(0.0).normalized().0, 0.0);
        assert_eq!(Degrees(-90.0).normalized().0, 270.0);
        assert_eq!(Degrees(-450.0).normalized().0, 270.0);
    }

    #[test]
    fn test_radians_normalization() {
        assert_relative_eq!(Radians(5.0 * PI).normalized().0, PI, epsilon = 1e-10);
        assert_relative_eq!(Radians(2.0 * PI).normalized().0, 0.0, epsilon = 1e-10);
        assert_relative_eq!(Radians(0.0).normalized().0, 0.0, epsilon = 1e-10);
        assert_relative_eq!(Radians(-PI / 2.0).normalized().0, 3.0 * PI / 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_trigonometric_functions_degrees() {
        let deg_0 = Degrees(0.0);
        assert_relative_eq!(deg_0.sin(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(deg_0.cos(), 1.0, epsilon = 1e-10);

        let deg_90 = Degrees(90.0);
        assert_relative_eq!(deg_90.sin(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(deg_90.cos(), 0.0, epsilon = 1e-10);

        let deg_45 = Degrees(45.0);
        assert_relative_eq!(deg_45.sin(), (PI / 4.0).sin(), epsilon = 1e-10);
        assert_relative_eq!(deg_45.cos(), (PI / 4.0).cos(), epsilon = 1e-10);
    }

    #[test]
    fn test_trigonometric_functions_radians() {
        let rad_0 = Radians(0.0);
        assert_relative_eq!(rad_0.sin(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(rad_0.cos(), 1.0, epsilon = 1e-10);

        let rad_90 = Radians(PI / 2.0);
        assert_relative_eq!(rad_90.sin(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(rad_90.cos(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_conversion_traits() {
        let deg = Degrees(90.0);
        let rad: Radians = deg.into();
        assert_relative_eq!(rad.0, PI / 2.0, epsilon = 1e-10);

        let rad = Radians(PI);
        let deg: Degrees = rad.into();
        assert_relative_eq!(deg.0, 180.0, epsilon = 1e-10);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Degrees::ZERO.0, 0.0);
        assert_eq!(Degrees::RIGHT.0, 0.0);
        assert_eq!(Degrees::UP.0, 90.0);
        assert_eq!(Degrees::LEFT.0, 180.0);
        assert_eq!(Degrees::DOWN.0, 270.0);
        assert_eq!(Degrees::FULL_CIRCLE.0, 360.0);

        assert_eq!(Radians::ZERO.0, 0.0);
        assert_eq!(Radians::RIGHT.0, 0.0);
        assert_relative_eq!(Radians::UP.0, PI / 2.0, epsilon = 1e-10);
        assert_relative_eq!(Radians::LEFT.0, PI, epsilon = 1e-10);
        assert_relative_eq!(Radians::DOWN.0, 3.0 * PI / 2.0, epsilon = 1e-10);
        assert_relative_eq!(Radians::FULL_CIRCLE.0, 2.0 * PI, epsilon = 1e-10);
    }
}