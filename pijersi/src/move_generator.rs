use crate::{
    bitboard::Bitboard,
    mv::Mv,
    position::{Piece, Position},
};

fn get_moves(
    us: Bitboard,
    them: Bitboard,
    short: Bitboard,
    tall: Bitboard,
    piece_mask: Bitboard,
    stackable: Bitboard,
    capturable: Bitboard,
    func: &mut impl FnMut(Mv) -> bool,
) -> bool {
    let empty = !(us | them);

    // SoloMove
    for solo in piece_mask & short {
        for mv in Bitboard::from_square(&solo).adjacent() & (empty | capturable) {
            if func(Mv::SoloMove(solo, mv)) {
                return true;
            }
        }
    }

    // SoloStack
    for solo in piece_mask & short {
        for stack in Bitboard::from_square(&solo).adjacent() & us & short & stackable {
            if func(Mv::SoloStack(solo, stack)) {
                return true;
            }
        }
    }

    // SoloStackMove
    for solo in piece_mask & short {
        for stack in Bitboard::from_square(&solo).adjacent() & us & short & stackable {
            let bb = Bitboard::from_square(&stack);

            let dist1 = bb.adjacent() & (empty | capturable | Bitboard::from_square(&solo));
            for mv in dist1 {
                if func(Mv::SoloStackMove(solo, stack, mv)) {
                    return true;
                }
            }

            let blockers = (us | them) ^ Bitboard::from_square(&solo);
            let dist2 = bb.doubles(blockers) & (empty | capturable);
            for mv in dist2 {
                if func(Mv::SoloStackMove(solo, stack, mv)) {
                    return true;
                }
            }
        }
    }

    // StackMove
    for stack in piece_mask & tall {
        let bb = Bitboard::from_square(&stack);

        let dist1 = bb.adjacent() & (empty | capturable);
        for mv in dist1 {
            if func(Mv::StackMove(stack, mv)) {
                return true;
            }
        }

        let dist2 = bb.doubles(us | them) & (empty | capturable);
        for mv in dist2 {
            if func(Mv::StackMove(stack, mv)) {
                return true;
            }
        }
    }

    // StackMoveDestack
    for stack in piece_mask & tall {
        let bb = Bitboard::from_square(&stack);

        let dist1 = bb.adjacent() & (empty | capturable);
        for mv in dist1 {
            for destack in
                Bitboard::from_square(&mv).adjacent() & (empty | stackable | capturable | bb)
            {
                if func(Mv::StackMoveDestack(stack, mv, destack)) {
                    return true;
                }
            }
        }

        let dist2 = bb.doubles(us | them) & (empty | capturable);
        for mv in dist2 {
            for destack in Bitboard::from_square(&mv).adjacent() & (empty | stackable | capturable)
            {
                if func(Mv::StackMoveDestack(stack, mv, destack)) {
                    return true;
                }
            }
        }
    }

    // StackDestack
    for stack in piece_mask & tall {
        for destack in Bitboard::from_square(&stack).adjacent() & (empty | capturable) {
            if func(Mv::StackDestack(stack, destack)) {
                return true;
            }
        }
    }

    // StackStack
    for stack1 in piece_mask & tall {
        for stack2 in Bitboard::from_square(&stack1).adjacent() & us & short & stackable {
            if func(Mv::StackStack(stack1, stack2)) {
                return true;
            }
        }
    }

    // StackStackMove
    for stack1 in piece_mask & tall {
        for stack2 in Bitboard::from_square(&stack1).adjacent() & us & short & stackable {
            let bb = Bitboard::from_square(&stack2);

            let dist1 = bb.adjacent() & (empty | capturable);
            for mv in dist1 {
                if func(Mv::StackStackMove(stack1, stack2, mv)) {
                    return true;
                }
            }

            let dist2 = bb.doubles(us | them) & (empty | capturable);
            for mv in dist2 {
                if func(Mv::StackStackMove(stack1, stack2, mv)) {
                    return true;
                }
            }
        }
    }

    false
}

impl Position {
    pub fn move_generator(&self, mut func: impl FnMut(Mv) -> bool) -> () {
        // Rock > Scissors
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Rock) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Scissors) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Paper > Rock
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Paper) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Rock) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Scissors > Paper
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Scissors) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Paper) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Wise > All
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Wise) & self.get_us(),
            self.get_us() & self.get_short() & self.get_wise(),
            Bitboard::empty(),
            &mut func,
        ) {
            return;
        }
    }
}
