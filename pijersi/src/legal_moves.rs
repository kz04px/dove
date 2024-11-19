use crate::{mv::Mv, position::Position};

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Mv> {
        let mut moves = vec![];

        self.move_generator(|mv| {
            moves.push(mv);
            false
        });

        moves
    }
}
