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
//! - Dunsany's chess
//!

use super::{Color, Move, Piece, Position, Square, BLACK, WHITE};
use crate::position::{
    A1, A2, A3, A4, A7, A8, B1, B5, B8, C1, C5, C8, D1, D8, E1, E8, F1, F5, F8, G1, G5, G8, H1, H8,
};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::cmp::Ordering;

// Modules
mod builder;
mod castling_rights;
mod types;
// Use
use castling_rights::CastlingRights;
// Export
pub use builder::BoardBuilder;
pub use types::{MoveResult, Promotion, RatedMove};

// -- Board

/// ## Board
///
/// Contains the Chess game itself
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    /// the 64 squares of the chess board
    squares: [Square; 64],
    /// tracks eventually a possible en passant position
    en_passant: Option<Position>,
    /// tracks eventually a taken piece on the last turn
    taken_piece: Option<Piece>,
    /// tracks eventually the possibility to promote a pawn
    promotion: Option<Position>,
    /// castling rights for white player
    white_castling_rights: CastlingRights,
    /// castling rights for black player
    black_castling_rights: CastlingRights,
    /// describes which player has to move the next turn
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
    // -- constructors

    /// ### empty
    ///
    /// Create an empty Board
    pub fn empty() -> Self {
        Self {
            squares: [Square::empty(); 64],
            en_passant: None,
            taken_piece: None,
            promotion: None,
            white_castling_rights: CastlingRights::default(),
            black_castling_rights: CastlingRights::default(),
            turn: WHITE,
        }
    }

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

    /// ### dunsany
    ///
    /// Create the default board for the dunsany's chess
    /// <https://en.wikipedia.org/wiki/Dunsany%27s_chess>
    pub fn dunsany() -> Self {
        BoardBuilder::from(Board::default())
            .row(Piece::Pawn(WHITE, A1))
            .row(Piece::Pawn(WHITE, A2))
            .row(Piece::Pawn(WHITE, A3))
            .row(Piece::Pawn(WHITE, A4))
            .build()
            .change_turn() // NOTE: Black moves first
    }

    // -- getters

    /// ### get_turn
    ///
    /// Get the color of the current player
    #[inline]
    pub fn get_turn(&self) -> Color {
        self.turn
    }

    /// ### get_en_passant
    ///
    /// Get the position of the En-Passant square
    pub fn get_en_passant(&self) -> Option<Position> {
        self.en_passant
    }

    /// ### get_taken_piece
    ///
    /// Get, if any, the taken piece on the last turn
    pub fn get_taken_piece(&self) -> Option<Piece> {
        self.taken_piece
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

    /// ### get_player_pieces
    ///
    /// Get player pieces
    pub fn get_player_pieces(&self, color: Color) -> Vec<Piece> {
        self.squares
            .iter()
            .map(|x| x.get_piece())
            .flatten()
            .filter(|x| x.get_color() == color)
            .collect()
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

    /// ### get_legal_moves
    ///
    /// Returns the list of available moves for player with color `color`
    #[inline]
    pub fn get_legal_moves(&self, color: Color) -> Vec<Move> {
        let mut result = vec![];
        for square in &self.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    result.extend(piece.get_legal_moves(self))
                }
            }
        }

        result
    }

    /// ### get_piece_legal_moves
    ///
    /// Get legal moves for piece at `pos` position
    #[inline]
    pub fn get_piece_legal_moves(&self, pos: Position) -> Vec<Move> {
        match self.get_piece(pos) {
            Some(piece) => piece.get_legal_moves(self),
            None => Vec::new(),
        }
    }

    /// ### get_player_value
    ///
    /// Get the value of the board for a given color.
    /// This subtracts the opponents value, and accounts for piece positions
    /// and material value.
    #[inline]
    pub fn get_player_value(&self, color: Color) -> f64 {
        self.squares
            .iter()
            .map(|square| match square.get_piece() {
                Some(piece) => {
                    if piece.get_color() == color {
                        piece.get_weighted_value()
                    } else {
                        -piece.get_weighted_value()
                    }
                }
                None => 0.0,
            })
            .sum()
    }

    /// ### get_rating
    ///
    /// get rating for two players in percentage.
    /// First value is for white, second value is for black
    pub fn get_rating(&self, depth: usize) -> (f64, f64) {
        // Get turn color in order to switch values later
        let turn_color: Color = self.get_turn();
        // Calculate best and worst move for current player
        let (best_m, your_best_val) = self.get_best_next_move(depth);
        let (_, your_lowest_val) = self.get_worst_next_move(depth);
        let mut your_val: f64 = your_best_val + your_lowest_val;
        // Apply best move and get best move for the other player
        let (_, their_best_val) = self
            .apply_move(best_m)
            .change_turn()
            .get_best_next_move(depth);
        // Apply best move and get worst or the other player
        let (_, their_lowest_val) = self
            .apply_move(best_m)
            .change_turn()
            .get_worst_next_move(depth);
        let mut their_val: f64 = their_best_val + their_lowest_val;

        if your_val < 0.0 {
            your_val = -your_val;
            their_val += your_val * 2.0;
        }

        if their_val < 0.0 {
            their_val = -their_val;
            your_val += their_val * 2.0;
        }

        // Return players percentage
        let your_percentage: f64 = (your_val / (your_val + their_val)) * 100.0;
        let their_percentage: f64 = (their_val / (your_val + their_val)) * 100.0;
        // If turn color is white, return first value as white, otherwise invert
        match turn_color {
            WHITE => (your_percentage, their_percentage),
            BLACK => (their_percentage, your_percentage),
        }
    }

    // -- checks

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
    /// upon it with an `ally_color` of `WHITE`, then it will return true.
    /// If called with `BLACK` upon the same square, however, it will return false.
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
        self.get_piece(pos).is_some()
    }

    /// ### has_no_piece
    ///
    /// If a square at a given position has no piece, return true.
    /// Otherwise, return false.
    #[inline]
    pub fn has_no_piece(&self, pos: Position) -> bool {
        self.get_piece(pos).is_none()
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
    ///
    /// Insufficient material consists of:
    ///
    /// 1. The player only has a king
    /// 2. The player only has a king and a knight
    /// 3. The player only has a king and two knights
    /// 4. The player only has a king and a bishop
    /// 5. The player only has a king and two bishops
    ///
    pub fn has_sufficient_material(&self, color: Color) -> bool {
        // Get and sort player pieces
        let mut pieces = self.get_player_pieces(color);
        pieces.sort();

        !(pieces.is_empty()
            || (pieces.len() == 1 && pieces[0].is_king())
            || (pieces.len() == 2 && pieces[0].is_king() && pieces[1].is_knight())
            || (pieces.len() == 2 && pieces[0].is_king() && pieces[1].is_bishop())
            || (pieces.len() == 3
                && pieces[0].is_king()
                && pieces[1].is_knight()
                && pieces[2].is_knight())
            || (pieces.len() == 3
                && pieces[0].is_king()
                && pieces[1].is_bishop()
                && pieces[2].is_bishop()))
    }

    /// ### has_insufficient_material
    ///
    /// Does the respective player have insufficient material?
    ///
    /// Insufficient material consists of:
    ///
    /// 1. The player only has a king
    /// 2. The player only has a king and a knight
    /// 3. The player only has a king and two knights
    /// 4. The player only has a king and a bishop
    /// 5. The player only has a king and two bishops
    ///
    #[inline]
    pub fn has_insufficient_material(&self, color: Color) -> bool {
        !self.has_sufficient_material(color)
    }

    /// ### is_stalemate
    ///
    /// Is the current player in stalemate?
    pub fn is_stalemate(&self) -> bool {
        (self.get_legal_moves(self.get_turn()).is_empty() && !self.is_in_check(self.get_turn()))
            || (self.has_insufficient_material(self.turn)
                && self.has_insufficient_material(!self.turn))
    }

    /// ### is_checkmate
    ///
    /// Is the current player in checkmate?
    pub fn is_checkmate(&self) -> bool {
        self.is_in_check(self.get_turn()) && self.get_legal_moves(self.get_turn()).is_empty()
    }

    // -- evaluation

    /// ### rate_legal_moves
    ///
    /// Returns the list of legal moves along with their score
    pub fn rate_legal_moves(&self, depth: usize) -> Vec<RatedMove> {
        // Get legal moves
        let legal_moves = self.get_legal_moves(self.get_turn());
        let color = self.get_turn();
        // Run minimax for each legal move
        let rated: Vec<RatedMove> = legal_moves
            .iter()
            .map(|x| {
                (
                    *x,
                    self.apply_move(*x)
                        .minimax(depth, -1000000.0, 1000000.0, false, color),
                )
            })
            .collect();
        rated
    }

    /// ### get_best_next_move
    ///
    /// Get the best move for the current player with `depth` number of moves
    /// of lookahead.
    ///
    /// This method returns
    /// 1. The best move
    /// 2. The rating of the best move
    ///
    /// It's best not to use the rating value by itself for anything, as it
    /// is relative to the other player's move ratings as well.
    pub fn get_best_next_move(&self, depth: usize) -> (Move, f64) {
        match self
            .rate_legal_moves(depth)
            .iter()
            .max_by(|a, b| match a.1 > b.1 {
                true => Ordering::Greater,
                false => Ordering::Less,
            }) {
            Some(values) => *values,
            None => (Move::Resign, -999999.0),
        }
    }

    /// ### get_worst_next_move
    ///
    /// Get the worst move for the current player with `depth` number of moves
    /// of lookahead.
    ///
    /// This method returns
    /// 1. The worst move
    /// 2. The rating of the best move
    ///
    /// It's best not to use the rating value by itself for anything, as it
    /// is relative to the other player's move ratings as well.
    pub fn get_worst_next_move(&self, depth: usize) -> RatedMove {
        let legal_moves = self.get_legal_moves(self.get_turn());

        let color = self.get_turn();
        // Calc worst move
        legal_moves
            .iter()
            .map(|x| {
                (
                    *x,
                    self.apply_move(*x)
                        .minimax(depth, -1000000.0, 1000000.0, true, !color),
                )
            })
            .max_by(|a, b| match a.1 > b.1 {
                true => Ordering::Greater,
                false => Ordering::Less,
            })
            .unwrap_or((Move::Resign, -999999.0))
    }

    // -- modifiers

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

    /// ### remove_piece
    ///
    /// Remove piece at `position`
    /// Does nothing if square is empty
    pub fn remove_piece(&self, position: Position) -> Self {
        let mut result = *self;
        *result.get_square(position) = Square::empty();
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
    /// Panics if a promotion must be performed first
    pub fn play_move(&self, m: Move) -> MoveResult {
        let current_color = self.get_turn();
        // Panic if promotion is available
        if let Some(pos) = self.promotion {
            panic!(
                "A promotion at '{}' must be performed before moving a piece",
                pos
            );
        }
        // Make move
        if m == Move::Resign {
            MoveResult::Victory(!current_color)
        } else if self.is_legal_move(m, current_color) {
            // Apply move and change turn
            let next_turn: Board = self.apply_move(m);
            // If is checkmate, return victory
            if next_turn.change_turn().is_checkmate() {
                MoveResult::Victory(current_color)
            } else if next_turn.change_turn().is_stalemate() {
                // Check stalemate
                MoveResult::Stalemate
            } else {
                // check for promotion
                let next_turn: Board = next_turn.check_available_pawn_promotion();
                // If there's a promotion available, return `Promote`; otherwise return `Continuing` changing player's turn
                match next_turn.promotion {
                    Some(pos_promotion) => MoveResult::Promote(next_turn, pos_promotion),
                    None => MoveResult::Continuing(next_turn.change_turn()),
                }
            }
        } else {
            MoveResult::IllegalMove(m)
        }
    }

    /// ### promote
    ///
    /// Promote the pawn on the last line.
    /// Panics if there is no pawn to promote.
    /// Returns the updated board
    pub fn promote(&self, promotion: Promotion) -> Board {
        let mut result = *self;
        match result.promotion.take() {
            Some(pos) => {
                let color: Color = result.get_turn();
                let promotion: Piece = match promotion {
                    Promotion::Bishop => Piece::Bishop(color, pos),
                    Promotion::Knight => Piece::Knight(color, pos),
                    Promotion::Queen => Piece::Queen(color, pos),
                    Promotion::Rook => Piece::Rook(color, pos),
                };
                result.add_piece(promotion);
                // Change turn and return
                result.change_turn()
            }
            None => panic!("There's no promotion available"),
        }
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
        result.taken_piece = None;

        // If off board, return self
        if from.is_off_board() || to.is_off_board() {
            return result;
        }

        let from_square = result.get_square(from);
        if let Some(piece) = from_square.get_piece() {
            *from_square = Square::empty();

            // Check en passant
            if piece.is_starting_pawn() && (from.get_row() - to.get_row()).abs() == 2 {
                result.en_passant = Some(to.pawn_back(piece.get_color()))
            }

            // Check if there is an enemy piece at `to`
            if result.has_enemy_piece(to, result.get_turn()) {
                result.taken_piece = result.get_piece(to);
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
    /// Apply a move to the board and return a new Board with the move applied
    fn apply_move(&self, m: Move) -> Self {
        match m {
            Move::KingSideCastle => self.apply_kingside_castle(),
            Move::QueenSideCastle => self.apply_queenside_castle(),
            Move::Piece(from, to) => self.apply_piece_move(from, to),
            Move::Resign => *self, // Resign does nothing
        }
    }

    /// ### apply_kingside_castle
    ///
    /// Apply kingside castle to board
    fn apply_kingside_castle(&self) -> Self {
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

    /// ### apply_queenside_castle
    ///
    /// Apply kingside castle to board
    fn apply_queenside_castle(&self) -> Self {
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

    /// ### apply_piece_move
    ///
    /// Move piece from `from` to `to` and eventually handle "en passant"
    fn apply_piece_move(&self, from: Position, to: Position) -> Self {
        // move piece
        let mut result = self.move_piece(from, to);

        // Handle en_passant
        if let (Some(en_passant), Some(Piece::Pawn(player_color, _))) =
            (self.en_passant, self.get_piece(from))
        {
            if (en_passant == from.pawn_up(player_color).next_left()
                || en_passant == from.pawn_up(player_color).next_right())
                && en_passant == to
            {
                result.squares[((7 - en_passant.pawn_back(player_color).get_row()) * 8
                    + en_passant.get_col()) as usize] = Square::empty();
            }
        }

        result
    }

    /// ### check_available_pawn_promotion
    ///
    /// Check whether there is a pawn promotion available
    fn check_available_pawn_promotion(mut self) -> Self {
        let mut promoting_pawn: Option<Position> = None;
        // Search for a pawn which can be promoted
        for square in self.squares.iter() {
            if let Some(piece) = square.get_piece() {
                if piece.is_promoting_pawn() && piece.get_color() == self.get_turn() {
                    promoting_pawn = Some(piece.get_pos());
                }
            }
        }
        // Set promotion
        self.promotion = promoting_pawn;
        self
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
        depth: usize,
        mut alpha: f64,
        mut beta: f64,
        is_maximizing: bool,
        getting_move_for: Color,
    ) -> f64 {
        if depth == 0 {
            return self.get_player_value(getting_move_for);
        }

        let legal_moves = self.get_legal_moves(self.get_turn());
        let mut best_move_value;

        if is_maximizing {
            best_move_value = -999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
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
                let child_board_value = self.apply_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
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

// -- board fmt

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        // Make progress bar
        let (white_score, black_score): (f64, f64) = self.get_rating(2);
        let (your_color, their_color) = match self.turn {
            WHITE => ("▓", "░"),
            BLACK => ("░", "▓"),
        };
        let (your_score, their_score): (f64, f64) = match self.turn {
            WHITE => (white_score, black_score),
            BLACK => (black_score, white_score),
        };

        let white = match self.turn {
            WHITE => your_color.repeat((your_score * 0.16_f64) as usize),
            BLACK => their_color.repeat((their_score * 0.16_f64) as usize),
        };

        let black = match self.turn {
            BLACK => your_color.repeat((your_score * 0.16_f64) as usize),
            WHITE => their_color.repeat((their_score * 0.16_f64) as usize),
        };
        let rating_bar = white + &black;
        // Prepare labels
        let abc = if self.turn == WHITE {
            "abcdefgh"
        } else {
            "hgfedcba"
        };
        // Write board
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::position::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let board: Board = Board::default();
        // Black
        assert_eq!(board.get_piece(A8).unwrap(), Piece::Rook(BLACK, A8));
        assert_eq!(board.get_piece(B8).unwrap(), Piece::Knight(BLACK, B8));
        assert_eq!(board.get_piece(C8).unwrap(), Piece::Bishop(BLACK, C8));
        assert_eq!(board.get_piece(D8).unwrap(), Piece::Queen(BLACK, D8));
        assert_eq!(board.get_piece(E8).unwrap(), Piece::King(BLACK, E8));
        assert_eq!(board.get_piece(F8).unwrap(), Piece::Bishop(BLACK, F8));
        assert_eq!(board.get_piece(G8).unwrap(), Piece::Knight(BLACK, G8));
        assert_eq!(board.get_piece(H8).unwrap(), Piece::Rook(BLACK, H8));
        assert_eq!(board.get_piece(A7).unwrap(), Piece::Pawn(BLACK, A7));
        assert_eq!(board.get_piece(B7).unwrap(), Piece::Pawn(BLACK, B7));
        assert_eq!(board.get_piece(C7).unwrap(), Piece::Pawn(BLACK, C7));
        assert_eq!(board.get_piece(D7).unwrap(), Piece::Pawn(BLACK, D7));
        assert_eq!(board.get_piece(E7).unwrap(), Piece::Pawn(BLACK, E7));
        assert_eq!(board.get_piece(F7).unwrap(), Piece::Pawn(BLACK, F7));
        assert_eq!(board.get_piece(G7).unwrap(), Piece::Pawn(BLACK, G7));
        assert_eq!(board.get_piece(H7).unwrap(), Piece::Pawn(BLACK, H7));
        // White
        assert_eq!(board.get_piece(A2).unwrap(), Piece::Pawn(WHITE, A2));
        assert_eq!(board.get_piece(B2).unwrap(), Piece::Pawn(WHITE, B2));
        assert_eq!(board.get_piece(C2).unwrap(), Piece::Pawn(WHITE, C2));
        assert_eq!(board.get_piece(D2).unwrap(), Piece::Pawn(WHITE, D2));
        assert_eq!(board.get_piece(E2).unwrap(), Piece::Pawn(WHITE, E2));
        assert_eq!(board.get_piece(F2).unwrap(), Piece::Pawn(WHITE, F2));
        assert_eq!(board.get_piece(G2).unwrap(), Piece::Pawn(WHITE, G2));
        assert_eq!(board.get_piece(H2).unwrap(), Piece::Pawn(WHITE, H2));
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Rook(WHITE, A1));
        assert_eq!(board.get_piece(B1).unwrap(), Piece::Knight(WHITE, B1));
        assert_eq!(board.get_piece(C1).unwrap(), Piece::Bishop(WHITE, C1));
        assert_eq!(board.get_piece(D1).unwrap(), Piece::Queen(WHITE, D1));
        assert_eq!(board.get_piece(E1).unwrap(), Piece::King(WHITE, E1));
        assert_eq!(board.get_piece(F1).unwrap(), Piece::Bishop(WHITE, F1));
        assert_eq!(board.get_piece(G1).unwrap(), Piece::Knight(WHITE, G1));
        assert_eq!(board.get_piece(H1).unwrap(), Piece::Rook(WHITE, H1));
        // Castling rights
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // en passant
        assert_eq!(board.en_passant, None);
        // Turn
        assert_eq!(board.turn, WHITE);
    }

    #[test]
    fn empty() {
        let board: Board = Board::empty();
        // Castling rights
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // en passant
        assert_eq!(board.en_passant, None);
        // Turn
        assert_eq!(board.turn, WHITE);
    }

    #[test]
    fn horde() {
        let board: Board = Board::horde();
        // Black
        assert_eq!(board.get_piece(A8).unwrap(), Piece::Rook(BLACK, A8));
        assert_eq!(board.get_piece(B8).unwrap(), Piece::Knight(BLACK, B8));
        assert_eq!(board.get_piece(C8).unwrap(), Piece::Bishop(BLACK, C8));
        assert_eq!(board.get_piece(D8).unwrap(), Piece::Queen(BLACK, D8));
        assert_eq!(board.get_piece(E8).unwrap(), Piece::King(BLACK, E8));
        assert_eq!(board.get_piece(F8).unwrap(), Piece::Bishop(BLACK, F8));
        assert_eq!(board.get_piece(G8).unwrap(), Piece::Knight(BLACK, G8));
        assert_eq!(board.get_piece(H8).unwrap(), Piece::Rook(BLACK, H8));
        assert_eq!(board.get_piece(A7).unwrap(), Piece::Pawn(BLACK, A7));
        assert_eq!(board.get_piece(B7).unwrap(), Piece::Pawn(BLACK, B7));
        assert_eq!(board.get_piece(C7).unwrap(), Piece::Pawn(BLACK, C7));
        assert_eq!(board.get_piece(D7).unwrap(), Piece::Pawn(BLACK, D7));
        assert_eq!(board.get_piece(E7).unwrap(), Piece::Pawn(BLACK, E7));
        assert_eq!(board.get_piece(F7).unwrap(), Piece::Pawn(BLACK, F7));
        assert_eq!(board.get_piece(G7).unwrap(), Piece::Pawn(BLACK, G7));
        assert_eq!(board.get_piece(H7).unwrap(), Piece::Pawn(BLACK, H7));
        // White
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Pawn(WHITE, A1));
        assert_eq!(board.get_piece(B1).unwrap(), Piece::Pawn(WHITE, B1));
        assert_eq!(board.get_piece(C1).unwrap(), Piece::Pawn(WHITE, C1));
        assert_eq!(board.get_piece(D1).unwrap(), Piece::Pawn(WHITE, D1));
        assert_eq!(board.get_piece(E1).unwrap(), Piece::Pawn(WHITE, E1));
        assert_eq!(board.get_piece(F1).unwrap(), Piece::Pawn(WHITE, F1));
        assert_eq!(board.get_piece(G1).unwrap(), Piece::Pawn(WHITE, G1));
        assert_eq!(board.get_piece(H1).unwrap(), Piece::Pawn(WHITE, H1));
        assert_eq!(board.get_piece(A2).unwrap(), Piece::Pawn(WHITE, A2));
        assert_eq!(board.get_piece(B2).unwrap(), Piece::Pawn(WHITE, B2));
        assert_eq!(board.get_piece(C2).unwrap(), Piece::Pawn(WHITE, C2));
        assert_eq!(board.get_piece(D2).unwrap(), Piece::Pawn(WHITE, D2));
        assert_eq!(board.get_piece(E2).unwrap(), Piece::Pawn(WHITE, E2));
        assert_eq!(board.get_piece(F2).unwrap(), Piece::Pawn(WHITE, F2));
        assert_eq!(board.get_piece(G2).unwrap(), Piece::Pawn(WHITE, G2));
        assert_eq!(board.get_piece(H2).unwrap(), Piece::Pawn(WHITE, H2));
        assert_eq!(board.get_piece(A3).unwrap(), Piece::Pawn(WHITE, A3));
        assert_eq!(board.get_piece(B3).unwrap(), Piece::Pawn(WHITE, B3));
        assert_eq!(board.get_piece(C3).unwrap(), Piece::Pawn(WHITE, C3));
        assert_eq!(board.get_piece(D3).unwrap(), Piece::Pawn(WHITE, D3));
        assert_eq!(board.get_piece(E3).unwrap(), Piece::Pawn(WHITE, E3));
        assert_eq!(board.get_piece(F3).unwrap(), Piece::Pawn(WHITE, F3));
        assert_eq!(board.get_piece(G3).unwrap(), Piece::Pawn(WHITE, G3));
        assert_eq!(board.get_piece(H3).unwrap(), Piece::Pawn(WHITE, H3));
        assert_eq!(board.get_piece(A4).unwrap(), Piece::Pawn(WHITE, A4));
        assert_eq!(board.get_piece(B4).unwrap(), Piece::Pawn(WHITE, B4));
        assert_eq!(board.get_piece(C4).unwrap(), Piece::Pawn(WHITE, C4));
        assert_eq!(board.get_piece(D4).unwrap(), Piece::Pawn(WHITE, D4));
        assert_eq!(board.get_piece(E4).unwrap(), Piece::Pawn(WHITE, E4));
        assert_eq!(board.get_piece(F4).unwrap(), Piece::Pawn(WHITE, F4));
        assert_eq!(board.get_piece(G4).unwrap(), Piece::Pawn(WHITE, G4));
        assert_eq!(board.get_piece(H4).unwrap(), Piece::Pawn(WHITE, H4));
        assert_eq!(board.get_piece(F5).unwrap(), Piece::Pawn(WHITE, F5));
        assert_eq!(board.get_piece(G5).unwrap(), Piece::Pawn(WHITE, G5));
        assert_eq!(board.get_piece(B5).unwrap(), Piece::Pawn(WHITE, B5));
        assert_eq!(board.get_piece(C5).unwrap(), Piece::Pawn(WHITE, C5));
        // Castling rights
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // en passant
        assert_eq!(board.en_passant, None);
        // Turn
        assert_eq!(board.turn, WHITE);
    }
    #[test]
    fn dunsany() {
        let board: Board = Board::dunsany();
        // Black
        assert_eq!(board.get_piece(A8).unwrap(), Piece::Rook(BLACK, A8));
        assert_eq!(board.get_piece(B8).unwrap(), Piece::Knight(BLACK, B8));
        assert_eq!(board.get_piece(C8).unwrap(), Piece::Bishop(BLACK, C8));
        assert_eq!(board.get_piece(D8).unwrap(), Piece::Queen(BLACK, D8));
        assert_eq!(board.get_piece(E8).unwrap(), Piece::King(BLACK, E8));
        assert_eq!(board.get_piece(F8).unwrap(), Piece::Bishop(BLACK, F8));
        assert_eq!(board.get_piece(G8).unwrap(), Piece::Knight(BLACK, G8));
        assert_eq!(board.get_piece(H8).unwrap(), Piece::Rook(BLACK, H8));
        assert_eq!(board.get_piece(A7).unwrap(), Piece::Pawn(BLACK, A7));
        assert_eq!(board.get_piece(B7).unwrap(), Piece::Pawn(BLACK, B7));
        assert_eq!(board.get_piece(C7).unwrap(), Piece::Pawn(BLACK, C7));
        assert_eq!(board.get_piece(D7).unwrap(), Piece::Pawn(BLACK, D7));
        assert_eq!(board.get_piece(E7).unwrap(), Piece::Pawn(BLACK, E7));
        assert_eq!(board.get_piece(F7).unwrap(), Piece::Pawn(BLACK, F7));
        assert_eq!(board.get_piece(G7).unwrap(), Piece::Pawn(BLACK, G7));
        assert_eq!(board.get_piece(H7).unwrap(), Piece::Pawn(BLACK, H7));
        // White
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Pawn(WHITE, A1));
        assert_eq!(board.get_piece(B1).unwrap(), Piece::Pawn(WHITE, B1));
        assert_eq!(board.get_piece(C1).unwrap(), Piece::Pawn(WHITE, C1));
        assert_eq!(board.get_piece(D1).unwrap(), Piece::Pawn(WHITE, D1));
        assert_eq!(board.get_piece(E1).unwrap(), Piece::Pawn(WHITE, E1));
        assert_eq!(board.get_piece(F1).unwrap(), Piece::Pawn(WHITE, F1));
        assert_eq!(board.get_piece(G1).unwrap(), Piece::Pawn(WHITE, G1));
        assert_eq!(board.get_piece(H1).unwrap(), Piece::Pawn(WHITE, H1));
        assert_eq!(board.get_piece(A2).unwrap(), Piece::Pawn(WHITE, A2));
        assert_eq!(board.get_piece(B2).unwrap(), Piece::Pawn(WHITE, B2));
        assert_eq!(board.get_piece(C2).unwrap(), Piece::Pawn(WHITE, C2));
        assert_eq!(board.get_piece(D2).unwrap(), Piece::Pawn(WHITE, D2));
        assert_eq!(board.get_piece(E2).unwrap(), Piece::Pawn(WHITE, E2));
        assert_eq!(board.get_piece(F2).unwrap(), Piece::Pawn(WHITE, F2));
        assert_eq!(board.get_piece(G2).unwrap(), Piece::Pawn(WHITE, G2));
        assert_eq!(board.get_piece(H2).unwrap(), Piece::Pawn(WHITE, H2));
        assert_eq!(board.get_piece(A3).unwrap(), Piece::Pawn(WHITE, A3));
        assert_eq!(board.get_piece(B3).unwrap(), Piece::Pawn(WHITE, B3));
        assert_eq!(board.get_piece(C3).unwrap(), Piece::Pawn(WHITE, C3));
        assert_eq!(board.get_piece(D3).unwrap(), Piece::Pawn(WHITE, D3));
        assert_eq!(board.get_piece(E3).unwrap(), Piece::Pawn(WHITE, E3));
        assert_eq!(board.get_piece(F3).unwrap(), Piece::Pawn(WHITE, F3));
        assert_eq!(board.get_piece(G3).unwrap(), Piece::Pawn(WHITE, G3));
        assert_eq!(board.get_piece(H3).unwrap(), Piece::Pawn(WHITE, H3));
        assert_eq!(board.get_piece(A4).unwrap(), Piece::Pawn(WHITE, A4));
        assert_eq!(board.get_piece(B4).unwrap(), Piece::Pawn(WHITE, B4));
        assert_eq!(board.get_piece(C4).unwrap(), Piece::Pawn(WHITE, C4));
        assert_eq!(board.get_piece(D4).unwrap(), Piece::Pawn(WHITE, D4));
        assert_eq!(board.get_piece(E4).unwrap(), Piece::Pawn(WHITE, E4));
        assert_eq!(board.get_piece(F4).unwrap(), Piece::Pawn(WHITE, F4));
        assert_eq!(board.get_piece(G4).unwrap(), Piece::Pawn(WHITE, G4));
        assert_eq!(board.get_piece(H4).unwrap(), Piece::Pawn(WHITE, H4));
        // Castling rights
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // en passant
        assert_eq!(board.en_passant, None);
        // Turn
        assert_eq!(board.turn, BLACK);
    }

    #[test]
    fn get_turn() {
        let board: Board = Board::default();
        assert_eq!(board.get_turn(), WHITE);
        let board: Board = Board::dunsany();
        assert_eq!(board.get_turn(), BLACK);
    }

    #[test]
    fn get_en_passant() {
        let mut board: Board = Board::default();
        assert_eq!(board.get_en_passant(), None);
        board.en_passant = Some(E6);
        assert_eq!(board.get_en_passant().unwrap(), E6);
    }

    #[test]
    fn get_material_advantage() {
        let board: Board = Board::default();
        // Even
        assert_eq!(board.get_material_advantage(WHITE), 0);
        assert_eq!(board.get_material_advantage(BLACK), 0);
        // Take a pawn
        let board: Board = board.remove_piece(E7);
        assert_eq!(board.get_material_advantage(WHITE), 1);
        assert_eq!(board.get_material_advantage(BLACK), -1);
        // Take queen
        let board: Board = board.remove_piece(D1);
        assert_eq!(board.get_material_advantage(WHITE), -8);
        assert_eq!(board.get_material_advantage(BLACK), 8);
    }

    #[test]
    fn get_piece() {
        let board: Board = Board::default();
        assert_eq!(board.get_piece(D1).unwrap(), Piece::Queen(WHITE, D1));
        assert_eq!(board.get_piece(D3), None);
        assert_eq!(board.get_piece(Position::new(24, 24)), None);
    }

    #[test]
    fn get_player_pieces() {
        let board: Board = BoardBuilder::default()
            .piece(Piece::Queen(WHITE, D1))
            .piece(Piece::Bishop(WHITE, D3))
            .piece(Piece::King(BLACK, E8))
            .build();
        assert_eq!(
            board.get_player_pieces(WHITE),
            vec![Piece::Bishop(WHITE, D3), Piece::Queen(WHITE, D1)]
        );
    }

    #[test]
    fn get_king_position() {
        let board: Board = Board::horde();
        assert_eq!(board.get_king_pos(BLACK).unwrap(), E8);
        assert_eq!(board.get_king_pos(WHITE), None);
    }

    #[test]
    fn get_legal_moves() {
        let board: Board = Board::default();
        // Get moves at start
        assert_eq!(
            board.get_legal_moves(WHITE),
            vec![
                // pawns
                Move::Piece(A2, A4),
                Move::Piece(A2, A3),
                Move::Piece(B2, B4),
                Move::Piece(B2, B3),
                Move::Piece(C2, C4),
                Move::Piece(C2, C3),
                Move::Piece(D2, D4),
                Move::Piece(D2, D3),
                Move::Piece(E2, E4),
                Move::Piece(E2, E3),
                Move::Piece(F2, F4),
                Move::Piece(F2, F3),
                Move::Piece(G2, G4),
                Move::Piece(G2, G3),
                Move::Piece(H2, H4),
                Move::Piece(H2, H3),
                // Knights
                Move::Piece(B1, A3),
                Move::Piece(B1, C3),
                Move::Piece(G1, F3),
                Move::Piece(G1, H3),
            ]
        );
        assert_eq!(
            board.get_legal_moves(BLACK),
            vec![
                // Knights
                Move::Piece(B8, A6),
                Move::Piece(B8, C6),
                Move::Piece(G8, F6),
                Move::Piece(G8, H6),
                // pawns
                Move::Piece(A7, A5),
                Move::Piece(A7, A6),
                Move::Piece(B7, B5),
                Move::Piece(B7, B6),
                Move::Piece(C7, C5),
                Move::Piece(C7, C6),
                Move::Piece(D7, D5),
                Move::Piece(D7, D6),
                Move::Piece(E7, E5),
                Move::Piece(E7, E6),
                Move::Piece(F7, F5),
                Move::Piece(F7, F6),
                Move::Piece(G7, G5),
                Move::Piece(G7, G6),
                Move::Piece(H7, H5),
                Move::Piece(H7, H6),
            ]
        );
    }

    #[test]
    fn get_piece_legal_moves() {
        let board: Board = Board::default();
        assert_eq!(
            board.get_piece_legal_moves(B8),
            vec![Move::Piece(B8, A6), Move::Piece(B8, C6)]
        );
        assert_eq!(board.get_piece_legal_moves(E1), vec![]);
        assert_eq!(board.get_piece_legal_moves(E5), vec![]);
    }

    #[test]
    fn get_player_value() {
        let board: Board = Board::default();
        assert_eq!(board.get_player_value(WHITE), 0.0);
        assert_eq!(board.get_player_value(BLACK), 0.0);
        // Take queen
        let board: Board = board.remove_piece(D1);
        assert_eq!(board.get_player_value(WHITE), -89.5);
        assert_eq!(board.get_player_value(BLACK), 89.5);
    }

    #[test]
    fn get_rating() {
        let board: Board = Board::default();
        let rating = board.get_rating(2);
        assert_eq!((rating.0.round() as i64, rating.1.round() as i64), (41, 59));
    }

    #[test]
    fn has_ally_piece() {
        let board: Board = Board::default();
        assert_eq!(board.has_ally_piece(A1, WHITE), true);
        assert_eq!(board.has_ally_piece(E1, BLACK), false);
        assert_eq!(board.has_ally_piece(G8, BLACK), true);
        assert_eq!(board.has_ally_piece(G7, WHITE), false);
    }

    #[test]
    fn has_enemy_piece() {
        let board: Board = Board::default();
        assert_eq!(board.has_enemy_piece(A1, WHITE), false);
        assert_eq!(board.has_enemy_piece(E1, BLACK), true);
        assert_eq!(board.has_enemy_piece(G8, BLACK), false);
        assert_eq!(board.has_enemy_piece(G7, WHITE), true);
    }

    #[test]
    fn has_piece() {
        let board: Board = Board::default();
        assert_eq!(board.has_piece(A1), true);
        assert_eq!(board.has_piece(E6), false);
    }

    #[test]
    fn has_no_piece() {
        let board: Board = Board::default();
        assert_eq!(board.has_no_piece(A1), false);
        assert_eq!(board.has_no_piece(E6), true);
    }

    #[test]
    fn is_threatened() {
        let mut board: Board = Board::default();
        assert_eq!(board.is_threatened(A8, BLACK), false);
        assert_eq!(board.is_threatened(B8, BLACK), false);
        assert_eq!(board.is_threatened(B7, BLACK), false);
        // let's put a white queen at A7
        board.add_piece(Piece::Queen(WHITE, A7));
        assert_eq!(board.is_threatened(A8, BLACK), true);
        assert_eq!(board.is_threatened(B8, BLACK), true);
        assert_eq!(board.is_threatened(B7, BLACK), true);
    }

    #[test]
    fn is_in_check() {
        let mut board: Board = Board::default();
        assert_eq!(board.is_in_check(WHITE), false);
        assert_eq!(board.is_in_check(BLACK), false);
        // Let's put a queen at E7
        board.add_piece(Piece::Queen(WHITE, E7));
        assert_eq!(board.is_in_check(WHITE), false);
        assert_eq!(board.is_in_check(BLACK), true);
        // let's put a queen at E2
        board.add_piece(Piece::Queen(BLACK, E2));
        assert_eq!(board.is_in_check(WHITE), true);
        assert_eq!(board.is_in_check(BLACK), true);
    }

    #[test]
    fn can_kingside_castle() {
        let board: Board = Board::default();
        // Can't castle on kingside at beginning
        assert_eq!(board.can_kingside_castle(WHITE), false);
        assert_eq!(board.can_kingside_castle(BLACK), false);
        // Let's play giuoco piano, which is good for castling
        let board = board.apply_move(Move::Piece(E2, E4)).change_turn();
        let board = board.apply_move(Move::Piece(E7, E5)).change_turn();
        let board = board.apply_move(Move::Piece(G1, G3)).change_turn();
        let board = board.apply_move(Move::Piece(B8, C6)).change_turn();
        let board = board.apply_move(Move::Piece(F1, C4)).change_turn();
        let board = board.apply_move(Move::Piece(F8, C5)).change_turn();
        // Now white can castle
        assert_eq!(board.can_kingside_castle(WHITE), true);
        assert_eq!(board.can_kingside_castle(BLACK), false);
        // White castle
        let board = board.apply_move(Move::KingSideCastle).change_turn();
        let board = board.apply_move(Move::Piece(G8, F6)).change_turn();
        let board = board.apply_move(Move::Piece(D2, D3)).change_turn();
        // Now black can castle
        assert_eq!(board.can_kingside_castle(WHITE), false);
        assert_eq!(board.can_kingside_castle(BLACK), true);
    }

    #[test]
    fn can_queenside_castle() {
        let board: Board = Board::default();
        // Can't castle on kingside at beginning
        assert_eq!(board.can_queenside_castle(WHITE), false);
        assert_eq!(board.can_queenside_castle(BLACK), false);
        // Let's play some random moves to get a queenside castle
        let board = board.apply_move(Move::Piece(E2, E4)).change_turn();
        let board = board.apply_move(Move::Piece(E7, E5)).change_turn();
        let board = board.apply_move(Move::Piece(D1, E2)).change_turn();
        let board = board.apply_move(Move::Piece(D8, E7)).change_turn();
        let board = board.apply_move(Move::Piece(D2, D3)).change_turn();
        let board = board.apply_move(Move::Piece(D7, D6)).change_turn();
        let board = board.apply_move(Move::Piece(C1, E3)).change_turn();
        let board = board.apply_move(Move::Piece(C8, E6)).change_turn();
        let board = board.apply_move(Move::Piece(B1, C3)).change_turn();
        // Now white can castle
        assert_eq!(board.can_queenside_castle(WHITE), true);
        assert_eq!(board.can_queenside_castle(BLACK), false);
        let board = board.apply_move(Move::Piece(B8, C6)).change_turn();
        // Now black can castle
        assert_eq!(board.can_queenside_castle(WHITE), true);
        assert_eq!(board.can_queenside_castle(BLACK), true);
        // Castle
        let board = board.apply_move(Move::QueenSideCastle).change_turn();
        assert_eq!(board.can_queenside_castle(WHITE), false);
        assert_eq!(board.can_queenside_castle(BLACK), true);
        let board = board.apply_move(Move::QueenSideCastle).change_turn();
        assert_eq!(board.can_queenside_castle(WHITE), false);
        assert_eq!(board.can_queenside_castle(BLACK), false);
    }

    #[test]
    fn is_legal_move() {
        let board: Board = Board::default();
        assert_eq!(board.is_legal_move(Move::Resign, WHITE), true);
        assert_eq!(board.is_legal_move(Move::Piece(A2, A4), WHITE), true);
        assert_eq!(board.is_legal_move(Move::Resign, BLACK), true);
        assert_eq!(board.is_legal_move(Move::Piece(A7, A5), BLACK), true);
        assert_eq!(board.is_legal_move(Move::Piece(D1, D3), WHITE), false);
        // Castling
        // Let's play giuoco piano, which is good for castling
        let board = board.apply_move(Move::Piece(E2, E4)).change_turn();
        let board = board.apply_move(Move::Piece(E7, E5)).change_turn();
        let board = board.apply_move(Move::Piece(G1, G3)).change_turn();
        let board = board.apply_move(Move::Piece(B8, C6)).change_turn();
        let board = board.apply_move(Move::Piece(F1, C4)).change_turn();
        let board = board.apply_move(Move::Piece(F8, C5)).change_turn();
        // Now white can castle
        assert_eq!(board.is_legal_move(Move::KingSideCastle, WHITE), true);
        // Queenside castle
        let board: Board = Board::default();
        let board = board.apply_move(Move::Piece(E2, E4)).change_turn();
        let board = board.apply_move(Move::Piece(E7, E5)).change_turn();
        let board = board.apply_move(Move::Piece(D1, E2)).change_turn();
        let board = board.apply_move(Move::Piece(D8, E7)).change_turn();
        let board = board.apply_move(Move::Piece(D2, D3)).change_turn();
        let board = board.apply_move(Move::Piece(D7, D6)).change_turn();
        let board = board.apply_move(Move::Piece(C1, E3)).change_turn();
        let board = board.apply_move(Move::Piece(C8, E6)).change_turn();
        let board = board.apply_move(Move::Piece(B1, C3)).change_turn();
        // Now white can castle
        assert_eq!(board.is_legal_move(Move::QueenSideCastle, WHITE), true);
    }

    #[test]
    fn has_sufficient_material() {
        let board: Board = Board::default();
        assert_eq!(board.has_sufficient_material(WHITE), true);
        assert_eq!(board.has_insufficient_material(WHITE), false);
        let mut board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E1))
            .build();
        // King only
        assert_eq!(board.has_sufficient_material(WHITE), false);
        assert_eq!(board.has_insufficient_material(WHITE), true);
        // One bishop
        board.add_piece(Piece::Bishop(WHITE, E2));
        assert_eq!(board.has_sufficient_material(WHITE), false);
        assert_eq!(board.has_insufficient_material(WHITE), true);
        // One knight
        board.add_piece(Piece::Knight(WHITE, E2));
        assert_eq!(board.has_sufficient_material(WHITE), false);
        assert_eq!(board.has_insufficient_material(WHITE), true);
        // Two knights
        board.add_piece(Piece::Knight(WHITE, E2));
        board.add_piece(Piece::Knight(WHITE, E3));
        assert_eq!(board.has_sufficient_material(WHITE), false);
        assert_eq!(board.has_insufficient_material(WHITE), true);
        // Two bishops
        board.add_piece(Piece::Bishop(WHITE, E2));
        board.add_piece(Piece::Bishop(WHITE, E3));
        assert_eq!(board.has_sufficient_material(WHITE), false);
        assert_eq!(board.has_insufficient_material(WHITE), true);
        // Leave condition
        board.add_piece(Piece::Bishop(WHITE, E2));
        board.add_piece(Piece::Bishop(WHITE, E3));
        board.add_piece(Piece::Knight(WHITE, E4));
        assert_eq!(board.has_sufficient_material(WHITE), true);
        assert_eq!(board.has_insufficient_material(WHITE), false);
    }

    #[test]
    fn is_stalemate() {
        let board: Board = Board::default();
        assert_eq!(board.is_stalemate(), false);
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, G3))
            .piece(Piece::Knight(WHITE, H3))
            .piece(Piece::King(BLACK, H1))
            .build()
            .change_turn();
        assert_eq!(board.is_stalemate(), true);
    }

    #[test]
    fn is_checkmate() {
        let board: Board = Board::default();
        assert_eq!(board.is_checkmate(), false);
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(BLACK, G8))
            .piece(Piece::Queen(WHITE, D8))
            .piece(Piece::Rook(WHITE, B7))
            .piece(Piece::King(WHITE, C3))
            .build()
            .change_turn();
        assert_eq!(board.is_checkmate(), true);
    }

    #[test]
    fn get_best_next_move() {
        let board: Board = Board::default();
        assert_eq!(board.get_best_next_move(2), (Move::Piece(E2, E4), 7.0));
        let board: Board = Board::empty();
        assert_eq!(board.get_best_next_move(2), (Move::Resign, -999999.0));
    }

    #[test]
    fn get_worst_next_move() {
        let board: Board = Board::default();
        assert_eq!(board.get_worst_next_move(2), (Move::Piece(G2, G3), -1.0));
    }

    #[test]
    fn remove_all() {
        let board: Board = Board::default().remove_all(WHITE);
        assert_eq!(board.has_no_piece(E1), true);
        assert_eq!(board.has_no_piece(E8), false);
        let board = board.remove_all(BLACK);
        assert_eq!(board.has_no_piece(E8), true);
    }

    #[test]
    fn remove_piece() {
        let board: Board = Board::default().remove_piece(D1);
        assert_eq!(board.has_no_piece(D1), true);
    }

    #[test]
    fn queen_all() {
        let board: Board = Board::default().queen_all(WHITE);
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Queen(WHITE, A1));
        let board = board.queen_all(BLACK);
        assert_eq!(board.get_piece(A7).unwrap(), Piece::Queen(BLACK, A7));
    }

    #[test]
    fn set_turn() {
        let board: Board = Board::default().set_turn(BLACK);
        assert_eq!(board.get_turn(), BLACK);
        let board = board.set_turn(WHITE);
        assert_eq!(board.get_turn(), WHITE);
    }

    #[test]
    fn change_turn() {
        let board: Board = Board::default().change_turn();
        assert_eq!(board.get_turn(), BLACK);
        let board = board.change_turn();
        assert_eq!(board.get_turn(), WHITE);
    }

    #[test]
    fn play_move() {
        let board: Board = Board::default();
        // Resign
        assert_eq!(board.play_move(Move::Resign), MoveResult::Victory(BLACK));
        // Try illegal move
        assert_eq!(
            board.play_move(Move::KingSideCastle),
            MoveResult::IllegalMove(Move::KingSideCastle)
        );
        // play legal move
        let test_board: Board = board.clone().apply_move(Move::Piece(E2, E4)).change_turn();
        assert_eq!(
            board.play_move(Move::Piece(E2, E4)),
            MoveResult::Continuing(test_board)
        );
        // Checkmate
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(BLACK, G8))
            .piece(Piece::Queen(WHITE, H6))
            .piece(Piece::King(WHITE, F6))
            .build();
        assert_eq!(
            board.play_move(Move::Piece(H6, G7)),
            MoveResult::Victory(WHITE)
        );
        // Stalemate
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(BLACK, H8))
            .piece(Piece::Queen(WHITE, F7))
            .piece(Piece::King(WHITE, E7))
            .build();
        assert_eq!(board.play_move(Move::Piece(E7, F8)), MoveResult::Stalemate);
        // Verify promotion
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, B7))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::King(BLACK, H8))
            .build();
        let mut test_board: Board = board.clone().apply_move(Move::Piece(B7, B8)); // Turn won't change
        test_board.promotion = Some(B8);
        assert_eq!(
            board.play_move(Move::Piece(B7, B8)),
            MoveResult::Promote(test_board, B8)
        );
    }

    #[test]
    #[should_panic]
    fn play_move_promotion_available() {
        let mut board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, B8))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::King(BLACK, H8))
            .build();
        board.promotion = Some(B8);
        board.play_move(Move::Piece(E1, E2));
    }

    #[test]
    fn taken_piece() {
        let mut board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, D4))
            .piece(Piece::Queen(BLACK, E5))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::King(BLACK, H8))
            .build();
        if let MoveResult::Continuing(b) = board.play_move(Move::Piece(D4, E5)) {
            board = b;
        }
        // Verify taken piece
        assert_eq!(board.get_taken_piece().unwrap(), Piece::Queen(BLACK, E5));
        // Make another move (taken piece becomes None)
        if let MoveResult::Continuing(b) = board.play_move(Move::Piece(H8, G7)) {
            board = b;
        }
        assert_eq!(board.get_taken_piece(), None);
    }

    #[test]
    fn promote() {
        let mut board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, B8))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::King(BLACK, H8))
            .build();
        board.promotion = Some(B8);
        assert_eq!(board.get_turn(), WHITE);
        let board: Board = board.promote(Promotion::Queen);
        assert_eq!(board.get_piece(B8).unwrap(), Piece::Queen(WHITE, B8));
        assert_eq!(board.get_turn(), BLACK);
    }

    #[test]
    #[should_panic]
    fn promote_none() {
        let board: Board = Board::default();
        board.promote(Promotion::Queen);
    }

    #[test]
    fn fmt_board() {
        Board::default().to_string();
    }
}
