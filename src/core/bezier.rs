//! Bézier curve utilities for smooth path generation.
//!
//! This module provides quadratic and cubic Bézier curve implementations
//! for creating smooth curves commonly used in computer graphics and animation.
//! Bézier curves are fundamental for creating smooth paths, animations, and
//! complex shapes with mathematical precision.
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::{QuadraticBezier, CubicBezier, Vector2D};
//!
//! let quad = QuadraticBezier::new(
//!     Vector2D::new(0.0, 0.0),
//!     Vector2D::new(1.0, 1.0),
//!     Vector2D::new(2.0, 0.0),
//! );
//!
//! let point = quad.evaluate(0.5);
//! let tangent = quad.tangent(0.5);
//! ```

use crate::core::{BoundingBox, Vector2D};

/// A quadratic Bézier curve defined by three control points.
///
/// Quadratic Bézier curves are defined by the parametric equation:
/// B(t) = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadraticBezier {
    /// Start point
    pub p0: Vector2D,
    /// Control point
    pub p1: Vector2D,
    /// End point
    pub p2: Vector2D,
}

/// A cubic Bézier curve defined by four control points.
///
/// Cubic Bézier curves are defined by the parametric equation:
/// B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezier {
    /// Start point
    pub p0: Vector2D,
    /// First control point
    pub p1: Vector2D,
    /// Second control point
    pub p2: Vector2D,
    /// End point
    pub p3: Vector2D,
}

impl QuadraticBezier {
    /// Creates a new quadratic Bézier curve.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 0.0),
    /// );
    /// ```
    #[inline]
    pub fn new(p0: Vector2D, p1: Vector2D, p2: Vector2D) -> Self {
        Self { p0, p1, p2 }
    }

    /// Evaluates the curve at parameter t ∈ [0, 1].
    ///
    /// Returns the point on the curve at the given parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0),
    ///     Vector2D::new(4.0, 0.0),
    /// );
    ///
    /// let start = curve.evaluate(0.0);
    /// let midpoint = curve.evaluate(0.5);
    /// let end = curve.evaluate(1.0);
    ///
    /// assert_eq!(start, Vector2D::new(0.0, 0.0));
    /// assert_eq!(end, Vector2D::new(4.0, 0.0));
    /// ```
    #[inline]
    pub fn evaluate(&self, t: f64) -> Vector2D {
        let t1 = 1.0 - t;
        let t_squared = t * t;
        let t1_squared = t1 * t1;

        // B(t) = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
        self.p0 * t1_squared + self.p1 * (2.0 * t1 * t) + self.p2 * t_squared
    }

    /// Computes the tangent vector at parameter t ∈ [0, 1].
    ///
    /// The tangent vector indicates the direction of the curve at that point.
    /// Note: This returns an unnormalized vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 0.0),
    /// );
    ///
    /// let tangent = curve.tangent(0.5);
    /// // Tangent should point in the direction of motion
    /// ```
    #[inline]
    pub fn tangent(&self, t: f64) -> Vector2D {
        let t1 = 1.0 - t;

        // B'(t) = 2(1-t)(P₁-P₀) + 2t(P₂-P₁)
        (self.p1 - self.p0) * (2.0 * t1) + (self.p2 - self.p1) * (2.0 * t)
    }

