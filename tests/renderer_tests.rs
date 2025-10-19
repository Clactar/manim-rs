//! Integration tests for the rendering system.

use manim_rs::core::{Color, Transform, Vector2D};
use manim_rs::renderer::{
    FontWeight, Path, PathFillRule, PathStyle, Renderer, TextAlignment, TextStyle,
};

/// Mock renderer for testing purposes.
struct MockRenderer {
    width: u32,
    height: u32,
    cleared: bool,
    clear_color: Option<Color>,
    paths_drawn: usize,
    texts_drawn: usize,
}

impl MockRenderer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cleared: false,
            clear_color: None,
            paths_drawn: 0,
            texts_drawn: 0,
        }
    }
}

impl Renderer for MockRenderer {
    fn clear(&mut self, color: Color) -> manim_rs::core::Result<()> {
        self.cleared = true;
        self.clear_color = Some(color);
        Ok(())
    }

    fn draw_path(&mut self, _path: &Path, _style: &PathStyle) -> manim_rs::core::Result<()> {
        self.paths_drawn += 1;
        Ok(())
    }

    fn draw_text(
        &mut self,
        _text: &str,
        _position: Vector2D,
        _style: &TextStyle,
    ) -> manim_rs::core::Result<()> {
        self.texts_drawn += 1;
        Ok(())
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[test]
fn test_render_circle() {
    let mut renderer = MockRenderer::new(1920, 1080);

    // Create a circle-like path using 4 cubic bezier curves
    let mut path = Path::new();
    let radius = 2.0;
    let magic = 0.551_915_024_493_510_6; // Magic number for circle approximation

    // Start at the rightmost point
    path.move_to(Vector2D::new(radius, 0.0));

    // Top-right quadrant
    path.cubic_to(
        Vector2D::new(radius, radius * magic),
        Vector2D::new(radius * magic, radius),
        Vector2D::new(0.0, radius),
    );

    // Top-left quadrant
    path.cubic_to(
        Vector2D::new(-radius * magic, radius),
        Vector2D::new(-radius, radius * magic),
        Vector2D::new(-radius, 0.0),
    );

    // Bottom-left quadrant
    path.cubic_to(
        Vector2D::new(-radius, -radius * magic),
        Vector2D::new(-radius * magic, -radius),
        Vector2D::new(0.0, -radius),
    );

    // Bottom-right quadrant
    path.cubic_to(
        Vector2D::new(radius * magic, -radius),
        Vector2D::new(radius, -radius * magic),
        Vector2D::new(radius, 0.0),
    );

    path.close();

    let style = PathStyle::stroke(Color::BLUE, 2.0);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::BLACK).unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    assert_eq!(renderer.dimensions(), (1920, 1080));
    assert!(renderer.cleared);
    assert_eq!(renderer.clear_color, Some(Color::BLACK));
    assert_eq!(renderer.paths_drawn, 1);

    // Verify the path has the right structure
    assert_eq!(path.len(), 6); // MoveTo + 4 CubicTo + Close
}

#[test]
fn test_render_rectangle() {
    let mut renderer = MockRenderer::new(800, 600);

    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(3.0, 0.0))
        .line_to(Vector2D::new(3.0, 2.0))
        .line_to(Vector2D::new(0.0, 2.0))
        .close();

    let style = PathStyle::default()
        .with_stroke(Color::RED, 1.0)
        .with_fill(Color::YELLOW)
        .with_opacity(0.8);

    renderer.draw_path(&path, &style).unwrap();

    assert_eq!(renderer.paths_drawn, 1);

    // Verify bounding box
    let bounds = path.bounding_box();
    assert_eq!(bounds.width(), 3.0);
    assert_eq!(bounds.height(), 2.0);
}

#[test]
fn test_render_text() {
    let mut renderer = MockRenderer::new(1920, 1080);

    let style = TextStyle::new(Color::WHITE, 48.0)
        .with_font_family("Arial")
        .with_weight(FontWeight::Bold)
        .with_alignment(TextAlignment::Center);

    renderer
        .draw_text("Hello, World!", Vector2D::new(960.0, 540.0), &style)
        .unwrap();

    assert_eq!(renderer.texts_drawn, 1);
}

