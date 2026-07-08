// Extension traits for external types
pub trait SquareIndexExt {
    /// Flips the square index vertically
    fn flip_vertical(self) -> Self;
}

pub trait BitboardExt {
    fn display(self) -> String;
}

impl SquareIndexExt for u8 {
    fn flip_vertical(self) -> Self {
        assert!(self < 64);
        self ^ 56
    }
}

impl BitboardExt for u64 {
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