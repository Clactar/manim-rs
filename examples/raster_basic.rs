//! Basic raster rendering example.
//!
//! This example demonstrates how to use the Raster backend to render shapes to PNG.

use manim_rs::backends::RasterRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::renderer::{Path, PathStyle, Renderer};

fn create_circle(radius: f64) -> Path {
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

fn main() -> manim_rs::core::Result<()> {
    println!("Rendering shapes to PNG...");

    // Create renderer with 1920x1080 resolution
    let mut renderer = RasterRenderer::new(1920, 1080);

    // Create a circle
    let circle = create_circle(150.0);
    let circle_style = PathStyle::stroke(Color::BLUE, 4.0)
        .with_fill(Color::from_hex("#87CEEB").unwrap())
        .with_opacity(0.9);

    // Create a square
    let mut square = Path::new();
    square
        .move_to(Vector2D::new(-200.0, -200.0))
        .line_to(Vector2D::new(200.0, -200.0))
        .line_to(Vector2D::new(200.0, 200.0))
        .line_to(Vector2D::new(-200.0, 200.0))
        .close();
    let square_style = PathStyle::stroke(Color::RED, 4.0)
        .with_fill(Color::from_hex("#FFB6C1").unwrap())
        .with_opacity(0.7);

    // Create a triangle
    let mut triangle = Path::new();
    triangle
        .move_to(Vector2D::new(0.0, 250.0))
        .line_to(Vector2D::new(216.5, -125.0))
        .line_to(Vector2D::new(-216.5, -125.0))
        .close();
    let triangle_style = PathStyle::stroke(Color::from_hex("#00FF00").unwrap(), 4.0)
        .with_fill(Color::from_hex("#90EE90").unwrap())
        .with_opacity(0.8);

    // Render the scene
    renderer.begin_frame()?;
    renderer.clear(Color::from_hex("#1E1E1E").unwrap())?;

    // Draw shapes
    renderer.draw_path(&circle, &circle_style)?;
    renderer.draw_path(&square, &square_style)?;
    renderer.draw_path(&triangle, &triangle_style)?;

    renderer.end_frame()?;

    // Create output directory and save
    std::fs::create_dir_all("output")?;
    renderer.save_png("output/raster_basic.png")?;

    println!("✓ Saved to output/raster_basic.png");
    println!("  Open the file in an image viewer to see the result.");

    Ok(())
}
