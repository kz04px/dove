use crate::{
    bitboard::{Bitboard, Bitmasks},
    position::Position,
};

impl Position {
    #[must_use]
    pub fn is_gameover(&self) -> bool {
        let white_rps = self.get_white() & self.get_rps();
        let black_rps = self.get_black() & self.get_rps();

        // Material
        self.get_white().is_empty()
            | self.get_black().is_empty()
        // Goal reached
            | (white_rps & Bitboard(Bitmasks::BlackHome as u64)).is_occupied()
            | (black_rps & Bitboard(Bitmasks::WhiteHome as u64)).is_occupied()
        // Halfmoves
            | (self.halfmoves >= 20)
    }
}
