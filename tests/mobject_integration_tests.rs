//! Integration tests for the mobject system.

use manim_rs::core::{Color, Result, Vector2D};
use manim_rs::mobject::geometry::{Circle, Ellipse, Line, Polygon, Rectangle, Square};
use manim_rs::mobject::{Mobject, MobjectGroup};

#[cfg(feature = "svg")]
use manim_rs::backends::SvgRenderer;
#[cfg(feature = "svg")]
use manim_rs::renderer::Renderer;

#[test]
fn test_all_shapes_can_be_created() {
    let _circle = Circle::new(1.0);
    let _rectangle = Rectangle::new(2.0, 1.0);
    let _square = Square::new(1.0);
    let _line = Line::new(Vector2D::ZERO, Vector2D::new(1.0, 1.0));
    let _polygon = Polygon::regular(5, 1.0);
    let _ellipse = Ellipse::new(2.0, 1.0);
}

#[test]
fn test_mobject_group_with_multiple_shapes() {
    let mut group = MobjectGroup::new();

    group
        .add(Box::new(Circle::new(1.0)))
        .add(Box::new(Square::new(2.0)))
        .add(Box::new(Polygon::regular(3, 1.5)));

    assert_eq!(group.len(), 3);
}

#[test]
fn test_nested_mobject_groups() {
    let mut inner_group = MobjectGroup::new();
    inner_group
        .add(Box::new(Circle::new(0.5)))
        .add(Box::new(Square::new(0.5)));

    let mut outer_group = MobjectGroup::new();
    outer_group
        .add(Box::new(inner_group))
        .add(Box::new(Rectangle::new(3.0, 2.0)));

    assert_eq!(outer_group.len(), 2);
}

#[test]
fn test_builder_patterns() {
    let circle = Circle::builder()
        .radius(2.0)
        .stroke_color(Color::BLUE)
        .fill_color(Color::RED)
        .opacity(0.8)
        .build();

    assert_eq!(circle.radius(), 2.0);
    assert_eq!(circle.opacity(), 0.8);
}

#[test]
fn test_regular_polygons() {
    let triangle = Polygon::regular(3, 1.0);
    let hexagon = Polygon::regular(6, 1.0);
    let decagon = Polygon::regular(10, 1.0);

    assert_eq!(triangle.vertices().len(), 3);
    assert_eq!(hexagon.vertices().len(), 6);
    assert_eq!(decagon.vertices().len(), 10);
}

#[test]
#[cfg(feature = "svg")]
fn test_render_all_shapes_to_svg() -> Result<()> {
    let mut renderer = SvgRenderer::new(800, 600);

    let circle = Circle::builder()
        .radius(50.0)
        .center(Vector2D::new(-200.0, 0.0))
        .stroke_color(Color::BLUE)
        .build();

    let square = Square::builder()
        .side_length(80.0)
        .center(Vector2D::new(0.0, 0.0))
        .fill_color(Color::RED)
        .build();

    let polygon = Polygon::builder()
        .regular(6, 60.0)
        .fill_color(Color::GREEN)
        .build();

    renderer.begin_frame()?;
    renderer.clear(Color::BLACK)?;

    circle.render(&mut renderer)?;
    square.render(&mut renderer)?;
    polygon.render(&mut renderer)?;

    renderer.end_frame()?;

    Ok(())
}

#[test]
fn test_transform_propagation() {
    use manim_rs::core::Transform;

    let mut group = MobjectGroup::new();
    let mut circle = Circle::new(1.0);
    circle.set_position(Vector2D::new(1.0, 1.0));

    group.add(Box::new(circle));

    let transform = Transform::translate(2.0, 3.0);
    group.apply_transform(&transform);

    assert_eq!(group.position(), Vector2D::new(2.0, 3.0));
}

#[test]
fn test_opacity_propagation() {
    let mut group = MobjectGroup::new();
    group
        .add(Box::new(Circle::new(1.0)))
        .add(Box::new(Square::new(1.0)));

    group.set_opacity(0.5);

    for mobject in group.iter() {
        assert_eq!(mobject.opacity(), 0.5);
    }
}

#[test]
fn test_line_properties() {
    let line = Line::new(Vector2D::new(0.0, 0.0), Vector2D::new(3.0, 4.0));

    assert!((line.length() - 5.0).abs() < 1e-10);
}

#[test]
fn test_ellipse_is_circle_when_equal() {
    let ellipse = Ellipse::new(2.0, 2.0);
    assert_eq!(ellipse.width(), ellipse.height());
}
