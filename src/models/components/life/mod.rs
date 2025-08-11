pub mod cell;
pub mod game;

#[cfg(feature = "hydrate")]
pub mod canvas;

// Re-export the main types for convenience
pub use cell::{Cell, CellState};
pub use game::{Universe, Life};

#[cfg(feature = "hydrate")]
pub use canvas::{CanvasConfig, CanvasRenderer, AnimationState, PatternManager, get_theme_colors};

#[cfg(not(feature = "hydrate"))]
pub fn get_theme_colors() -> (String, String) {
    ("#f9fafb".to_string(), "#374151".to_string())
}