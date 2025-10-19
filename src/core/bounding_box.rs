//! Axis-aligned bounding boxes for spatial queries and collision detection.
//!
//! This module provides efficient bounding box operations for 2D geometric queries,
//! spatial partitioning, and collision detection. All operations are designed to be
//! zero-cost where possible.
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::{BoundingBox, Vector2D};
//!
//! let points = vec![
//!     Vector2D::new(0.0, 0.0),
//!     Vector2D::new(2.0, 3.0),
//!     Vector2D::new(-1.0, 1.0),
//! ];
//! let bbox = BoundingBox::from_points(points).unwrap();
//!
//! assert_eq!(bbox.min(), Vector2D::new(-1.0, 0.0));
//! assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
//! assert!(bbox.contains_point(Vector2D::new(0.0, 1.0)));
//! ```

use crate::core::Vector2D;
use std::fmt;

/// An axis-aligned bounding box in 2D space.
///
/// Bounding boxes are defined by their minimum and maximum corners.
/// They are commonly used for:
/// - Spatial queries and collision detection
/// - View frustum culling
/// - Efficient rendering bounds calculation
///
/// # Performance
///
/// All operations are O(1) and designed for SIMD optimization where beneficial.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBox {
    /// The minimum corner (bottom-left in typical coordinate systems)
    pub min: Vector2D,
    /// The maximum corner (top-right in typical coordinate systems)
    pub max: Vector2D,
}

impl BoundingBox {
    /// Creates a new bounding box from minimum and maximum corners.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum corner coordinates
    /// * `max` - The maximum corner coordinates
    ///
    /// # Panics
    ///
    /// Panics if `min.x > max.x` or `min.y > max.y`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 3.0)
    /// );
    /// ```
    #[inline]
    pub fn new(min: Vector2D, max: Vector2D) -> Self {
        assert!(min.x <= max.x, "min.x must be <= max.x");
        assert!(min.y <= max.y, "min.y must be <= max.y");

        Self { min, max }
    }

