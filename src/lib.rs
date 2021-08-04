//! # chess-engine-harmon
//!
//! [chess-engine-harmon](https://github.com/veeso/chess-engine-harmon) is a fork of the original [chess engine](https://github.com/adam-mcdaniel/chess-engine) library for rust.
//! It has been designed to provide some extra functionalities compared to the original library and to fit with the tui game [Harmon](https://github.com/veeso/harmon).
//! In addition to this, I've also made some bugfix and added test units in order to make the library safer and more reliable.
//! Feel free to use this library instead of the original one if you prefer.
//!
//! ![chess-image](https://raw.githubusercontent.com/veeso/chess-engine-harmon/main/assets/web-board.png)
//!
//! ## Get Started
//!
//! ### Adding `chess-engine-harmon` as dependency
//!
//! ```toml
//! chess-engine-harmon = "0.1.2"
//! ```
//!
//! ## Example
//!
//! It's quite easy to setup the chess engine, for example this creates a default chess match, with a demonstration
//! on how to make the computer to make a move.
//!
//! ```rust,no_run
//! extern crate chess_engine_harmon;
//!
//! use chess_engine_harmon::{Board, Evaluate, GameResult, Move};
//!
//! fn main() {
//!     let board = Board::default();
//!
//!     // Get the best move with 4 moves of lookahead
//!     let (best_move, _, _) = board.get_best_next_move(4);
//!     // Get the worst move with 3 moves of lookahead
//!     let worst_move = board.get_worst_next_move(3);
//!
//!     // Get all of the possible legal moves for the given player
//!     let legal_moves = board.get_legal_moves();
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
//!         GameResult::Continuing(next_board) => {
//!             println!("{}", next_board);
//!         }
//!         
//!         GameResult::Victory(winner) => {
//!             // You can use the ! operator on a player's
//!             // color to invert.
//!             println!("{} loses. {} is victorious.",
//!               !winner, winner
//!             );
//!         }
//!         
//!         GameResult::IllegalMove(x) => {
//!             eprintln!("{} is an illegal move.", x);
//!         }
//!         
//!         GameResult::Stalemate => {
//!             println!("Drawn game.");
//!         }
//!     }
//! }
//! ```
//!
//! Chess-engine also supports other game modes, such as Horde:
//! ```rust,no_run
//! extern crate chess_engine_harmon;
//!
//! use chess_engine_harmon::{Board, Evaluate, GameResult, Move};
//!
//! // TODO: complete with other variants
//!
//! fn main() {
//!
//! }
//!
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/chess-engine-harmon/main/assets/cargo/chess-engine-harmon-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/chess-engine-harmon/main/assets/cargo/chess-engine-harmon-512.png"
)]
#![no_std]
#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

#[cfg(test)] // NOTE: Enable std for test units
extern crate std;

// -- modules

mod board;
pub use board::{Board, BoardBuilder};

mod piece;
pub use piece::Piece;

mod position;
pub use position::*;

mod square;
pub use square::Square;

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

/// ## GameResult
///
/// Describes the result of a move being played on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    /// The game is not finished, and the game is still in play.
    Continuing(Board),
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner.
    Victory(Color),
    /// The game is drawn. This can be a result of the current player
    /// having no legal moves and not being in check, or because
    /// both players have insufficient material on the board.
    ///
    /// Insufficient material consists of:
    /// 1. The player only has a king
    /// 2. The player only has a king and a knight
    /// 3. The player only has a king and two knights
    /// 4. The player only has a king and a bishop
    /// 5. The player only has a king and two bishops
    ///
    /// In a regular game of chess, threefold repetition also triggers
    /// a stalemate, but this engine does not have builtin support for
    /// threefold repetition detection yet.
    Stalemate,
    /// An illegal move was made. This can include many things,
    /// such as moving a piece through another piece, attempting
    /// to capture an allied piece, moving non-orthogonally or
    /// non-diagonally, or non-knight-like according the rules
    /// governing the movement of the piece. Additionally,
    /// moves that put the player in check, (for example, moving a pinned piece),
    /// are also illegal.
    IllegalMove(Move),
}

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

/// ## Evaluate
///
/// Evaluate a board and extract information, such as the best and worst moves.
pub trait Evaluate: Sized {
    /// ### value_for
    ///
    /// Get the value of the board for a given color.
    /// This subtracts the opponents value, and accounts for piece positions
    /// and material value.
    fn value_for(&self, color: Color) -> f64;

    /// ### get_current_player_color
    ///
    /// Get the current player's color.
    fn get_current_player_color(&self) -> Color;

    /// ### get_legal_moves
    ///
    /// Get the legal moves for the current player.
    fn get_legal_moves(&self) -> Vec<Move>;

    /// ### get_piece_legal_moves
    ///
    /// Get legal moves for piece at provided position
    fn get_piece_legal_moves(&self, pos: Position) -> Vec<Move>;

    /// ### apply_eval_move
    ///
    /// Apply a move to the board for evaluation.
    fn apply_eval_move(&self, m: Move) -> Self;

    /// ### get_best_next_move
    ///
    /// Get the best move for the current player with `depth` number of moves
    /// of lookahead.
    ///
    /// This method returns
    /// 1. The best move
    /// 2. The number of boards evaluated to come to a conclusion
    /// 3. The rating of the best move
    ///
    /// It's best not to use the rating value by itself for anything, as it
    /// is relative to the other player's move ratings as well.
    fn get_best_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -999999.0;
        let mut best_move = Move::Resign;

        let color = self.get_current_player_color();

        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.apply_eval_move(*m).minimax(
                depth,
                -1000000.0,
                1000000.0,
                false,
                color,
                &mut board_count,
            );
            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        (best_move, board_count, best_move_value)
    }

    /// ### get_worst_next_move
    ///
    /// Get the worst move for the current player with `depth` number of moves
    /// of lookahead.
    ///
    /// This method returns
    /// 1. The worst move
    /// 2. The number of boards evaluated to come to a conclusion
    /// 3. The rating of the best move
    ///
    /// It's best not to use the rating value by itself for anything, as it
    /// is relative to the other player's move ratings as well.
    fn get_worst_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -999999.0;
        let mut best_move = Move::Resign;

        let color = self.get_current_player_color();

        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.apply_eval_move(*m).minimax(
                depth,
                -1000000.0,
                1000000.0,
                true,
                !color,
                &mut board_count,
            );

            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        (best_move, board_count, best_move_value)
    }

    /// ### minimax
    ///
    /// Perform minimax on a certain position, and get the minimum or maximum value
    /// for a board. To get the best move, you minimize the values of the possible outcomes from your
    /// own position, and maximize the values of the replies made by the other player.
    ///
    /// In other words, choose moves with the assumption that your opponent will make the
    /// best possible replies to your moves. Moves that are seemingly good, but are easily countered,
    /// are categorically eliminated by this algorithm.
    fn minimax(
        &self,
        depth: i32,
        mut alpha: f64,
        mut beta: f64,
        is_maximizing: bool,
        getting_move_for: Color,
        board_count: &mut u64,
    ) -> f64 {
        *board_count += 1;

        if depth == 0 {
            return self.value_for(getting_move_for);
        }

        let legal_moves = self.get_legal_moves();
        let mut best_move_value;

        if is_maximizing {
            best_move_value = -999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );

                if child_board_value > best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value > alpha {
                    alpha = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        } else {
            best_move_value = 999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );
                if child_board_value < best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value < beta {
                    beta = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        }

        best_move_value
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fmt_color() {
        assert_eq!(BLACK.to_string(), "Black");
        assert_eq!(WHITE.to_string(), "White");
    }
}
