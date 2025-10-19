use thiserror::Error;

/// Error types for manim-rs operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Rendering error
    #[error("Rendering error: {0}")]
    Render(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Animation error
    #[error("Animation error: {0}")]
    Animation(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type for manim-rs operations.
pub type Result<T> = std::result::Result<T, Error>;
