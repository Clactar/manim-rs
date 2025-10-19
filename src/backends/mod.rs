//! Rendering backend implementations.
//!
//! This module provides concrete implementations for different output formats:
//! - **SVG** - Vector graphics (behind `svg` feature flag)
//! - **Raster** - Bitmap rendering via tiny-skia (behind `raster` feature flag)
//!
//! # Feature Flags
//!
//! - `svg` - Enables SVG backend (enabled by default)
//! - `raster` - Enables raster/bitmap backend
//!
//! # Examples
//!
//! ## SVG Rendering
//!
//! ```rust,no_run
//! # #[cfg(feature = "svg")]
//! # {
//! use manim_rs::backends::SvgRenderer;
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::renderer::{Path, PathStyle, Renderer};
//!
//! let mut renderer = SvgRenderer::new(1920, 1080);
//! let mut path = Path::new();
//! path.move_to(Vector2D::new(0.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 1.0));
//!
//! renderer.begin_frame().unwrap();
//! renderer.draw_path(&path, &PathStyle::default()).unwrap();
//! renderer.end_frame().unwrap();
//! # }
//! ```
//!
//! ## Raster Rendering
//!
//! ```rust,no_run
//! # #[cfg(feature = "raster")]
//! # {
//! use manim_rs::backends::RasterRenderer;
//! use manim_rs::core::{Color, Vector2D};
//! use manim_rs::renderer::{Path, PathStyle, Renderer};
//!
//! let mut renderer = RasterRenderer::new(1920, 1080);
//! let mut path = Path::new();
//! path.move_to(Vector2D::new(0.0, 0.0))
//!     .line_to(Vector2D::new(1.0, 1.0));
//!
//! renderer.begin_frame().unwrap();
//! renderer.draw_path(&path, &PathStyle::default()).unwrap();
//! renderer.end_frame().unwrap();
//! # }
//! ```

#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use svg::SvgRenderer;

#[cfg(feature = "raster")]
mod raster;
#[cfg(feature = "raster")]
pub use raster::RasterRenderer;
