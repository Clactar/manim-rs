//! Basic vector operations demonstration
//!
//! Demonstrates:
//! - Creating vectors
//! - Vector arithmetic
//! - Normalization and magnitude
//! - Interpolation

use manim_rs::core::Vector2D;

fn main() {
    println!("🦀 Manim-rs Vector Demo\n");

    // Creating vectors
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);

    println!("Vector 1: ({}, {})", v1.x, v1.y);
    println!("Vector 2: ({}, {})", v2.x, v2.y);

    // Vector arithmetic
    let sum = v1 + v2;
    println!("\nAddition: ({}, {})", sum.x, sum.y);

    let diff = v1 - v2;
    println!("Subtraction: ({}, {})", diff.x, diff.y);

    let scaled = v1 * 2.0;
    println!("Scaled by 2: ({}, {})", scaled.x, scaled.y);

    // Magnitude and normalization
    let magnitude = v1.magnitude();
    println!("\nMagnitude of v1: {:.2}", magnitude);

    if let Some(normalized) = v1.normalize() {
        println!("Normalized v1: ({:.2}, {:.2})", normalized.x, normalized.y);
        println!("Magnitude of normalized: {:.2}", normalized.magnitude());
    }

    // Dot and cross products
    let dot = v1.dot(v2);
    let cross = v1.cross(v2);
    println!("\nDot product: {:.2}", dot);
    println!("Cross product: {:.2}", cross);

    // Interpolation
    println!("\nInterpolation from v1 to v2:");
    for i in 0..=5 {
        let t = i as f64 / 5.0;
        let interpolated = v1.lerp(v2, t);
        println!(
            "  t={:.1}: ({:.2}, {:.2})",
            t, interpolated.x, interpolated.y
        );
    }

    // Using constants
    println!("\nVector constants:");
    println!("  UP: ({}, {})", Vector2D::UP.x, Vector2D::UP.y);
    println!("  RIGHT: ({}, {})", Vector2D::RIGHT.x, Vector2D::RIGHT.y);
    println!("  ZERO: ({}, {})", Vector2D::ZERO.x, Vector2D::ZERO.y);
}
