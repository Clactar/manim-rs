//! Comprehensive geometry showcase.
//!
//! Demonstrates all geometric primitives available in manim-rs.

use manim_rs::backends::SvgRenderer;
use manim_rs::core::{Color, Vector2D};
use manim_rs::mobject::geometry::{Circle, Ellipse, Line, Polygon, Rectangle, Square};
use manim_rs::mobject::Mobject;
use manim_rs::renderer::Renderer;

fn main() -> manim_rs::core::Result<()> {
    println!("Creating comprehensive geometry showcase...");

    let mut renderer = SvgRenderer::new(1920, 1080);
    renderer.clear(Color::from_hex("#1E1E1E").unwrap())?;

    // Row 1: Circles and ellipses
    let circle1 = Circle::builder()
        .radius(60.0)
        .center(Vector2D::new(-600.0, 300.0))
        .stroke_color(Color::from_hex("#FF6B6B").unwrap())
        .stroke_width(3.0)
        .fill_color(Color::from_hex("#FF6B6B").unwrap())
        .opacity(0.3)
        .build();

    let circle2 = Circle::builder()
        .radius(60.0)
        .center(Vector2D::new(-400.0, 300.0))
        .stroke_color(Color::from_hex("#4ECDC4").unwrap())
        .stroke_width(5.0)
        .build();

    let ellipse = Ellipse::builder()
        .width(160.0)
        .height(80.0)
        .center(Vector2D::new(-150.0, 300.0))
        .stroke_color(Color::from_hex("#95E1D3").unwrap())
        .fill_color(Color::from_hex("#95E1D3").unwrap())
        .opacity(0.4)
        .build();

    // Row 2: Rectangles and squares
    let rectangle = Rectangle::builder()
        .width(140.0)
        .height(80.0)
        .center(Vector2D::new(-600.0, 100.0))
        .stroke_color(Color::from_hex("#F38181").unwrap())
        .fill_color(Color::from_hex("#F38181").unwrap())
        .opacity(0.5)
        .build();

    let square = Square::builder()
        .side_length(100.0)
        .center(Vector2D::new(-400.0, 100.0))
        .stroke_color(Color::from_hex("#AA96DA").unwrap())
        .stroke_width(4.0)
        .build();

    // Row 3: Polygons
    let triangle = Polygon::builder()
        .regular(3, 70.0)
        .stroke_color(Color::from_hex("#FCBAD3").unwrap())
        .fill_color(Color::from_hex("#FCBAD3").unwrap())
        .opacity(0.4)
        .build();
    let mut triangle_mob = triangle;
    triangle_mob.set_position(Vector2D::new(-600.0, -120.0));

    let pentagon = Polygon::builder()
        .regular(5, 60.0)
        .stroke_color(Color::from_hex("#FFFFD2").unwrap())
        .fill_color(Color::from_hex("#FFFFD2").unwrap())
        .opacity(0.3)
        .build();
    let mut pentagon_mob = pentagon;
    pentagon_mob.set_position(Vector2D::new(-400.0, -120.0));

    let hexagon = Polygon::builder()
        .regular(6, 60.0)
        .stroke_color(Color::from_hex("#A8DADC").unwrap())
        .stroke_width(3.0)
        .fill_color(Color::from_hex("#A8DADC").unwrap())
        .opacity(0.5)
        .build();
    let mut hexagon_mob = hexagon;
    hexagon_mob.set_position(Vector2D::new(-150.0, -120.0));

    let octagon = Polygon::builder()
        .regular(8, 60.0)
        .stroke_color(Color::from_hex("#457B9D").unwrap())
        .fill_color(Color::from_hex("#457B9D").unwrap())
        .opacity(0.4)
        .build();
    let mut octagon_mob = octagon;
    octagon_mob.set_position(Vector2D::new(100.0, -120.0));

    // Right side: Lines and custom shapes
    let line1 = Line::builder()
        .start(Vector2D::new(250.0, 350.0))
        .end(Vector2D::new(450.0, 350.0))
        .stroke_color(Color::from_hex("#E63946").unwrap())
        .stroke_width(4.0)
        .build();

    let line2 = Line::builder()
        .start(Vector2D::new(250.0, 280.0))
        .end(Vector2D::new(450.0, 280.0))
        .stroke_color(Color::from_hex("#F1FAEE").unwrap())
        .stroke_width(6.0)
        .build();

    let line3 = Line::builder()
        .start(Vector2D::new(250.0, 210.0))
        .end(Vector2D::new(450.0, 210.0))
        .stroke_color(Color::from_hex("#A8DADC").unwrap())
        .stroke_width(8.0)
        .build();

    // Diagonal lines
    let diagonal1 = Line::builder()
        .start(Vector2D::new(250.0, 100.0))
        .end(Vector2D::new(350.0, 0.0))
        .stroke_color(Color::from_hex("#457B9D").unwrap())
        .stroke_width(3.0)
        .build();

    let diagonal2 = Line::builder()
        .start(Vector2D::new(350.0, 100.0))
        .end(Vector2D::new(250.0, 0.0))
        .stroke_color(Color::from_hex("#1D3557").unwrap())
        .stroke_width(3.0)
        .build();

    // Custom polygon (star approximation using pentagon)
    let star_vertices = vec![
        Vector2D::new(0.0, 80.0),
        Vector2D::new(20.0, 20.0),
        Vector2D::new(80.0, 20.0),
        Vector2D::new(30.0, -20.0),
        Vector2D::new(50.0, -80.0),
        Vector2D::new(0.0, -40.0),
        Vector2D::new(-50.0, -80.0),
        Vector2D::new(-30.0, -20.0),
        Vector2D::new(-80.0, 20.0),
        Vector2D::new(-20.0, 20.0),
    ];

    let star = Polygon::builder()
        .vertices(star_vertices)
        .stroke_color(Color::from_hex("#FFD700").unwrap())
        .fill_color(Color::from_hex("#FFD700").unwrap())
        .stroke_width(2.0)
        .opacity(0.6)
        .build();
    let mut star_mob = star;
    star_mob.set_position(Vector2D::new(550.0, 100.0));

    // Multiple concentric circles
    for i in 0..4 {
        let radius = 30.0 + i as f64 * 20.0;
        let circle = Circle::builder()
            .radius(radius)
            .center(Vector2D::new(350.0, -200.0))
            .stroke_color(Color::from_hex("#B5838D").unwrap())
            .stroke_width(2.0)
            .opacity(0.6 - i as f64 * 0.1)
            .build();

        renderer.begin_frame()?;
        circle.render(&mut renderer)?;
        renderer.end_frame()?;
    }

    // Render all shapes
    renderer.begin_frame()?;

    circle1.render(&mut renderer)?;
    circle2.render(&mut renderer)?;
    ellipse.render(&mut renderer)?;

    rectangle.render(&mut renderer)?;
    square.render(&mut renderer)?;

    triangle_mob.render(&mut renderer)?;
    pentagon_mob.render(&mut renderer)?;
    hexagon_mob.render(&mut renderer)?;
    octagon_mob.render(&mut renderer)?;

    line1.render(&mut renderer)?;
    line2.render(&mut renderer)?;
    line3.render(&mut renderer)?;
    diagonal1.render(&mut renderer)?;
    diagonal2.render(&mut renderer)?;

    star_mob.render(&mut renderer)?;

    renderer.end_frame()?;

    // Save
    std::fs::create_dir_all("output")?;
    renderer.save("output/geometry_showcase.svg")?;

    println!("✓ Comprehensive geometry showcase saved!");
    println!("  File: output/geometry_showcase.svg");
    println!("  Shapes: Circles, Ellipses, Rectangles, Squares, Polygons, Lines");
    println!("  Polygons: Triangle, Pentagon, Hexagon, Octagon, Custom Star");

    Ok(())
}

