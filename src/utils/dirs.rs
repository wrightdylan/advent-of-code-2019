#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveError {
    NumericOverflow,
    InvalidTypeConversion,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumericOverflow => write!(f, "Coordinate operation caused a numeric overflow or underflow"),
            Self::InvalidTypeConversion => write!(f, "Failed to cast coordinate variable safely between types"),
        }
    }
}

impl std::error::Error for MoveError {}

// Orthogonals
pub const ORTHO: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ortho {
    North,
    East,
    South,
    West,
}

// Note that for enumerations, the coordinates are actually inverted for north and south
impl Ortho {
    pub const UP: Self = Ortho::North;
    pub const RIGHT: Self = Ortho::East;
    pub const DOWN: Self = Ortho::South;
    pub const LEFT: Self = Ortho::West;
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
    pub const UP: Self = Cando::North;
    pub const RIGHT: Self = Cando::East;
    pub const DOWN: Self = Cando::South;
    pub const LEFT: Self = Cando::West;

    pub const UP_RIGHT: Self = Cando::Northeast;
    pub const UP_LEFT: Self = Cando::Northwest;
    pub const DOWN_RIGHT: Self = Cando::Southeast;
    pub const DOWN_LEFT: Self = Cando::Southwest;
}

/// Provides a nice interface for functions to choose whether to use Ortho or Cando
pub trait DirectionProvider: Sized + Copy + Eq {
    type Iter: Iterator<Item = (i32, i32)>;
    type EnumIter: Iterator<Item = Self>;

    fn enumerate(dx: i32, dy: i32) -> Self;

    fn flip(&self) -> Self {
        let (dx, dy) = self.to_dir();
        Self::enumerate(-dx, -dy)
    }

    fn get_directions() -> Self::Iter;
    fn get_enum(offset: (i32, i32)) -> Self;

    fn iter() -> Self::EnumIter;

    fn move_from<T>(&self, coord: (T, T)) -> Result<(T, T), MoveError>
    where
        T: num_traits::NumCast + std::ops::Add<Output = T>,
    {
        let (dx, dy) = self.to_dir();
        
        // Convert generic components to i32 for safe algebraic mixing
        let cx = T::to_i32(&coord.0).ok_or(MoveError::InvalidTypeConversion)?;
        let cy = T::to_i32(&coord.1).ok_or(MoveError::InvalidTypeConversion)?;

        // Check for integer overflow/underflow BEFORE casting back
        let target_x = cx.checked_add(dx).ok_or(MoveError::NumericOverflow)?;
        let target_y = cy.checked_add(dy).ok_or(MoveError::NumericOverflow)?;
        
        // Convert back to original generic layout type T safely
        let tx = T::from(target_x).ok_or(MoveError::NumericOverflow)?;
        let ty = T::from(target_y).ok_or(MoveError::NumericOverflow)?;
        
        Ok((tx, ty))
    }

    fn to_dir(&self) -> (i32, i32);
    fn turn_left(&self) -> Self;
    fn turn_right(&self) -> Self;
}

/// Returns all four directions
impl DirectionProvider for Ortho {
    type Iter = std::array::IntoIter<(i32, i32), 4>;
    type EnumIter = std::array::IntoIter<Self, 4>;

    fn enumerate(dx: i32, dy: i32) -> Self {
        match (dx, dy) {
            (0, -1) => Ortho::North,
            (1, 0)  => Ortho::East,
            (0, 1)  => Ortho::South,
            (-1, 0) => Ortho::West,
            _ => panic!("Invalid direction: ({}, {})", dx, dy),
        }
    }

    fn get_directions() -> Self::Iter {
        ORTHO.into_iter()
    }

    fn get_enum((dx, dy): (i32, i32)) -> Self {
        Ortho::enumerate(dx, dy)
    }

    /// Creates an iterator of orthogonal directions.
    fn iter() -> Self::EnumIter {
        [Ortho::North, Ortho::East, Ortho::South, Ortho::West].into_iter()
    }

    /// Converts an enum direction to coordinates.
    fn to_dir(&self) -> (i32, i32) {
        match self {
            Ortho::North => (0, -1),
            Ortho::South => (0, 1),
            Ortho::East  => (1, 0),
            Ortho::West  => (-1, 0),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Ortho::North => Ortho::West,
            Ortho::South => Ortho::East,
            Ortho::East  => Ortho::North,
            Ortho::West  => Ortho::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Ortho::North => Ortho::East,
            Ortho::South => Ortho::West,
            Ortho::East  => Ortho::South,
            Ortho::West  => Ortho::North,
        }
    }
}

/// Returns all eight directions
impl DirectionProvider for Cando {
    type Iter = std::array::IntoIter<(i32, i32), 8>;
    type EnumIter = std::array::IntoIter<Self, 8>;

    fn enumerate(dx: i32, dy: i32) -> Self {
        match (dx, dy) {
            (0, -1)  => Cando::North,
            (1, 0)   => Cando::East,
            (0, 1)   => Cando::South,
            (-1, 0)  => Cando::West,
            (-1, -1) => Cando::Northwest,
            (1, -1)  => Cando::Northeast,
            (-1, 1)  => Cando::Southwest,
            (1, 1)   => Cando::Southeast,
            _ => panic!("Invalid direction: ({}, {})", dx, dy),
        }
    }

    fn get_directions() -> Self::Iter {
        CANDO.into_iter()
    }

    fn get_enum((dx, dy): (i32, i32)) -> Self {
        Cando::enumerate(dx, dy)
    }

    /// Creates an iterator of cardinal and ordinal directions.
    fn iter() -> Self::EnumIter {
        [
            Cando::North, Cando::Northeast, Cando::East, Cando::Southeast,
            Cando::South, Cando::Southwest, Cando::West, Cando::Northwest,
        ].into_iter()
    }

    /// Converts an enum direction to coordinates.
    fn to_dir(&self) -> (i32, i32) {
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

    fn turn_left(&self) -> Self {
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

    fn turn_right(&self) -> Self {
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

pub trait CoordinateExt<D: DirectionProvider> {
    fn move_dir(self, dir: D) -> Result<Self, MoveError> where Self: Sized;
}

impl<T, D> CoordinateExt<D> for (T, T)
where
    D: DirectionProvider,
    T: Copy + num_traits::NumCast + std::ops::Add<Output = T>,
{
    fn move_dir(self, dir: D) -> Result<Self, MoveError> {
        dir.move_from(self)
    }
}