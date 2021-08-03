// TODO: PGN parsers

// <https://en.wikipedia.org/wiki/Portable_Game_Notation>

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::convert::TryFrom;

/// Try to parse a Move from a string.
///
/// Possible valid formats include:
/// - `"resign"`
/// - `"resigns"`
/// - `"castle queenside"`
/// - `"O-O-O"` (correct notation)
/// - `"o-o-o"` (incorrect notation, but will accept)
/// - `"0-0-0"` (incorrect notation, but will accept)
/// - `"castle kingside"`
/// - `"O-O"` (correct notation)
/// - `"o-o"` (incorrect notation, but will accept)
/// - `"0-0"` (incorrect notation, but will accept)
/// - `"e2e4"`
/// - `"e2 e4"`
/// - `"e2 to e4"`
///
/// Parsing a move such as `"knight to e4"` or `"Qxe4"` will NOT work.
impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(repr: String) -> Result<Self, Self::Error> {
        let repr = repr.trim().to_string();

        Ok(match repr.as_str() {
            "resign" | "resigns" => Self::Resign,
            "queenside castle" | "castle queenside" | "O-O-O" | "0-0-0" | "o-o-o" => {
                Self::QueenSideCastle
            }
            "kingside castle" | "castle kingside" | "O-O" | "0-0" | "o-o" => Self::KingSideCastle,
            other => {
                let words = other.split_whitespace().collect::<Vec<&str>>();

                if words.len() == 1 && words[0].len() == 4 {
                    Self::Piece(
                        Position::pgn(&words[0][..2])?,
                        Position::pgn(&words[0][2..4])?,
                    )
                } else if words.len() == 2 {
                    Self::Piece(Position::pgn(&words[0])?, Position::pgn(&words[1])?)
                } else if words.len() == 3 && words[1] == "to" {
                    Self::Piece(Position::pgn(&words[0])?, Position::pgn(&words[2])?)
                } else {
                    return Err(format!("invalid move format `{}`", other));
                }
            }
        })
    }
}
