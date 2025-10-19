//! Mobject group for hierarchical composition.
//!
//! [`MobjectGroup`] provides a container for organizing multiple mobjects into
//! a single logical unit. Groups support hierarchical transformations where
//! operations on the group affect all children.

use crate::core::{BoundingBox, Result, Transform, Vector2D};
use crate::mobject::Mobject;
use crate::renderer::Renderer;

/// A container for multiple mobjects with hierarchical transformation support.
///
/// [`MobjectGroup`] allows composing multiple mobjects into a single unit.
/// Transformations and property changes applied to the group affect all children.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
/// use manim_rs::mobject::{MobjectGroup, VMobject};
/// use manim_rs::renderer::Path;
///
/// let mut group = MobjectGroup::new();
/// group.add(Box::new(VMobject::new(Path::new())))
///      .add(Box::new(VMobject::new(Path::new())));
///
/// assert_eq!(group.len(), 2);
/// ```
pub struct MobjectGroup {
    mobjects: Vec<Box<dyn Mobject>>,
    position: Vector2D,
    opacity: f64,
}

impl Default for MobjectGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl MobjectGroup {
    /// Creates a new empty mobject group.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::MobjectGroup;
    ///
    /// let group = MobjectGroup::new();
    /// assert_eq!(group.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            mobjects: Vec::new(),
            position: Vector2D::ZERO,
            opacity: 1.0,
        }
    }

    /// Adds a mobject to the group.
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{MobjectGroup, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut group = MobjectGroup::new();
    /// group.add(Box::new(VMobject::new(Path::new())));
    /// assert_eq!(group.len(), 1);
    /// ```
    pub fn add(&mut self, mobject: Box<dyn Mobject>) -> &mut Self {
        self.mobjects.push(mobject);
        self
    }

    /// Removes and returns the mobject at the specified index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{MobjectGroup, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut group = MobjectGroup::new();
    /// group.add(Box::new(VMobject::new(Path::new())));
    ///
    /// let removed = group.remove(0);
    /// assert!(removed.is_some());
    /// assert_eq!(group.len(), 0);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Mobject>> {
        if index < self.mobjects.len() {
            Some(self.mobjects.remove(index))
        } else {
            None
        }
    }

    /// Returns the number of mobjects in the group.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::MobjectGroup;
    ///
    /// let group = MobjectGroup::new();
    /// assert_eq!(group.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.mobjects.len()
    }

    /// Returns `true` if the group contains no mobjects.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::MobjectGroup;
    ///
    /// let group = MobjectGroup::new();
    /// assert!(group.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.mobjects.is_empty()
    }

    /// Returns an iterator over references to the mobjects.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{MobjectGroup, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut group = MobjectGroup::new();
    /// group.add(Box::new(VMobject::new(Path::new())));
    ///
    /// for mobject in group.iter() {
    ///     // Process each mobject
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &dyn Mobject> {
        self.mobjects.iter().map(|b| b.as_ref())
    }

    /// Returns a mutable reference to the mobjects vector.
    ///
    /// This allows direct mutable access to the mobjects for complex operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::{MobjectGroup, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut group = MobjectGroup::new();
    /// group.add(Box::new(VMobject::new(Path::new())));
    ///
    /// for mobject in group.mobjects_mut() {
    ///     mobject.set_opacity(0.5);
    /// }
    /// ```
    pub fn mobjects_mut(&mut self) -> &mut [Box<dyn Mobject>] {
        &mut self.mobjects
    }

    /// Clears all mobjects from the group.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::{MobjectGroup, VMobject};
    /// use manim_rs::renderer::Path;
    ///
    /// let mut group = MobjectGroup::new();
    /// group.add(Box::new(VMobject::new(Path::new())));
    /// group.clear();
    /// assert!(group.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.mobjects.clear();
    }
}

