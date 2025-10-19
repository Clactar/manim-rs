//! Geometric primitives for manim-rs.
//!
//! This module provides fundamental geometric shapes that can be rendered and animated:
//! - [`Circle`] - Perfect circles using Bézier curve approximation
//! - [`Rectangle`] / [`Square`] - Rectangular shapes
//! - [`Line`] - Line segments
//! - [`Polygon`] - Regular and irregular polygons
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

mod arc;
mod arrow;
mod circle;
mod ellipse;
mod line;
mod polygon;
mod rectangle;

pub use arc::{Arc, ArcBuilder};
pub use arrow::{Arrow, ArrowBuilder};
pub use circle::{Circle, CircleBuilder};
pub use ellipse::{Ellipse, EllipseBuilder};
pub use line::{Line, LineBuilder};
pub use polygon::{Polygon, PolygonBuilder};
pub use rectangle::{Rectangle, RectangleBuilder, Square, SquareBuilder};
