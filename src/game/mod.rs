//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).
//!

mod types;
pub use types::{EndGame, GameResult};

// TODO: game
// TODO: player
// TODO: THREEFOLD REPETITION
