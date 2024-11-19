use crate::{
    bitboard::Bitboard,
    layer::Layer,
    mv::Mv,
    position::{Piece, Position},
};

impl Position {
    pub fn makemove(&mut self, mv: &Mv) -> () {
        match mv {
            Mv::SoloMove(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());

                let piece = self.get_piece_on(Layer::Lower, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece as usize][Layer::Lower as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece as usize][Layer::Lower as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::SoloStack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                let piece = self.get_piece_on(Layer::Lower, *fr).unwrap();

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece as usize][Layer::Lower as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::SoloStackMove(fr, sq1, sq2) => {
                debug_assert_ne!(fr, sq1);
                debug_assert_ne!(sq1, sq2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*sq1).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq1).is_none());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Lower, *sq1).unwrap();

                // Remove piece1
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= Bitboard::from_square(fr);

                // Remove piece2
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= Bitboard::from_square(sq1);

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);

                // Add pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*sq1) == None);
                debug_assert!(self.get_piece_on(Layer::Lower, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq2).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackMove(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= Bitboard::from_square(to);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackMoveDestack(fr, sq1, sq2) => {
                debug_assert_ne!(fr, sq1);
                debug_assert_ne!(sq1, sq2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*sq1) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2: Piece = self.get_piece_on(Layer::Upper, *fr).unwrap();
                let destack_layer = if self.get_us().is_set(sq2) && fr != sq2 {
                    Layer::Upper
                } else {
                    Layer::Lower
                };

                let capture_2 = Bitboard::from_square(sq2) & self.get_them();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq1);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq1);

                // Remove captured
                self.sides[!self.turn as usize] &= !capture_2;
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &= !capture_2;

                // Remove pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= Bitboard::from_square(sq1);

                // Add piece
                self.sides[self.turn as usize] |= Bitboard::from_square(sq2);
                self.pieces[piece2 as usize][destack_layer as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*sq1) == Some(self.turn));
                debug_assert!(self.get_side_on(*sq2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *sq1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackDestack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(to);

                // Remove piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece as usize][Layer::Lower as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackStack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                let piece = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Remove piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackStackMove(fr, sq1, sq2) => {
                debug_assert_ne!(fr, sq1);
                debug_assert_ne!(sq1, sq2);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*sq1) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq1).is_none());

                let piece1 = self.get_piece_on(Layer::Upper, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Lower, *sq1).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !Bitboard::from_square(sq2);

                // Remove piece
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= Bitboard::from_square(fr);

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= Bitboard::from_square(sq1);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*sq1) == None);
                debug_assert!(self.get_side_on(*sq2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *sq2).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
        }

        debug_assert!(self.is_valid());
    }
}
