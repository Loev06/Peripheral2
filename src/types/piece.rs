use anyhow::{anyhow, Result};

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