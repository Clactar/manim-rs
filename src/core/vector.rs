/// A 2D vector in Euclidean space.
///
/// This type is optimized for performance with inline operations
/// and SIMD-friendly memory layout.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
///
/// let v1 = Vector2D::new(3.0, 4.0);
/// let v2 = Vector2D::new(1.0, 2.0);
///
/// let sum = v1 + v2;
/// assert_eq!(sum.x, 4.0);
/// assert_eq!(sum.y, 6.0);
///
/// let magnitude = v1.magnitude();
/// assert!((magnitude - 5.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    /// Creates a new vector with the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v = Vector2D::new(1.0, 2.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// ```
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// The zero vector (0, 0).
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Unit vector pointing right (1, 0).
    pub const RIGHT: Self = Self::new(1.0, 0.0);

    /// Unit vector pointing up (0, 1).
    pub const UP: Self = Self::new(0.0, 1.0);

    /// Unit vector pointing left (-1, 0).
    pub const LEFT: Self = Self::new(-1.0, 0.0);

    /// Unit vector pointing down (0, -1).
    pub const DOWN: Self = Self::new(0.0, -1.0);

    /// Calculates the magnitude (length) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v = Vector2D::new(3.0, 4.0);
    /// assert!((v.magnitude() - 5.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates the squared magnitude of the vector.
    ///
    /// This is faster than `magnitude()` as it avoids the square root.
    /// Useful for distance comparisons.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v = Vector2D::new(3.0, 4.0);
    /// assert_eq!(v.magnitude_squared(), 25.0);
    /// ```
    #[inline]
    pub fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let zero = Vector2D::zero();
    /// assert_eq!(zero, Vector2D::ZERO);
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Self::ZERO
    }

    /// Creates a vector with the same value in all components.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v = Vector2D::splat(2.5);
    /// assert_eq!(v, Vector2D::new(2.5, 2.5));
    /// ```
    #[inline]
    pub fn splat(value: f64) -> Self {
        Self::new(value, value)
    }

    /// Returns a new vector with the minimum components of this vector and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v1 = Vector2D::new(1.0, 3.0);
    /// let v2 = Vector2D::new(2.0, 2.0);
    /// let min = v1.min_components(v2);
    /// assert_eq!(min, Vector2D::new(1.0, 2.0));
    /// ```
    #[inline]
    pub fn min_components(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns a new vector with the maximum components of this vector and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v1 = Vector2D::new(1.0, 3.0);
    /// let v2 = Vector2D::new(2.0, 2.0);
    /// let max = v1.max_components(v2);
    /// assert_eq!(max, Vector2D::new(2.0, 3.0));
    /// ```
    #[inline]
    pub fn max_components(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Returns a normalized (unit length) version of the vector.
    ///
    /// Returns `None` if the vector has zero length.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v = Vector2D::new(3.0, 4.0);
    /// let normalized = v.normalize().unwrap();
    /// assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn normalize(self) -> Option<Self> {
        let mag = self.magnitude();
        if mag > 0.0 {
            Some(Self::new(self.x / mag, self.y / mag))
        } else {
            None
        }
    }

    /// Calculates the dot product with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v1 = Vector2D::new(1.0, 2.0);
    /// let v2 = Vector2D::new(3.0, 4.0);
    /// assert_eq!(v1.dot(v2), 11.0);
    /// ```
    #[inline]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the 2D cross product (scalar).
    ///
    /// Returns the z-component of the 3D cross product when both
    /// vectors are treated as 3D vectors with z=0.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v1 = Vector2D::new(1.0, 0.0);
    /// let v2 = Vector2D::new(0.0, 1.0);
    /// assert_eq!(v1.cross(v2), 1.0);
    /// ```
    #[inline]
    pub fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Linearly interpolates between this vector and another.
    ///
    /// # Arguments
    ///
    /// * `other` - The target vector
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    ///
    /// let v1 = Vector2D::new(0.0, 0.0);
    /// let v2 = Vector2D::new(10.0, 10.0);
    /// let mid = v1.lerp(v2, 0.5);
    /// assert_eq!(mid, Vector2D::new(5.0, 5.0));
    /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }
}

// Operator overloads
impl std::ops::Add for Vector2D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Vector2D {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<f64> for Vector2D {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::Neg for Vector2D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl std::fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector2D::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector2D::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize() {
        let v = Vector2D::new(3.0, 4.0);
        let n = v.normalize().unwrap();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
        assert!((n.x - 0.6).abs() < 1e-10);
        assert!((n.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_zero() {
        let v = Vector2D::ZERO;
        assert!(v.normalize().is_none());
    }

    #[test]
    fn test_addition() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result, Vector2D::new(4.0, 6.0));
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vector2D::new(5.0, 7.0);
        let v2 = Vector2D::new(2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result, Vector2D::new(3.0, 4.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vector2D::new(2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result, Vector2D::new(4.0, 6.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        assert_eq!(v1.dot(v2), 11.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vector2D::new(1.0, 0.0);
        let v2 = Vector2D::new(0.0, 1.0);
        assert_eq!(v1.cross(v2), 1.0);
    }

    #[test]
    fn test_lerp() {
        let v1 = Vector2D::new(0.0, 0.0);
        let v2 = Vector2D::new(10.0, 10.0);
        let mid = v1.lerp(v2, 0.5);
        assert_eq!(mid, Vector2D::new(5.0, 5.0));
    }
}
