use std::fmt;

use crate::{bitboard::Bitboard, layer::Layer, side::Side, square::Square};

#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub sides: [Bitboard; 2],
    pub pieces: [[Bitboard; 2]; 4],
    pub turn: Side,
    pub halfmoves: i32,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[must_use]
pub enum Piece {
    Rock,
    Paper,
    Scissors,
    Wise,
}

impl Position {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            sides: [Bitboard::empty(), Bitboard::empty()],
            pieces: [
                [Bitboard::empty(), Bitboard::empty()],
                [Bitboard::empty(), Bitboard::empty()],
                [Bitboard::empty(), Bitboard::empty()],
                [Bitboard::empty(), Bitboard::empty()],
            ],
            turn: Side::White,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let mut pos = Self::empty();
        pos.set_fen(&fen);
        pos
    }

    #[must_use]
    pub fn get_us(&self) -> Bitboard {
        self.sides[self.turn as usize]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard {
        self.sides[!self.turn as usize]
    }

    #[must_use]
    pub fn get_white(&self) -> Bitboard {
        self.sides[Side::White as usize]
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard {
        self.sides[Side::Black as usize]
    }

    #[must_use]
    pub fn get_rock(&self) -> Bitboard {
        self.pieces[Piece::Rock as usize][Layer::Lower as usize]
            | self.pieces[Piece::Rock as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_paper(&self) -> Bitboard {
        self.pieces[Piece::Paper as usize][Layer::Lower as usize]
            | self.pieces[Piece::Paper as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_scissors(&self) -> Bitboard {
        self.pieces[Piece::Scissors as usize][Layer::Lower as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_rps(&self) -> Bitboard {
        self.get_rock() | self.get_paper() | self.get_scissors()
    }

    #[must_use]
    pub fn get_wise(&self) -> Bitboard {
        self.pieces[Piece::Wise as usize][Layer::Lower as usize]
            | self.pieces[Piece::Wise as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_lower(&self) -> Bitboard {
        self.pieces[Piece::Rock as usize][Layer::Lower as usize]
            | self.pieces[Piece::Paper as usize][Layer::Lower as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Lower as usize]
            | self.pieces[Piece::Wise as usize][Layer::Lower as usize]
    }

    #[must_use]
    pub fn get_upper(&self) -> Bitboard {
        self.pieces[Piece::Rock as usize][Layer::Upper as usize]
            | self.pieces[Piece::Paper as usize][Layer::Upper as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Upper as usize]
            | self.pieces[Piece::Wise as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_short(&self) -> Bitboard {
        self.get_lower() ^ self.get_upper()
    }

    #[must_use]
    pub fn get_tall(&self) -> Bitboard {
        self.get_upper()
    }

    #[must_use]
    pub fn get_visible(&self, piece: Piece) -> Bitboard {
        self.pieces[piece as usize][Layer::Upper as usize]
            | (self.pieces[piece as usize][Layer::Lower as usize] & !self.get_upper())
    }

    #[must_use]
    pub fn get_occupied(&self) -> Bitboard {
        self.sides[Side::White as usize] | self.sides[Side::Black as usize]
    }

    #[must_use]
    pub fn get_piece_on(&self, layer: Layer, sq: Square) -> Option<Piece> {
        if self.pieces[Piece::Rock as usize][layer as usize].is_set(&sq) {
            Some(Piece::Rock)
        } else if self.pieces[Piece::Paper as usize][layer as usize].is_set(&sq) {
            Some(Piece::Paper)
        } else if self.pieces[Piece::Scissors as usize][layer as usize].is_set(&sq) {
            Some(Piece::Scissors)
        } else if self.pieces[Piece::Wise as usize][layer as usize].is_set(&sq) {
            Some(Piece::Wise)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_side_on(&self, sq: Square) -> Option<Side> {
        if self.sides[Side::White as usize].is_set(&sq) {
            Some(Side::White)
        } else if self.sides[Side::Black as usize].is_set(&sq) {
            Some(Side::Black)
        } else {
            None
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..7).rev() {
            let is_long = y % 2 == 1;

            if !is_long {
                write!(f, "  ")?;
            }

            for x in 0..6 + is_long as i32 {
                write!(f, "  ")?;

                let sq = Square::from_coords(x, y);
                let bottom = self.get_piece_on(Layer::Lower, sq);
                let top = self.get_piece_on(Layer::Upper, sq);
                let side = self.get_side_on(sq);

                match (bottom, side) {
                    (Some(Piece::Rock), Some(Side::White)) => write!(f, "R")?,
                    (Some(Piece::Rock), Some(Side::Black)) => write!(f, "r")?,
                    (Some(Piece::Paper), Some(Side::White)) => write!(f, "P")?,
                    (Some(Piece::Paper), Some(Side::Black)) => write!(f, "p")?,
                    (Some(Piece::Scissors), Some(Side::White)) => write!(f, "S")?,
                    (Some(Piece::Scissors), Some(Side::Black)) => write!(f, "s")?,
                    (Some(Piece::Wise), Some(Side::White)) => write!(f, "W")?,
                    (Some(Piece::Wise), Some(Side::Black)) => write!(f, "w")?,
                    (None, _) => write!(f, ".")?,
                    (_, _) => write!(f, "?")?,
                }

                match (top, side) {
                    (Some(Piece::Rock), Some(Side::White)) => write!(f, "R")?,
                    (Some(Piece::Rock), Some(Side::Black)) => write!(f, "r")?,
                    (Some(Piece::Paper), Some(Side::White)) => write!(f, "P")?,
                    (Some(Piece::Paper), Some(Side::Black)) => write!(f, "p")?,
                    (Some(Piece::Scissors), Some(Side::White)) => write!(f, "S")?,
                    (Some(Piece::Scissors), Some(Side::Black)) => write!(f, "s")?,
                    (Some(Piece::Wise), Some(Side::White)) => write!(f, "W")?,
                    (Some(Piece::Wise), Some(Side::Black)) => write!(f, "w")?,
                    (None, _) => write!(f, ".")?,
                    (_, _) => write!(f, "?")?,
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "Turn: {}", self.turn)?;
        writeln!(f, "Halfmoves: {}", self.halfmoves)?;

        Ok(())
    }
}
