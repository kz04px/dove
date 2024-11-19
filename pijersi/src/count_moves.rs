use crate::{
    bitboard::Bitboard,
    position::{Piece, Position},
};

#[must_use]
fn get_moves(
    us: Bitboard,
    them: Bitboard,
    short: Bitboard,
    tall: Bitboard,
    piece_mask: Bitboard,
    stackable: Bitboard,
    capturable: Bitboard,
) -> i32 {
    let empty = !(us | them);
    let mut nodes = 0;

    // SoloMove
    for solo in piece_mask & short {
        nodes += (Bitboard::from_square(&solo).adjacent() & (empty | capturable)).count();
    }

    // SoloStack
    for solo in piece_mask & short {
        nodes += (Bitboard::from_square(&solo).adjacent() & us & short & stackable).count();
    }

    // SoloStackMove
    for solo in piece_mask & short {
        for stack in Bitboard::from_square(&solo).adjacent() & us & short & stackable {
            let bb = Bitboard::from_square(&stack);

            let blockers = (us | them) ^ Bitboard::from_square(&solo);
            let dist1 = bb.adjacent() & (empty | capturable | Bitboard::from_square(&solo));
            let dist2 = bb.doubles(blockers) & (empty | capturable);

            nodes += dist1.count();
            nodes += dist2.count();
        }
    }

    // StackMove
    for stack in piece_mask & tall {
        let bb = Bitboard::from_square(&stack);
        let dist1 = bb.adjacent() & (empty | capturable);
        let dist2 = bb.doubles(us | them) & (empty | capturable);

        nodes += dist1.count();
        nodes += dist2.count();
    }

    // StackMoveDestack
    for stack in piece_mask & tall {
        let bb = Bitboard::from_square(&stack);
        let dist1 = bb.adjacent() & (empty | capturable);
        let dist2 = bb.doubles(us | them) & (empty | capturable);

        for mv in dist1 {
            nodes += (Bitboard::from_square(&mv).adjacent()
                & (empty | stackable | capturable | bb))
                .count();
        }

        for mv in dist2 {
            nodes +=
                (Bitboard::from_square(&mv).adjacent() & (empty | stackable | capturable)).count();
        }
    }

    // StackDestack
    for stack in piece_mask & tall {
        nodes += (Bitboard::from_square(&stack).adjacent() & (empty | capturable)).count();
    }

    // StackStack
    for stack1 in piece_mask & tall {
        nodes += (Bitboard::from_square(&stack1).adjacent() & us & short & stackable).count();
    }

    // StackStackMove
    for stack1 in piece_mask & tall {
        for stack2 in Bitboard::from_square(&stack1).adjacent() & us & short & stackable {
            let bb = Bitboard::from_square(&stack2);
            let dist1 = bb.adjacent() & (empty | capturable);
            let dist2 = bb.doubles(us | them) & (empty | capturable);

            nodes += dist1.count();
            nodes += dist2.count();
        }
    }

    nodes
}

impl Position {
    #[must_use]
    pub fn count_moves(&self) -> i32 {
        let mut count = 0;

        self.move_generator(&mut |_| {
            count += 1;
            false
        });

        count
    }

    #[must_use]
    pub fn count_moves_sneaky(&self) -> i32 {
        let mut nodes = 0;

        // Rock > Scissors
        nodes += get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Rock) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Scissors) & self.get_them(),
        );

        // Paper > Rock
        nodes += get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Paper) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Rock) & self.get_them(),
        );

        // Scissors > Paper
        nodes += get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Scissors) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Paper) & self.get_them(),
        );

        // Wise > All
        nodes += get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Wise) & self.get_us(),
            self.get_us() & self.get_short() & self.get_wise(),
            Bitboard::empty(),
        );

        nodes
    }
}
