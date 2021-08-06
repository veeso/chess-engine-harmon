//! # Piece
//!
//! Exposes the piece type and its related functions

use super::{Board, Color, Move, Position, BLACK, WHITE};
use alloc::vec::Vec;

/// ## Piece
///
/// A piece on a board.
///
/// Every piece has both a color and a position.
/// These, combined with the type of piece it is,
/// determine things like
/// 1. The validity of legal moves
/// 2. The validity of legal attacks
/// 3. Move generation
/// 4. Material and positional value
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

const WHITE_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
];

const BLACK_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
];

const WHITE_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
];
const BLACK_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
];

const WHITE_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
];

const BLACK_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const WHITE_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const BLACK_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const WHITE_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const BLACK_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const WHITE_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const BLACK_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

impl Piece {
    /// ### get_name
    ///
    /// Get the name of the piece such as `"pawn"` or `"king"`.
    /// All names are lowercase.
    #[inline]
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::King(_, _) => "king",
            Self::Queen(_, _) => "queen",
            Self::Rook(_, _) => "rook",
            Self::Bishop(_, _) => "bishop",
            Self::Knight(_, _) => "knight",
            Self::Pawn(_, _) => "pawn",
        }
    }

    /// ### get_color
    ///
    /// Get the color of a given piece.
    #[inline]
    pub fn get_color(&self) -> Color {
        match self {
            Self::King(c, _)
            | Self::Queen(c, _)
            | Self::Rook(c, _)
            | Self::Bishop(c, _)
            | Self::Knight(c, _)
            | Self::Pawn(c, _) => *c,
        }
    }

    /// ### get_pos
    ///
    /// Get the position of a piece.
    #[inline]
    pub fn get_pos(&self) -> Position {
        match self {
            Self::King(_, p)
            | Self::Queen(_, p)
            | Self::Rook(_, p)
            | Self::Bishop(_, p)
            | Self::Knight(_, p)
            | Self::Pawn(_, p) => *p,
        }
    }

    /// ### get_material_value
    ///
    /// Get the material value for a piece.
    /// | Name | Value |
    /// |-|-|
    /// | King | 99999 |
    /// | Queen | 9 |
    /// | Rook | 5 |
    /// | Bishop | 3 |
    /// | Knight | 3 |
    /// | Pawn | 1 |
    #[inline]
    pub fn get_material_value(&self) -> i32 {
        match self {
            Self::King(_, _) => 99999,
            Self::Queen(_, _) => 9,
            Self::Rook(_, _) => 5,
            Self::Bishop(_, _) => 3,
            Self::Knight(_, _) => 3,
            Self::Pawn(_, _) => 1,
        }
    }

    /// ### get_weighted_value
    ///
    /// Get the weighted value of a piece. This simply factors in position
    /// to the pieces value. For example, a knight that is in the center is
    /// more favorable than a knight on the side of the board. Similarly,
    /// a king in the center of the board is highly unfavorable compared to
    /// a king its respective side.
    ///
    /// Additionally, the weighted value of the piece is 10 times greater than
    /// its material value, plus or minus a weight ranging between 5.0 and -5.0.
    #[inline]
    pub fn get_weighted_value(&self) -> f64 {
        let weights = match self {
            Self::King(c, _) => match c {
                Color::White => WHITE_KING_POSITION_WEIGHTS,
                Color::Black => BLACK_KING_POSITION_WEIGHTS,
            },
            Self::Queen(c, _) => match c {
                Color::White => WHITE_QUEEN_POSITION_WEIGHTS,
                Color::Black => BLACK_QUEEN_POSITION_WEIGHTS,
            },
            Self::Rook(c, _) => match c {
                Color::White => WHITE_ROOK_POSITION_WEIGHTS,
                Color::Black => BLACK_ROOK_POSITION_WEIGHTS,
            },
            Self::Bishop(c, _) => match c {
                Color::White => WHITE_BISHOP_POSITION_WEIGHTS,
                Color::Black => BLACK_BISHOP_POSITION_WEIGHTS,
            },
            Self::Knight(c, _) => match c {
                Color::White => WHITE_KNIGHT_POSITION_WEIGHTS,
                Color::Black => BLACK_KNIGHT_POSITION_WEIGHTS,
            },
            Self::Pawn(c, _) => match c {
                Color::White => WHITE_PAWN_POSITION_WEIGHTS,
                Color::Black => BLACK_PAWN_POSITION_WEIGHTS,
            },
        };
        weights[(7 - self.get_pos().get_row()) as usize][self.get_pos().get_col() as usize]
            + (self.get_material_value() * 10) as f64
    }

    /// ### is_king
    ///
    /// Is this piece a king?
    #[inline]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_, _))
    }

    /// ### is_queen
    ///
    /// Is this piece a queen?
    #[inline]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_, _))
    }

    /// ### is_rook
    ///
    /// Is this piece a rook?
    #[inline]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_, _))
    }

    /// ### is_bishop
    ///
    /// Is this piece a bishop?
    #[inline]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_, _))
    }

    /// ### is_knight
    ///
    /// Is this piece a knight?
    #[inline]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_, _))
    }

    /// ### is_pawn
    ///
    /// Is this piece a pawn?
    #[inline]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_, _))
    }

    /// ### is_starting_pawn
    ///
    /// Is this piece a starting pawn?
    ///
    /// A starting pawn is a pawn that has not been pushed
    /// yet whatsoever.
    #[inline]
    pub fn is_starting_pawn(&self) -> bool {
        if let Self::Pawn(c, pos) = self {
            pos.is_starting_pawn(*c)
        } else {
            false
        }
    }

    /// ### is_queenside_rook
    ///
    /// Is this piece in the starting position for the queenside rook?
    ///
    /// This method will only return true for rooks that are in the position
    /// of the queenside rook, not for any particular rook.
    #[inline]
    pub fn is_queenside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_queenside_rook(self.get_color())
        } else {
            false
        }
    }

    /// ### is_kingside_rook
    ///
    /// Is this piece in the starting position for the kingside rook?
    ///
    /// This method will only return true for rooks that are in the position
    /// of the kingside rook, not for any particular rook.
    #[inline]
    pub fn is_kingside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_kingside_rook(self.get_color())
        } else {
            false
        }
    }

    /// ### move_to
    ///
    /// Change the position of this piece to a new position.
    ///
    /// For example, `Pawn(WHITE, E4).move_to(E5)` will result in
    /// `Pawn(WHITE, E5)`. This does not check for move legality,
    /// it merely creates a new piece with the same color and type, but
    /// with a new position.
    #[inline]
    pub fn move_to(&self, new_pos: Position) -> Self {
        match *self {
            Self::King(c, _) => Self::King(c, new_pos),
            Self::Queen(c, _) => Self::Queen(c, new_pos),
            Self::Rook(c, _) => Self::Rook(c, new_pos),
            Self::Bishop(c, _) => Self::Bishop(c, new_pos),
            Self::Knight(c, _) => Self::Knight(c, new_pos),
            Self::Pawn(c, _) => Self::Pawn(c, new_pos),
        }
    }

    /// ### with_color
    ///
    /// Given a piece and a color, create a copy of the given piece, with the provided color
    #[inline]
    pub fn with_color(&self, color: Color) -> Self {
        match *self {
            Self::King(_, pos) => Self::King(color, pos),
            Self::Queen(_, pos) => Self::Queen(color, pos),
            Self::Rook(_, pos) => Self::Rook(color, pos),
            Self::Bishop(_, pos) => Self::Bishop(color, pos),
            Self::Knight(_, pos) => Self::Knight(color, pos),
            Self::Pawn(_, pos) => Self::Pawn(color, pos),
        }
    }

    /// ### get_legal_moves
    ///
    /// Get the exhaustive list of legal moves for a given piece.
    ///
    /// This is used for move generation.
    #[inline]
    pub(crate) fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        let result: Vec<Move> = match *self {
            Self::Pawn(ally_color, pos) => Self::get_pawn_legal_moves(ally_color, pos, board),
            Self::King(ally_color, pos) => Self::get_king_legal_moves(ally_color, pos, board),
            Self::Queen(ally_color, pos) => Self::get_queen_legal_moves(ally_color, pos, board),
            Self::Rook(ally_color, pos) => Self::get_rook_legal_moves(ally_color, pos, board),
            Self::Bishop(ally_color, pos) => Self::get_bishop_legal_moves(ally_color, pos, board),
            Self::Knight(ally_color, pos) => Self::get_knight_legal_moves(ally_color, pos, board),
        };

        let color = self.get_color();
        // Filter illegal moves and off-boards from result
        result
            .into_iter()
            .filter(|x| match x {
                Move::Piece(from, to) => {
                    if from.is_on_board() && to.is_on_board() {
                        board.is_legal_move(*x, color)
                    } else {
                        false
                    }
                }
                _ => board.is_legal_move(*x, color),
            })
            .collect::<Vec<Move>>()
    }

    /// ### is_legal_move
    ///
    /// Verify that moving to a new position is a legal move.
    #[inline]
    pub(crate) fn is_legal_move(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                Self::is_legal_pawn_move(ally_color, pos, new_pos, board)
            }
            Self::King(_, pos) => pos.is_adjacent_to(new_pos),
            Self::Queen(_, pos) => Self::is_legal_queen_move(pos, new_pos, board),
            Self::Rook(_, pos) => Self::is_legal_rook_move(pos, new_pos, board),
            Self::Bishop(_, pos) => Self::is_legal_bishop_move(pos, new_pos, board),
            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        }
    }

    /// ### is_legal_attack
    ///
    /// Verify that attacking a given square is a legal move.
    #[inline]
    pub(crate) fn is_legal_attack(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                Self::is_legal_pawn_attack(ally_color, pos, new_pos, board)
            }
            Self::King(_, pos) => pos.is_adjacent_to(new_pos),
            Self::Queen(_, pos) => Self::is_legal_queen_attack(pos, new_pos, board),
            Self::Rook(_, pos) => Self::is_legal_rook_attack(pos, new_pos, board),
            Self::Bishop(_, pos) => Self::is_legal_bishop_attack(pos, new_pos, board),

            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        }
    }

    // -- private

    /// ### get_pawn_legal_moves
    ///
    /// Get all legal moves for provided pawn
    fn get_pawn_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let up = pos.pawn_up(ally_color);
        let next_up = up.pawn_up(ally_color);
        let up_left = up.next_left();
        let up_right = up.next_right();

        if let Some(en_passant) = board.get_en_passant() {
            if en_passant == up_left || en_passant == up_right {
                result.push(Move::Piece(pos, en_passant));
            }
        }

        if next_up.is_on_board()
            && pos.is_starting_pawn(ally_color)
            && board.has_no_piece(up)
            && board.has_no_piece(next_up)
        {
            result.push(Move::Piece(pos, next_up))
        }

        if up.is_on_board() && board.has_no_piece(up) {
            result.push(Move::Piece(pos, up))
        }

        // Check up_left NOTE: don't use else if, you can have both of them
        if up_left.is_on_board() && board.has_enemy_piece(up_left, ally_color) {
            result.push(Move::Piece(pos, up.next_left()))
        }
        // Check up_right
        if up_right.is_on_board() && board.has_enemy_piece(up.next_right(), ally_color) {
            result.push(Move::Piece(pos, up.next_right()))
        }
        result
    }

    /// ### get_king_legal_moves
    ///
    /// Get all legal moves for provided king
    fn get_king_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for p in &[
            pos.next_left(),
            pos.next_right(),
            pos.next_above(),
            pos.next_below(),
            pos.next_left().next_above(),
            pos.next_left().next_below(),
            pos.next_right().next_above(),
            pos.next_right().next_below(),
        ] {
            if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                result.push(Move::Piece(pos, *p))
            }
        }
        // Castling; don't  check with else if, you can sometimes do both moves
        if board.can_kingside_castle(ally_color) {
            result.push(Move::KingSideCastle);
        }
        if board.can_queenside_castle(ally_color) {
            result.push(Move::QueenSideCastle);
        }
        result
    }

    /// ### get_queen_legal_moves
    ///
    /// Get all legal moves for provided queen
    fn get_queen_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result = Self::get_bishop_legal_moves(ally_color, pos, board);
        result.extend(Self::get_rook_legal_moves(ally_color, pos, board));
        result
    }

    /// ### get_rook_legal_moves
    ///
    /// Get all legal moves for provided rook
    fn get_rook_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for row in 0..8 {
            let new_pos = Position::new(row, pos.get_col());
            if new_pos != pos
                && !board.has_ally_piece(new_pos, ally_color)
                && new_pos.is_orthogonal_to(pos)
            {
                result.push(Move::Piece(pos, new_pos));
            }
        }
        for col in 0..8 {
            let new_pos = Position::new(pos.get_row(), col);
            if new_pos != pos
                && !board.has_ally_piece(new_pos, ally_color)
                && new_pos.is_orthogonal_to(pos)
            {
                result.push(Move::Piece(pos, new_pos));
            }
        }
        result
    }

    /// ### get_bishop_legal_moves
    ///
    /// Get all legal moves for provided bishop
    fn get_bishop_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                let new_pos = Position::new(row, col);
                if new_pos != pos
                    && !board.has_ally_piece(new_pos, ally_color)
                    && new_pos.is_diagonal_to(pos)
                {
                    result.push(Move::Piece(pos, new_pos));
                }
            }
        }
        result
    }

    /// ### get_pawn_legal_moves
    ///
    /// Get all legal moves for provided pawn
    fn get_knight_legal_moves(ally_color: Color, pos: Position, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for p in &[
            pos.next_left().next_left().next_above(),
            pos.next_left().next_above().next_above(),
            pos.next_left().next_left().next_below(),
            pos.next_left().next_below().next_below(),
            pos.next_right().next_right().next_above(),
            pos.next_right().next_above().next_above(),
            pos.next_right().next_right().next_below(),
            pos.next_right().next_below().next_below(),
        ] {
            if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                result.push(Move::Piece(pos, *p))
            }
        }
        result
    }

    /// ### is_legal_pawn_move
    ///
    /// Checks whether provided move is legal for a pawn
    fn is_legal_pawn_move(
        ally_color: Color,
        pos: Position,
        new_pos: Position,
        board: &Board,
    ) -> bool {
        let up = pos.pawn_up(ally_color);
        let up_left = up.next_left();
        let up_right = up.next_right();

        (if let Some(en_passant) = board.get_en_passant() {
            (en_passant == up_left || en_passant == up_right) && (new_pos == en_passant)
        } else {
            false
        }) || (pos.is_starting_pawn(ally_color)
            && board.has_no_piece(new_pos)
            && board.has_no_piece(up)
            && new_pos == up.pawn_up(ally_color))
            || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_left)
            || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_right)
            || (board.has_no_piece(new_pos) && new_pos == up)
    }

    /// ### is_legal_queen_move
    ///
    /// Checks whether provided move is legal for a queen
    fn is_legal_queen_move(pos: Position, new_pos: Position, board: &Board) -> bool {
        // Queen is union of bishop and rook
        Self::is_legal_rook_move(pos, new_pos, board)
            || Self::is_legal_bishop_move(pos, new_pos, board)
    }

    /// ### is_legal_rook_move
    ///
    /// Checks whether provided move is legal for a rook
    fn is_legal_rook_move(pos: Position, new_pos: Position, board: &Board) -> bool {
        if pos.is_orthogonal_to(new_pos) {
            let mut traveling = pos.orthogonals_to(new_pos);
            traveling.pop();

            for pos in traveling {
                if board.has_piece(pos) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// ### is_legal_bishop_move
    ///
    /// Checks whether provided move is legal for a bishop
    fn is_legal_bishop_move(pos: Position, new_pos: Position, board: &Board) -> bool {
        if pos.is_diagonal_to(new_pos) {
            let mut traveling = pos.diagonals_to(new_pos);
            traveling.pop();

            for pos in traveling {
                if board.has_piece(pos) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// ### is_legal_pawn_attack
    ///
    /// Checks whether provided position is a valid attack for a pawn
    fn is_legal_pawn_attack(
        ally_color: Color,
        pos: Position,
        new_pos: Position,
        board: &Board,
    ) -> bool {
        let up = pos.pawn_up(ally_color);
        (if let Some(en_passant) = board.get_en_passant() {
            (en_passant == up.next_left() || en_passant == up.next_right())
                && (new_pos == en_passant)
        } else {
            false
        }) || new_pos == up.next_left()
            || new_pos == up.next_right()
    }

    /// ### is_legal_queen_attack
    ///
    /// Checks whether provided position is a valid attack for a queen
    fn is_legal_queen_attack(pos: Position, new_pos: Position, board: &Board) -> bool {
        Self::is_legal_rook_attack(pos, new_pos, board)
            || Self::is_legal_bishop_attack(pos, new_pos, board)
    }

    /// ### is_legal_rook_attack
    ///
    /// Checks whether provided position is a valid attack for a rook
    fn is_legal_rook_attack(pos: Position, new_pos: Position, board: &Board) -> bool {
        if pos.is_orthogonal_to(new_pos) {
            let mut traveling = pos.orthogonals_to(new_pos);
            traveling.pop();

            for pos in traveling {
                if board.has_piece(pos) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// ### is_legal_bishop_attack
    ///
    /// Checks whether provided position is a valid attack for a bishop
    fn is_legal_bishop_attack(pos: Position, new_pos: Position, board: &Board) -> bool {
        if pos.is_diagonal_to(new_pos) {
            let mut traveling = pos.diagonals_to(new_pos);
            traveling.pop();

            for pos in traveling {
                if board.has_piece(pos) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self.get_color() {
                WHITE => match self {
                    Self::King(_, _) => "♔",
                    Self::Queen(_, _) => "♕",
                    Self::Rook(_, _) => "♖",
                    Self::Knight(_, _) => "♘",
                    Self::Bishop(_, _) => "♗",
                    Self::Pawn(_, _) => "♙",
                },
                BLACK => match self {
                    Self::King(_, _) => "♚",
                    Self::Queen(_, _) => "♛",
                    Self::Rook(_, _) => "♜",
                    Self::Knight(_, _) => "♞",
                    Self::Bishop(_, _) => "♝",
                    Self::Pawn(_, _) => "♟",
                },
            }
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::board::BoardBuilder;
    use crate::position::*;
    use crate::GameResult;

    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_piece_get_name() {
        assert_eq!(Piece::Bishop(WHITE, A1).get_name(), "bishop");
        assert_eq!(Piece::King(WHITE, A1).get_name(), "king");
        assert_eq!(Piece::Knight(WHITE, A1).get_name(), "knight");
        assert_eq!(Piece::Queen(WHITE, A1).get_name(), "queen");
        assert_eq!(Piece::Pawn(WHITE, A1).get_name(), "pawn");
        assert_eq!(Piece::Rook(WHITE, A1).get_name(), "rook");
    }

    #[test]
    fn test_piece_get_color() {
        assert_eq!(Piece::Bishop(WHITE, A1).get_color(), WHITE);
        assert_eq!(Piece::King(BLACK, A1).get_color(), BLACK);
        assert_eq!(Piece::Knight(WHITE, A1).get_color(), WHITE);
        assert_eq!(Piece::Queen(BLACK, A1).get_color(), BLACK);
        assert_eq!(Piece::Pawn(WHITE, A1).get_color(), WHITE);
        assert_eq!(Piece::Rook(BLACK, A1).get_color(), BLACK);
    }

    #[test]
    fn test_piece_get_pos() {
        assert_eq!(Piece::Bishop(WHITE, D4).get_pos(), D4);
        assert_eq!(Piece::King(BLACK, D4).get_pos(), D4);
        assert_eq!(Piece::Knight(WHITE, D4).get_pos(), D4);
        assert_eq!(Piece::Queen(BLACK, D4).get_pos(), D4);
        assert_eq!(Piece::Pawn(WHITE, D4).get_pos(), D4);
        assert_eq!(Piece::Rook(BLACK, D4).get_pos(), D4);
    }

    #[test]
    fn test_piece_get_material_value() {
        assert_eq!(Piece::Bishop(WHITE, D4).get_material_value(), 3);
        assert_eq!(Piece::King(BLACK, D4).get_material_value(), 99999);
        assert_eq!(Piece::Knight(WHITE, D4).get_material_value(), 3);
        assert_eq!(Piece::Queen(BLACK, D4).get_material_value(), 9);
        assert_eq!(Piece::Pawn(WHITE, D4).get_material_value(), 1);
        assert_eq!(Piece::Rook(BLACK, D4).get_material_value(), 5);
    }

    #[test]
    fn test_piece_get_weighted_value() {
        assert_eq!(Piece::Bishop(WHITE, C2).get_weighted_value(), 30.0);
        assert_eq!(Piece::Bishop(BLACK, C2).get_weighted_value(), 30.0);
        assert_eq!(Piece::King(WHITE, D4).get_weighted_value(), 999986.0);
        assert_eq!(Piece::King(BLACK, D4).get_weighted_value(), 999985.0);
        assert_eq!(Piece::Knight(WHITE, E5).get_weighted_value(), 32.0);
        assert_eq!(Piece::Knight(BLACK, E5).get_weighted_value(), 32.0);
        assert_eq!(Piece::Queen(WHITE, F6).get_weighted_value(), 90.5);
        assert_eq!(Piece::Queen(BLACK, F6).get_weighted_value(), 90.5);
        assert_eq!(Piece::Pawn(WHITE, H4).get_weighted_value(), 10.0);
        assert_eq!(Piece::Pawn(BLACK, H4).get_weighted_value(), 10.5);
        assert_eq!(Piece::Rook(WHITE, A2).get_weighted_value(), 49.5);
        assert_eq!(Piece::Rook(BLACK, A2).get_weighted_value(), 50.5);
    }

    #[test]
    fn test_piece_is_king() {
        assert_eq!(Piece::King(WHITE, D4).is_king(), true);
        assert_eq!(Piece::Bishop(BLACK, C2).is_king(), false);
    }

    #[test]
    fn test_piece_is_queen() {
        assert_eq!(Piece::Queen(WHITE, D4).is_queen(), true);
        assert_eq!(Piece::Bishop(BLACK, C2).is_queen(), false);
    }

    #[test]
    fn test_piece_is_rook() {
        assert_eq!(Piece::Rook(WHITE, D4).is_rook(), true);
        assert_eq!(Piece::Bishop(BLACK, C2).is_rook(), false);
    }

    #[test]
    fn test_piece_is_bishop() {
        assert_eq!(Piece::Bishop(BLACK, C2).is_bishop(), true);
        assert_eq!(Piece::Rook(WHITE, D4).is_bishop(), false);
    }

    #[test]
    fn test_piece_is_pawn() {
        assert_eq!(Piece::Pawn(BLACK, C2).is_pawn(), true);
        assert_eq!(Piece::Rook(WHITE, D4).is_pawn(), false);
    }

    #[test]
    fn test_piece_is_knight() {
        assert_eq!(Piece::Knight(BLACK, C2).is_knight(), true);
        assert_eq!(Piece::Rook(WHITE, D4).is_knight(), false);
    }

    #[test]
    fn test_piece_is_starting_pawn() {
        // White pawns
        assert_eq!(Piece::Pawn(WHITE, B2).is_starting_pawn(), true);
        assert_eq!(Piece::Pawn(WHITE, C2).is_starting_pawn(), true);
        assert_eq!(Piece::Pawn(BLACK, B2).is_starting_pawn(), false);
        assert_eq!(Piece::Pawn(BLACK, C2).is_starting_pawn(), false);
        // Black pawns
        assert_eq!(Piece::Pawn(BLACK, B7).is_starting_pawn(), true);
        assert_eq!(Piece::Pawn(BLACK, C7).is_starting_pawn(), true);
        assert_eq!(Piece::Pawn(WHITE, B7).is_starting_pawn(), false);
        assert_eq!(Piece::Pawn(WHITE, C7).is_starting_pawn(), false);
        // others
        assert_eq!(Piece::Queen(WHITE, C2).is_starting_pawn(), false);
        assert_eq!(Piece::Bishop(BLACK, C7).is_starting_pawn(), false);
    }

    #[test]
    fn test_piece_is_queenside_rook() {
        assert_eq!(Piece::Rook(WHITE, A1).is_queenside_rook(), true);
        assert_eq!(Piece::Rook(BLACK, A8).is_queenside_rook(), true);
        assert_eq!(Piece::Rook(BLACK, A1).is_queenside_rook(), false);
        assert_eq!(Piece::Rook(WHITE, A8).is_queenside_rook(), false);
    }

    #[test]
    fn test_piece_is_kingside_rook() {
        assert_eq!(Piece::Rook(WHITE, H1).is_kingside_rook(), true);
        assert_eq!(Piece::Rook(BLACK, H8).is_kingside_rook(), true);
        assert_eq!(Piece::Rook(BLACK, H1).is_kingside_rook(), false);
        assert_eq!(Piece::Rook(WHITE, H8).is_kingside_rook(), false);
    }

    #[test]
    fn test_piece_move_to() {
        assert_eq!(Piece::Rook(WHITE, H1).move_to(H8), Piece::Rook(WHITE, H8));
    }

    #[test]
    fn test_piece_with_color() {
        assert_eq!(
            Piece::Rook(WHITE, H1).with_color(BLACK),
            Piece::Rook(BLACK, H1)
        );
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_starting_position() {
        // Starting pawn (white)
        assert_eq!(
            Piece::Pawn(WHITE, E2).get_legal_moves(&Board::default()),
            vec![Move::Piece(E2, E4), Move::Piece(E2, E3)]
        );
        // Starting pawn (black)
        assert_eq!(
            Piece::Pawn(BLACK, F7).get_legal_moves(&Board::default()),
            vec![Move::Piece(F7, F5), Move::Piece(F7, F6)]
        );
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_last_position() {
        // Starting pawn (white)
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, H8))
            .piece(Piece::Pawn(BLACK, A1))
            .build();
        assert_eq!(Piece::Pawn(WHITE, H8).get_legal_moves(&board), vec![]);
        // Starting pawn (black)
        assert_eq!(Piece::Pawn(BLACK, A1).get_legal_moves(&board), vec![]);
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_already_moved() {
        // Not starting position pawn
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, E3))
            .build();
        assert_eq!(
            Piece::Pawn(WHITE, E3).get_legal_moves(&board),
            vec![Move::Piece(E3, E4)]
        );
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_can_take() {
        // Can take pawn (white)
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, E4))
            .piece(Piece::Pawn(BLACK, D5))
            .piece(Piece::Pawn(BLACK, F5))
            .build();
        assert_eq!(
            Piece::Pawn(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E5),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5)
            ]
        );
        // Can take pawn (black)
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(BLACK, E5))
            .piece(Piece::Pawn(WHITE, D4))
            .piece(Piece::Pawn(WHITE, F4))
            .build();
        assert_eq!(
            Piece::Pawn(BLACK, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, E4),
                Move::Piece(E5, D4),
                Move::Piece(E5, F4)
            ]
        );
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_opposite() {
        // Opposite pawn
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, E4))
            .piece(Piece::Pawn(BLACK, E5))
            .build();
        assert_eq!(Piece::Pawn(WHITE, E4).get_legal_moves(&board), vec![]);
        assert_eq!(Piece::Pawn(BLACK, E5).get_legal_moves(&board), vec![]);
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_en_passant() {
        // En passant - edge case 😈
        let mut board: Board = Board::default();
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(E2, E4)) {
            // White pawn
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(H7, H5)) {
            // Black pawn
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(E4, E5)) {
            // White pawn
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(F7, F5)) {
            // Black pawn
            board = next_board;
        }
        assert_eq!(
            Piece::Pawn(WHITE, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, F6), // En passant
                Move::Piece(E5, E6),
            ]
        );
    }

    #[test]
    fn test_piece_get_pawn_legal_moves_discovered_check() {
        // Discovered check by rook
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Pawn(WHITE, E4))
            .piece(Piece::Pawn(BLACK, F5)) // Could capture, but discovered check by rook
            .piece(Piece::Rook(BLACK, E8))
            .piece(Piece::King(WHITE, E1))
            .build();
        assert_eq!(
            Piece::Pawn(WHITE, E4).get_legal_moves(&board),
            vec![Move::Piece(E4, E5)]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_starting_position() {
        // King without free squares
        assert_eq!(
            Piece::King(WHITE, E1).get_legal_moves(&Board::default()),
            vec![]
        );
        assert_eq!(
            Piece::King(BLACK, E8).get_legal_moves(&Board::default()),
            vec![]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_free_squares() {
        // King in the middle of the board, no threatened
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E5))
            .build();
        assert_eq!(
            Piece::King(WHITE, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, D5),
                Move::Piece(E5, F5),
                Move::Piece(E5, E6),
                Move::Piece(E5, E4),
                Move::Piece(E5, D6),
                Move::Piece(E5, D4),
                Move::Piece(E5, F6),
                Move::Piece(E5, F4),
            ]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_rook_on_left() {
        // King in the middle of the board, rook on the left
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E5))
            .piece(Piece::Rook(BLACK, D8))
            .build();
        assert_eq!(
            Piece::King(WHITE, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, F5),
                Move::Piece(E5, E6),
                Move::Piece(E5, E4),
                Move::Piece(E5, F6),
                Move::Piece(E5, F4),
            ]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_rook_on_left_can_take() {
        // King in the middle of the board, rook on the left, can take rook
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E5))
            .piece(Piece::Rook(BLACK, D5))
            .build();
        assert_eq!(
            Piece::King(WHITE, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, D5),
                Move::Piece(E5, E6),
                Move::Piece(E5, E4),
                Move::Piece(E5, F6),
                Move::Piece(E5, F4),
            ]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_protected_rook_check() {
        // King, checked by a protected rook on the left
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E5))
            .piece(Piece::Rook(BLACK, D5))
            .piece(Piece::Pawn(BLACK, C6))
            .build();
        assert_eq!(
            Piece::King(WHITE, E5).get_legal_moves(&board),
            vec![
                Move::Piece(E5, E6),
                Move::Piece(E5, E4),
                Move::Piece(E5, F6),
                Move::Piece(E5, F4),
            ]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_castling() {
        // Kingside Castling (giuoco piano)
        let mut board: Board = Board::default();
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(E2, E4)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(E7, E5)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(G1, F3)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(B8, C6)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(F1, C4)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(F8, C5)) {
            board = next_board;
        }
        assert_eq!(
            Piece::King(WHITE, E1).get_legal_moves(&board),
            vec![
                Move::Piece(E1, F1),
                Move::Piece(E1, E2),
                Move::KingSideCastle, // Castle
            ]
        );
        // Also black can castle now
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(C2, C3)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(G8, F6)) {
            board = next_board;
        }
        assert_eq!(
            Piece::King(BLACK, E8).get_legal_moves(&board),
            vec![
                Move::Piece(E8, F8),
                Move::Piece(E8, E7),
                Move::KingSideCastle
            ]
        );
        // Allow also queenside castling
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(D1, A4)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(D8, E7)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(B1, A3)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(B7, B5)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(D2, D4)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(C8, B7)) {
            board = next_board;
        }
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(C1, G5)) {
            board = next_board;
        }
        assert_eq!(
            Piece::King(BLACK, E8).get_legal_moves(&board),
            vec![
                Move::Piece(E8, D8),
                Move::Piece(E8, F8),
                Move::KingSideCastle,
                Move::QueenSideCastle
            ]
        );
        if let GameResult::Continuing(next_board) = board.play_move(Move::Piece(E8, C8)) {
            board = next_board;
        }
        assert_eq!(
            Piece::King(WHITE, E1).get_legal_moves(&board),
            vec![
                Move::Piece(E1, D1),
                Move::Piece(E1, F1),
                Move::Piece(E1, E2),
                Move::Piece(E1, D2),
                Move::KingSideCastle,
                Move::QueenSideCastle
            ]
        );
    }

    #[test]
    fn test_piece_get_king_legal_moves_check_mate() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::Queen(BLACK, E2))
            .piece(Piece::Rook(BLACK, E8))
            .build();
        assert_eq!(Piece::King(WHITE, E1).get_legal_moves(&board), vec![]);
    }

    #[test]
    fn test_piece_get_queen_legal_moves_free() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Queen(WHITE, E4))
            .build();
        assert_eq!(
            Piece::Queen(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, B1),
                Move::Piece(E4, H1),
                Move::Piece(E4, C2),
                Move::Piece(E4, G2),
                Move::Piece(E4, D3),
                Move::Piece(E4, F3),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5),
                Move::Piece(E4, C6),
                Move::Piece(E4, G6),
                Move::Piece(E4, B7),
                Move::Piece(E4, H7),
                Move::Piece(E4, A8),
                Move::Piece(E4, E1),
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
                Move::Piece(E4, E8),
                Move::Piece(E4, A4),
                Move::Piece(E4, B4),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
                Move::Piece(E4, H4),
            ]
        );
    }

    #[test]
    fn test_piece_get_queen_legal_moves_blocked() {
        assert_eq!(
            Piece::Queen(WHITE, D1).get_legal_moves(&Board::default()),
            vec![]
        );
        assert_eq!(
            Piece::Queen(BLACK, D8).get_legal_moves(&Board::default()),
            vec![]
        );
    }

    #[test]
    fn test_piece_get_queen_legal_moves_piece_on_diagonal() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Queen(WHITE, D4))
            .piece(Piece::Bishop(BLACK, E3))
            .piece(Piece::Bishop(BLACK, E5))
            .piece(Piece::Bishop(BLACK, C3))
            .piece(Piece::Bishop(BLACK, C5))
            .build();
        assert_eq!(
            Piece::Queen(WHITE, D4).get_legal_moves(&board),
            vec![
                Move::Piece(D4, C3),
                Move::Piece(D4, E3),
                Move::Piece(D4, C5),
                Move::Piece(D4, E5),
                Move::Piece(D4, D1),
                Move::Piece(D4, D2),
                Move::Piece(D4, D3),
                Move::Piece(D4, D5),
                Move::Piece(D4, D6),
                Move::Piece(D4, D7),
                Move::Piece(D4, D8),
                Move::Piece(D4, A4),
                Move::Piece(D4, B4),
                Move::Piece(D4, C4),
                Move::Piece(D4, E4),
                Move::Piece(D4, F4),
                Move::Piece(D4, G4),
                Move::Piece(D4, H4),
            ]
        );
    }

    #[test]
    fn test_piece_get_queen_legal_moves_piece_on_line() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Queen(WHITE, E4))
            .piece(Piece::Rook(BLACK, C4))
            .piece(Piece::Rook(BLACK, G4))
            .build();
        assert_eq!(
            Piece::Queen(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, B1),
                Move::Piece(E4, H1),
                Move::Piece(E4, C2),
                Move::Piece(E4, G2),
                Move::Piece(E4, D3),
                Move::Piece(E4, F3),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5),
                Move::Piece(E4, C6),
                Move::Piece(E4, G6),
                Move::Piece(E4, B7),
                Move::Piece(E4, H7),
                Move::Piece(E4, A8),
                Move::Piece(E4, E1),
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
                Move::Piece(E4, E8),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
            ]
        );
    }

    #[test]
    fn test_piece_get_queen_legal_moves_piece_on_column() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Queen(WHITE, E4))
            .piece(Piece::Rook(BLACK, E2))
            .piece(Piece::Rook(BLACK, E6))
            .build();
        assert_eq!(
            Piece::Queen(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, B1),
                Move::Piece(E4, H1),
                Move::Piece(E4, C2),
                Move::Piece(E4, G2),
                Move::Piece(E4, D3),
                Move::Piece(E4, F3),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5),
                Move::Piece(E4, C6),
                Move::Piece(E4, G6),
                Move::Piece(E4, B7),
                Move::Piece(E4, H7),
                Move::Piece(E4, A8),
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, A4),
                Move::Piece(E4, B4),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
                Move::Piece(E4, H4),
            ]
        );
    }

    #[test]
    fn test_piece_get_queen_legal_moves_discovered_check() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Queen(WHITE, E4))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::Rook(BLACK, E8))
            .build();
        assert_eq!(
            Piece::Queen(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
                Move::Piece(E4, E8),
            ]
        );
    }

    #[test]
    fn test_piece_get_rook_legal_moves_free() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Rook(WHITE, E4))
            .build();
        assert_eq!(
            Piece::Rook(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E1),
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
                Move::Piece(E4, E8),
                Move::Piece(E4, A4),
                Move::Piece(E4, B4),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
                Move::Piece(E4, H4),
            ]
        );
    }

    #[test]
    fn test_piece_get_rook_legal_moves_blocked() {
        assert_eq!(
            Piece::Rook(WHITE, A1).get_legal_moves(&Board::default()),
            vec![]
        );
        assert_eq!(
            Piece::Rook(BLACK, H8).get_legal_moves(&Board::default()),
            vec![]
        );
    }

    #[test]
    fn test_piece_get_rook_legal_moves_piece_on_col() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Rook(WHITE, E4))
            .piece(Piece::Knight(BLACK, E6))
            .piece(Piece::Knight(WHITE, E1))
            .build();
        assert_eq!(
            Piece::Rook(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, A4),
                Move::Piece(E4, B4),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
                Move::Piece(E4, H4),
            ]
        );
    }

    #[test]
    fn test_piece_get_rook_legal_moves_piece_on_row() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Rook(WHITE, E4))
            .piece(Piece::Queen(BLACK, G4))
            .piece(Piece::Queen(WHITE, B4))
            .build();
        assert_eq!(
            Piece::Rook(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E1),
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
                Move::Piece(E4, E8),
                Move::Piece(E4, C4),
                Move::Piece(E4, D4),
                Move::Piece(E4, F4),
                Move::Piece(E4, G4),
            ]
        );
    }

    #[test]
    fn test_piece_get_rook_legal_moves_discovered_check() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Rook(WHITE, E4))
            .piece(Piece::King(WHITE, E1))
            .piece(Piece::Rook(BLACK, E7))
            .build();
        assert_eq!(
            Piece::Rook(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, E2),
                Move::Piece(E4, E3),
                Move::Piece(E4, E5),
                Move::Piece(E4, E6),
                Move::Piece(E4, E7),
            ]
        );
    }

    #[test]
    fn test_piece_get_bishop_legal_moves_free() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Bishop(WHITE, E4))
            .build();
        assert_eq!(
            Piece::Bishop(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, B1),
                Move::Piece(E4, H1),
                Move::Piece(E4, C2),
                Move::Piece(E4, G2),
                Move::Piece(E4, D3),
                Move::Piece(E4, F3),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5),
                Move::Piece(E4, C6),
                Move::Piece(E4, G6),
                Move::Piece(E4, B7),
                Move::Piece(E4, H7),
                Move::Piece(E4, A8),
            ]
        );
    }
    #[test]
    fn test_piece_get_bishop_legal_moves_blocked() {
        assert_eq!(
            Piece::Bishop(WHITE, C1).get_legal_moves(&Board::default()),
            vec![]
        );
        assert_eq!(
            Piece::Bishop(BLACK, F8).get_legal_moves(&Board::default()),
            vec![]
        );
    }

    #[test]
    fn test_piece_get_bishop_legal_moves_piece_on_diagonal() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Bishop(WHITE, E4))
            .piece(Piece::Bishop(BLACK, G6))
            .piece(Piece::Pawn(WHITE, C2))
            .build();
        assert_eq!(
            Piece::Bishop(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, H1),
                Move::Piece(E4, G2),
                Move::Piece(E4, D3),
                Move::Piece(E4, F3),
                Move::Piece(E4, D5),
                Move::Piece(E4, F5),
                Move::Piece(E4, C6),
                Move::Piece(E4, G6),
                Move::Piece(E4, B7),
                Move::Piece(E4, A8),
            ]
        );
    }

    #[test]
    fn test_piece_get_bishop_legal_moves_discovered_check() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Bishop(WHITE, E4))
            .piece(Piece::Bishop(BLACK, G6))
            .piece(Piece::King(WHITE, C2))
            .build();
        assert_eq!(
            Piece::Bishop(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, D3),
                Move::Piece(E4, F5),
                Move::Piece(E4, G6),
            ]
        );
    }

    #[test]
    fn test_piece_get_knight_legal_moves_free() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Knight(WHITE, E4))
            .build();
        assert_eq!(
            Piece::Knight(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, C5),
                Move::Piece(E4, D6),
                Move::Piece(E4, C3),
                Move::Piece(E4, D2),
                Move::Piece(E4, G5),
                Move::Piece(E4, F6),
                Move::Piece(E4, G3),
                Move::Piece(E4, F2),
            ]
        );
    }

    #[test]
    fn test_piece_get_knight_legal_moves_busy() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Knight(WHITE, E4))
            .piece(Piece::Knight(BLACK, C5))
            .piece(Piece::Knight(WHITE, D6))
            .build();
        assert_eq!(
            Piece::Knight(WHITE, E4).get_legal_moves(&board),
            vec![
                Move::Piece(E4, C5),
                Move::Piece(E4, C3),
                Move::Piece(E4, D2),
                Move::Piece(E4, G5),
                Move::Piece(E4, F6),
                Move::Piece(E4, G3),
                Move::Piece(E4, F2),
            ]
        );
    }

    #[test]
    fn test_piece_get_knight_legal_moves_discovered_check() {
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .piece(Piece::Knight(WHITE, E4))
            .piece(Piece::King(WHITE, B1))
            .piece(Piece::Bishop(BLACK, H7))
            .build();
        assert_eq!(Piece::Knight(WHITE, E4).get_legal_moves(&board), vec![]);
    }

    #[test]
    fn test_piece_fmt() {
        assert_eq!(Piece::Bishop(WHITE, A1).to_string(), "♗");
        assert_eq!(Piece::Bishop(BLACK, A1).to_string(), "♝");
        assert_eq!(Piece::King(WHITE, A1).to_string(), "♔");
        assert_eq!(Piece::King(BLACK, A1).to_string(), "♚");
        assert_eq!(Piece::Knight(WHITE, A1).to_string(), "♘");
        assert_eq!(Piece::Knight(BLACK, A1).to_string(), "♞");
        assert_eq!(Piece::Queen(WHITE, A1).to_string(), "♕");
        assert_eq!(Piece::Queen(BLACK, A1).to_string(), "♛");
        assert_eq!(Piece::Pawn(WHITE, A1).to_string(), "♙");
        assert_eq!(Piece::Pawn(BLACK, A1).to_string(), "♟");
        assert_eq!(Piece::Rook(WHITE, A1).to_string(), "♖");
        assert_eq!(Piece::Rook(BLACK, A1).to_string(), "♜");
    }
}
