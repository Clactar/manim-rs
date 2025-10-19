use super::Vector2D;

/// A 2D transformation matrix (3x3 for affine transformations).
///
/// Represents transformations like translation, rotation, scaling, and shearing.
/// Uses column-major order for compatibility with graphics libraries.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{Transform, Vector2D};
///
/// let translation = Transform::translate(5.0, 3.0);
/// let rotation = Transform::rotate(std::f64::consts::PI / 2.0);
/// let combined = translation * rotation;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    // Column-major 3x3 matrix for 2D affine transformations
    // [m11 m12 m13]   [a  c  tx]
    // [m21 m22 m23] = [b  d  ty]
    // [m31 m32 m33]   [0  0  1 ]
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Transform {
    /// Creates an identity transformation (no change).
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Transform;
    ///
    /// let identity = Transform::identity();
    /// ```
    #[inline]
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Creates a translation transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Transform;
    ///
    /// let translate = Transform::translate(5.0, 3.0);
    /// ```
    #[inline]
    pub const fn translate(x: f64, y: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: x,
            ty: y,
        }
    }

    /// Creates a rotation transformation around the origin.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle in radians (counterclockwise)
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Transform;
    ///
    /// let rotate_90 = Transform::rotate(std::f64::consts::PI / 2.0);
    /// ```
    #[inline]
    pub fn rotate(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Creates a uniform scaling transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Transform;
    ///
    /// let scale_2x = Transform::scale(2.0, 2.0);
    /// ```
    #[inline]
    pub const fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Applies the transformation to a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{Transform, Vector2D};
    ///
    /// let t = Transform::translate(5.0, 3.0);
    /// let v = Vector2D::new(1.0, 2.0);
    /// let result = t.apply(v);
    /// assert_eq!(result, Vector2D::new(6.0, 5.0));
    /// ```
    #[inline]
    pub fn apply(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(
            self.a * v.x + self.c * v.y + self.tx,
            self.b * v.x + self.d * v.y + self.ty,
        )
    }
}

// Matrix multiplication for combining transformations
impl std::ops::Mul for Transform {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::identity();
        let v = Vector2D::new(3.0, 4.0);
        let result = t.apply(v);
        assert_eq!(result, v);
    }

    #[test]
    fn test_translation() {
        let t = Transform::translate(5.0, 3.0);
        let v = Vector2D::new(1.0, 2.0);
        let result = t.apply(v);
        assert_eq!(result, Vector2D::new(6.0, 5.0));
    }

    #[test]
    fn test_scaling() {
        let t = Transform::scale(2.0, 3.0);
        let v = Vector2D::new(4.0, 5.0);
        let result = t.apply(v);
        assert_eq!(result, Vector2D::new(8.0, 15.0));
    }

    #[test]
    fn test_rotation_90() {
        let t = Transform::rotate(std::f64::consts::PI / 2.0);
        let v = Vector2D::new(1.0, 0.0);
        let result = t.apply(v);
        assert!((result.x - 0.0).abs() < 1e-10);
        assert!((result.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_combined_transforms() {
        let translate = Transform::translate(5.0, 0.0);
        let rotate = Transform::rotate(std::f64::consts::PI / 2.0);
        let combined = translate * rotate;
        
        let v = Vector2D::new(1.0, 0.0);
        let result = combined.apply(v);
        
        assert!((result.x - 5.0).abs() < 1e-10);
        assert!((result.y - 1.0).abs() < 1e-10);
    }
}

