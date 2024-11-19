use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bitboard(pub u64);

pub enum Bitmasks {
    All = 0x1fffffffffff,
    ShortRows = 0x1f80fc07e03f,
    LongRows = 0x7f03f81fc0,
    Leftmost = 0x8104082041,
    Rightmost = 0x104082041020,
    WhiteHome = 0x3f,
    BlackHome = 0x1f8000000000,
}

impl Bitboard {
    #[must_use]
    pub fn empty() -> Self {
        Bitboard(0)
    }

    #[must_use]
    pub fn all() -> Self {
        Bitboard(Bitmasks::All as u64)
    }

    #[must_use]
    pub fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    #[must_use]
    pub fn lsb(&self) -> Square {
        Square(self.0.trailing_zeros() as u8)
    }

    #[must_use]
    pub fn from_square(sq: &Square) -> Self {
        Bitboard(1u64 << sq.0)
    }

    #[must_use]
    pub fn from_idx(idx: &i32) -> Self {
        Bitboard(1u64 << idx)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub fn is_set(&self, sq: &Square) -> bool {
        ((self.0 >> sq.0) & 1) == 1
    }

    #[must_use]
    pub fn adjacent(&self) -> Self {
        self.left()
            | self.right()
            | self.up_left()
            | self.up_right()
            | self.down_left()
            | self.down_right()
    }

    #[must_use]
    pub fn doubles(&self, blockers: Self) -> Self {
        (self.left() & !blockers).left()
            | (self.right() & !blockers).right()
            | (self.up_left() & !blockers).up_left()
            | (self.up_right() & !blockers).up_right()
            | (self.down_left() & !blockers).down_left()
            | (self.down_right() & !blockers).down_right()
    }

    #[must_use]
    pub fn left(&self) -> Self {
        Bitboard(self.0 >> 1) & !Bitboard(Bitmasks::Rightmost as u64)
    }

    #[must_use]
    pub fn right(&self) -> Self {
        Bitboard(self.0 << 1) & !Bitboard(Bitmasks::Leftmost as u64)
    }

    #[must_use]
    pub fn up_left(&self) -> Self {
        Bitboard(self.0 << 6) & !Bitboard(0x4002001000u64)
    }

    #[must_use]
    pub fn up_right(&self) -> Self {
        Bitboard(self.0 << 7) & !Bitboard(0x100080040u64)
    }

    #[must_use]
    pub fn down_left(&self) -> Self {
        Bitboard(self.0 >> 7) & !Bitboard(0x4002001000u64)
    }

    #[must_use]
    pub fn down_right(&self) -> Self {
        Bitboard(self.0 >> 6) & !Bitboard(0x100080040u64)
    }
}

impl Not for Bitboard {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        Bitboard(!self.0 & Bitmasks::All as u64)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[must_use]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    #[must_use]
    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    #[must_use]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) -> () {
        *self = *self | rhs;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) -> () {
        *self = *self ^ rhs;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) -> () {
        *self = *self & rhs;
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..7).rev() {
            let is_long = y % 2 == 1;

            if !is_long {
                write!(f, " ")?;
            }

            for x in 0..6 + is_long as i32 {
                let sq = Square::from_coords(x, y);

                if self.is_set(&sq) {
                    write!(f, " 1")?
                } else {
                    write!(f, " 0")?
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitor() {
        assert_eq!(Bitboard(0x0) | Bitboard(0x0), Bitboard(0x0));
        assert_eq!(Bitboard(0x0) | Bitboard(0x1), Bitboard(0x1));
        assert_eq!(Bitboard(0x1) | Bitboard(0x2), Bitboard(0x3));
    }

    #[test]
    fn bitxor() {
        assert_eq!(Bitboard(0x0) ^ Bitboard(0x0), Bitboard(0x0));
        assert_eq!(Bitboard(0x0) ^ Bitboard(0x1), Bitboard(0x1));
        assert_eq!(Bitboard(0x1) ^ Bitboard(0x2), Bitboard(0x3));
    }

    #[test]
    fn left() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(1u64 << 21)),
            (Bitboard(0x1fffffffffffu64), Bitboard(0xfbf7dfbefdfu64)),
            // Left edge wrap
            (Bitboard(1u64 << 0), Bitboard(0x0)),
            (Bitboard(1u64 << 6), Bitboard(0x0)),
            (Bitboard(1u64 << 13), Bitboard(0x0)),
            (Bitboard(1u64 << 19), Bitboard(0x0)),
            (Bitboard(1u64 << 26), Bitboard(0x0)),
            (Bitboard(1u64 << 32), Bitboard(0x0)),
            (Bitboard(1u64 << 39), Bitboard(0x0)),
        ];

        for (a, b) in tests {
            assert_eq!(a.left(), b);
        }
    }

