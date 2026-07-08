use anyhow::Result;

mod types;
use types::board::Board;
fn main() -> Result<()> {
    let b = Board::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")?;
    println!("{}", b.all_pieces());
    Ok(())
}
