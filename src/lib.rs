//! # harmon
//!
//! [harmon](https://github.com/veeso/harmon) is a dependency-free chess engine library written in rust.
//! It is a fork of the original [chess engine](https://github.com/adam-mcdaniel/chess-engine) library for rust written by Adam McDaniel.
//!
//! Harmon comes with these features:
//!
//! TODO: define features
//!
//! In addition to this, I've also made some bugfix and added test units in order to make the library safer and more reliable.
//!
//! ![chess-image](https://raw.githubusercontent.com/veeso/harmon/main/assets/web-board.png)
//!
//! ## Get Started
//!
//! ### Adding `harmon` as dependency
//!
//! ```toml
//! harmon = "0.2.0"
//! ```
//!
//!
//! ## Example
//!
//! It's quite easy to setup the chess engine, for example this creates a default chess match, with a demonstration
//! on how to make the computer to make a move.
//!
//! TODO: use game instead of board
//!
//! ```rust,no_run
//! extern crate harmon;
//!
//! use harmon::{Board, MoveResult, Move, Promotion};
//!
//! fn main() {
//!     let mut board = Board::default();
//!
//!     // Get the best move with 4 moves of lookahead
//!     let (best_move, _) = board.get_best_next_move(4);
//!     // Get the worst move with 3 moves of lookahead
//!     let worst_move = board.get_worst_next_move(3);
//!
//!     // Get all of the possible legal moves for the given player
//!     let legal_moves = board.get_legal_moves(board.get_turn());
//!     // Print the board
//!     println!("{}", board);
//!
//!     print!("CPU chose to ");
//!     match best_move {
//!         Move::Piece(from, to) => println!("move {} to {}", from, to),
//!         Move::KingSideCastle => println!("castle kingside"),
//!         Move::QueenSideCastle => println!("castle queenside"),
//!         Move::Resign => println!("resign")
//!     }
//!     match board.play_move(best_move) {
//!         MoveResult::Continuing(next_board) => {
//!             println!("{}", next_board);
//!         }
//!
//!         MoveResult::Promote(next_board, pos) => {
//!             println!("{}", next_board);
//!             println!("Pawn promotion available at {}", pos);
//!             if let MoveResult::Continuing(b) = next_board.promote(Promotion::Queen) {
//!                 board = b;
//!             }
//!         }
//!         
//!         MoveResult::Victory(winner) => {
//!             // You can use the ! operator on a player's
//!             // color to invert.
//!             println!("{} loses. {} is victorious.",
//!               !winner, winner
//!             );
//!         }
//!         
//!         MoveResult::IllegalMove(x) => {
//!             eprintln!("{} is an illegal move.", x);
//!         }
//!         
//!         MoveResult::Stalemate => {
//!             println!("Drawn game.");
//!         }
//!     }
//! }
//! ```
//!
//! ## Game variants
//!
//! Chess-engine also supports other game modes, such as Horde:
//! ```rust,no_run
//! extern crate harmon;
//!
//! use harmon::{Board, MoveResult, Move};
//!
//! // TODO: complete with other variants
//!
//! fn main() {
//!
//! }
//! ```
//!
//! ## Game Vs. Board
//!
//! TODO: fill
//!
//! ## Promoting pawns
//!
//! Whenever you move one of your pawns to the last rank, a `Promotion` variant will be returned.
//! Promotion is handled "asynchronously", because it is handled directly when moving the piece, but requires you to call another
//! function to promote.
//! Once the `Promotion` variant is returned, you have to call the `promote(Promotion)` function to promote the pawn.
//! At this point you can keep playing moves as usual.
//! Be careful though, if you don't promote when a `Promotion` is returned after playing move and, instead, you try to move another piece,
//! the engine will panic.
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/harmon/main/assets/cargo/harmon-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/harmon/main/assets/cargo/harmon-512.png"
)]
#![no_std]
#[macro_use]
extern crate alloc;

#[cfg(test)] // NOTE: Enable std for test units
extern crate std;

// -- modules

mod board;
pub use board::{Board, BoardBuilder, MoveResult, Promotion};

pub mod game;
pub use game::{Game, GameResult};

mod piece;
pub use piece::Piece;

mod position;
pub use position::*;

mod square;
pub use square::Square;

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

/// ## Color
///
/// The color of a piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

/// A color can be inverted using the `!` operator.
/// `!Color::White` becomes `Color::Black` and vice versa.
impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/// ## Move
///
/// A move that can be applied to a board.
/// When applied to a board, the board assumes that the move is
/// being applied for the current turn's player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    /// If the current player is white, move the king to the C1 square, and the kingside rook to
    /// the D1 square. If the current player is black, however, move the king to the C8 square,
    /// and the kingside rook to the D8 square.
    ///
    /// Castling can only be performed if
    /// 1. The king has not moved at all since the game began
    /// 2. The respective rook (kingside or queenside) has also not moved
    /// 3. The square adjacent to the king on the respective side is not threatened by an enemy piece
    ///
    /// If all of these conditions are satisfied, castling is a legal move
    QueenSideCastle,
    /// If the current player is white, move the king to the G1 square, and the kingside rook to
    /// the F1 square. If the current player is black, however, move the king to the G8 square,
    /// and the kingside rook to the F8 square.
    KingSideCastle,
    /// Move a piece from one square to another.
    /// This can allow the player to capture another piece, by
    /// simply moving a piece to the position of an enemy piece.
    ///
    /// Additionally, this can be used to [en-passant capture](https://en.wikipedia.org/wiki/En_passant),
    /// even though the en-passant square itself does not contain any capturable pieces.
    ///
    /// En-passant captures MUST be performed with a pawn, upon an enemy pawn
    /// that has just surpassed it by move two squares. An en-passant capture
    /// must also be performed the turn immediately after the enemy pawn surpasses
    /// the allied pawn. After the one turn a player has to en-passant capture, the
    /// en-passant square is forgotten and can no longer be used.
    Piece(Position, Position),
    /// When played by another player, it awards victory to the other.
    Resign,
}

impl core::fmt::Display for Move {
    // TODO: use PGN formatter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            // Move::EnPassant(from) => write!(f, "ep {}", from),
            Move::Piece(from, to) => write!(f, "{} to {}", from, to),
            Move::KingSideCastle => write!(f, "O-O"),
            Move::QueenSideCastle => write!(f, "O-O-O"),
            Move::Resign => write!(f, "Resign"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn fmt_color() {
        assert_eq!(BLACK.to_string(), "Black");
        assert_eq!(WHITE.to_string(), "White");
    }
}
