use anyhow::Result;

mod types;
use types::{
    board::Board,
    extensions::{BitboardExt, SquareIndexExt},
    piece::{ColoredPiece::*, Piece::*},
};

fn main() -> Result<()> {
    let b = Board::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")?;
    println!("{}", b.colored_pieces(White(Queen)).display());
    println!("{}", b.at(u8::from_square_name("h8")?).unwrap());
    Ok(())
}
