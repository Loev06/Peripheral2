use anyhow::{Result, anyhow};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColoredPiece {
    White(Piece),
    Black(Piece),
}

impl Piece {
    pub fn from_char(c: char) -> Result<Self> {
        match c.to_ascii_lowercase() {
            'k' => Ok(Piece::King),
            'q' => Ok(Piece::Queen),
            'r' => Ok(Piece::Rook),
            'b' => Ok(Piece::Bishop),
            'n' => Ok(Piece::Knight),
            'p' => Ok(Piece::Pawn),
            _ => Err(anyhow!("Invalid piece character: {}", c)),
        }
    }
}

impl ColoredPiece {
    pub fn from_char(c: char) -> Result<Self> {
        let pt = Piece::from_char(c).map_err(|_| anyhow!("Invalid piece character: {}", c))?;

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
            Piece::King => 'K',
            Piece::Queen => 'Q',
            Piece::Rook => 'R',
            Piece::Bishop => 'B',
            Piece::Knight => 'N',
            Piece::Pawn => 'P',
        }
        .fmt(f)
    }
}

impl Display for ColoredPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColoredPiece::White(pt) => pt.to_string().to_uppercase().fmt(f),
            ColoredPiece::Black(pt) => pt.to_string().to_lowercase().fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_from_char() {
        assert_eq!(Piece::from_char('Q').unwrap(), Piece::Queen);
        assert_eq!(Piece::from_char('q').unwrap(), Piece::Queen);
        assert_eq!(Piece::from_char('k').unwrap(), Piece::King);
        assert!(Piece::from_char('x').is_err());
    }

    #[test]
    fn test_colored_piece_from_char() {
        assert_eq!(
            ColoredPiece::from_char('Q').unwrap(),
            ColoredPiece::White(Piece::Queen)
        );
        assert_eq!(
            ColoredPiece::from_char('q').unwrap(),
            ColoredPiece::Black(Piece::Queen)
        );
        assert_eq!(
            ColoredPiece::from_char('k').unwrap(),
            ColoredPiece::Black(Piece::King)
        );
        assert!(ColoredPiece::from_char('x').is_err());
    }

    #[test]
    fn test_piece_display() {
        "KQRBNP".chars().for_each(|c| {
            let piece = Piece::from_char(c).unwrap();
            assert_eq!(piece.to_string(), c.to_string());
        });
    }

    #[test]
    fn test_colored_piece_display() {
        "KQRBNPkqrbnp".chars().for_each(|c| {
            let colored_piece = ColoredPiece::from_char(c).unwrap();
            assert_eq!(colored_piece.to_string(), c.to_string());
        });
    }
}
