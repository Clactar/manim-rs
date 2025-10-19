//! Core types and utilities for manim-rs.
//!
//! This module provides fundamental types used throughout the library:
//! - [`Vector2D`] - 2D vector with SIMD optimizations
//! - [`Color`] - RGBA color representation
//! - [`Transform`] - 2D transformation matrices
//! - [`Error`] - Error types for the library

mod color;
mod error;
mod transform;
mod vector;

pub use color::Color;
pub use error::{Error, Result};
pub use transform::Transform;
pub use vector::Vector2D;

