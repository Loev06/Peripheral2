use crate::types::{Board, CastlingRights, Player, SquareOption};

struct GameState {
    pub board: Board,
    pub turn: Player,
    pub castling: CastlingRights,
    pub en_passant: Option<SquareOption>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<GameState>(), 40);
    }
}