    /// Computes the normal vector at parameter t ∈ [0, 1].
    ///
    /// The normal vector is perpendicular to the tangent vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 0.0),
    /// );
    ///
    /// let normal = curve.normal(0.5);
    /// let tangent = curve.tangent(0.5);
    ///
    /// // Normal should be perpendicular to tangent
    /// assert!((normal.dot(tangent)).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn normal(&self, t: f64) -> Vector2D {
        let tangent = self.tangent(t);
        Vector2D::new(-tangent.y, tangent.x) // Rotate 90 degrees counterclockwise
    }

    /// Returns the start point of the curve.
    #[inline]
    pub fn start(&self) -> Vector2D {
        self.p0
    }

    /// Returns the end point of the curve.
    #[inline]
    pub fn end(&self) -> Vector2D {
        self.p2
    }

    /// Computes an axis-aligned bounding box for the curve.
    ///
    /// This is a conservative bounding box that may be larger than the
    /// actual bounding box of the curve.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0),
    ///     Vector2D::new(4.0, 0.0),
    /// );
    ///
    /// let bbox = curve.bounding_box();
    /// assert!(bbox.contains_point(curve.start()));
    /// assert!(bbox.contains_point(curve.end()));
    /// ```
    pub fn bounding_box(&self) -> BoundingBox {
        // For quadratic Bézier curves, we need to check all control points
        // and potentially evaluate the curve at the point where the derivative is zero
        let mut points = vec![self.p0, self.p1, self.p2];

        // Check if the curve has local extrema
        // For quadratic Bézier: B'(t) = 2(1-t)(P1-P0) + 2t(P2-P1) = 0
        // Solving for t: 2(1-t)(P1-P0) = -2t(P2-P1)
        // (1-t)(P1-P0) = -t(P2-P1)
        // P1-P0 + t(P2-P1) = t(P1-P0)
        // P1-P0 = t(P2-P1 + P1-P0)
        // t = (P1-P0) / (P2-P1 + P1-P0) = (P1-P0) / (P2-P0)

        // Check for extrema in x direction
        let dx = self.p2.x - self.p0.x;
        if dx != 0.0 {
            let t = (self.p1.x - self.p0.x) / dx;
            if (0.0..=1.0).contains(&t) {
                points.push(self.evaluate(t));
            }
        }

        // Check for extrema in y direction
        let dy = self.p2.y - self.p0.y;
        if dy != 0.0 {
            let t = (self.p1.y - self.p0.y) / dy;
            if (0.0..=1.0).contains(&t) {
                points.push(self.evaluate(t));
            }
        }

        BoundingBox::from_points(points).unwrap_or_else(|| BoundingBox::zero())
    }

    /// Splits the curve at parameter t ∈ [0, 1].
    ///
    /// Returns two new curves: one from the start to t, and one from t to the end.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0),
    ///     Vector2D::new(4.0, 0.0),
    /// );
    ///
    /// let (first, second) = curve.split(0.5);
    /// assert_eq!(first.end(), second.start());
    /// ```
    pub fn split(&self, t: f64) -> (QuadraticBezier, QuadraticBezier) {
        // De Casteljau algorithm for quadratic Bézier
        let q0 = self.p0.lerp(self.p1, t);
        let q1 = self.p1.lerp(self.p2, t);
        let r = q0.lerp(q1, t);

        let first = QuadraticBezier::new(self.p0, q0, r);
        let second = QuadraticBezier::new(r, q1, self.p2);

        (first, second)
    }

    /// Estimates the arc length of the curve.
    ///
    /// This uses numerical integration with a fixed number of samples.
    /// For more accuracy, use more samples.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{QuadraticBezier, Vector2D};
    ///
    /// let curve = QuadraticBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 0.0),
    /// );
    ///
    /// let length = curve.arc_length_estimate(100);
    /// // Length should be positive
    /// assert!(length > 0.0);
    /// ```
    pub fn arc_length_estimate(&self, samples: usize) -> f64 {
        let mut length = 0.0;
        let mut prev_point = self.evaluate(0.0);

        for i in 1..=samples {
            let t = i as f64 / samples as f64;
            let point = self.evaluate(t);
            length += (point - prev_point).magnitude();
            prev_point = point;
        }

        length
    }
}

impl CubicBezier {
    /// Creates a new cubic Bézier curve.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{CubicBezier, Vector2D};
    ///
    /// let curve = CubicBezier::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 1.0),
    ///     Vector2D::new(3.0, 0.0),
    /// );
    /// ```
    #[inline]
    pub fn new(p0: Vector2D, p1: Vector2D, p2: Vector2D, p3: Vector2D) -> Self {
        Self { p0, p1, p2, p3 }
    }

    /// Evaluates the curve at parameter t ∈ [0, 1].
    ///
    /// Returns the point on the curve at the given parameter.
    #[inline]
    pub fn evaluate(&self, t: f64) -> Vector2D {
        let t1 = 1.0 - t;
        let t_squared = t * t;
        let t_cubed = t_squared * t;
        let t1_squared = t1 * t1;
        let t1_cubed = t1_squared * t1;

        // B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
        self.p0 * t1_cubed
            + self.p1 * (3.0 * t1_squared * t)
            + self.p2 * (3.0 * t1 * t_squared)
            + self.p3 * t_cubed
    }

    /// Computes the tangent vector at parameter t ∈ [0, 1].
    ///
    /// The tangent vector indicates the direction of the curve at that point.
    #[inline]
    pub fn tangent(&self, t: f64) -> Vector2D {
        let t1 = 1.0 - t;

        // B'(t) = 3(1-t)²(P₁-P₀) + 6(1-t)t(P₂-P₁) + 3t²(P₃-P₂)
        (self.p1 - self.p0) * (3.0 * t1 * t1)
            + (self.p2 - self.p1) * (6.0 * t1 * t)
            + (self.p3 - self.p2) * (3.0 * t * t)
    }

