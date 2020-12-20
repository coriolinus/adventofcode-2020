#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    /// `(dx, dy)`, for `Right` is `+x` and `Up` is `+y`
    pub fn deltas(self) -> (i32, i32) {
        use Direction::*;
        match self {
            Up => (0, 1),
            Down => (0, -1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }

    pub fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn turn_left(self) -> Direction {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }

    /// Iterate over the four orthogonal directions
    pub fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Up, Down, Left, Right].iter().copied()
    }

    /// Iterate over the four diagonal direction-pairs
    ///
    /// Each pair takes the form `(vertical, horizontal)`.
    pub fn iter_diag() -> impl Iterator<Item = (Direction, Direction)> {
        use Direction::*;
        [(Up, Left), (Up, Right), (Down, Left), (Down, Right)]
            .iter()
            .copied()
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}
