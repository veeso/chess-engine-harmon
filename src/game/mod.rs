//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).
//!

// -- modules
mod clock;
pub mod metadata;
mod types;

// -- imports
use crate::Board;
use metadata::Metadata;

// -- export
pub use clock::Clock;
pub use types::{EndGame, GameResult};

// TODO: game
// TODO: THREEFOLD REPETITION
// TODO: times; move tracks time; add_time, subtract_time; use duration
// Moves are tuple of Move and Duration

#[derive(Debug, Clone)]
pub struct Game {
    /// Current board state
    board: Board,
    /// Game clocks
    clock: Clock,
    /// Game metadata
    metadata: Metadata,
}