    /// Computes the normal vector at parameter t ∈ [0, 1].
    ///
    /// The normal vector is perpendicular to the tangent vector.
    #[inline]
    pub fn normal(&self, t: f64) -> Vector2D {
        let tangent = self.tangent(t);
        Vector2D::new(-tangent.y, tangent.x) // Rotate 90 degrees counterclockwise
    }

    /// Returns the start point of the curve.
    #[inline]
    pub fn start(&self) -> Vector2D {
        self.p0
    }

    /// Returns the end point of the curve.
    #[inline]
    pub fn end(&self) -> Vector2D {
        self.p3
    }

    /// Computes an axis-aligned bounding box for the curve.
    ///
    /// This is a conservative bounding box that may be larger than the
    /// actual bounding box of the curve.
    pub fn bounding_box(&self) -> BoundingBox {
        // For cubic Bézier curves, we need to check all control points
        // and potentially evaluate at points where derivatives are zero
        let mut points = vec![self.p0, self.p3, self.p1, self.p2];

        // For cubic curves, B'(t) = 3(1-t)²(P1-P0) + 6(1-t)t(P2-P1) + 3t²(P3-P2) = 0
        // This is a quadratic equation: at² + bt + c = 0
        // where a = 3(P0 - 2P1 + P2), b = 2(P1 - P0), c = P0 - P1 (wait, let me recalculate)

        // Actually, let's use the standard formula:
        // B'(t) = -3P0(1-t)² + 3P1(1-t)² - 6P1(1-t)t + 6P2(1-t)t - 3P2 t² + 3P3 t²
        // Simplified: B'(t) = 3(1-t)²(P1-P0) + 6(1-t)t(P2-P1) + 3t²(P3-P2)

        // Set B'(t) = 0 and solve for t. This gives a quadratic equation:
        // A t² + B t + C = 0 where:
        let a_coeff = (self.p0 - self.p1 * 2.0 + self.p2) * 3.0; // 3(P0 - 2P1 + P2)
        let b_coeff = (self.p1 - self.p0) * 6.0; // 6(P1 - P0)
        let c_coeff = (self.p2 - self.p1) * 3.0; // 3(P2 - P1)

        // For each dimension, solve quadratic equation
        for dim in 0..2 {
            let a_val = if dim == 0 { a_coeff.x } else { a_coeff.y };
            let b_val = if dim == 0 { b_coeff.x } else { b_coeff.y };
            let c_val = if dim == 0 { c_coeff.x } else { c_coeff.y };

            if a_val.abs() > 1e-10 {
                // Quadratic formula: t = [-b ± sqrt(b² - 4ac)] / 2a
                let discriminant = b_val * b_val - 4.0 * a_val * c_val;
                if discriminant >= 0.0 {
                    let sqrt_d = discriminant.sqrt();
                    let t1 = (-b_val - sqrt_d) / (2.0 * a_val);
                    let t2 = (-b_val + sqrt_d) / (2.0 * a_val);

                    if (0.0..=1.0).contains(&t1) {
                        points.push(self.evaluate(t1));
                    }
                    if (0.0..=1.0).contains(&t2) {
                        points.push(self.evaluate(t2));
                    }
                }
            } else if b_val.abs() > 1e-10 {
                // Linear case
                let t = -c_val / b_val;
                if (0.0..=1.0).contains(&t) {
                    points.push(self.evaluate(t));
                }
            }
        }

        BoundingBox::from_points(points).unwrap_or_else(|| BoundingBox::zero())
    }

    /// Splits the curve at parameter t ∈ [0, 1].
    ///
    /// Returns two new curves: one from the start to t, and one from t to the end.
    pub fn split(&self, t: f64) -> (CubicBezier, CubicBezier) {
        // De Casteljau algorithm for cubic Bézier
        let q0 = self.p0.lerp(self.p1, t);
        let q1 = self.p1.lerp(self.p2, t);
        let q2 = self.p2.lerp(self.p3, t);

        let r0 = q0.lerp(q1, t);
        let r1 = q1.lerp(q2, t);

        let s = r0.lerp(r1, t);

        let first = CubicBezier::new(self.p0, q0, r0, s);
        let second = CubicBezier::new(s, r1, q2, self.p3);

        (first, second)
    }

