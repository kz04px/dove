use std::{fmt, ops::Not};

#[derive(Clone, Copy, PartialEq)]
#[must_use]
pub enum Layer {
    Lower,
    Upper,
}

impl Not for Layer {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        match self {
            Layer::Lower => Layer::Upper,
            Layer::Upper => Layer::Lower,
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Layer::Lower => write!(f, "lower"),
            Layer::Upper => write!(f, "upper"),
        }
    }
}
