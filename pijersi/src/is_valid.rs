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

        let lower_rock = self.pieces[Piece::Rock as usize][Layer::Lower as usize];
        let lower_paper = self.pieces[Piece::Paper as usize][Layer::Lower as usize];
        let lower_scissors = self.pieces[Piece::Scissors as usize][Layer::Lower as usize];
        let lower_wise = self.pieces[Piece::Wise as usize][Layer::Lower as usize];
        let lower_rps = lower_rock | lower_paper | lower_scissors;

        let upper_rock = self.pieces[Piece::Rock as usize][Layer::Upper as usize];
        let upper_paper = self.pieces[Piece::Paper as usize][Layer::Upper as usize];
        let upper_scissors = self.pieces[Piece::Scissors as usize][Layer::Upper as usize];
        let upper_wise = self.pieces[Piece::Wise as usize][Layer::Upper as usize];
        let _upper_rps = upper_rock | upper_paper | upper_scissors;

        let lower = lower_rock | lower_paper | lower_scissors | lower_wise;
        let upper = upper_rock | upper_paper | upper_scissors | upper_wise;

        // Floating pieces
        debug_assert_eq!(upper & lower, upper);

        // Side overlaps
        debug_assert!((white & black).is_empty());

        // Lower piece overlaps
        debug_assert!((lower_rock & lower_paper).is_empty());
        debug_assert!((lower_rock & lower_scissors).is_empty());
        debug_assert!((lower_rock & lower_wise).is_empty());
        debug_assert!((lower_paper & lower_scissors).is_empty());
        debug_assert!((lower_paper & lower_wise).is_empty());
        debug_assert!((lower_scissors & lower_wise).is_empty());

        // Upper piece overlaps
        debug_assert!((upper_rock & upper_paper).is_empty());
        debug_assert!((upper_rock & upper_scissors).is_empty());
        debug_assert!((upper_rock & upper_wise).is_empty());
        debug_assert!((upper_paper & upper_scissors).is_empty());
        debug_assert!((upper_paper & upper_wise).is_empty());
        debug_assert!((upper_scissors & upper_wise).is_empty());

        // Wise on RPS
        debug_assert!((upper_wise & lower_rps).is_empty());

        true
    }
}