    /// Estimates the arc length of the curve.
    ///
    /// This uses numerical integration with a fixed number of samples.
    pub fn arc_length_estimate(&self, samples: usize) -> f64 {
        let mut length = 0.0;
        let mut prev_point = self.evaluate(0.0);

        for i in 1..=samples {
            let t = i as f64 / samples as f64;
            let point = self.evaluate(t);
            length += (point - prev_point).magnitude();
            prev_point = point;
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_quadratic_bezier_creation() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 0.0),
        );

        assert_eq!(bezier.p0, Vector2D::new(0.0, 0.0));
        assert_eq!(bezier.p1, Vector2D::new(1.0, 1.0));
        assert_eq!(bezier.p2, Vector2D::new(2.0, 0.0));
    }

    #[test]
    fn test_cubic_bezier_creation() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 1.0),
            Vector2D::new(3.0, 0.0),
        );

        assert_eq!(bezier.p0, Vector2D::new(0.0, 0.0));
        assert_eq!(bezier.p1, Vector2D::new(1.0, 1.0));
        assert_eq!(bezier.p2, Vector2D::new(2.0, 1.0));
        assert_eq!(bezier.p3, Vector2D::new(3.0, 0.0));
    }

    #[test]
    fn test_quadratic_bezier_evaluate() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        // Test endpoints
        assert_eq!(bezier.evaluate(0.0), Vector2D::new(0.0, 0.0));
        assert_eq!(bezier.evaluate(1.0), Vector2D::new(4.0, 0.0));

        // Test midpoint
        let mid = bezier.evaluate(0.5);
        assert_relative_eq!(mid.x, 2.0, epsilon = 1e-10);
        assert_relative_eq!(mid.y, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_cubic_bezier_evaluate() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(3.0, 1.0),
            Vector2D::new(4.0, 0.0),
        );

        // Test endpoints
        assert_eq!(bezier.evaluate(0.0), Vector2D::new(0.0, 0.0));
        assert_eq!(bezier.evaluate(1.0), Vector2D::new(4.0, 0.0));

        // Test midpoint (should be influenced by control points)
        let mid = bezier.evaluate(0.5);
        assert!(mid.x > 1.5 && mid.x < 2.5); // Should be between control points
        assert!(mid.y > 0.5); // Should be above x-axis due to control points
    }

    #[test]
    fn test_quadratic_bezier_start_end() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(1.0, 2.0),
            Vector2D::new(3.0, 4.0),
            Vector2D::new(5.0, 6.0),
        );

        assert_eq!(bezier.start(), Vector2D::new(1.0, 2.0));
        assert_eq!(bezier.end(), Vector2D::new(5.0, 6.0));
    }

    #[test]
    fn test_cubic_bezier_start_end() {
        let bezier = CubicBezier::new(
            Vector2D::new(1.0, 2.0),
            Vector2D::new(3.0, 4.0),
            Vector2D::new(5.0, 6.0),
            Vector2D::new(7.0, 8.0),
        );

        assert_eq!(bezier.start(), Vector2D::new(1.0, 2.0));
        assert_eq!(bezier.end(), Vector2D::new(7.0, 8.0));
    }

    #[test]
    fn test_quadratic_bezier_tangent() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        let tangent_start = bezier.tangent(0.0);
        let tangent_end = bezier.tangent(1.0);

        // Tangent at start should point towards first control point
        assert!(tangent_start.x > 0.0);
        assert!(tangent_start.y > 0.0);

        // Tangent at end should point towards end from last control point
        assert!(tangent_end.x > 0.0);
        assert!(tangent_end.y < 0.0); // Should be downward
    }

    #[test]
    fn test_quadratic_bezier_normal() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 0.0),
        );

        let tangent = bezier.tangent(0.5);
        let normal = bezier.normal(0.5);

        // Normal should be perpendicular to tangent
        assert_relative_eq!(tangent.dot(normal), 0.0, epsilon = 1e-10);

        // Normal should have same magnitude relationship (both unnormalized)
        assert!(normal.magnitude() > 0.0);
    }

    #[test]
    fn test_cubic_bezier_normal() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(3.0, 1.0),
            Vector2D::new(4.0, 0.0),
        );

        let tangent = bezier.tangent(0.5);
        let normal = bezier.normal(0.5);

        // Normal should be perpendicular to tangent
        assert_relative_eq!(tangent.dot(normal), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_quadratic_bezier_bounding_box() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        let bbox = bezier.bounding_box();

        // Should contain all control points and extrema
        assert!(bbox.contains_point(Vector2D::new(0.0, 0.0)));
        assert!(bbox.contains_point(Vector2D::new(2.0, 2.0)));
        assert!(bbox.contains_point(Vector2D::new(4.0, 0.0)));

        // The curve reaches y=1 at its peak
        assert!(bbox.contains_point(Vector2D::new(2.0, 1.0)));
    }

    #[test]
    fn test_cubic_bezier_bounding_box() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 2.0),
            Vector2D::new(3.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        let bbox = bezier.bounding_box();

        // Should contain all control points
        assert!(bbox.contains_point(Vector2D::new(0.0, 0.0)));
        assert!(bbox.contains_point(Vector2D::new(1.0, 2.0)));
        assert!(bbox.contains_point(Vector2D::new(3.0, 2.0)));
        assert!(bbox.contains_point(Vector2D::new(4.0, 0.0)));
    }

    #[test]
    fn test_quadratic_bezier_split() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        let (first, second) = bezier.split(0.5);

        // First curve should start at original start
        assert_eq!(first.start(), bezier.start());
        // Second curve should end at original end
        assert_eq!(second.end(), bezier.end());
        // They should meet at the split point
        assert_eq!(first.end(), second.start());

        // The split point should be the midpoint of the original curve
        let midpoint = bezier.evaluate(0.5);
        assert_eq!(first.end(), midpoint);
        assert_eq!(second.start(), midpoint);
    }

    #[test]
    fn test_cubic_bezier_split() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(3.0, 1.0),
            Vector2D::new(4.0, 0.0),
        );

        let (first, second) = bezier.split(0.5);

        // First curve should start at original start
        assert_eq!(first.start(), bezier.start());
        // Second curve should end at original end
        assert_eq!(second.end(), bezier.end());
        // They should meet at the split point
        assert_eq!(first.end(), second.start());

        // The split point should be the midpoint of the original curve
        let midpoint = bezier.evaluate(0.5);
        assert_eq!(first.end(), midpoint);
        assert_eq!(second.start(), midpoint);
    }

    #[test]
    fn test_quadratic_bezier_arc_length() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 0.0),
        );

        let length = bezier.arc_length_estimate(100);
        assert!(length > 0.0);

        // Straight line case
        let straight = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 0.0),
            Vector2D::new(2.0, 0.0),
        );
        let straight_length = straight.arc_length_estimate(100);
        assert_relative_eq!(straight_length, 2.0, epsilon = 0.01);
    }

    #[test]
    fn test_cubic_bezier_arc_length() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(3.0, 1.0),
            Vector2D::new(4.0, 0.0),
        );

        let length = bezier.arc_length_estimate(100);
        assert!(length > 0.0);

        // Straight line case
        let straight = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 0.0),
            Vector2D::new(2.0, 0.0),
            Vector2D::new(3.0, 0.0),
        );
        let straight_length = straight.arc_length_estimate(100);
        assert_relative_eq!(straight_length, 3.0, epsilon = 0.01);
    }

    #[test]
    fn test_quadratic_bezier_split_endpoints() {
        let bezier = QuadraticBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(4.0, 0.0),
        );

        // Split at start
        let (first, second) = bezier.split(0.0);
        assert_eq!(first.start(), bezier.start());
        assert_eq!(first.end(), bezier.start());
        assert_eq!(second.start(), bezier.start());
        assert_eq!(second.end(), bezier.end());

        // Split at end
        let (first, second) = bezier.split(1.0);
        assert_eq!(first.start(), bezier.start());
        assert_eq!(first.end(), bezier.end());
        assert_eq!(second.start(), bezier.end());
        assert_eq!(second.end(), bezier.end());
    }

    #[test]
    fn test_cubic_bezier_split_endpoints() {
        let bezier = CubicBezier::new(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(3.0, 1.0),
            Vector2D::new(4.0, 0.0),
        );

        // Split at start
        let (first, second) = bezier.split(0.0);
        assert_eq!(first.start(), bezier.start());
        assert_eq!(first.end(), bezier.start());
        assert_eq!(second.start(), bezier.start());
        assert_eq!(second.end(), bezier.end());

        // Split at end
        let (first, second) = bezier.split(1.0);
        assert_eq!(first.start(), bezier.start());
        assert_eq!(first.end(), bezier.end());
        assert_eq!(second.start(), bezier.end());
        assert_eq!(second.end(), bezier.end());
    }
}