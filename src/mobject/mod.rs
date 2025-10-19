//! Mathematical objects and shapes.
//!
//! This module provides the core abstractions for drawable objects in manim-rs:
//! - [`Mobject`] - The fundamental trait for all drawable objects
//! - [`VMobject`] - Vector-based mobject implementation
//! - [`MobjectGroup`] - Container for hierarchical object composition
//!
//! # Overview
//!
//! Mobjects (mathematical objects) are the building blocks of animations. They
//! encapsulate geometry, styling, and transformations, providing a unified
//! interface for rendering and animation.
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::mobject::Mobject;
//! use manim_rs::renderer::Path;
//!
//! // Mobjects can be rendered, transformed, and composed
//! // See VMobject and geometry submodules for concrete implementations
//! ```

use crate::core::{BoundingBox, Result, Transform, Vector2D};
use crate::renderer::Renderer;

mod bezier_path;
pub mod geometry;
mod group;
mod vmobject;

pub use bezier_path::BezierPath;
pub use group::MobjectGroup;
pub use vmobject::VMobject;

/// Core trait for all mathematical objects that can be rendered and animated.
///
/// [`Mobject`] defines the essential interface that all drawable objects must
/// implement. It provides methods for rendering, spatial queries, transformations,
/// and property manipulation.
///
/// # Object Safety
///
/// This trait is object-safe, allowing for heterogeneous collections via
/// `Box<dyn Mobject>`. The [`clone_mobject`](Mobject::clone_mobject) method
/// works around Rust's orphan rules for trait object cloning.
///
/// # Thread Safety
///
/// Mobjects must implement `Send + Sync` to support parallel rendering pipelines.
///
/// # Examples
///
/// ```
/// use manim_rs::core::{BoundingBox, Result, Transform, Vector2D};
/// use manim_rs::mobject::Mobject;
/// use manim_rs::renderer::{Path, Renderer};
///
/// // Implement Mobject for a custom type
/// struct CustomShape {
///     position: Vector2D,
///     opacity: f64,
/// }
///
/// impl Mobject for CustomShape {
///     fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
///         // Custom rendering logic
///         Ok(())
///     }
///
///     fn bounding_box(&self) -> BoundingBox {
///         BoundingBox::zero()
///     }
///
///     fn apply_transform(&mut self, transform: &Transform) {
///         self.position = transform.apply(self.position);
///     }
///
///     fn position(&self) -> Vector2D {
///         self.position
///     }
///
///     fn set_position(&mut self, pos: Vector2D) {
///         self.position = pos;
///     }
///
///     fn opacity(&self) -> f64 {
///         self.opacity
///     }
///
///     fn set_opacity(&mut self, opacity: f64) {
///         self.opacity = opacity.clamp(0.0, 1.0);
///     }
///
///     fn clone_mobject(&self) -> Box<dyn Mobject> {
///         Box::new(CustomShape {
///             position: self.position,
///             opacity: self.opacity,
///         })
///     }
/// }
/// ```
pub trait Mobject: Send + Sync {
    /// Renders the mobject using the provided renderer.
    ///
    /// This method is called during the rendering phase to draw the mobject
    /// to the current frame. Implementations should use the renderer's
    /// [`draw_path`](Renderer::draw_path) and
    /// [`draw_text`](Renderer::draw_text) methods.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails (e.g., invalid geometry, backend error).
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()>;

    /// Returns the axis-aligned bounding box of the mobject.
    ///
    /// The bounding box is used for spatial queries, culling, and layout
    /// calculations. Implementations should cache this value when possible.
    fn bounding_box(&self) -> BoundingBox;

    /// Applies a transformation to the mobject in-place.
    ///
    /// This modifies the mobject's geometry according to the given transform
    /// (translation, rotation, scaling, etc.). Implementations should update
    /// all relevant state, including cached bounding boxes.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::{Transform, Vector2D};
    /// use manim_rs::mobject::{Mobject, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut mobject = VMobject::new(Path::new());
    /// let transform = Transform::translate(1.0, 2.0);
    /// mobject.apply_transform(&transform);
    /// ```
    fn apply_transform(&mut self, transform: &Transform);

    /// Returns the current position of the mobject.
    ///
    /// The position typically represents the center or anchor point of the object.
    fn position(&self) -> Vector2D;

    /// Sets the position of the mobject.
    ///
    /// This moves the mobject to the specified position, typically by applying
    /// a translation relative to its current position.
    fn set_position(&mut self, pos: Vector2D);

    /// Returns the current opacity of the mobject.
    ///
    /// Opacity ranges from 0.0 (fully transparent) to 1.0 (fully opaque).
    fn opacity(&self) -> f64;

    /// Sets the opacity of the mobject.
    ///
    /// Values outside [0.0, 1.0] should be clamped by implementations.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{Mobject, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut mobject = VMobject::new(Path::new());
    /// mobject.set_opacity(0.5);
    /// assert_eq!(mobject.opacity(), 0.5);
    /// ```
    fn set_opacity(&mut self, opacity: f64);

