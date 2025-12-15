// Orthogonals
pub const ORTHO: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ortho {
    North,
    East,
    South,
    West,
}

impl Ortho {
    pub const UP: Self = Ortho::North;
    pub const RIGHT: Self = Ortho::East;
    pub const DOWN: Self = Ortho::South;
    pub const LEFT: Self = Ortho::West;

    pub fn enumerate(dx: &i32, dy: &i32) -> Self {
        match (dx, dy) {
            (0, -1) => Ortho::North,
            (1, 0)  => Ortho::East,
            (0, 1)  => Ortho::South,
            (-1, 0) => Ortho::West,
            _ => unreachable!(),
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            Ortho::North => Ortho::South,
            Ortho::South => Ortho::North,
            Ortho::East  => Ortho::West,
            Ortho::West  => Ortho::East,
        }
    }

    /// Creates an iterator of orthogonal directions.
    pub fn iter() -> impl Iterator<Item = Self> {
        [Ortho::North, Ortho::East, Ortho::South, Ortho::West].iter().copied()
    }

    /// Converts an enum direction to coordinates.
    pub fn to_dir(&self) -> (i32, i32) {
        match self {
            Ortho::North => (0, -1),
            Ortho::South => (0, 1),
            Ortho::East  => (1, 0),
            Ortho::West  => (-1, 0),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Ortho::North => Ortho::West,
            Ortho::South => Ortho::East,
            Ortho::East  => Ortho::North,
            Ortho::West  => Ortho::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Ortho::North => Ortho::East,
            Ortho::South => Ortho::West,
            Ortho::East  => Ortho::South,
            Ortho::West  => Ortho::North,
        }
    }
}

// Cardinals and ordinals (or intercardinals)
pub const CANDO: [(i32, i32); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cando {
    North,
    East,
    South,
    West,
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

impl Cando {
    pub fn enumerate(dx: &i32, dy: &i32) -> Self {
        match (dx, dy) {
            (0, -1)  => Cando::North,
            (1, 0)   => Cando::East,
            (0, 1)   => Cando::South,
            (-1, 0)  => Cando::West,
            (-1, -1) => Cando::Northwest,
            (1, -1)  => Cando::Northeast,
            (-1, 1)  => Cando::Southwest,
            (1, 1)   => Cando::Southeast,
            _ => unreachable!(),
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            Cando::North     => Cando::South,
            Cando::South     => Cando::North,
            Cando::East      => Cando::West,
            Cando::West      => Cando::East,
            Cando::Northwest => Cando::Southeast,
            Cando::Northeast => Cando::Southwest,
            Cando::Southwest => Cando::Northeast,
            Cando::Southeast => Cando::Northwest,
        }
    }

    /// Creates an iterator of cardinal and ordinal directions.
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Cando::North, Cando::Northeast, Cando::East, Cando::Southeast,
            Cando::South, Cando::Southwest, Cando::West, Cando::Northwest,
        ].iter().copied()
    }

    /// Converts an enum direction to coordinates.
    pub fn to_dir(&self) -> (i32, i32) {
        match self {
            Cando::North     => (0, -1),
            Cando::South     => (0, 1),
            Cando::East      => (1, 0),
            Cando::West      => (-1, 0),
            Cando::Northwest => (-1, -1),
            Cando::Northeast => (1, -1),
            Cando::Southwest => (-1, 1),
            Cando::Southeast => (1, 1),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Cando::North     => Cando::Northwest,
            Cando::South     => Cando::Southeast,
            Cando::East      => Cando::Northeast,
            Cando::West      => Cando::Southwest,
            Cando::Northwest => Cando::West,
            Cando::Northeast => Cando::North,
            Cando::Southwest => Cando::South,
            Cando::Southeast => Cando::East,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Cando::North     => Cando::Northeast,
            Cando::South     => Cando::Southwest,
            Cando::East      => Cando::Southeast,
            Cando::West      => Cando::Northwest,
            Cando::Northwest => Cando::North,
            Cando::Northeast => Cando::East,
            Cando::Southwest => Cando::West,
            Cando::Southeast => Cando::South,
        }
    }
}