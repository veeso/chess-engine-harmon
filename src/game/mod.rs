//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).
//!

use alloc::vec::Vec;
use core::time::Duration;

// -- modules
mod builder;
mod clock;
pub mod metadata;
mod types;

// -- imports
use crate::Board;
use metadata::Metadata;

// -- export
pub use builder::GameBuilder;
pub use clock::Clock;
pub use types::{EndGame, GameMove, GameResult};

// TODO: game
// TODO: THREEFOLD REPETITION

/// ## Game
///
/// A wrapper around the `Board`, which also tracks time, moves and match metadata (e.g. player data, date, event, location...).
/// The game also adds some missing logics to the board, such as the "threefold repetition" stalemate.
/// Game exposes a limited api to the board, which allows only to play a "standard and clean" chess match.
#[derive(Debug, Clone)]
pub struct Game {
    /// Current board state
    board: Board,
    /// Game clocks
    clock: Clock,
    /// Game metadata
    metadata: Metadata,
    /// Game moves
    moves: Vec<GameMove>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            clock: Clock::new(Duration::MAX, Duration::MAX),
            metadata: Metadata::default(),
            moves: Vec::default(),
        }
    }
}
