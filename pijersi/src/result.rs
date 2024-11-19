use crate::{
    bitboard::{Bitboard, Bitmasks},
    position::Position,
};

#[derive(PartialEq, Debug)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

impl Position {
    #[must_use]
    pub fn result(&self) -> Option<GameResult> {
        let white_rps = self.get_white() & self.get_rps();
        let black_rps = self.get_black() & self.get_rps();
        let is_white_home = (white_rps & Bitboard(Bitmasks::BlackHome as u64)).is_occupied();
        let is_black_home = (black_rps & Bitboard(Bitmasks::WhiteHome as u64)).is_occupied();

        // Material
        if self.get_white().is_empty() && self.get_black().is_empty() {
            Some(GameResult::Draw)
        } else if self.get_white().is_empty() && self.get_black().is_occupied() {
            Some(GameResult::BlackWin)
        } else if self.get_black().is_empty() && self.get_white().is_occupied() {
            Some(GameResult::WhiteWin)
        }
        // Goal reached
        else if is_white_home {
            Some(GameResult::WhiteWin)
        } else if is_black_home {
            Some(GameResult::BlackWin)
        }
        // Halfmoves
        else if self.halfmoves >= 20 {
            Some(GameResult::Draw)
        }
        // Edge cases
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_results() {
        let tests = [
            // None
            ("startpos", None),
            ("r-5/7/6/7/6/7/R-5 w 0 1", None),
            ("r-5/7/6/7/6/7/R-5 w 19 1", None),
            // Goal reached
            ("r-R-4/7/6/7/6/7/6 w 0 1", Some(GameResult::WhiteWin)),
            ("6/7/6/7/6/7/r-R-4 w 0 1", Some(GameResult::BlackWin)),
            // Material
            ("6/7/6/7/6/7/R-5 w 0 1", Some(GameResult::WhiteWin)),
            ("r-5/7/6/7/6/7/6 w 0 1", Some(GameResult::BlackWin)),
            // Halfmoves
            ("r-5/7/6/7/6/7/R-5 w 20 1", Some(GameResult::Draw)),
        ];

        for (fen, result) in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(pos.result(), result);
        }
    }
}
