//! # Manim-rs: High-Performance Mathematical Animation Engine
//!
//! Manim-rs is a Rust reimagining of Manim, focusing on performance,
//! flexibility, and type safety for creating mathematical animations.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use manim_rs::prelude::*;
//!
//! fn main() -> Result<()> {
//!     let mut scene = Scene::new(SceneConfig::default());
//!     
//!     let circle = Circle::builder()
//!         .center(Vector2D::ZERO)
//!         .radius(2.0)
//!         .build();
//!     
//!     scene.add(circle);
//!     scene.render("output.svg")?;
//!     
//!     Ok(())
//! }
//! ```
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
    pub use crate::core::{Color, Vector2D};
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
