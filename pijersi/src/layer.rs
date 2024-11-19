use std::{fmt, ops::Not};

#[derive(Clone, Copy, PartialEq)]
#[must_use]
pub enum Layer {
    Hidden,
    Visible,
}

impl Not for Layer {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        match self {
            Layer::Hidden => Layer::Visible,
            Layer::Visible => Layer::Hidden,
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Layer::Hidden => write!(f, "hidden"),
            Layer::Visible => write!(f, "visible"),
        }
    }
}
