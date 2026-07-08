use anyhow::{anyhow, Result};
use std::fmt::Display;

pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

pub enum ColoredPiece {
    White(Piece),
    Black(Piece)
}

impl Piece {
    pub fn from_lowercase_char(c: char) -> Result<Self> {
        match c {
            'k' => Ok(Piece::King),
            'q' => Ok(Piece::Queen),
            'r' => Ok(Piece::Rook),
            'b' => Ok(Piece::Bishop),
            'n' => Ok(Piece::Knight),
            'p' => Ok(Piece::Pawn),
            _   => Err(anyhow!("Invalid piece character: {}", c))
        }
    }
}

impl ColoredPiece {
    pub fn from_char(c: char) -> Result<Self> {
        let pt = Piece::from_lowercase_char(c.to_ascii_lowercase())
            .map_err(|_| anyhow!("Invalid piece character: {}", c))?;

        if c.is_lowercase() {
            Ok(ColoredPiece::Black(pt))
        } else {
            Ok(ColoredPiece::White(pt))
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::King   => 'K',
            Piece::Queen  => 'Q',
            Piece::Rook   => 'R',
            Piece::Bishop => 'B',
            Piece::Knight => 'N',
            Piece::Pawn   => 'P'
        }.fmt(f)
    }
}

impl Display for ColoredPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColoredPiece::White(pt) => pt.to_string().to_uppercase().fmt(f),
            ColoredPiece::Black(pt) => pt.to_string().to_lowercase().fmt(f)
        }
    }
}