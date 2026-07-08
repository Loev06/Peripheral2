use anyhow::{anyhow, Result};

// Extension traits for external types
pub trait SquareIndexExt: Sized {
    fn from_square_name(name: &str) -> Result<Self>;
    fn to_square_name(self) -> Result<String>;
    /// Flips the square index vertically
    fn flip_vertical(self) -> Self;
}

pub trait BitboardExt {
    fn bit_set(&self, sq: u8) -> bool;
    fn display(self) -> String;
}

impl SquareIndexExt for u8 {
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

    fn to_square_name(self) -> Result<String> {
        if self >= 64 {
            return Err(anyhow!("Invalid square index: {}", self));
        }

        let file_index = self % 8;
        let rank_index = self / 8;

        let file = (b'a' + file_index) as char;
        let rank = (b'1' + rank_index) as char;

        Ok(format!("{}{}", file, rank)) 
    }

    fn flip_vertical(self) -> Self {
        assert!(self < 64);
        self ^ 56
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
                    s.push('1');
                } else {
                    s.push('.');
                }
                if file < 7 {
                    s.push(' ');
                }
            }
            if rank > 0 {
                s.push('\n');
            }
        }
        s
    }
}

#[cfg(test)]
mod square_index_tests {
    use super::*;

    #[test]
    fn test_from_name() {
        assert_eq!(u8::from_square_name("a1").unwrap(), 0);
        assert_eq!(u8::from_square_name("H1").unwrap(), 7);
        assert_eq!(u8::from_square_name("A8").unwrap(), 56);
        assert_eq!(u8::from_square_name("h8").unwrap(), 63);
        assert!(u8::from_square_name("i1").is_err());
        assert!(u8::from_square_name("a9").is_err());
        assert!(u8::from_square_name("a").is_err());
    }

    #[test]
    fn test_from_to_name() {
        for sq in 0..64 {
            let name = sq.to_square_name().unwrap();
            let index = u8::from_square_name(&name).unwrap();
            assert_eq!(sq, index);
        }
    }

    #[test]
    fn test_flip_vertical() {
        for sq in 0..64 {
            let file_index = sq % 8;
            let rank_index = sq / 8;
            let flipped_rank = 7 - rank_index;
            let flipped_sq = flipped_rank * 8 + file_index;
            assert_eq!(sq.flip_vertical(), flipped_sq);
        }
    }
}

#[cfg(test)]
mod bitboard_tests {
    use super::*;

    #[test]
    fn test_bit_set() {
        let bb: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010;
        for sq in 0..64 {
            let expected = (sq % 2) == 1;
            assert_eq!(bb.bit_set(sq), expected);
        }
    }

    #[test]
    fn test_display() {
        // bb value generated using https://tearth.dev/bitboard-viewer/ (Layout 1)
        let bb = 0b0000000100000001000000000000000000000000000000000000000000000011;
        assert_eq!(bb.display(), "\
1 . . . . . . .
1 . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
1 1 . . . . . .");
    }
}