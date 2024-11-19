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
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none(),);
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());

                let piece = self.get_piece_on(Layer::Visible, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(to);

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece as usize][Layer::Visible as usize] &= !Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::SoloStack(fr, to) => {
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_side_on(*to).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                let piece = self.get_piece_on(Layer::Visible, *fr).unwrap();
                let bottom = self.get_piece_on(Layer::Visible, *to).unwrap();

                // Remove pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);
                self.pieces[bottom as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);

                // Add piece
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);
                self.pieces[bottom as usize][Layer::Hidden as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::SoloStackMove(fr, sq1, sq2) => {
                debug_assert_ne!(fr, sq1);
                debug_assert_ne!(sq1, sq2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*sq1).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq1).is_some());

                let piece1 = self.get_piece_on(Layer::Visible, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Visible, *sq1).unwrap();

                // Remove piece1
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Visible as usize] &= !Bitboard::from_square(fr);

                // Remove piece2
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[piece2 as usize][Layer::Visible as usize] &=
                    !Bitboard::from_square(sq1);

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(sq2);

                // Add pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece2 as usize][Layer::Hidden as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece1 as usize][Layer::Visible as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*sq1) == None);
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq2).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackMove(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());

                let piece1 = self.get_piece_on(Layer::Hidden, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Visible, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(to);

                // Remove piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Hidden as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece2 as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece1 as usize][Layer::Hidden as usize] ^= Bitboard::from_square(to);
                self.pieces[piece2 as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == None);
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackMoveDestack(fr, sq1, sq2) => {
                debug_assert_ne!(fr, sq1);
                debug_assert_ne!(sq1, sq2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*sq1) != Some(self.turn));
                // debug_assert!(self.get_side_on(*sq2) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                // debug_assert!(self.get_piece_on(Layer::Visible, *sq2) != Some(Piece::Wise));

                let piece1 = self.get_piece_on(Layer::Hidden, *fr).unwrap();
                let piece2: Piece = self.get_piece_on(Layer::Visible, *fr).unwrap();

                let capture_2 = Bitboard::from_square(sq2) & self.get_them();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(sq1);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(sq1);

                // Remove captured
                self.sides[!self.turn as usize] &= !capture_2;
                self.pieces[Piece::Rock as usize][0] &= !capture_2;
                self.pieces[Piece::Rock as usize][1] &= !capture_2;
                self.pieces[Piece::Paper as usize][0] &= !capture_2;
                self.pieces[Piece::Paper as usize][1] &= !capture_2;
                self.pieces[Piece::Scissors as usize][0] &= !capture_2;
                self.pieces[Piece::Scissors as usize][1] &= !capture_2;
                self.pieces[Piece::Wise as usize][0] &= !capture_2;
                self.pieces[Piece::Wise as usize][1] &= !capture_2;

                // Remove pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece1 as usize][Layer::Hidden as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece2 as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[piece1 as usize][Layer::Visible as usize] ^= Bitboard::from_square(sq1);

                // Add piece
                if self.sides[self.turn as usize].is_set(sq2) {
                    let sad = self.get_piece_on(Layer::Visible, *sq2).unwrap();
                    self.pieces[sad as usize][Layer::Visible as usize] ^=
                        Bitboard::from_square(sq2);
                    self.pieces[sad as usize][Layer::Hidden as usize] ^= Bitboard::from_square(sq2);
                }
                self.sides[self.turn as usize] |= Bitboard::from_square(sq2);
                self.pieces[piece2 as usize][Layer::Visible as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*sq1) == Some(self.turn));
                debug_assert!(self.get_side_on(*sq2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq1).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackDestack(fr, to) => {
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());

                let piece = self.get_piece_on(Layer::Visible, *fr).unwrap();
                let bottom = self.get_piece_on(Layer::Hidden, *fr).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(to);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(to);

                // Remove pieces
                self.pieces[bottom as usize][Layer::Hidden as usize] ^= Bitboard::from_square(fr);
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Add pieces
                self.sides[self.turn as usize] ^= Bitboard::from_square(to);
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);
                self.pieces[bottom as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackStack(fr, to) => {
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*to).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                let piece = self.get_piece_on(Layer::Visible, *fr).unwrap();
                let aaa = self.get_piece_on(Layer::Hidden, *fr).unwrap();
                let bbb = self.get_piece_on(Layer::Visible, *to).unwrap();

                // Remove piece
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Hmm from
                self.pieces[aaa as usize][Layer::Hidden as usize] ^= Bitboard::from_square(fr);
                self.pieces[aaa as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Hmm to
                self.pieces[bbb as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);
                self.pieces[bbb as usize][Layer::Hidden as usize] ^= Bitboard::from_square(to);

                // Add piece
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(to);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Hidden, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *to).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
            Mv::StackStackMove(fr, sq1, sq2) => {
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*sq1).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq1).is_some());

                let piece = self.get_piece_on(Layer::Visible, *fr).unwrap();
                let aaa = self.get_piece_on(Layer::Hidden, *fr).unwrap();
                let bbb = self.get_piece_on(Layer::Visible, *sq1).unwrap();

                // Remove captured
                self.sides[!self.turn as usize] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Rock as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Paper as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Scissors as usize][1] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][0] &= !Bitboard::from_square(sq2);
                self.pieces[Piece::Wise as usize][1] &= !Bitboard::from_square(sq2);

                // Remove piece
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Hmm from
                self.pieces[aaa as usize][Layer::Hidden as usize] ^= Bitboard::from_square(fr);
                self.pieces[aaa as usize][Layer::Visible as usize] ^= Bitboard::from_square(fr);

                // Hmm to
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq1);
                self.pieces[bbb as usize][Layer::Visible as usize] ^= Bitboard::from_square(sq1);

                // Add piece
                self.sides[self.turn as usize] ^= Bitboard::from_square(sq2);
                self.pieces[bbb as usize][Layer::Hidden as usize] ^= Bitboard::from_square(sq2);
                self.pieces[piece as usize][Layer::Visible as usize] ^= Bitboard::from_square(sq2);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*sq1) == None);
                debug_assert!(self.get_side_on(*sq2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Hidden, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq1).is_none());
                debug_assert!(self.get_piece_on(Layer::Hidden, *sq2).is_some());
                debug_assert!(self.get_piece_on(Layer::Visible, *sq2).is_some());

                self.halfmoves += 1;
                self.turn = !self.turn;
            }
        }

        debug_assert!(self.is_valid());
    }
}
