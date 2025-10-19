//! Integration tests for the SVG rendering backend.

#![cfg(feature = "svg")]

use manim_rs::backends::SvgRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::renderer::{Path, PathStyle, Renderer, TextStyle};

/// Helper function to create a circle path using 4 cubic bezier curves
fn create_circle_path(radius: f64) -> Path {
    let mut path = Path::new();
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
    path
}

#[test]
fn test_svg_renderer_creation() {
    let renderer = SvgRenderer::new(1920, 1080);
    assert_eq!(renderer.dimensions(), (1920, 1080));
}

#[test]
fn test_svg_renderer_clear() {
    let mut renderer = SvgRenderer::new(800, 600);
    renderer.clear(Color::BLACK).unwrap();

    let svg = renderer.to_svg_string();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("width=\"800\""));
    assert!(svg.contains("height=\"600\""));
}

#[test]
fn test_render_circle_to_svg() {
    let mut renderer = SvgRenderer::new(800, 600);

    let path = create_circle_path(2.0);
    let style = PathStyle::stroke(Color::BLUE, 2.0);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::BLACK).unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    let svg_output = renderer.to_svg_string();
    assert!(svg_output.contains("<svg"));
    assert!(svg_output.contains("width=\"800\""));
    assert!(svg_output.contains("<path"));
    assert!(svg_output.contains("stroke"));
}

#[test]
fn test_render_rectangle_to_svg() {
    let mut renderer = SvgRenderer::new(800, 600);

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

    renderer.begin_frame().unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    let svg = renderer.to_svg_string();
    assert!(svg.contains("<path"));
    assert!(svg.contains("stroke"));
    assert!(svg.contains("fill"));
}

#[test]
fn test_render_text_to_svg() {
    let mut renderer = SvgRenderer::new(1920, 1080);

    let style = TextStyle::new(Color::WHITE, 48.0);

    renderer.begin_frame().unwrap();
    renderer
        .draw_text("Hello, SVG!", Vector2D::new(100.0, 200.0), &style)
        .unwrap();
    renderer.end_frame().unwrap();

    let svg = renderer.to_svg_string();
    assert!(svg.contains("<text"));
    assert!(svg.contains("Hello, SVG!"));
}

#[test]
fn test_multiple_shapes() {
    let mut renderer = SvgRenderer::new(1920, 1080);

    let circle = create_circle_path(1.0);
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

    let svg = renderer.to_svg_string();
    // Should have 2 path elements plus the background rectangle
    let path_count = svg.matches("<path").count();
    assert!(path_count >= 2);
}

#[test]
fn test_svg_save_to_file() {
    use std::fs;
    use std::path::Path as FilePath;

    let mut renderer = SvgRenderer::new(400, 300);

    let circle = create_circle_path(1.5);
    let style = PathStyle::fill(Color::from_hex("#FF5733").unwrap());

    renderer.begin_frame().unwrap();
    renderer.clear(Color::WHITE).unwrap();
    renderer.draw_path(&circle, &style).unwrap();
    renderer.end_frame().unwrap();

    let test_dir = "test_output";
    fs::create_dir_all(test_dir).unwrap();
    let file_path = format!("{}/test_circle.svg", test_dir);

    renderer.save(&file_path).unwrap();

    assert!(FilePath::new(&file_path).exists());

    // Read and verify content
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("<svg"));
    assert!(content.contains("</svg>"));

    // Cleanup
    fs::remove_file(&file_path).ok();
}