    /// Creates a boxed clone of the mobject.
    ///
    /// This method enables cloning through trait objects. Implementations
    /// should return `Box::new(self.clone())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{Mobject, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mobject: Box<dyn Mobject> = Box::new(VMobject::new(Path::new()));
    /// let cloned = mobject.clone_mobject();
    /// ```
    fn clone_mobject(&self) -> Box<dyn Mobject>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Color;
    use crate::renderer::{Path, PathStyle, TextStyle};

    /// Mock mobject for testing the trait interface
    #[derive(Clone)]
    struct MockMobject {
        position: Vector2D,
        opacity: f64,
    }

    impl MockMobject {
        fn new() -> Self {
            Self {
                position: Vector2D::ZERO,
                opacity: 1.0,
            }
        }
    }

    impl Mobject for MockMobject {
        fn render(&self, _renderer: &mut dyn Renderer) -> Result<()> {
            Ok(())
        }

        fn bounding_box(&self) -> BoundingBox {
            BoundingBox::zero()
        }

        fn apply_transform(&mut self, transform: &Transform) {
            self.position = transform.apply(self.position);
        }

        fn position(&self) -> Vector2D {
            self.position
        }

        fn set_position(&mut self, pos: Vector2D) {
            self.position = pos;
        }

        fn opacity(&self) -> f64 {
            self.opacity
        }

        fn set_opacity(&mut self, opacity: f64) {
            self.opacity = opacity.clamp(0.0, 1.0);
        }

        fn clone_mobject(&self) -> Box<dyn Mobject> {
            Box::new(self.clone())
        }
    }

    /// Test renderer for checking render calls
    struct TestRenderer {
        render_count: usize,
    }

    impl TestRenderer {
        fn new() -> Self {
            Self { render_count: 0 }
        }
    }

    impl Renderer for TestRenderer {
        fn clear(&mut self, _color: Color) -> Result<()> {
            Ok(())
        }

        fn draw_path(&mut self, _path: &Path, _style: &PathStyle) -> Result<()> {
            self.render_count += 1;
            Ok(())
        }

        fn draw_text(
            &mut self,
            _text: &str,
            _position: Vector2D,
            _style: &TextStyle,
        ) -> Result<()> {
            self.render_count += 1;
            Ok(())
        }

        fn dimensions(&self) -> (u32, u32) {
            (800, 600)
        }
    }

    #[test]
    fn test_mobject_trait_is_object_safe() {
        // This test verifies that we can create trait objects
        let mobject: Box<dyn Mobject> = Box::new(MockMobject::new());
        assert_eq!(mobject.position(), Vector2D::ZERO);
    }

    #[test]
    fn test_mobject_clone() {
        let mobject = MockMobject::new();
        let boxed: Box<dyn Mobject> = Box::new(mobject);
        let cloned = boxed.clone_mobject();

        assert_eq!(cloned.position(), Vector2D::ZERO);
        assert_eq!(cloned.opacity(), 1.0);
    }

    #[test]
    fn test_mobject_position() {
        let mut mobject = MockMobject::new();
        assert_eq!(mobject.position(), Vector2D::ZERO);

        mobject.set_position(Vector2D::new(1.0, 2.0));
        assert_eq!(mobject.position(), Vector2D::new(1.0, 2.0));
    }

    #[test]
    fn test_mobject_opacity() {
        let mut mobject = MockMobject::new();
        assert_eq!(mobject.opacity(), 1.0);

        mobject.set_opacity(0.5);
        assert_eq!(mobject.opacity(), 0.5);

        // Test clamping
        mobject.set_opacity(1.5);
        assert_eq!(mobject.opacity(), 1.0);

        mobject.set_opacity(-0.5);
        assert_eq!(mobject.opacity(), 0.0);
    }

    #[test]
    fn test_mobject_transform() {
        let mut mobject = MockMobject::new();
        let transform = Transform::translate(3.0, 4.0);
        mobject.apply_transform(&transform);

        assert_eq!(mobject.position(), Vector2D::new(3.0, 4.0));
    }

    #[test]
    fn test_mobject_render() {
        let mobject = MockMobject::new();
        let mut renderer = TestRenderer::new();

        mobject.render(&mut renderer).unwrap();
        // MockMobject doesn't actually draw anything, so count stays 0
        assert_eq!(renderer.render_count, 0);
    }

    #[test]
    fn test_mobject_bounding_box() {
        let mobject = MockMobject::new();
        let bbox = mobject.bounding_box();
        assert_eq!(bbox, BoundingBox::zero());
    }

    #[test]
    fn test_trait_object_in_vec() {
        let mobjects: Vec<Box<dyn Mobject>> =
            vec![Box::new(MockMobject::new()), Box::new(MockMobject::new())];

        assert_eq!(mobjects.len(), 2);
        assert_eq!(mobjects[0].opacity(), 1.0);
    }

    #[test]
    fn test_trait_object_mutation() {
        let mut mobject: Box<dyn Mobject> = Box::new(MockMobject::new());
        mobject.set_position(Vector2D::new(5.0, 6.0));
        mobject.set_opacity(0.7);

        assert_eq!(mobject.position(), Vector2D::new(5.0, 6.0));
        assert_eq!(mobject.opacity(), 0.7);
    }
}