    /// Creates a bounding box from a set of points.
    ///
    /// Returns the smallest axis-aligned bounding box that contains all points.
    ///
    /// # Arguments
    ///
    /// * `points` - Iterator of points to bound
    ///
    /// # Returns
    ///
    /// `None` if the points iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let points = vec![
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 3.0),
    ///     Vector2D::new(-1.0, 1.0),
    /// ];
    ///
    /// let bbox = BoundingBox::from_points(points).unwrap();
    /// assert_eq!(bbox.min(), Vector2D::new(-1.0, 0.0));
    /// assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
    /// ```
    #[inline]
    pub fn from_points<I>(points: I) -> Option<Self>
    where
        I: IntoIterator<Item = Vector2D>,
    {
        let mut iter = points.into_iter();
        let first = iter.next()?;

        let mut min = first;
        let mut max = first;

        for point in iter {
            min = min.min_components(point);
            max = max.max_components(point);
        }

        Some(Self::new(min, max))
    }

    /// Creates an empty bounding box centered at the origin with zero size.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::zero();
    /// assert_eq!(bbox.min(), Vector2D::zero());
    /// assert_eq!(bbox.max(), Vector2D::zero());
    /// assert!(bbox.is_empty());
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Self {
            min: Vector2D::zero(),
            max: Vector2D::zero(),
        }
    }

    /// Creates an infinite bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::infinite();
    /// assert!(bbox.contains_point(Vector2D::new(f64::INFINITY, f64::NEG_INFINITY)));
    /// ```
    #[inline]
    pub fn infinite() -> Self {
        Self {
            min: Vector2D::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: Vector2D::new(f64::INFINITY, f64::INFINITY),
        }
    }

    /// Returns the minimum corner of the bounding box.
    #[inline]
    pub fn min(&self) -> Vector2D {
        self.min
    }

    /// Returns the maximum corner of the bounding box.
    #[inline]
    pub fn max(&self) -> Vector2D {
        self.max
    }

    /// Returns the width of the bounding box.
    #[inline]
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Returns the height of the bounding box.
    #[inline]
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    /// Returns the size (width, height) of the bounding box.
    #[inline]
    pub fn size(&self) -> Vector2D {
        Vector2D::new(self.width(), self.height())
    }

    /// Returns the center point of the bounding box.
    #[inline]
    pub fn center(&self) -> Vector2D {
        (self.min + self.max) * 0.5
    }

    /// Returns the area of the bounding box.
    #[inline]
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Returns the perimeter of the bounding box.
    #[inline]
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width() + self.height())
    }

    /// Checks if the bounding box is empty (has zero area).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width() == 0.0 && self.height() == 0.0
    }

    /// Checks if the bounding box contains a point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to test
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    ///
    /// assert!(bbox.contains_point(Vector2D::new(1.0, 1.0)));
    /// assert!(!bbox.contains_point(Vector2D::new(3.0, 1.0)));
    /// ```
    #[inline]
    pub fn contains_point(&self, point: Vector2D) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// Checks if this bounding box completely contains another.
    ///
    /// # Arguments
    ///
    /// * `other` - The bounding box to test
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let outer = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(4.0, 4.0)
    /// );
    /// let inner = BoundingBox::new(
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    ///
    /// assert!(outer.contains_bbox(&inner));
    /// ```
    #[inline]
    pub fn contains_bbox(&self, other: &BoundingBox) -> bool {
        other.min.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.max.y <= self.max.y
    }

    /// Checks if this bounding box intersects with another.
    ///
    /// # Arguments
    ///
    /// * `other` - The bounding box to test for intersection
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox1 = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    /// let bbox2 = BoundingBox::new(
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(3.0, 3.0)
    /// );
    ///
    /// assert!(bbox1.intersects(&bbox2));
    /// ```
    #[inline]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Computes the intersection of this bounding box with another.
    ///
    /// Returns `None` if the bounding boxes don't intersect.
    ///
    /// # Arguments
    ///
    /// * `other` - The bounding box to intersect with
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox1 = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    /// let bbox2 = BoundingBox::new(
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(3.0, 3.0)
    /// );
    ///
    /// let intersection = bbox1.intersection(&bbox2).unwrap();
    /// assert_eq!(intersection.min(), Vector2D::new(1.0, 1.0));
    /// assert_eq!(intersection.max(), Vector2D::new(2.0, 2.0));
    /// ```
    #[inline]
    pub fn intersection(&self, other: &BoundingBox) -> Option<BoundingBox> {
        let min_x = self.min.x.max(other.min.x);
        let min_y = self.min.y.max(other.min.y);
        let max_x = self.max.x.min(other.max.x);
        let max_y = self.max.y.min(other.max.y);

        if min_x <= max_x && min_y <= max_y {
            Some(BoundingBox::new(
                Vector2D::new(min_x, min_y),
                Vector2D::new(max_x, max_y),
            ))
        } else {
            None
        }
    }

    /// Computes the union of this bounding box with another.
    ///
    /// # Arguments
    ///
    /// * `other` - The bounding box to union with
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox1 = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    /// let bbox2 = BoundingBox::new(
    ///     Vector2D::new(1.0, 1.0),
    ///     Vector2D::new(3.0, 3.0)
    /// );
    ///
    /// let union = bbox1.union(&bbox2);
    /// assert_eq!(union.min(), Vector2D::new(0.0, 0.0));
    /// assert_eq!(union.max(), Vector2D::new(3.0, 3.0));
    /// ```
    #[inline]
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            Vector2D::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            Vector2D::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        )
    }

    /// Expands the bounding box to include a point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to include
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let mut bbox = BoundingBox::zero();
    /// bbox.expand_to_include(Vector2D::new(2.0, 3.0));
    /// bbox.expand_to_include(Vector2D::new(-1.0, -1.0));
    ///
    /// assert_eq!(bbox.min(), Vector2D::new(-1.0, -1.0));
    /// assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
    /// ```
    #[inline]
    pub fn expand_to_include(&mut self, point: Vector2D) {
        self.min = self.min.min_components(point);
        self.max = self.max.max_components(point);
    }

    /// Expands the bounding box by a margin in all directions.
    ///
    /// # Arguments
    ///
    /// * `margin` - The amount to expand in each direction
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    ///
    /// let expanded = bbox.expand_by_margin(1.0);
    /// assert_eq!(expanded.min(), Vector2D::new(-1.0, -1.0));
    /// assert_eq!(expanded.max(), Vector2D::new(3.0, 3.0));
    /// ```
    #[inline]
    pub fn expand_by_margin(&self, margin: f64) -> BoundingBox {
        BoundingBox::new(
            self.min - Vector2D::splat(margin),
            self.max + Vector2D::splat(margin),
        )
    }

    /// Translates the bounding box by a vector.
    ///
    /// # Arguments
    ///
    /// * `translation` - The translation vector
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    ///
    /// let translated = bbox.translate(Vector2D::new(1.0, -1.0));
    /// assert_eq!(translated.min(), Vector2D::new(1.0, -1.0));
    /// assert_eq!(translated.max(), Vector2D::new(3.0, 1.0));
    /// ```
    #[inline]
    pub fn translate(&self, translation: Vector2D) -> BoundingBox {
        BoundingBox::new(self.min + translation, self.max + translation)
    }

    /// Scales the bounding box around its center.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scale factor (applied to both dimensions)
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{BoundingBox, Vector2D};
    ///
    /// let bbox = BoundingBox::new(
    ///     Vector2D::new(0.0, 0.0),
    ///     Vector2D::new(2.0, 2.0)
    /// );
    ///
    /// let scaled = bbox.scale(2.0);
    /// assert_eq!(scaled.center(), Vector2D::new(1.0, 1.0)); // center unchanged
    /// assert_eq!(scaled.size(), Vector2D::new(4.0, 4.0)); // size doubled
    /// ```
    #[inline]
    pub fn scale(&self, scale: f64) -> BoundingBox {
        let center = self.center();
        let half_size = self.size() * 0.5 * scale;

        BoundingBox::new(center - half_size, center + half_size)
    }
}

