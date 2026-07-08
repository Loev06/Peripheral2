use anyhow::{anyhow, Result};
use std::fmt::Display;

use crate::types::{
    extensions::{BitboardExt, SquareIndexExt},
    piece::{ColoredPiece, Piece::{self, *}}
};

pub struct Board {
    white: u64,   // White pieces
    sliders: u64, // queens, rooks
    minor: u64,   // bishops, knights, pawns
    royal: u64    // kings, queens, pawns
}

impl Board {
    fn uncolored_at(&self, sq: u8) -> Option<Piece> {
        assert!(sq < 64);

        if self.royal.bit_set(sq) {
            if self.sliders.bit_set(sq) {
                Some(Queen)
            } else if self.minor.bit_set(sq) {
                Some(Pawn)
            } else {
                Some(King)
            }
        } else if self.minor.bit_set(sq) {
            if self.sliders.bit_set(sq) {
                Some(Bishop)
            } else {
                Some(Knight)
            }
        } else if self.sliders.bit_set(sq) {
            Some(Rook)
        } else {
            None
        }
    }

    pub fn at(&self, sq: u8) -> Option<ColoredPiece> {
        assert!(sq < 64);
        
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
            King   => self.royal,
            Queen  => self.sliders & self.royal,
            Rook   => self.sliders,
            Bishop => self.sliders & self.minor,
            Knight => self.minor,
            Pawn   => self.minor & self.royal
        }
    }

    pub fn colored_pieces(&self, pt: ColoredPiece) -> u64 {
        match pt {
            ColoredPiece::White(p) => self.pieces(p) & self.white,
            ColoredPiece::Black(p) => self.pieces(p) & !self.white
        }
    }

    pub fn all_pieces(&self) -> u64 {
        self.sliders | self.minor | self.royal
    }

    pub fn put_piece(&mut self, pt: Piece, mask: u64) {
        match pt {
            King   => self.royal |= mask,
            Queen  => { self.sliders |= mask; self.royal |= mask; },
            Rook   => self.sliders |= mask,
            Bishop => { self.sliders |= mask; self.minor |= mask; },
            Knight => self.minor |= mask,
            Pawn   => { self.minor |= mask; self.royal |= mask; }
        }
    }

    pub fn put_colored_piece(&mut self, pt: ColoredPiece, sq: u8) {
        assert !(sq < 64);

        let mask = 1 << sq;
        match pt {
            ColoredPiece::White(p) => {
                self.white |= mask;
                self.put_piece(p, mask);
            },
            ColoredPiece::Black(p) => {
                self.white &= !mask;
                self.put_piece(p, mask);
            }
        }
    }
}

impl TryFrom<&str> for Board {
    fn try_from(s: &str) -> Result<Self> {
        let mut b = Board { white: 0, sliders: 0, minor: 0, royal: 0 };
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