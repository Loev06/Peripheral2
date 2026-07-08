use anyhow::{Result, anyhow};
use std::fmt::Display;

use crate::types::{
    BitboardExt, ColoredPiece,
    Piece::{self, *},
    SquareIndexExt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Empty square = 0000
// unused square patterns: 1000, 0111, 1111
pub struct Board {
    white: u64,   // White pieces
    sliders: u64, // queens, rooks
    minor: u64,   // bishops, knights, pawns
    royal: u64,   // kings, queens, pawns
}

impl Board {
    fn uncolored_at(&self, sq: u8) -> Option<Piece> {
        assert!(sq < 64);
        match (
            self.sliders.bit_set(sq),
            self.minor.bit_set(sq),
            self.royal.bit_set(sq),
        ) {
            // There is room for a special square marking with (true, true, true)
            (true, true, true) => panic!("invalid board state: {:?} at square {}", self, sq),
            (false, true, true) => Some(Pawn),
            (true, false, true) => Some(Queen),
            (false, false, true) => Some(King),
            (true, true, false) => Some(Bishop),
            (false, true, false) => Some(Knight),
            (true, false, false) => Some(Rook),
            (false, false, false) => None,
        }
    }

    pub fn at(&self, sq: u8) -> Option<ColoredPiece> {
        assert!(sq < 64);

        // There is room to treat None differently whether self.white.bit_set(sq)
        self.uncolored_at(sq).map(|pt| {
            if self.white.bit_set(sq) {
                ColoredPiece::White(pt)
            } else {
                ColoredPiece::Black(pt)
            }
        })
    }

    pub fn pieces(&self, pt: Piece) -> u64 {
        match pt {
            King => self.royal,
            Queen => self.sliders & self.royal,
            Rook => self.sliders,
            Bishop => self.sliders & self.minor,
            Knight => self.minor,
            Pawn => self.minor & self.royal,
        }
    }

    pub fn colored_pieces(&self, pt: ColoredPiece) -> u64 {
        match pt {
            ColoredPiece::White(p) => self.pieces(p) & self.white,
            ColoredPiece::Black(p) => self.pieces(p) & !self.white,
        }
    }

    pub fn all_pieces(&self) -> u64 {
        self.sliders | self.minor | self.royal
    }

    pub fn put_pieces(&mut self, pt: Piece, mask: u64) {
        match pt {
            King => self.royal |= mask,
            Queen => {
                self.sliders |= mask;
                self.royal |= mask;
            }
            Rook => self.sliders |= mask,
            Bishop => {
                self.sliders |= mask;
                self.minor |= mask;
            }
            Knight => self.minor |= mask,
            Pawn => {
                self.minor |= mask;
                self.royal |= mask;
            }
        }
    }

    pub fn put_colored_pieces(&mut self, pt: ColoredPiece, mask: u64) {
        match pt {
            ColoredPiece::White(p) => {
                self.white |= mask;
                self.put_pieces(p, mask);
            }
            ColoredPiece::Black(p) => {
                self.white &= !mask;
                self.put_pieces(p, mask);
            }
        }
    }

    pub fn put_piece(&mut self, pt: Piece, sq: u8) {
        assert!(sq < 64);
        self.put_pieces(pt, 1 << sq);
    }

    pub fn put_colored_piece(&mut self, pt: ColoredPiece, sq: u8) {
        assert!(sq < 64);
        self.put_colored_pieces(pt, 1 << sq);
    }
}

impl TryFrom<&str> for Board {
    fn try_from(s: &str) -> Result<Self> {
        let mut b = Board {
            white: 0,
            sliders: 0,
            minor: 0,
            royal: 0,
        };
        let mut sq = 0;
        for c in s.chars() {
            if sq >= 64 {
                return Err(anyhow!("Invalid board string: {}", s));
            }

            if c == '/' {
                if sq == 0 || sq % 8 != 0 {
                    return Err(anyhow!("Invalid board string: {}", s));
                }
                continue;
            }

            if c.is_ascii_digit() {
                let step = c.to_digit(10).expect("character is a digit") as u8;

                if step == 0 || step > 8 {
                    return Err(anyhow!("Invalid digit in board string: {}", c));
                }
                sq += step;
                continue;
            }

            let pt = ColoredPiece::from_char(c)?;
            b.put_colored_piece(pt, sq.flip_vertical());
            sq += 1;
        }
        Ok(b)
    }

    type Error = anyhow::Error;
}

impl Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let div1 = "\n+---+---+---+---+---+---+---+---+\n";
        let div2 = " | ";

        f.write_str(
            (0..8).rev().fold(String::from(div1), |acc, rank| {
                acc + (0..8).fold(String::from(div2), |acc, file| {
                    acc + self.at(rank * 8 + file)
                        .map(|p| p.to_string())
                        .unwrap_or(String::from(" "))
                        .as_str() + div2
                }).trim() + div1
            }).trim()
        )
    }
}
