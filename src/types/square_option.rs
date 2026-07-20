use anyhow::Result;
use std::num::NonZeroU8;

use crate::types::{Square, SquareIndexExt};

#[repr(transparent)]
/// An alternate representation of a square index,
/// with the guarantee that `size_of::<Option<SquareOption>>() == 1`
pub struct SquareOption(NonZeroU8);

impl SquareOption {
    fn new(index: Square) -> Self {
        assert!(index < 64);
        SquareOption(NonZeroU8::new(index + 1).expect("index + 1 is non-zero"))
    }

    fn index(&self) -> Square {
        self.0.get() - 1
    }
}

impl SquareIndexExt for SquareOption {
    fn from_square_name(name: &str) -> Result<Self> {
        let index = Square::from_square_name(name)?;
        Ok(SquareOption::new(index))
    }

    fn to_square_name(&self) -> Result<String> {
        self.index().to_square_name()
    }

    fn flip_vertical(&self) -> Self {
        Self::new(self.index() ^ 56)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_option() {
        let sq_opt = SquareOption::new(0);
        assert_eq!(sq_opt.index(), 0);
        assert_eq!(sq_opt.to_square_name().unwrap(), "a1");
        let flipped = sq_opt.flip_vertical();
        assert_eq!(flipped.index(), 56);
        assert_eq!(flipped.to_square_name().unwrap(), "a8");
    }

    #[test]
    fn test_option_size() {
        assert_eq!(std::mem::size_of::<Option<SquareOption>>(), 1);
    }
}
