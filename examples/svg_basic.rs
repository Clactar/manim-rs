//! Basic SVG rendering example.
//!
//! This example demonstrates how to use the SVG backend to render simple shapes.

use manim_rs::backends::SvgRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::renderer::{Path, PathStyle, Renderer, TextStyle};

fn create_circle(radius: f64) -> Path {
    let mut path = Path::new();
    let magic = 0.551_915_024_493_510_6; // Magic number for circle approximation with cubic beziers

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

fn main() -> manim_rs::core::Result<()> {
    println!("Rendering basic shapes to SVG...");

    // Create renderer with 1920x1080 resolution
    let mut renderer = SvgRenderer::new(1920, 1080);

    // Create a circle
    let circle = create_circle(100.0);
    let circle_style = PathStyle::stroke(Color::BLUE, 3.0)
        .with_fill(Color::from_hex("#87CEEB").unwrap())
        .with_opacity(0.8);

    // Create a square
    let mut square = Path::new();
    square
        .move_to(Vector2D::new(-150.0, -150.0))
        .line_to(Vector2D::new(150.0, -150.0))
        .line_to(Vector2D::new(150.0, 150.0))
        .line_to(Vector2D::new(-150.0, 150.0))
        .close();
    let square_style = PathStyle::stroke(Color::RED, 3.0)
        .with_fill(Color::from_hex("#FFB6C1").unwrap())
        .with_opacity(0.6);

    // Create a triangle
    let mut triangle = Path::new();
    triangle
        .move_to(Vector2D::new(0.0, 200.0))
        .line_to(Vector2D::new(173.2, -100.0))
        .line_to(Vector2D::new(-173.2, -100.0))
        .close();
    let triangle_style = PathStyle::stroke(Color::from_hex("#00FF00").unwrap(), 3.0)
        .with_fill(Color::from_hex("#90EE90").unwrap())
        .with_opacity(0.7);

    // Add title text
    let title_style = TextStyle::new(Color::WHITE, 60.0);

    // Render the scene
    renderer.begin_frame()?;
    renderer.clear(Color::from_hex("#1E1E1E").unwrap())?;

    // Draw shapes
    renderer.draw_path(&circle, &circle_style)?;
    renderer.draw_path(&square, &square_style)?;
    renderer.draw_path(&triangle, &triangle_style)?;

    // Draw title
    renderer.draw_text(
        "manim-rs: SVG Backend",
        Vector2D::new(0.0, 450.0),
        &title_style,
    )?;

    renderer.end_frame()?;

    // Create output directory and save
    std::fs::create_dir_all("output")?;
    renderer.save("output/svg_basic.svg")?;

    println!("✓ Saved to output/svg_basic.svg");
    println!("  Open the file in a browser or SVG viewer to see the result.");

    Ok(())
}
