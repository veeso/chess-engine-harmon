//! # Board
//!
//! This module contains the data types to work with the chess board.
//! The two most important struct are the `Board` which contains the "game" itself and the
//! `BoardBuilder` which is an helper to build different kind of "games".
//!
//! The board, instead, already provides constructor for:
//!
//! - default chess
//! - horde variant
//! - Dunsany's chess (TODO:)
//!

use super::{Color, Evaluate, GameResult, Move, Piece, Position, Square, BLACK, WHITE};
use crate::position::{
    A1, A2, A3, A4, A7, A8, B1, B5, B8, C1, C5, C8, D1, D8, E1, E8, F1, F5, F8, G1, G5, G8, H1, H8,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::cmp::Ordering;

// Modules
pub mod builder;
// Export
pub use builder::BoardBuilder;

// -- Board

/// ## Board
///
/// Contains the Chess game
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    squares: [Square; 64],

    en_passant: Option<Position>,

    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,

    turn: Color,
}

impl Default for Board {
    fn default() -> Self {
        BoardBuilder::default()
            .piece(Piece::Rook(BLACK, A8))
            .piece(Piece::Knight(BLACK, B8))
            .piece(Piece::Bishop(BLACK, C8))
            .piece(Piece::Queen(BLACK, D8))
            .piece(Piece::King(BLACK, E8))
            .piece(Piece::Bishop(BLACK, F8))
            .piece(Piece::Knight(BLACK, G8))
            .piece(Piece::Rook(BLACK, H8))
            .row(Piece::Pawn(BLACK, A7))
            .row(Piece::Pawn(WHITE, A2))
            .piece(Piece::Rook(WHITE, A1))
            .piece(Piece::Knight(WHITE, B1))
            .piece(Piece::Bishop(WHITE, C1))
            .piece(Piece::Queen(WHITE, D1))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::Bishop(WHITE, F1))
            .piece(Piece::Knight(WHITE, G1))
            .piece(Piece::Rook(WHITE, H1))
            .enable_castling()
            .build()
    }
}

impl Board {
    /// ### horde
    ///
    /// Create the default board for the Horde variant
    pub fn horde() -> Self {
        BoardBuilder::from(Board::default())
            .row(Piece::Pawn(WHITE, A1))
            .row(Piece::Pawn(WHITE, A2))
            .row(Piece::Pawn(WHITE, A3))
            .row(Piece::Pawn(WHITE, A4))
            .piece(Piece::Pawn(WHITE, F5))
            .piece(Piece::Pawn(WHITE, G5))
            .piece(Piece::Pawn(WHITE, B5))
            .piece(Piece::Pawn(WHITE, C5))
            .build()
    }

    /// ### empty
    ///
    /// Create an empty Board
    pub fn empty() -> Self {
        Self {
            squares: [Square::empty(); 64],
            en_passant: None,

            white_castling_rights: CastlingRights::default(),
            black_castling_rights: CastlingRights::default(),

            turn: WHITE,
        }
    }

    /// ### get_turn_color
    ///
    /// Get the color of the current player
    #[inline]
    pub fn get_turn_color(&self) -> Color {
        self.turn
    }

    /// ### get_en_passant
    ///
    /// Get the position of the En-Passant square
    pub fn get_en_passant(&self) -> Option<Position> {
        self.en_passant
    }