impl Mobject for MobjectGroup {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        for mobject in &self.mobjects {
            mobject.render(renderer)?;
        }
        Ok(())
    }

    fn bounding_box(&self) -> BoundingBox {
        if self.mobjects.is_empty() {
            return BoundingBox::zero();
        }

        let mut bbox = self.mobjects[0].bounding_box();
        for mobject in self.mobjects.iter().skip(1) {
            bbox = bbox.union(&mobject.bounding_box());
        }
        bbox
    }

    fn apply_transform(&mut self, transform: &Transform) {
        for mobject in &mut self.mobjects {
            mobject.apply_transform(transform);
        }
        self.position = transform.apply(self.position);
    }

    fn position(&self) -> Vector2D {
        self.position
    }

    fn set_position(&mut self, pos: Vector2D) {
        let delta = pos - self.position;
        let translation = Transform::translate(delta.x, delta.y);
        for mobject in &mut self.mobjects {
            mobject.apply_transform(&translation);
        }
        self.position = pos;
    }

    fn opacity(&self) -> f64 {
        self.opacity
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.clamp(0.0, 1.0);
        // Apply relative opacity change to all children
        for mobject in &mut self.mobjects {
            mobject.set_opacity(self.opacity);
        }
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        let mut group = MobjectGroup::new();
        group.position = self.position;
        group.opacity = self.opacity;
        for mobject in &self.mobjects {
            group.add(mobject.clone_mobject());
        }
        Box::new(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Color;
    use crate::mobject::VMobject;
    use crate::renderer::{Path, PathStyle, TextStyle};
    use approx::assert_relative_eq;

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
    fn test_group_new() {
        let group = MobjectGroup::new();
        assert_eq!(group.len(), 0);
        assert!(group.is_empty());
        assert_eq!(group.position(), Vector2D::ZERO);
        assert_eq!(group.opacity(), 1.0);
    }

    #[test]
    fn test_group_add_single() {
        let mut group = MobjectGroup::new();
        let vmobject = VMobject::new(Path::new());
        group.add(Box::new(vmobject));

        assert_eq!(group.len(), 1);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_group_add_multiple() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        assert_eq!(group.len(), 3);
    }

    #[test]
    fn test_group_remove() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        let removed = group.remove(0);
        assert!(removed.is_some());
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn test_group_remove_out_of_bounds() {
        let mut group = MobjectGroup::new();
        let removed = group.remove(0);
        assert!(removed.is_none());
    }

    #[test]
    fn test_group_clear() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        group.clear();
        assert!(group.is_empty());
    }

    #[test]
    fn test_group_iter() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        let count = group.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_group_mobjects_mut() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        for mobject in group.mobjects_mut() {
            mobject.set_opacity(0.5);
        }

        for mobject in group.iter() {
            assert_relative_eq!(mobject.opacity(), 0.5);
        }
    }

    #[test]
    fn test_group_render_all_children() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        let mut renderer = TestRenderer::new();
        group.render(&mut renderer).unwrap();

        // Each VMobject calls draw_path once
        assert_eq!(renderer.render_count, 3);
    }

    #[test]
    fn test_group_render_empty() {
        let group = MobjectGroup::new();
        let mut renderer = TestRenderer::new();
        group.render(&mut renderer).unwrap();

        assert_eq!(renderer.render_count, 0);
    }

    #[test]
    fn test_group_bounding_box_empty() {
        let group = MobjectGroup::new();
        let bbox = group.bounding_box();
        assert_eq!(bbox, BoundingBox::zero());
    }

    #[test]
    fn test_group_bounding_box_union() {
        let mut group = MobjectGroup::new();

        let mut path1 = Path::new();
        path1
            .move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 1.0));
        group.add(Box::new(VMobject::new(path1)));

        let mut path2 = Path::new();
        path2
            .move_to(Vector2D::new(2.0, 2.0))
            .line_to(Vector2D::new(3.0, 3.0));
        group.add(Box::new(VMobject::new(path2)));

        let bbox = group.bounding_box();
        // Bounding box should encompass both paths (with stroke expansion)
        assert!(bbox.width() >= 3.0);
        assert!(bbox.height() >= 3.0);
    }

    #[test]
    fn test_group_transform() {
        let mut group = MobjectGroup::new();
        let mut vmobject = VMobject::new(Path::new());
        vmobject.set_position(Vector2D::new(1.0, 1.0));
        group.add(Box::new(vmobject));

        let transform = Transform::translate(2.0, 3.0);
        group.apply_transform(&transform);

        assert_eq!(group.position(), Vector2D::new(2.0, 3.0));
        // Children should also be transformed
        let child_pos = group.iter().next().unwrap().position();
        assert_eq!(child_pos, Vector2D::new(3.0, 4.0));
    }

    #[test]
    fn test_group_set_position() {
        let mut group = MobjectGroup::new();
        group.add(Box::new(VMobject::new(Path::new())));

        group.set_position(Vector2D::new(5.0, 7.0));
        assert_eq!(group.position(), Vector2D::new(5.0, 7.0));
    }

    #[test]
    fn test_group_opacity_affects_children() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));

        group.set_opacity(0.5);
        assert_eq!(group.opacity(), 0.5);

        for mobject in group.iter() {
            assert_relative_eq!(mobject.opacity(), 0.5);
        }
    }

    #[test]
    fn test_group_opacity_clamping() {
        let mut group = MobjectGroup::new();
        group.add(Box::new(VMobject::new(Path::new())));

        group.set_opacity(1.5);
        assert_eq!(group.opacity(), 1.0);

        group.set_opacity(-0.5);
        assert_eq!(group.opacity(), 0.0);
    }

    #[test]
    fn test_group_clone() {
        let mut group = MobjectGroup::new();
        group
            .add(Box::new(VMobject::new(Path::new())))
            .add(Box::new(VMobject::new(Path::new())));
        group.set_position(Vector2D::new(1.0, 2.0));
        group.set_opacity(0.8);

        let boxed: Box<dyn Mobject> = Box::new(group);
        let cloned = boxed.clone_mobject();

        assert_eq!(cloned.position(), Vector2D::new(1.0, 2.0));
        assert_relative_eq!(cloned.opacity(), 0.8);
    }

    #[test]
    fn test_group_nested() {
        let mut inner_group = MobjectGroup::new();
        inner_group.add(Box::new(VMobject::new(Path::new())));

        let mut outer_group = MobjectGroup::new();
        outer_group.add(Box::new(inner_group));

        assert_eq!(outer_group.len(), 1);
    }
}