    #[test]
    fn up_left() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(1u64 << 28)),
            (Bitboard(0x1fffffffffffu64), Bitboard(0x1fbffdffefc0u64)),
            // Left edge wrap
            (Bitboard(1u64 << 0), Bitboard(1u64 << 6)),
            (Bitboard(1u64 << 6), Bitboard(0x0)),
            (Bitboard(1u64 << 13), Bitboard(1u64 << 19)),
            (Bitboard(1u64 << 19), Bitboard(0x0)),
            (Bitboard(1u64 << 26), Bitboard(1u64 << 32)),
            (Bitboard(1u64 << 32), Bitboard(0x0)),
            (Bitboard(1u64 << 39), Bitboard(0x0)),
        ];

        for (a, b) in tests {
            assert_eq!(a.up_left(), b);
        }
    }

    #[test]
    fn down_left() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(1u64 << 15)),
            (Bitboard(0x1fffffffffffu64), Bitboard(0x3ffdffefffu64)),
            // Left edge wrap
            (Bitboard(1u64 << 0), Bitboard(0x0)),
            (Bitboard(1u64 << 6), Bitboard(0x0)),
            (Bitboard(1u64 << 13), Bitboard(1u64 << 6)),
            (Bitboard(1u64 << 19), Bitboard(0x0)),
            (Bitboard(1u64 << 26), Bitboard(1u64 << 19)),
            (Bitboard(1u64 << 32), Bitboard(0x0)),
            (Bitboard(1u64 << 39), Bitboard(1u64 << 32)),
        ];

        for (a, b) in tests {
            assert_eq!(a.down_left(), b);
        }
    }

    #[test]
    fn up_right() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(1u64 << 29)),
            (Bitboard(0x1fffffffffffu64), Bitboard(0x1ffefff7ff80u64)),
            // Right edge wrap
            (Bitboard(1u64 << 5), Bitboard(1u64 << 12)),
            (Bitboard(1u64 << 12), Bitboard(0x0)),
            (Bitboard(1u64 << 18), Bitboard(1u64 << 25)),
            (Bitboard(1u64 << 25), Bitboard(0x0)),
            (Bitboard(1u64 << 31), Bitboard(1u64 << 38)),
            (Bitboard(1u64 << 38), Bitboard(0x0)),
            (Bitboard(1u64 << 44), Bitboard(0x0)),
        ];

        for (a, b) in tests {
            assert_eq!(a.up_right(), b);
        }
    }

    #[test]
    fn down_right() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(1u64 << 16)),
            (Bitboard(0x1fffffffffffu64), Bitboard(0x7efff7ffbfu64)),
            // Right edge wrap
            (Bitboard(1u64 << 5), Bitboard(0x0)),
            (Bitboard(1u64 << 12), Bitboard(0x0)),
            (Bitboard(1u64 << 18), Bitboard(1u64 << 12)),
            (Bitboard(1u64 << 25), Bitboard(0x0)),
            (Bitboard(1u64 << 31), Bitboard(1u64 << 25)),
            (Bitboard(1u64 << 38), Bitboard(0x0)),
            (Bitboard(1u64 << 44), Bitboard(1u64 << 38)),
        ];

        for (a, b) in tests {
            assert_eq!(a.down_right(), b);
        }
    }

    #[test]
    fn adjacent() {
        let tests = [
            (Bitboard(0x0), Bitboard(0x0)),
            (Bitboard(1u64 << 22), Bitboard(0x30a18000u64)),
        ];

        for (a, b) in tests {
            assert_eq!(a.adjacent(), b);
        }
    }
}
