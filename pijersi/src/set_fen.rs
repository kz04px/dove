use crate::{
    bitboard::Bitboard,
    layer::Layer,
    position::{Piece, Position},
    side::Side,
    square::Square,
};

impl Position {
    pub fn set_fen(&mut self, fen: &str) -> () {
        if fen == "startpos" {
            self.set_fen("s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-P-/R-P-S-R-P-S- w 0 1");
        } else {
            *self = Self {
                sides: [Bitboard::empty(), Bitboard::empty()],
                pieces: [
                    // Rock
                    [Bitboard::empty(), Bitboard::empty()],
                    // Paper
                    [Bitboard::empty(), Bitboard::empty()],
                    // Scissors
                    [Bitboard::empty(), Bitboard::empty()],
                    // Wise
                    [Bitboard::empty(), Bitboard::empty()],
                ],
                turn: Side::White,
                halfmoves: 0,
                fullmoves: 1,
            };

            let mut parts = fen.split(' ');
            let mut x = 0;
            let mut y = 6;
            let mut layer = Layer::Lower;

            // Board
            if let Some(part) = parts.next() {
                for c in part.chars() {
                    let sq = Square::from_coords(x, y);
                    let bb = Bitboard::from_square(&sq);

                    match c {
                        'R' => {
                            self.sides[Side::White as usize] |= bb;
                            self.pieces[Piece::Rock as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        'P' => {
                            self.sides[Side::White as usize] |= bb;
                            self.pieces[Piece::Paper as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        'S' => {
                            self.sides[Side::White as usize] |= bb;
                            self.pieces[Piece::Scissors as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        'W' => {
                            self.sides[Side::White as usize] |= bb;
                            self.pieces[Piece::Wise as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += if layer == Layer::Lower { 1 } else { 0 };
                        }
                        'r' => {
                            self.sides[Side::Black as usize] |= bb;
                            self.pieces[Piece::Rock as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        'p' => {
                            self.sides[Side::Black as usize] |= bb;
                            self.pieces[Piece::Paper as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        's' => {
                            self.sides[Side::Black as usize] |= bb;
                            self.pieces[Piece::Scissors as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        'w' => {
                            self.sides[Side::Black as usize] |= bb;
                            self.pieces[Piece::Wise as usize][layer as usize] ^= bb;
                            layer = !layer;
                            x += (layer == Layer::Lower) as i32;
                        }
                        '-' => {
                            x += 1;
                            layer = Layer::Lower;
                        }
                        '1'..='7' => {
                            x += (c as u8 - b'0') as i32;
                            layer = Layer::Lower;
                        }
                        '/' => {
                            x = 0;
                            y -= 1;
                            layer = Layer::Lower;
                        }
                        _ => panic!("Unrecognised FEN board token {}", c),
                    }
                }
            }

            // Side to move
            match parts.next() {
                Some("W" | "w") => self.turn = Side::White,
                Some("B" | "b") => self.turn = Side::Black,
                _ => panic!("Unrecognised FEN side token"),
            }

            // Halfmoves
            match parts.next() {
                Some(n) => self.halfmoves = n.parse::<i32>().unwrap(),
                _ => {}
            }

            // Fullmoves
            match parts.next() {
                Some(n) => self.fullmoves = n.parse::<i32>().unwrap(),
                _ => {}
            }
        }

        debug_assert!(self.is_valid());
    }

    #[must_use]
    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        // Board
        for y in (0..=6).rev() {
            let mut num_spaces: i32 = 0;

            for x in 0..6 + y % 2 {
                let sq = Square::from_coords(x, y);
                let piece_lower = self.get_piece_on(Layer::Lower, sq);
                let piece_upper = self.get_piece_on(Layer::Upper, sq);
                let side = self.get_side_on(sq);

                let is_empty = side.is_none();

                if !is_empty && num_spaces > 0 {
                    fen += &num_spaces.to_string();
                    num_spaces = 0;
                }

                // Lower
                match (piece_lower, side) {
                    (Some(Piece::Rock), Some(Side::White)) => fen += "R",
                    (Some(Piece::Paper), Some(Side::White)) => fen += "P",
                    (Some(Piece::Scissors), Some(Side::White)) => fen += "S",
                    (Some(Piece::Wise), Some(Side::White)) => fen += "W",
                    (Some(Piece::Rock), Some(Side::Black)) => fen += "r",
                    (Some(Piece::Paper), Some(Side::Black)) => fen += "p",
                    (Some(Piece::Scissors), Some(Side::Black)) => fen += "s",
                    (Some(Piece::Wise), Some(Side::Black)) => fen += "w",
                    (None, None) => num_spaces += 1,
                    (_, _) => {}
                }

                // Upper
                match (piece_upper, side) {
                    (Some(Piece::Rock), Some(Side::White)) => fen += "R",
                    (Some(Piece::Paper), Some(Side::White)) => fen += "P",
                    (Some(Piece::Scissors), Some(Side::White)) => fen += "S",
                    (Some(Piece::Wise), Some(Side::White)) => fen += "W",
                    (Some(Piece::Rock), Some(Side::Black)) => fen += "r",
                    (Some(Piece::Paper), Some(Side::Black)) => fen += "p",
                    (Some(Piece::Scissors), Some(Side::Black)) => fen += "s",
                    (Some(Piece::Wise), Some(Side::Black)) => fen += "w",
                    (None, Some(_)) => {
                        fen += "-";
                    }
                    (_, _) => {}
                }
            }

            if num_spaces > 0 {
                fen += &num_spaces.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        // Side to move
        match self.turn {
            Side::White => fen += " w",
            Side::Black => fen += " b",
        }

        // Halfmove counter
        fen += " ";
        fen += &self.halfmoves.to_string();

        // Fullmove counter
        fen += " ";
        fen += &self.fullmoves.to_string();

        fen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos() {
        assert_eq!(
            Position::from_fen("startpos").get_fen(),
            "s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-P-/R-P-S-R-P-S- w 0 1"
        );
    }

    #[test]
    fn parsing() {
        let tests = [
            // Side to move
            "6/7/6/7/6/7/6 w 0 1",
            "6/7/6/7/6/7/6 b 0 1",
            // Halfmoves
            "6/7/6/7/6/7/6 w 10 1",
            "6/7/6/7/6/7/6 w 100 1",
            // Fullmoves
            "6/7/6/7/6/7/6 w 0 10",
            "6/7/6/7/6/7/6 w 0 100",
            // Basic
            "r-5/7/6/7/6/7/R-5 w 0 1",
            "p-5/7/6/7/6/7/P-5 w 0 1",
            "s-5/7/6/7/6/7/S-5 w 0 1",
            "w-5/7/6/7/6/7/W-5 w 0 1",
            // Basic stacks
            "rr5/7/6/7/6/7/RR5 w 0 1",
            "pp5/7/6/7/6/7/PP5 w 0 1",
            "ss5/7/6/7/6/7/SS5 w 0 1",
            "ww5/7/6/7/6/7/WW5 w 0 1",
            // Mixed stacks
            "rp5/7/6/7/6/7/RP5 w 0 1",
            "ps5/7/6/7/6/7/PS5 w 0 1",
            "sr5/7/6/7/6/7/SR5 w 0 1",
            "wr5/7/6/7/6/7/WR5 w 0 1",
            "wp5/7/6/7/6/7/WP5 w 0 1",
            "ws5/7/6/7/6/7/WS5 w 0 1",
            // General
            "s-p-r-s-p-r-/p-r-s-wwr-s-p-/6/7/6/P-S-R-WWS-R-P-/R-P-S-R-P-S- w 0 1",
            "srp-r-s-2/p-1s-1r-1p-/4p-1/1RPw-3sr/2w-1RSP-/P-2WWS-2/R-S-S-1PR1 w 0 1",
            "1p-sr1p-r-/p-r-1w-w-s-p-/4r-1/3ss3/SPR-S-2R-/P-2WW2P-/R-2R-SPS- w 0 1",
            "1sp1rsp-1/p-2wwr-1pr/5s-/rs3R-2/1W-W-P-2/P-S-RP1S-2/R-1S-RP1S- w 0 1",
            "1p-r-1p-r-/1r-s-sr3/ps1W-w-2/SP1W-1w-S-s-/4R-1/P-4R-P-/R-1SR1P-S- w 0 1",
        ];

        for fen in tests {
            println!("{}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(fen, pos.get_fen());
        }
    }
}
