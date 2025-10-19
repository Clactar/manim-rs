//! Basic shapes example.
//!
//! Demonstrates rendering geometric primitives using the mobject system.

use manim_rs::backends::SvgRenderer;
use manim_rs::core::{Color, Result, Vector2D};
use manim_rs::mobject::geometry::{Circle, Rectangle, Square};
use manim_rs::mobject::Mobject;
use manim_rs::renderer::Renderer;

fn main() -> Result<()> {
    println!("Rendering basic shapes using mobject system...");

    // Create renderer
    let mut renderer = SvgRenderer::new(1920, 1080);

    // Create shapes using builder pattern
    let circle = Circle::builder()
        .radius(80.0)
        .center(Vector2D::new(-300.0, 0.0))
        .stroke_color(Color::BLUE)
        .stroke_width(3.0)
        .fill_color(Color::from_hex("#87CEEB").unwrap())
        .opacity(0.8)
        .build();

    let square = Square::builder()
        .side_length(150.0)
        .center(Vector2D::new(0.0, 0.0))
        .stroke_color(Color::RED)
        .stroke_width(3.0)
        .fill_color(Color::from_hex("#FFB6C1").unwrap())
        .opacity(0.7)
        .build();

    let rectangle = Rectangle::builder()
        .width(200.0)
        .height(100.0)
        .center(Vector2D::new(350.0, 0.0))
        .stroke_color(Color::from_hex("#00FF00").unwrap())
        .stroke_width(3.0)
        .fill_color(Color::from_hex("#90EE90").unwrap())
        .opacity(0.6)
        .build();

    // Render the scene
    renderer.begin_frame()?;
    renderer.clear(Color::from_hex("#1E1E1E").unwrap())?;

    circle.render(&mut renderer)?;
    square.render(&mut renderer)?;
    rectangle.render(&mut renderer)?;

    renderer.end_frame()?;

    // Save output
    std::fs::create_dir_all("output")?;
    renderer.save("output/shapes_basic.svg")?;

    println!("✓ Saved to output/shapes_basic.svg");
    println!("  Circle: radius=80, blue fill");
    println!("  Square: side=150, pink fill");
    println!("  Rectangle: 200x100, green fill");

    Ok(())
}
