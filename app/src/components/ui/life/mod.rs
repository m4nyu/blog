pub mod cell;
pub mod game;

#[cfg(feature = "hydrate")]
pub mod canvas;

pub use cell::{Cell, CellState};
pub use game::{Life, Universe};

#[cfg(feature = "hydrate")]
pub use canvas::{CanvasConfig, CanvasRenderer};
