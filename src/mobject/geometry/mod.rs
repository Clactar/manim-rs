//! Geometric primitives for manim-rs.
//!
//! This module provides fundamental geometric shapes that can be rendered and animated:
//! - [`Circle`] - Perfect circles using Bézier curve approximation
//! - [`Rectangle`] / [`Square`] - Rectangular shapes with optional rounded corners
//! - [`Line`] - Line segments
//! - [`Polygon`] - Regular and irregular polygons
//! - [`Arc`] - Circular arcs
//! - [`Arrow`] - Arrows with customizable tips
//! - [`Ellipse`] - Ellipses
//!
//! # Examples
//!
//! ```
//! use manim_rs::core::Color;
//! use manim_rs::mobject::geometry::Circle;
//!
//! let circle = Circle::builder()
//!     .radius(2.0)
//!     .stroke_color(Color::BLUE)
//!     .fill_color(Color::from_hex("#87CEEB").unwrap())
//!     .build();
//! ```

mod circle;
mod rectangle;

pub use circle::{Circle, CircleBuilder};
pub use rectangle::{Rectangle, RectangleBuilder, Square, SquareBuilder};
