//! Core types and utilities for manim-rs.
//!
//! This module provides fundamental types used throughout the library:
//! - [`Vector2D`] - 2D vector with SIMD optimizations
//! - [`Color`] - RGBA color representation
//! - [`Transform`] - 2D transformation matrices
//! - [`BoundingBox`] - Axis-aligned bounding boxes for spatial queries
//! - [`Degrees`]/[`Radians`] - Type-safe angle representations with conversions
//! - [`QuadraticBezier`]/[`CubicBezier`] - Bézier curve utilities
//! - [`Error`] - Error types for the library

mod angle;
mod bezier;
mod bounding_box;
mod color;
mod error;
mod transform;
mod vector;

pub use angle::{Degrees, Radians};
pub use bezier::{CubicBezier, QuadraticBezier};
pub use bounding_box::BoundingBox;
pub use color::Color;
pub use error::{Error, Result};
pub use transform::Transform;
pub use vector::Vector2D;
