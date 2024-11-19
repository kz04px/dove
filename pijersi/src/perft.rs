use crate::position::Position;

impl Position {
    #[must_use]
    pub fn perft(&self, depth: i32) -> u64 {
        if depth <= 0 {
            1
        } else if self.is_gameover() {
            0
        } else if depth == 1 {
            self.count_moves() as u64
            // self.count_moves_sneaky()
        } else {
            let mut nodes = 0u64;

            self.move_generator(|mv| {
                let npos = self.after_move(&mv);
                nodes += npos.perft(depth - 1);
                false
            });

            nodes
        }
    }
}
