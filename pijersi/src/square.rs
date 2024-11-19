use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Square(pub u8);

impl Square {
    #[must_use]
    pub fn from_coords(x: i32, y: i32) -> Self {
        Self(6 * y as u8 + x as u8 + y as u8 / 2)
    }

    #[must_use]
    pub fn file(&self) -> i32 {
        let lut = [
            0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2,
            3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5,
        ];
        lut[self.0 as usize]
    }

    #[must_use]
    pub fn rank(&self) -> i32 {
        let lut = [
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4,
            4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
        ];
        lut[self.0 as usize]
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let files = ['1', '2', '3', '4', '5', '6', '7'];
        let ranks = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let file_char = files[self.file() as usize];
        let rank_char = ranks[self.rank() as usize];
        write!(f, "{}{}", rank_char, file_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords() {
        // a1 a2 a3
        assert_eq!(Square::from_coords(0, 0), Square(0));
        assert_eq!(Square::from_coords(1, 0), Square(1));
        assert_eq!(Square::from_coords(2, 0), Square(2));
        // b1 b2 b3
        assert_eq!(Square::from_coords(0, 1), Square(6));
        assert_eq!(Square::from_coords(1, 1), Square(7));
        assert_eq!(Square::from_coords(2, 1), Square(8));
        // c1 c2 c3
        assert_eq!(Square::from_coords(0, 2), Square(13));
        assert_eq!(Square::from_coords(1, 2), Square(14));
        assert_eq!(Square::from_coords(2, 2), Square(15));
    }

    #[test]
    fn files() {
        assert_eq!(Square(0).file(), 0);
        assert_eq!(Square(1).file(), 1);
        assert_eq!(Square(5).file(), 5);
        assert_eq!(Square(6).file(), 0);
    }

    #[test]
    fn ranks() {
        assert_eq!(Square(0).rank(), 0);
        assert_eq!(Square(1).rank(), 0);
        assert_eq!(Square(5).rank(), 0);
        assert_eq!(Square(6).rank(), 1);
    }

    #[test]
    fn string() {
        assert_eq!(format!("{}", Square(0)), "a1");
        assert_eq!(format!("{}", Square(1)), "a2");
        assert_eq!(format!("{}", Square(5)), "a6");
        assert_eq!(format!("{}", Square(6)), "b1");
        assert_eq!(format!("{}", Square(22)), "d4");
        assert_eq!(format!("{}", Square(43)), "g5");
        assert_eq!(format!("{}", Square(44)), "g6");
    }
}
