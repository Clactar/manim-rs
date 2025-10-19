//! Polygon mobject.
//!
//! Provides regular and irregular polygons.

use std::f64::consts::PI;

use crate::core::{BoundingBox, Color, Result, Transform, Vector2D};
use crate::mobject::{Mobject, VMobject};
use crate::renderer::{Path, Renderer};

/// A polygon mobject.
///
/// # Examples
///
/// ```
/// use manim_rs::core::Vector2D;
/// use manim_rs::mobject::geometry::Polygon;
///
/// // Triangle
/// let triangle = Polygon::new(vec![
///     Vector2D::new(0.0, 1.0),
///     Vector2D::new(1.0, -1.0),
///     Vector2D::new(-1.0, -1.0),
/// ]);
///
/// // Regular hexagon
/// let hexagon = Polygon::regular(6, 1.0);
/// ```
#[derive(Clone, Debug)]
pub struct Polygon {
    vmobject: VMobject,
    vertices: Vec<Vector2D>,
}

impl Polygon {
    /// Creates a new polygon from the given vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::core::Vector2D;
    /// use manim_rs::mobject::geometry::Polygon;
    ///
    /// let triangle = Polygon::new(vec![
    ///     Vector2D::new(0.0, 1.0),
    ///     Vector2D::new(1.0, -1.0),
    ///     Vector2D::new(-1.0, -1.0),
    /// ]);
    /// ```
    pub fn new(vertices: Vec<Vector2D>) -> Self {
        let path = Self::create_polygon_path(&vertices);
        Self {
            vmobject: VMobject::new(path),
            vertices,
        }
    }

    /// Creates a regular polygon with the given number of sides and radius.
    ///
    /// The polygon is oriented with one vertex pointing upward.
    ///
    /// # Examples
    ///
    /// ```
    /// use manim_rs::mobject::geometry::Polygon;
    ///
    /// let triangle = Polygon::regular(3, 1.0);
    /// let hexagon = Polygon::regular(6, 2.0);
    /// ```
    pub fn regular(sides: usize, radius: f64) -> Self {
        assert!(sides >= 3, "Polygon must have at least 3 sides");

        let mut vertices = Vec::with_capacity(sides);
        let angle_step = 2.0 * PI / sides as f64;
        let start_angle = PI / 2.0; // Start from top

        for i in 0..sides {
            let angle = start_angle + i as f64 * angle_step;
            vertices.push(Vector2D::new(radius * angle.cos(), radius * angle.sin()));
        }

        Self::new(vertices)
    }

    /// Returns a builder for constructing a polygon.
    pub fn builder() -> PolygonBuilder {
        PolygonBuilder::new()
    }

    /// Returns the vertices of the polygon.
    pub fn vertices(&self) -> &[Vector2D] {
        &self.vertices
    }

    /// Sets the stroke color and width.
    pub fn set_stroke(&mut self, color: Color, width: f64) -> &mut Self {
        self.vmobject.set_stroke(color, width);
        self
    }

    /// Sets the fill color.
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.vmobject.set_fill(color);
        self
    }

    /// Creates a polygon path from vertices.
    fn create_polygon_path(vertices: &[Vector2D]) -> Path {
        let mut path = Path::new();

        if let Some(first) = vertices.first() {
            path.move_to(*first);
            for vertex in vertices.iter().skip(1) {
                path.line_to(*vertex);
            }
            path.close();
        }

        path
    }
}

impl Mobject for Polygon {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.vmobject.render(renderer)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.vmobject.bounding_box()
    }

    fn apply_transform(&mut self, transform: &Transform) {
        self.vmobject.apply_transform(transform);
        for vertex in &mut self.vertices {
            *vertex = transform.apply(*vertex);
        }
    }

    fn position(&self) -> Vector2D {
        self.vmobject.position()
    }

    fn set_position(&mut self, pos: Vector2D) {
        self.vmobject.set_position(pos);
    }

    fn opacity(&self) -> f64 {
        self.vmobject.opacity()
    }

    fn set_opacity(&mut self, opacity: f64) {
        self.vmobject.set_opacity(opacity);
    }

    fn clone_mobject(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

/// Builder for constructing polygons.
#[derive(Clone, Debug)]
pub struct PolygonBuilder {
    vertices: Vec<Vector2D>,
    stroke_color: Option<Color>,
    stroke_width: f64,
    fill_color: Option<Color>,
    opacity: f64,
}

impl PolygonBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            stroke_color: Some(Color::WHITE),
            stroke_width: 2.0,
            fill_color: None,
            opacity: 1.0,
        }
    }

    pub fn vertices(mut self, vertices: Vec<Vector2D>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn regular(mut self, sides: usize, radius: f64) -> Self {
        self.vertices = Self::calculate_regular_vertices(sides, radius);
        self
    }

    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_color = None;
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> Polygon {
        let mut polygon = Polygon::new(self.vertices);

        if let Some(color) = self.stroke_color {
            polygon.set_stroke(color, self.stroke_width);
        } else {
            polygon.vmobject.clear_stroke();
        }

        if let Some(color) = self.fill_color {
            polygon.set_fill(color);
        }

        polygon.set_opacity(self.opacity);

        polygon
    }

    fn calculate_regular_vertices(sides: usize, radius: f64) -> Vec<Vector2D> {
        let mut vertices = Vec::with_capacity(sides);
        let angle_step = 2.0 * PI / sides as f64;
        let start_angle = PI / 2.0;

        for i in 0..sides {
            let angle = start_angle + i as f64 * angle_step;
            vertices.push(Vector2D::new(radius * angle.cos(), radius * angle.sin()));
        }

        vertices
    }
}

impl Default for PolygonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_polygon_triangle() {
        let vertices = vec![
            Vector2D::new(0.0, 1.0),
            Vector2D::new(1.0, -1.0),
            Vector2D::new(-1.0, -1.0),
        ];
        let polygon = Polygon::new(vertices.clone());

        assert_eq!(polygon.vertices().len(), 3);
        assert_eq!(polygon.vertices()[0], vertices[0]);
    }

    #[test]
    fn test_polygon_regular_triangle() {
        let triangle = Polygon::regular(3, 1.0);
        assert_eq!(triangle.vertices().len(), 3);
    }

    #[test]
    fn test_polygon_regular_hexagon() {
        let hexagon = Polygon::regular(6, 1.0);
        assert_eq!(hexagon.vertices().len(), 6);

        // Check that it's actually regular (all vertices equidistant from origin)
        for vertex in hexagon.vertices() {
            assert_relative_eq!(vertex.magnitude(), 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    #[should_panic(expected = "Polygon must have at least 3 sides")]
    fn test_polygon_regular_invalid_sides() {
        Polygon::regular(2, 1.0);
    }

    #[test]
    fn test_polygon_builder() {
        let vertices = vec![
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 0.0),
            Vector2D::new(0.5, 1.0),
        ];

        let polygon = Polygon::builder()
            .vertices(vertices.clone())
            .stroke_color(Color::BLUE)
            .fill_color(Color::RED)
            .build();

        assert_eq!(polygon.vertices().len(), 3);
    }

    #[test]
    fn test_polygon_builder_regular() {
        let polygon = Polygon::builder()
            .regular(5, 2.0)
            .fill_color(Color::GREEN)
            .build();

        assert_eq!(polygon.vertices().len(), 5);
    }
}
