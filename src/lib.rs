//! # Manim-rs: High-Performance Mathematical Animation Engine
//!
//! Manim-rs is a Rust reimagining of Manim, focusing on performance,
//! flexibility, and type safety for creating mathematical animations.
//!
//! ## Quick Start
//!
//! ```rust
//! use manim_rs::core::{Vector2D, Color, Transform};
//!
//! // Create vectors and colors
//! let position = Vector2D::new(1.0, 2.0);
//! let color = Color::rgb(255, 0, 0);
//! 
//! // Use transformations
//! let transform = Transform::translate(5.0, 3.0);
//! let new_pos = transform.apply(position);
//! 
//! assert_eq!(new_pos.x, 6.0);
//! assert_eq!(new_pos.y, 5.0);
//! ```
//!
//! Note: Full scene rendering is coming in Phase 2. Currently provides core math types.
//!
//! ## Architecture
//!
//! - [`core`] - Fundamental types and utilities
//! - [`scene`] - Scene management and composition
//! - [`animation`] - Animation primitives and timing
//! - [`mobject`] - Mathematical objects and shapes
//! - [`renderer`] - Rendering traits and backends

pub mod animation;
pub mod backends;
pub mod core;
pub mod mobject;
pub mod renderer;
pub mod scene;
pub mod utils;

/// Commonly used types and traits
pub mod prelude {
    pub use crate::core::{Color, Transform, Vector2D};
    pub use crate::scene::{Scene, SceneConfig};

    /// Result type for manim-rs operations
    pub type Result<T> = std::result::Result<T, crate::core::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_imports() {
        use prelude::*;
        let _v = Vector2D::new(1.0, 2.0);
        let _c = Color::rgb(255, 0, 0);
    }
}
