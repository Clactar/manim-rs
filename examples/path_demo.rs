//! Path demonstration example.
//!
//! This example shows how to create and manipulate paths in manim-rs.

use manim_rs::core::{Transform, Vector2D};
use manim_rs::renderer::{Path, PathCursor};

fn main() {
    println!("=== Manim-rs Path Demo ===\n");

    // Example 1: Create a triangle using Path
    println!("1. Creating a triangle:");
    let mut triangle = Path::new();
    triangle
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(0.5, 1.0))
        .close();

    println!("   Triangle has {} commands", triangle.len());
    println!("   Bounding box: {:?}", triangle.bounding_box());
    println!();

    // Example 2: Create a square using Path
    println!("2. Creating a square:");
    let mut square = Path::new();
    square
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(2.0, 0.0))
        .line_to(Vector2D::new(2.0, 2.0))
        .line_to(Vector2D::new(0.0, 2.0))
        .close();

    println!("   Square has {} commands", square.len());
    let bounds = square.bounding_box();
    println!(
        "   Bounding box: width={}, height={}",
        bounds.width(),
        bounds.height()
    );
    println!();

    // Example 3: Create a circle using cubic bezier curves
    println!("3. Creating a circle:");
    let mut circle = Path::new();
    let radius = 1.0;
    let magic = 0.551_915_024_493_510_6; // Magic number for circle approximation

    circle.move_to(Vector2D::new(radius, 0.0));

    // Top-right quadrant
    circle.cubic_to(
        Vector2D::new(radius, radius * magic),
        Vector2D::new(radius * magic, radius),
        Vector2D::new(0.0, radius),
    );

    // Top-left quadrant
    circle.cubic_to(
        Vector2D::new(-radius * magic, radius),
        Vector2D::new(-radius, radius * magic),
        Vector2D::new(-radius, 0.0),
    );

    // Bottom-left quadrant
    circle.cubic_to(
        Vector2D::new(-radius, -radius * magic),
        Vector2D::new(-radius * magic, -radius),
        Vector2D::new(0.0, -radius),
    );

    // Bottom-right quadrant
    circle.cubic_to(
        Vector2D::new(radius * magic, -radius),
        Vector2D::new(radius, -radius * magic),
        Vector2D::new(radius, 0.0),
    );

    circle.close();

    println!("   Circle has {} commands", circle.len());
    println!("   Bounding box: {:?}", circle.bounding_box());
    println!();

    // Example 4: Using PathCursor for relative movements
    println!("4. Using PathCursor for relative movements:");
    let mut cursor = PathCursor::new();
    cursor
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .relative_line_to(Vector2D::new(0.0, 1.0)) // Goes to (1.0, 1.0)
        .relative_line_to(Vector2D::new(-1.0, 0.0)) // Goes to (0.0, 1.0)
        .close();

    let path_from_cursor = cursor.into_path();
    println!("   Path has {} commands", path_from_cursor.len());
    println!("   Bounding box: {:?}", path_from_cursor.bounding_box());
    println!();

    // Example 5: Apply transformations
    println!("5. Applying transformations:");
    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(1.0, 1.0))
        .line_to(Vector2D::new(0.0, 1.0))
        .close();

    println!("   Original bounding box: {:?}", path.bounding_box());

    // Apply translation
    let translation = Transform::translate(2.0, 3.0);
    path.apply_transform(&translation);
    println!("   After translation: {:?}", path.bounding_box());

    // Apply scaling
    let mut path2 = Path::new();
    path2
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 1.0))
        .close();

    let scale = Transform::scale(2.0, 2.0);
    path2.apply_transform(&scale);
    println!("   After scaling 2x: {:?}", path2.bounding_box());
    println!();

    // Example 6: Path with preallocated capacity
    println!("6. Creating a path with preallocated capacity:");
    let mut large_path = Path::with_capacity(50);
    large_path.move_to(Vector2D::new(0.0, 0.0));
    for i in 1..50 {
        let x = (i as f64) * 0.1;
        let y = (i as f64 * 0.2).sin();
        large_path.line_to(Vector2D::new(x, y));
    }

    println!("   Large path has {} commands", large_path.len());
    println!("   Bounding box: {:?}", large_path.bounding_box());
    println!();

    // Example 7: Demonstrate bounding box caching
    println!("7. Demonstrating bounding box caching:");
    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 1.0))
        .close();

    // First call computes the bounding box
    let bbox1 = path.bounding_box();
    println!("   First bounding box call: {:?}", bbox1);

    // Second call uses cached value (much faster)
    let bbox2 = path.bounding_box();
    println!("   Second bounding box call (cached): {:?}", bbox2);

    // Modifying the path invalidates the cache
    path.line_to(Vector2D::new(5.0, 5.0));
    let bbox3 = path.bounding_box();
    println!("   After modification: {:?}", bbox3);
    println!();

    // Example 8: Quadratic bezier curves
    println!("8. Using quadratic bezier curves:");
    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .quadratic_to(Vector2D::new(1.0, 2.0), Vector2D::new(2.0, 0.0));

    println!("   Path with quadratic bezier: {} commands", path.len());
    println!("   Bounding box: {:?}", path.bounding_box());
    println!();

    println!("=== Demo Complete ===");
}
