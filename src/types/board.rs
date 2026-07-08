use super::piece::{ColoredPiece, Piece::{self, *}};

pub struct Board {
    white: u64,   // White pieces
    sliders: u64, // queens, rooks
    minor: u64,   // bishops, knights, pawns
    royal: u64    // kings, queens, pawns
}

impl Board {
    fn pieces(&self, pt: Piece) -> u64 {
        match pt {
            King   => self.royal,
            Queen  => self.sliders & self.royal,
            Rook   => self.sliders,
            Bishop => self.sliders & self.minor,
            Knight => self.minor,
            Pawn   => self.minor & self.royal
        }
    }

    fn colored_pieces(&self, pt: ColoredPiece) -> u64 {
        match pt {
            ColoredPiece::White(p) => self.pieces(p) & self.white,
            ColoredPiece::Black(p) => self.pieces(p) & !self.white
        }
    }

    fn all_pieces(&self) -> u64 {
        self.sliders | self.minor | self.royal
    }
}