use anyhow::Result;

mod types;
use types::{
    board::Board,
    extensions::BitboardExt,
    piece::{ColoredPiece::*, Piece::*}
};


fn main() -> Result<()> {
    let b = Board::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")?;
    println!("{}", b.colored_pieces(White(Queen)).display());
    Ok(())
}
