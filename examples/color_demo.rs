//! Basic color operations demonstration
//!
//! Demonstrates:
//! - Creating colors from RGB values
//! - Hex color parsing and generation
//! - Color interpolation
//! - Alpha channel manipulation

use manim_rs::core::Color;

fn main() {
    println!("🎨 Manim-rs Color Demo\n");

    // Creating colors
    let red = Color::rgb(255, 0, 0);
    let blue = Color::rgb(0, 0, 255);
    let green = Color::GREEN;

    println!("Red:   {}", red.to_hex());
    println!("Blue:  {}", blue.to_hex());
    println!("Green: {}", green.to_hex());

    // From hex
    let purple = Color::from_hex("#800080").unwrap();
    println!("\nPurple from hex: {}", purple.to_hex());

    // Color interpolation
    println!("\nInterpolating from red to blue:");
    for i in 0..=5 {
        let t = i as f64 / 5.0;
        let color = red.lerp(blue, t);
        print!("  t={:.1}: {} ", t, color.to_hex());
        print!("RGB({:.2}, {:.2}, {:.2})\n", color.r, color.g, color.b);
    }

    // Alpha channel
    println!("\nAlpha channel:");
    let opaque = Color::RED;
    let transparent = opaque.with_alpha(0.5);
    println!("  Opaque: alpha = {:.2}", opaque.a);
    println!("  Transparent: alpha = {:.2}", transparent.a);

    // Color constants
    println!("\nColor constants:");
    println!("  WHITE:   {}", Color::WHITE.to_hex());
    println!("  BLACK:   {}", Color::BLACK.to_hex());
    println!("  CYAN:    {}", Color::CYAN.to_hex());
    println!("  MAGENTA: {}", Color::MAGENTA.to_hex());
    println!("  YELLOW:  {}", Color::YELLOW.to_hex());
}
