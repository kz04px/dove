use crate::{
    layer::Layer,
    position::{Piece, Position},
    side::Side,
};

impl Position {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let white = self.sides[Side::White as usize];
        let black = self.sides[Side::Black as usize];

        let hidden_rock = self.pieces[Piece::Rock as usize][Layer::Hidden as usize];
        let hidden_paper = self.pieces[Piece::Paper as usize][Layer::Hidden as usize];
        let hidden_scissors = self.pieces[Piece::Scissors as usize][Layer::Hidden as usize];
        let hidden_wise = self.pieces[Piece::Wise as usize][Layer::Hidden as usize];
        let hidden_rps = hidden_rock | hidden_paper | hidden_scissors;

        let visible_rock = self.pieces[Piece::Rock as usize][Layer::Visible as usize];
        let visible_paper = self.pieces[Piece::Paper as usize][Layer::Visible as usize];
        let visible_scissors = self.pieces[Piece::Scissors as usize][Layer::Visible as usize];
        let visible_wise = self.pieces[Piece::Wise as usize][Layer::Visible as usize];
        let _visible_rps = visible_rock | visible_paper | visible_scissors;

        let hidden = hidden_rock | hidden_paper | hidden_scissors | hidden_wise;
        let visible = visible_rock | visible_paper | visible_scissors | visible_wise;

        // Exposed hidden
        debug_assert_eq!(hidden & visible, hidden);

        // Side overlaps
        debug_assert!((white & black).is_empty());

        // Lower piece overlaps
        debug_assert!((hidden_rock & hidden_paper).is_empty());
        debug_assert!((hidden_rock & hidden_scissors).is_empty());
        debug_assert!((hidden_rock & hidden_wise).is_empty());
        debug_assert!((hidden_paper & hidden_scissors).is_empty());
        debug_assert!((hidden_paper & hidden_wise).is_empty());
        debug_assert!((hidden_scissors & hidden_wise).is_empty());

        // Upper piece overlaps
        debug_assert!((visible_rock & visible_paper).is_empty());
        debug_assert!((visible_rock & visible_scissors).is_empty());
        debug_assert!((visible_rock & visible_wise).is_empty());
        debug_assert!((visible_paper & visible_scissors).is_empty());
        debug_assert!((visible_paper & visible_wise).is_empty());
        debug_assert!((visible_scissors & visible_wise).is_empty());

        // Wise on RPS
        debug_assert!((visible_wise & hidden_rps).is_empty());

        true
    }
}
