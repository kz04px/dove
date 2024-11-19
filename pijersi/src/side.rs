use std::{fmt, ops::Not};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[must_use]
pub enum Side {
    #[default]
    White,
    Black,
}

impl Not for Side {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::White => write!(f, "white"),
            Side::Black => write!(f, "black"),
        }
    }
}