    /// ### remove_all
    ///
    /// Remove all of the pieces for a given player
    pub fn remove_all(&self, color: Color) -> Self {
        let mut result = *self;
        for square in &mut result.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    *square = Square::empty()
                }
            }
        }

        result
    }

    /// ### queen_all
    ///
    /// Convert all of a given players pieces to queens
    pub fn queen_all(&self, color: Color) -> Self {
        let mut result = *self;
        for square in &mut result.squares {
            if let Some(piece) = square.get_piece() {
                if !piece.is_king() && piece.get_color() == color {
                    *square = Square::from(Piece::Queen(color, piece.get_pos()))
                }
            }
        }

        result
    }

    /// ### set_turn
    ///
    /// Make the game a certain player's turn
    #[inline]
    pub fn set_turn(&self, color: Color) -> Self {
        let mut result = *self;
        result.turn = color;
        result
    }

    /// ### get_material_advantage
    ///
    /// Get the value of the material advantage of a certain player
    #[inline]
    pub fn get_material_advantage(&self, color: Color) -> i32 {
        self.squares
            .iter()
            .map(|square| match square.get_piece() {
                Some(piece) => {
                    if piece.get_color() == color {
                        piece.get_material_value()
                    } else {
                        -piece.get_material_value()
                    }
                }
                None => 0,
            })
            .sum()
    }

    /// ### get_piece
    ///
    /// Returns the piece at `pos` position
    #[inline]
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_off_board() {
            return None;
        }
        self.squares[((7 - pos.get_row()) * 8 + pos.get_col()) as usize].get_piece()
    }

    /// ### has_ally_piece
    ///
    /// Does a square have an ally piece?
    #[inline]
    pub fn has_ally_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == ally_color
        } else {
            false
        }
    }

    /// ### has_enemy_piece
    ///
    /// If a square at a given position has an enemy piece from a given
    /// ally color, return true. Otherwise, return false.
    ///
    /// For example, if a square has a black piece, and this method is called
    /// upon it with an `ally_color` of `Color::White`, then it will return true.
    /// If called with `Color::Black` upon the same square, however, it will return false.
    #[inline]
    pub fn has_enemy_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == !ally_color
        } else {
            false
        }
    }

    /// ### has_piece
    ///
    /// If a square at a given position has any piece, return true.
    /// Otherwise, return false.
    #[inline]
    pub fn has_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) != None
    }

    /// ### has_no_piece
    ///
    /// If a square at a given position has no piece, return true.
    /// Otherwise, return false.
    #[inline]
    pub fn has_no_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) == None
    }

    /// ### get_king_pos
    ///
    /// If there is a king on the board, return the position that it sits on.
    pub fn get_king_pos(&self, color: Color) -> Option<Position> {
        let mut king_pos = None;
        for square in &self.squares {
            if let Some(Piece::King(c, pos)) = square.get_piece() {
                if c == color {
                    king_pos = Some(pos);
                }
            }
        }
        king_pos
    }

    /// ### is_threatened
    ///
    /// Is a square threatened by an enemy piece?
    pub fn is_threatened(&self, pos: Position, ally_color: Color) -> bool {
        for (i, square) in self.squares.iter().enumerate() {
            let row = 7 - i / 8;
            let col = i % 8;
            let square_pos = Position::new(row as i32, col as i32);
            if !square_pos.is_orthogonal_to(pos)
                && !square_pos.is_diagonal_to(pos)
                && !square_pos.is_knight_move(pos)
            {
                continue;
            }

            if let Some(piece) = square.get_piece() {
                if piece.get_color() == ally_color {
                    continue;
                }

                if piece.is_legal_attack(pos, self) {
                    return true;
                }
            }
        }

        false
    }

    /// ### is_in_check
    ///
    /// Get whether or not the king of a given color is in check.
    #[inline]
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_pos) = self.get_king_pos(color) {
            self.is_threatened(king_pos, color)
        } else {
            false
        }
    }

    /// ### can_kingside_castle
    ///
    /// Can a given player castle kingside?
    pub fn can_kingside_castle(&self, color: Color) -> bool {
        let right_of_king = Position::king_pos(color).next_right();
        match color {
            WHITE => {
                self.has_no_piece(Position::new(0, 5))
                    && self.has_no_piece(Position::new(0, 6))
                    && self.get_piece(Position::new(0, 7))
                        == Some(Piece::Rook(color, Position::new(0, 7)))
                    && self.white_castling_rights.can_kingside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(right_of_king, color)
                    && !self.is_threatened(right_of_king.next_right(), color)
            }
            BLACK => {
                self.has_no_piece(Position::new(7, 5))
                    && self.has_no_piece(Position::new(7, 6))
                    && self.get_piece(Position::new(7, 7))
                        == Some(Piece::Rook(color, Position::new(7, 7)))
                    && self.black_castling_rights.can_kingside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(right_of_king, color)
                    && !self.is_threatened(right_of_king.next_right(), color)
            }
        }
    }

    /// ### can_queenside_castle
    ///
    /// Can a given player castle queenside?
    pub fn can_queenside_castle(&self, color: Color) -> bool {
        match color {
            WHITE => {
                self.has_no_piece(Position::new(0, 1))
                    && self.has_no_piece(Position::new(0, 2))
                    && self.has_no_piece(Position::new(0, 3))
                    && self.get_piece(Position::new(0, 0))
                        == Some(Piece::Rook(color, Position::new(0, 0)))
                    && self.white_castling_rights.can_queenside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::queen_pos(color), color)
            }
            BLACK => {
                self.has_no_piece(Position::new(7, 1))
                    && self.has_no_piece(Position::new(7, 2))
                    && self.has_no_piece(Position::new(7, 3))
                    && self.get_piece(Position::new(7, 0))
                        == Some(Piece::Rook(color, Position::new(7, 0)))
                    && self.black_castling_rights.can_queenside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::queen_pos(color), color)
            }
        }
    }

    /// ### is_legal_move
    ///
    /// Returns whether provided move is a legal move for player
    pub(crate) fn is_legal_move(&self, m: Move, player_color: Color) -> bool {
        match m {
            Move::KingSideCastle => self.can_kingside_castle(player_color),
            Move::QueenSideCastle => self.can_queenside_castle(player_color),
            Move::Piece(from, to) => match self.get_piece(from) {
                Some(Piece::Pawn(c, pos)) => {
                    let piece = Piece::Pawn(c, pos);
                    ((if let Some(en_passant) = self.en_passant {
                        (en_passant == from.pawn_up(player_color).next_left()
                            || en_passant == from.pawn_up(player_color).next_right()
                                && en_passant == to)
                            && c == player_color
                    } else {
                        false
                    }) || piece.is_legal_move(to, self) && piece.get_color() == player_color)
                        && !self.apply_move(m).is_in_check(player_color)
                }
                Some(piece) => {
                    piece.is_legal_move(to, self)
                        && piece.get_color() == player_color
                        && !self.apply_move(m).is_in_check(player_color)
                }
                _ => false,
            },
            Move::Resign => true,
        }
    }
    /// ### has_sufficient_material
    ///
    /// Does the respective player have sufficient material?
    pub fn has_sufficient_material(&self, color: Color) -> bool {
        let mut pieces = vec![];
        for square in &self.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    pieces.push(piece);
                }
            }
        }

        pieces.sort();

        if pieces.len() == 0 {
            false
        } else if pieces.len() == 1 && pieces[0].is_king() {
            false
        } else if pieces.len() == 2 && pieces[0].is_king() && pieces[1].is_knight() {
            false
        } else if pieces.len() == 2 && pieces[0].is_king() && pieces[1].is_bishop() {
            false
        } else if pieces.len() == 3
            && pieces[0].is_king()
            && pieces[1].is_knight()
            && pieces[2].is_knight()
        {
            false
        } else if pieces.len() == 3
            && pieces[0].is_king()
            && pieces[1].is_bishop()
            && pieces[2].is_bishop()
        {
            false
        } else {
            true
        }
    }

    /// ### has_insufficient_material
    ///
    /// Does the respective player have insufficient material?
    #[inline]
    pub fn has_insufficient_material(&self, color: Color) -> bool {
        !self.has_sufficient_material(color)
    }

    /// ### is_stalemate
    ///
    /// Is the current player in stalemate?
    pub fn is_stalemate(&self) -> bool {
        (self.get_legal_moves().is_empty() && !self.is_in_check(self.get_current_player_color()))
            || (self.has_insufficient_material(self.turn)
                && self.has_insufficient_material(!self.turn))
    }

    /// ### is_checkmate
    ///
    /// Is the current player in checkmate?
    pub fn is_checkmate(&self) -> bool {
        self.is_in_check(self.get_current_player_color()) && self.get_legal_moves().is_empty()
    }

    /// ### change_turn
    ///
    /// Change the current turn to the next player.
    #[inline]
    pub fn change_turn(mut self) -> Self {
        self.turn = !self.turn;
        self
    }

    /// ### play_move
    ///
    /// Play a move and confirm it is legal.
    pub fn play_move(&self, m: Move) -> GameResult {
        let current_color = self.get_turn_color();

        if m == Move::Resign {
            GameResult::Victory(!current_color)
        } else if self.is_legal_move(m, current_color) {
            let next_turn = self.apply_move(m).change_turn();
            if next_turn.is_checkmate() {
                GameResult::Victory(current_color)
            } else if next_turn.is_stalemate() {
                GameResult::Stalemate
            } else {
                GameResult::Continuing(next_turn)
            }
        } else {
            GameResult::IllegalMove(m)
        }
    }

    /// ### rating_bar
    ///
    /// print rating bar
    pub fn rating_bar(&self, len: usize) -> String {
        let (best_m, _, your_best_val) = self.get_best_next_move(2);
        let (_, _, your_lowest_val) = self.get_worst_next_move(2);
        let mut your_val = your_best_val + your_lowest_val;
        let (_, _, their_best_val) = self.apply_move(best_m).change_turn().get_best_next_move(2);
        let (_, _, their_lowest_val) = self.apply_move(best_m).change_turn().get_worst_next_move(2);
        let mut their_val = their_best_val + their_lowest_val;

        if your_val < 0.0 {
            your_val = -your_val;
            their_val += your_val * 2.0;
        }

        if their_val < 0.0 {
            their_val = -their_val;
            your_val += their_val * 2.0;
        }

        let your_percentage = your_val / (your_val + their_val);
        let their_percentage = their_val / (your_val + their_val);

        let (your_color, their_color) = match self.turn {
            WHITE => ("▓", "░"),
            BLACK => ("░", "▓"),
        };

        let white = match self.turn {
            WHITE => your_color.repeat((your_percentage * len as f64) as usize),
            BLACK => their_color.repeat((their_percentage * len as f64) as usize),
        };

        let black = match self.turn {
            BLACK => your_color.repeat((your_percentage * len as f64) as usize),
            WHITE => their_color.repeat((their_percentage * len as f64) as usize),
        };

        white + &black
    }

    // -- private

    /// ### get_square
    ///
    /// Get a mutable reference to the square with the provided position.
    /// Panics if position is off_board
    #[inline]
    fn get_square(&mut self, pos: Position) -> &mut Square {
        &mut self.squares[((7 - pos.get_row()) * 8 + pos.get_col()) as usize]
    }

    /// ### add_piece
    ///
    /// Add piece to board
    #[inline]
    fn add_piece(&mut self, piece: Piece) {
        let pos = piece.get_pos();
        *self.get_square(pos) = Square::from(piece);
    }

    /// ### move_piece
    ///
    /// Move piece from `from` position to `to` position
    fn move_piece(&self, from: Position, to: Position) -> Self {
        let mut result = *self;
        result.en_passant = None;

        if from.is_off_board() || to.is_off_board() {
            return result;
        }

        let from_square = result.get_square(from);
        if let Some(mut piece) = from_square.get_piece() {
            *from_square = Square::empty();

            if piece.is_pawn() && (to.get_row() == 0 || to.get_row() == 7) {
                piece = Piece::Queen(piece.get_color(), piece.get_pos());
            }

            if piece.is_starting_pawn() && (from.get_row() - to.get_row()).abs() == 2 {
                result.en_passant = Some(to.pawn_back(piece.get_color()))
            }

            result.add_piece(piece.move_to(to));

            let castling_rights = match piece.get_color() {
                WHITE => &mut result.white_castling_rights,
                BLACK => &mut result.black_castling_rights,
            };

            if piece.is_king() {
                castling_rights.disable_all();
            } else if piece.is_queenside_rook() {
                castling_rights.disable_queenside();
            } else if piece.is_kingside_rook() {
                castling_rights.disable_kingside();
            }
        }

        result
    }

    /// ### apply_move
    ///
    /// TODO: must understand difference between apply and make move
    /// TODO: move match cases to private functions
    fn apply_move(&self, m: Move) -> Self {
        match m {
            Move::KingSideCastle => {
                if let Some(king_pos) = self.get_king_pos(self.turn) {
                    let rook_pos = match self.turn {
                        WHITE => Position::new(0, 7),
                        BLACK => Position::new(7, 7),
                    };
                    self.move_piece(king_pos, rook_pos.next_left())
                        .move_piece(rook_pos, king_pos.next_right())
                } else {
                    *self
                }
            }
            Move::QueenSideCastle => {
                if let Some(king_pos) = self.get_king_pos(self.turn) {
                    let rook_pos = match self.turn {
                        WHITE => Position::new(0, 0),
                        BLACK => Position::new(7, 0),
                    };
                    self.move_piece(king_pos, king_pos.next_left().next_left())
                        .move_piece(rook_pos, king_pos.next_left())
                } else {
                    *self
                }
            }

            Move::Piece(from, to) => {
                let mut result = self.move_piece(from, to);

                if let (Some(en_passant), Some(Piece::Pawn(player_color, _))) =
                    (self.en_passant, self.get_piece(from))
                {
                    if (en_passant == from.pawn_up(player_color).next_left()
                        || en_passant == from.pawn_up(player_color).next_right())
                        && en_passant == to
                    {
                        result.squares[((7 - en_passant.pawn_back(player_color).get_row()) * 8
                            + en_passant.get_col())
                            as usize] = Square::empty();
                    }
                }

                result
            }
            Move::Resign => self.remove_all(self.turn).queen_all(!self.turn),
        }
    }
}

