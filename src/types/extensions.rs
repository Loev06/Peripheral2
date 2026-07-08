use anyhow::{anyhow, Result};

// Extension traits for external types
pub trait SquareIndexExt where Self: Sized {
    /// Flips the square index vertically
    fn flip_vertical(self) -> Self;

    /// Converts a square name to a square index
    fn from_square_name(name: &str) -> Result<Self>;
}

pub trait BitboardExt {
    fn bit_set(&self, sq: u8) -> bool;
    fn display(self) -> String;
}

impl SquareIndexExt for u8 {
    fn flip_vertical(self) -> Self {
        assert!(self < 64);
        self ^ 56
    }

    fn from_square_name(name: &str) -> Result<Self> {
        if name.len() != 2 {
            return Err(anyhow!("Invalid square name: {}", name));
        }

        let file = name.chars().nth(0).expect("character at index 0").to_ascii_lowercase();
        let rank = name.chars().nth(1).expect("character at index 1").to_ascii_lowercase();

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return Err(anyhow!("Invalid square name: {}", name));
        }

        let file_index = (file as u8) - b'a';
        let rank_index = (rank as u8) - b'1';

        Ok(rank_index * 8 + file_index)
    }
}

impl BitboardExt for u64 {
    fn bit_set(&self, sq: u8) -> bool {
        assert!(sq < 64);
        (self & (1 << sq)) != 0
    }

    fn display(self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                let mask = 1 << sq;
                if self & mask != 0 {
                    s.push_str("1 ");
                } else {
                    s.push_str(". ");
                }
            }
            s.push('\n');
        }
        s
    }
}