impl Default for BoundingBox {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoundingBox(min: {}, max: {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 1.0), Vector2D::new(2.0, 3.0));

        assert_eq!(bbox.min(), Vector2D::new(0.0, 1.0));
        assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
    }

    #[test]
    #[should_panic(expected = "min.x must be <= max.x")]
    fn test_new_invalid_x() {
        BoundingBox::new(Vector2D::new(2.0, 0.0), Vector2D::new(0.0, 1.0));
    }

    #[test]
    #[should_panic(expected = "min.y must be <= max.y")]
    fn test_new_invalid_y() {
        BoundingBox::new(Vector2D::new(0.0, 2.0), Vector2D::new(1.0, 0.0));
    }

    #[test]
    fn test_from_points() {
        let points = vec![
            Vector2D::new(0.0, 0.0),
            Vector2D::new(2.0, 3.0),
            Vector2D::new(-1.0, 1.0),
        ];

        let bbox = BoundingBox::from_points(points).unwrap();

        assert_eq!(bbox.min(), Vector2D::new(-1.0, 0.0));
        assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
    }

    #[test]
    fn test_from_points_empty() {
        let bbox = BoundingBox::from_points(Vec::<Vector2D>::new());
        assert!(bbox.is_none());
    }

    #[test]
    fn test_from_points_single_point() {
        let bbox = BoundingBox::from_points(vec![Vector2D::new(1.0, 2.0)]).unwrap();
        assert_eq!(bbox.min(), Vector2D::new(1.0, 2.0));
        assert_eq!(bbox.max(), Vector2D::new(1.0, 2.0));
    }

    #[test]
    fn test_zero() {
        let bbox = BoundingBox::zero();
        assert_eq!(bbox.min(), Vector2D::zero());
        assert_eq!(bbox.max(), Vector2D::zero());
        assert!(bbox.is_empty());
        assert_eq!(bbox.area(), 0.0);
    }

    #[test]
    fn test_infinite() {
        let bbox = BoundingBox::infinite();
        assert!(bbox.contains_point(Vector2D::new(f64::INFINITY, f64::NEG_INFINITY)));
    }

    #[test]
    fn test_dimensions() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 1.0), Vector2D::new(3.0, 4.0));

        assert_eq!(bbox.width(), 3.0);
        assert_eq!(bbox.height(), 3.0);
        assert_eq!(bbox.size(), Vector2D::new(3.0, 3.0));
        assert_eq!(bbox.center(), Vector2D::new(1.5, 2.5));
        assert_eq!(bbox.area(), 9.0);
        assert_eq!(bbox.perimeter(), 12.0);
    }

    #[test]
    fn test_contains_point() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));

        assert!(bbox.contains_point(Vector2D::new(0.0, 0.0))); // boundary
        assert!(bbox.contains_point(Vector2D::new(1.0, 1.0))); // interior
        assert!(bbox.contains_point(Vector2D::new(2.0, 2.0))); // boundary

        assert!(!bbox.contains_point(Vector2D::new(-0.1, 1.0))); // outside
        assert!(!bbox.contains_point(Vector2D::new(1.0, -0.1))); // outside
        assert!(!bbox.contains_point(Vector2D::new(2.1, 1.0))); // outside
        assert!(!bbox.contains_point(Vector2D::new(1.0, 2.1))); // outside
    }

    #[test]
    fn test_contains_bbox() {
        let outer = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(4.0, 4.0));
        let inner = BoundingBox::new(Vector2D::new(1.0, 1.0), Vector2D::new(2.0, 2.0));
        let overlapping = BoundingBox::new(Vector2D::new(1.0, 1.0), Vector2D::new(5.0, 5.0));

        assert!(outer.contains_bbox(&inner));
        assert!(!inner.contains_bbox(&outer));
        assert!(!outer.contains_bbox(&overlapping));
    }

    #[test]
    fn test_intersects() {
        let bbox1 = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));
        let bbox2 = BoundingBox::new(Vector2D::new(1.0, 1.0), Vector2D::new(3.0, 3.0));
        let bbox3 = BoundingBox::new(Vector2D::new(3.0, 3.0), Vector2D::new(4.0, 4.0));

        assert!(bbox1.intersects(&bbox2));
        assert!(!bbox1.intersects(&bbox3));
    }

    #[test]
    fn test_intersection() {
        let bbox1 = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));
        let bbox2 = BoundingBox::new(Vector2D::new(1.0, 1.0), Vector2D::new(3.0, 3.0));

        let intersection = bbox1.intersection(&bbox2).unwrap();
        assert_eq!(intersection.min(), Vector2D::new(1.0, 1.0));
        assert_eq!(intersection.max(), Vector2D::new(2.0, 2.0));

        let bbox3 = BoundingBox::new(Vector2D::new(3.0, 3.0), Vector2D::new(4.0, 4.0));

        assert!(bbox1.intersection(&bbox3).is_none());
    }

    #[test]
    fn test_union() {
        let bbox1 = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));
        let bbox2 = BoundingBox::new(Vector2D::new(1.0, 1.0), Vector2D::new(3.0, 3.0));

        let union = bbox1.union(&bbox2);
        assert_eq!(union.min(), Vector2D::new(0.0, 0.0));
        assert_eq!(union.max(), Vector2D::new(3.0, 3.0));
    }

    #[test]
    fn test_expand_to_include() {
        let mut bbox = BoundingBox::zero();
        bbox.expand_to_include(Vector2D::new(2.0, 3.0));
        bbox.expand_to_include(Vector2D::new(-1.0, -1.0));

        assert_eq!(bbox.min(), Vector2D::new(-1.0, -1.0));
        assert_eq!(bbox.max(), Vector2D::new(2.0, 3.0));
    }

    #[test]
    fn test_expand_by_margin() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));

        let expanded = bbox.expand_by_margin(1.0);
        assert_eq!(expanded.min(), Vector2D::new(-1.0, -1.0));
        assert_eq!(expanded.max(), Vector2D::new(3.0, 3.0));
    }

    #[test]
    fn test_translate() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));

        let translated = bbox.translate(Vector2D::new(1.0, -1.0));
        assert_eq!(translated.min(), Vector2D::new(1.0, -1.0));
        assert_eq!(translated.max(), Vector2D::new(3.0, 1.0));
    }

    #[test]
    fn test_scale() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));

        let scaled = bbox.scale(2.0);
        assert_eq!(scaled.center(), Vector2D::new(1.0, 1.0)); // center unchanged
        assert_eq!(scaled.size(), Vector2D::new(4.0, 4.0)); // size doubled
    }

    #[test]
    fn test_display() {
        let bbox = BoundingBox::new(Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));

        let display = format!("{}", bbox);
        assert!(display.contains("BoundingBox"));
        assert!(display.contains("min: (0"));
        assert!(display.contains("max: (2"));
    }

    #[test]
    fn test_default() {
        let bbox = BoundingBox::default();
        assert_eq!(bbox, BoundingBox::zero());
    }
}
