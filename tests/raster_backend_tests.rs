//! Integration tests for the Raster rendering backend.

#![cfg(feature = "raster")]

use manim_rs::backends::RasterRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::renderer::{Path, PathStyle, Renderer};
use std::fs;
use std::path::Path as FilePath;

/// Helper function to create a circle path using 4 cubic bezier curves
fn create_circle_path(radius: f64) -> Path {
    let mut path = Path::new();
    let magic = 0.551_915_024_493_510_6;

    path.move_to(Vector2D::new(radius, 0.0));

    path.cubic_to(
        Vector2D::new(radius, radius * magic),
        Vector2D::new(radius * magic, radius),
        Vector2D::new(0.0, radius),
    );

    path.cubic_to(
        Vector2D::new(-radius * magic, radius),
        Vector2D::new(-radius, radius * magic),
        Vector2D::new(-radius, 0.0),
    );

    path.cubic_to(
        Vector2D::new(-radius, -radius * magic),
        Vector2D::new(-radius * magic, -radius),
        Vector2D::new(0.0, -radius),
    );

    path.cubic_to(
        Vector2D::new(radius * magic, -radius),
        Vector2D::new(radius, -radius * magic),
        Vector2D::new(radius, 0.0),
    );

    path.close();
    path
}

#[test]
fn test_raster_renderer_creation() {
    let renderer = RasterRenderer::new(1920, 1080);
    assert_eq!(renderer.dimensions(), (1920, 1080));
}

#[test]
fn test_raster_renderer_clear() {
    let mut renderer = RasterRenderer::new(800, 600);
    renderer.clear(Color::WHITE).unwrap();

    // Verify dimensions
    assert_eq!(renderer.dimensions(), (800, 600));
}

#[test]
fn test_render_circle_to_png() {
    let mut renderer = RasterRenderer::new(800, 600);

    let path = create_circle_path(100.0);
    let style = PathStyle::fill(Color::RED);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::WHITE).unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    let test_dir = "test_output";
    fs::create_dir_all(test_dir).unwrap();
    let file_path = format!("{}/test_circle.png", test_dir);

    renderer.save_png(&file_path).unwrap();

    assert!(FilePath::new(&file_path).exists());

    // Cleanup
    fs::remove_file(&file_path).ok();
}

#[test]
fn test_render_rectangle() {
    let mut renderer = RasterRenderer::new(800, 600);

    let mut path = Path::new();
    path.move_to(Vector2D::new(-50.0, -50.0))
        .line_to(Vector2D::new(50.0, -50.0))
        .line_to(Vector2D::new(50.0, 50.0))
        .line_to(Vector2D::new(-50.0, 50.0))
        .close();

    let style = PathStyle::default()
        .with_stroke(Color::BLUE, 2.0)
        .with_fill(Color::from_hex("#FFFF00").unwrap())
        .with_opacity(0.8);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::WHITE).unwrap();
    renderer.draw_path(&path, &style).unwrap();
    renderer.end_frame().unwrap();

    // Just verify it doesn't crash
    assert_eq!(renderer.dimensions(), (800, 600));
}

#[test]
fn test_multiple_shapes() {
    let mut renderer = RasterRenderer::new(1920, 1080);

    let circle = create_circle_path(50.0);
    let mut square = Path::new();
    square
        .move_to(Vector2D::new(100.0, 0.0))
        .line_to(Vector2D::new(200.0, 0.0))
        .line_to(Vector2D::new(200.0, 100.0))
        .line_to(Vector2D::new(100.0, 100.0))
        .close();

    let circle_style = PathStyle::stroke(Color::BLUE, 2.0);
    let square_style = PathStyle::fill(Color::RED);

    renderer.begin_frame().unwrap();
    renderer.clear(Color::BLACK).unwrap();
    renderer.draw_path(&circle, &circle_style).unwrap();
    renderer.draw_path(&square, &square_style).unwrap();
    renderer.end_frame().unwrap();

    assert_eq!(renderer.dimensions(), (1920, 1080));
}

#[test]
fn test_render_with_transparency() {
    let mut renderer = RasterRenderer::new(400, 300);

    let circle = create_circle_path(75.0);
    let style = PathStyle::fill(Color::rgba(1.0, 0.0, 0.0, 0.5));

    renderer.begin_frame().unwrap();
    renderer.clear(Color::WHITE).unwrap();
    renderer.draw_path(&circle, &style).unwrap();
    renderer.end_frame().unwrap();

    assert_eq!(renderer.dimensions(), (400, 300));
}
