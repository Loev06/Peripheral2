use crate::types::{Board, Player};

struct GameState {
    pub board: Board,
    pub turn: Player,
    pub castling: Castling,
    pub en_passant: Option<EnPassant>
}

struct Castling {
    rights: [bool; 4]
}

struct EnPassant {
    file: [bool; 3]
}