#[test]
fn test_transform_application() {
    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(1.0, 1.0))
        .line_to(Vector2D::new(0.0, 1.0))
        .close();

    let initial_bounds = path.bounding_box();
    assert_eq!(initial_bounds.width(), 1.0);
    assert_eq!(initial_bounds.height(), 1.0);

    // Apply scaling
    let scale_transform = Transform::scale(2.0, 2.0);
    path.apply_transform(&scale_transform);

    let scaled_bounds = path.bounding_box();
    assert!((scaled_bounds.width() - 2.0).abs() < 1e-10);
    assert!((scaled_bounds.height() - 2.0).abs() < 1e-10);
}

#[test]
fn test_multiple_shapes() {
    let mut renderer = MockRenderer::new(1920, 1080);

    // Create multiple shapes
    let mut circle = Path::new();
    circle
        .move_to(Vector2D::new(1.0, 0.0))
        .cubic_to(
            Vector2D::new(1.0, 0.55),
            Vector2D::new(0.55, 1.0),
            Vector2D::new(0.0, 1.0),
        )
        .close();

    let mut square = Path::new();
    square
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(1.0, 1.0))
        .line_to(Vector2D::new(0.0, 1.0))
        .close();

    let circle_style = PathStyle::stroke(Color::BLUE, 2.0);
    let square_style = PathStyle::fill(Color::RED);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::BLACK).unwrap();
    renderer.draw_path(&circle, &circle_style).unwrap();
    renderer.draw_path(&square, &square_style).unwrap();
    renderer.end_frame().unwrap();

    assert_eq!(renderer.paths_drawn, 2);
}

#[test]
fn test_path_style_variants() {
    // Test stroke-only style
    let stroke_only = PathStyle::stroke(Color::BLUE, 3.0);
    assert!(stroke_only.stroke_color.is_some());
    assert!(stroke_only.fill_color.is_none());

    // Test fill-only style
    let fill_only = PathStyle::fill(Color::RED);
    assert!(fill_only.stroke_color.is_none());
    assert!(fill_only.fill_color.is_some());

    // Test combined style
    let fill_color = Color::from_hex("#FF5733").unwrap();
    let combined = PathStyle::default()
        .with_stroke(Color::BLACK, 1.0)
        .with_fill(fill_color)
        .with_fill_rule(PathFillRule::EvenOdd)
        .with_opacity(0.75);

    assert_eq!(combined.stroke_color, Some(Color::BLACK));
    assert_eq!(combined.fill_color, Some(fill_color));
    assert_eq!(combined.fill_rule, PathFillRule::EvenOdd);
    assert_eq!(combined.opacity, 0.75);
}

#[test]
fn test_bounding_box_accuracy() {
    let mut path = Path::new();
    path.move_to(Vector2D::new(-1.0, -1.0))
        .line_to(Vector2D::new(2.0, -1.0))
        .line_to(Vector2D::new(2.0, 3.0))
        .line_to(Vector2D::new(-1.0, 3.0))
        .close();

    let bounds = path.bounding_box();

    assert_eq!(bounds.min.x, -1.0);
    assert_eq!(bounds.min.y, -1.0);
    assert_eq!(bounds.max.x, 2.0);
    assert_eq!(bounds.max.y, 3.0);
    assert_eq!(bounds.width(), 3.0);
    assert_eq!(bounds.height(), 4.0);
}

#[test]
fn test_text_style_variants() {
    let default_style = TextStyle::default();
    assert_eq!(default_style.color, Color::WHITE);
    assert_eq!(default_style.font_size, 48.0);
    assert_eq!(default_style.font_family, "sans-serif");

    let custom_style = TextStyle::new(Color::BLACK, 24.0)
        .with_font_family("Helvetica")
        .with_weight(FontWeight::Bold)
        .with_alignment(TextAlignment::Right)
        .with_opacity(0.9);

    assert_eq!(custom_style.color, Color::BLACK);
    assert_eq!(custom_style.font_size, 24.0);
    assert_eq!(custom_style.font_family, "Helvetica");
    assert_eq!(custom_style.font_weight, FontWeight::Bold);
    assert_eq!(custom_style.alignment, TextAlignment::Right);
    assert_eq!(custom_style.opacity, 0.9);
}