// -- Evaluate

impl Evaluate for Board {
    #[inline]
    fn value_for(&self, ally_color: Color) -> f64 {
        self.squares
            .iter()
            .map(|square| match square.get_piece() {
                Some(piece) => {
                    if piece.get_color() == ally_color {
                        piece.get_weighted_value()
                    } else {
                        -piece.get_weighted_value()
                    }
                }
                None => 0.0,
            })
            .sum()
    }

    #[inline]
    fn get_current_player_color(&self) -> Color {
        self.turn
    }

    #[inline]
    fn apply_eval_move(&self, m: Move) -> Self {
        self.apply_move(m).change_turn()
    }

    #[inline]
    fn get_legal_moves(&self) -> Vec<Move> {
        let mut result = vec![];
        let color = self.get_current_player_color();
        for square in &self.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    result.extend(piece.get_legal_moves(self))
                }
            }
        }

        result
    }

    #[inline]
    fn get_piece_legal_moves(&self, pos: Position) -> Vec<Move> {
        let color = self.get_current_player_color();
        if let Some(piece) = self.get_piece(pos) {
            if piece.get_color() == color {
                return piece.get_legal_moves(self);
            }
        }
        Vec::new()
    }
}

// -- board fmt

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let rating_bar = self.rating_bar(16);
        let abc = if self.turn == WHITE {
            "abcdefgh"
        } else {
            "hgfedcba"
        };

        write!(f, "   {}\n  ╔════════╗", abc)?;
        let mut square_color = !self.turn;
        let height = 8;
        let width = 8;

        for row in 0..height {
            writeln!(f)?;

            let print_row = match self.turn {
                WHITE => height - row - 1,
                BLACK => row,
            };
            write!(f, "{} ║", print_row + 1)?;

            for col in 0..width {
                let print_col = match self.turn {
                    BLACK => width - col - 1,
                    WHITE => col,
                };

                let pos = Position::new(print_row, print_col);

                let s = if let Some(piece) = self.get_piece(pos) {
                    piece.to_string()
                } else {
                    String::from(match square_color {
                        WHITE => "░",
                        BLACK => "▓",
                    })
                };
                if Some(pos) == self.en_passant {
                    write!(f, "\x1b[34m{}\x1b[m\x1b[0m", s)?;
                } else if self.is_threatened(pos, self.turn) {
                    write!(f, "\x1b[31m{}\x1b[m\x1b[0m", s)?;
                } else if self.is_threatened(pos, !self.turn) {
                    write!(f, "\x1b[32m{}\x1b[m\x1b[0m", s)?;
                } else {
                    write!(f, "{}", s)?;
                }

                square_color = !square_color;
            }
            write!(f, "║")?;

            if row == 2 {
                let white_adv = self.get_material_advantage(WHITE);
                let black_adv = self.get_material_advantage(BLACK);

                match white_adv.cmp(&black_adv) {
                    Ordering::Equal => write!(f, " Both sides have equal material")?,
                    Ordering::Greater => write!(f, " White +{} points", white_adv)?,
                    Ordering::Less => write!(f, " Black +{} points", black_adv)?,
                }
            } else if row == 3 {
                write!(f, " {} to move", self.turn)?;
            } else if row == 4 {
                write!(f, " [{}]", rating_bar)?;
            }
            square_color = !square_color;
        }

        write!(f, "\n  ╚════════╝\n   {}\n", abc)
    }
}

// -- Castling rights

/// ### CastlingRights
///
/// Defines the castling rights for the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights {
    kingside: bool,
    queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

impl CastlingRights {
    fn can_kingside_castle(&self) -> bool {
        self.kingside
    }

    fn can_queenside_castle(&self) -> bool {
        self.queenside
    }

    fn disable_kingside(&mut self) {
        self.kingside = false
    }

    fn disable_queenside(&mut self) {
        self.queenside = false
    }

    fn disable_all(&mut self) {
        self.disable_kingside();
        self.disable_queenside()
    }

    fn enable_kingside(&mut self) {
        self.kingside = true
    }

    fn enable_queenside(&mut self) {
        self.queenside = true
    }

    fn enable_all(&mut self) {
        self.enable_kingside();
        self.enable_queenside()
    }
}

// CHECK: promotions?
/*
pub enum Promotion {
    Queen,
    Knight,
    Bishop,
    Rook,
}

pub fn promote(&self, pos: Position, promote: Promotion) -> GameResult;

*/
