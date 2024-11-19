use crate::{mv::Mv, position::Position};

impl Position {
    #[must_use]
    pub fn after_move(&self, mv: &Mv) -> Self {
        let mut npos = *self;
        npos.makemove(mv);
        npos
    }
}
