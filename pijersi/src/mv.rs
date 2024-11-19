use std::fmt;

use crate::square::Square;

#[derive(Clone, Copy)]
pub enum Mv {
    SoloMove(Square, Square),
    SoloStack(Square, Square),
    SoloStackMove(Square, Square, Square),
    StackMove(Square, Square),
    StackMoveDestack(Square, Square, Square),
    StackDestack(Square, Square),
    StackStack(Square, Square),
    StackStackMove(Square, Square, Square),
}

impl fmt::Display for Mv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mv::SoloMove(fr, to) => write!(f, "{}{}", fr, to),
            Mv::SoloStack(fr, to) => write!(f, "{}{}", fr, to),
            Mv::SoloStackMove(fr, sq1, sq2) => write!(f, "{}{}{}", fr, sq1, sq2),
            Mv::StackMove(fr, to) => write!(f, "{}{}{}", fr, to, to),
            Mv::StackMoveDestack(fr, sq1, sq2) => write!(f, "{}{}{}", fr, sq1, sq2),
            Mv::StackDestack(fr, to) => write!(f, "{}{}{}", fr, fr, to),
            Mv::StackStack(fr, to) => write!(f, "{}{}{}", fr, fr, to),
            Mv::StackStackMove(fr, sq1, sq2) => write!(f, "{}{}{}", fr, sq1, sq2),
        }
    }
}

impl fmt::Debug for Mv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mv::SoloMove(fr, to) => write!(f, "{}{} (SoloMove)", fr, to),
            Mv::SoloStack(fr, to) => write!(f, "{}{} (SoloStack)", fr, to),
            Mv::SoloStackMove(fr, sq1, sq2) => write!(f, "{}{}{} (SoloStackMove)", fr, sq1, sq2),
            Mv::StackMove(fr, to) => write!(f, "{}{}{} (StackMove)", fr, to, to),
            Mv::StackMoveDestack(fr, sq1, sq2) => {
                write!(f, "{}{}{} (StackMoveDestack)", fr, sq1, sq2)
            }
            Mv::StackDestack(fr, to) => write!(f, "{}{}{} (StackDestack)", fr, fr, to),
            Mv::StackStack(fr, to) => write!(f, "{}{}{} (StackStack)", fr, fr, to),
            Mv::StackStackMove(fr, sq1, sq2) => write!(f, "{}{}{} (StackStackMove)", fr, sq1, sq2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_strings() {
        let tests = [
            (Mv::SoloMove(Square(0), Square(1)), "a1a2"),
            (Mv::SoloStack(Square(0), Square(1)), "a1a2"),
            (Mv::SoloStackMove(Square(0), Square(1), Square(2)), "a1a2a3"),
            (Mv::StackMove(Square(0), Square(1)), "a1a2a2"),
            (
                Mv::StackMoveDestack(Square(0), Square(1), Square(2)),
                "a1a2a3",
            ),
        ];

        for (mv, str) in tests {
            assert_eq!(format!("{}", mv), str);
        }
    }
}
