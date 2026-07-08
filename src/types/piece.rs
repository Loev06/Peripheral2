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