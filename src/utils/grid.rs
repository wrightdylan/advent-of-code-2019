use crate::prelude::*;

type GridPos = (usize, usize);

/// Specific grid errors
pub enum GridError {
    OutOfBounds,
    Collision,
}

/// 1D gridness
#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    entity: Vec<T>,
}

impl<T: Clone + Copy + PartialEq> Grid<T> {
    /// New blank grid
    pub fn new(width: usize, height: usize, entity: Vec<T>) -> Self {
        Self { width, height, entity }
    }

    /// New grid with fill
    pub fn new_fill(width: usize, height: usize, fill: T) -> Self {
        let entity = vec![fill.clone(); width * height];
        Self { width, height, entity }
    }

    /// New grid from an ASCII block of chars
    pub fn new_from_block(input: &str) -> Grid<char> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let entity = input.lines().flat_map(|line| line.chars()).collect();

        Grid::new(width, height, entity)
    }

    /// Converts a DynaMap to Grid
    pub fn new_from_dynamap(dynamap: DynaMap<T>, fill: T) -> Self
    where
        T: Eq + std::hash::Hash
    {
        let (min_x, max_x, min_y, max_y) = dynamap.get_bounds().unwrap();
        let width = (max_x - min_x) as usize;
        let height = (max_y - min_y) as usize;
        let mut grid = Grid::new_fill(width, height, fill);
        let offset_x = 0 - min_x;
        let offset_y = 0 - min_y;

        for i in min_y..=max_y {
            for j in min_x..=max_x {
                if let Some(&val) = dynamap.get(&(j, i)) {
                    grid[(j + offset_x, i + offset_y)] = val;
                }
            }
        }

        grid
    }

    /// Creates a list of all valid adjacent points in an orthogonal, or cardinal
    /// and ordinal pattern from a given position, selected by a generic direction
    /// provider (Ortho/Cando).
    /// 
    /// # Example
    /// ```
    /// # use aoc_2019::prelude::*;
    /// let grid = Grid::new_fill(3, 3, '.');
    /// let cardinals = grid.adjacent::<Ortho>(&(1, 1)).unwrap();
    /// 
    /// assert_eq!(cardinals, vec![(1, 2), (2, 1), (1, 0), (0, 1)]);
    /// ```
    pub fn adjacent<U: DirectionProvider>(&self, pos: &GridPos) -> Option<Vec<GridPos>> {
        let valid: Vec<GridPos> = U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;
                
                (new_x < self.width && new_y < self.height).then_some((new_x, new_y))
            })
            .collect();

        (!valid.is_empty()).then_some(valid)
    }

    /// Creates a list of all valid neighbours by type in an orthogonal, or
    /// cardinal and ordinal pattern from a given position, selected by a
    /// generic direction provider (Ortho/Cando), and returns the directional
    /// enum.
    ///
    /// # Example
    /// ```
    /// # use aoc_2019::prelude::*;
    /// let grid = Grid::new_fill(3, 3, '.');
    /// let cardinals = grid.adjacent_with_enum::<Ortho>(&(1, 1)).unwrap();
    /// 
    /// assert_eq!(cardinals, vec![((1, 2), Ortho::South), ((2, 1), Ortho::East), ((1, 0), Ortho::North), ((0, 1), Ortho::West)]);
    /// ```
    pub fn adjacent_with_enum<U: DirectionProvider>(&self, pos: &GridPos) -> Option<Vec<(GridPos, U)>> {
        let valid: Vec<(GridPos, U)> = U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;

                (new_x < self.width && new_y < self.height).then_some(((new_x, new_y), U::get_enum((dx, dy))))
            })
            .collect();

        (!valid.is_empty()).then_some(valid)
    }

    /// Counts the number of occurrances of all items
    pub fn count_all(&self) -> HashMap<&T, usize>
    where
        T: Eq + std::hash::Hash + Clone
    {
        let mut counts: HashMap<&T, usize> = HashMap::new();

        for item in &self.entity {
            *counts.entry(item).or_insert(0) += 1;
        }

        counts
    }

    /// Counts the number of adjacent tiles matching the given type.
    pub fn count_neighbours_by_type<U>(&self, pos: &GridPos, value: T) -> usize
    where
        T: PartialEq,
        U: DirectionProvider,
    {
        U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;

                let idx = (new_x < self.width && new_y < self.height)
                    .then(|| (self.width * new_y) + new_x)?;
                
                let &tile = self.entity.get(idx)?;
                (tile == value).then_some(())
            })
            .count()
    }

    /// Counts the number of occurrances of a given target
    pub fn count_type(&self, target: &T) -> usize {
        self.entity.iter().filter(|&x| x == target).count()
    }

    /// Finds the position of only the first instance of a matching target
    pub fn find_first(&self, target: &T) -> Option<GridPos> {
        self.entity
            .iter()
            .position(|item| item == target)
            .map(|index| (index % self.width, index / self.width))
    }

    /// Finds all coordinates of a matching target 
    pub fn find_pos(&self, target: &T) -> Vec<GridPos> {
        self.entity
            .iter()
            .enumerate()
            .filter_map(|(index, item)|
                (item == target)
                    .then_some((index % self.width, index / self.width))
            )
            .collect()
    }

    /// Returns the entity at a specific index
    pub fn get(&self, idx: usize) -> T {
        self.entity[idx]
    }

    /// Creates a list of all valid neighbours by type in an orthogonal, or
    /// cardinal and ordinal pattern from a given position, selected by a
    /// generic direction provider (Ortho/Cando).
    pub fn get_neighbours_by_type<U>(&self, pos: &GridPos, value: T) -> Option<Vec<GridPos>>
    where
        T: PartialEq,
        U: DirectionProvider,
    {
        let valid: Vec<GridPos> = U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;

                (new_x < self.width && new_y < self.height)
                    .then(|| (self.width * new_y) + new_x)
                    .and_then(|idx| self.entity.get(idx))
                    .and_then(|&tile| (tile == value).then_some((new_x, new_y)))
            })
            .collect();

        (!valid.is_empty()).then_some(valid)
    }

    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a list of points that are within the given Manhattan distance
    /// of the start point.
    pub fn in_range(&self, pos: &GridPos, dist: usize) -> Vec<(GridPos, usize)> {
        let mut points = Vec::new();

        for y in max(pos.1 as i32 - dist as i32, 0) as usize..=min(pos.1 + dist, self.height - 1) {
            for x in max(pos.0 as i32 - dist as i32, 0) as usize..=min(pos.0 + dist, self.width - 1) {
                if (pos.0 as i32 - x as i32).abs() + (pos.1 as i32 - y as i32).abs() <= dist as i32 {
                    let md = (pos.0 as i32 - x as i32).abs() + (pos.1 as i32 - y as i32).abs();
                    points.push(((x, y), md as usize));
                }
            }
        }

        points
    }

    /// Returns a list of points that are within the given Manhattan distance
    /// of the start point that contain the given entity.
    pub fn in_range_as(&self, pos: &GridPos, dist: usize, ent_type: T) -> Vec<(GridPos, usize)> {
        let mut points = Vec::new();

        for y in max(pos.1 as i32 - dist as i32, 0) as usize..=min(pos.1 + dist, self.height - 1) {
            for x in max(pos.0 as i32 - dist as i32, 0) as usize..=min(pos.0 + dist, self.width - 1) {
                if (pos.0 as i32 - x as i32).abs() + (pos.1 as i32 - y as i32).abs() <= dist as i32 {
                    let idx = y * self.width + x;
                    if let Some(entity) = self.entity.get(idx) {
                        if *entity == ent_type {
                            let md = (pos.0 as i32 - x as i32).abs() + (pos.1 as i32 - y as i32).abs();
                            points.push(((x, y), md as usize));
                        }
                    }
                }
            }
        }

        points
    }

    /// Checks if movement in a certain direction is valid
    pub fn is_valid(&self, pos: &GridPos, dir: Ortho) -> bool {
        match dir {
            Ortho::North => if pos.1 == 0 { return false },
            Ortho::East => if pos.0 == self.width - 1 { return false },
            Ortho::South => if pos.1 == self.height - 1 { return false },
            Ortho::West => if pos.0 == 0 { return false },
        }

        true
    }

    /// Returns a list of elements in order from the start position in the direction
    /// looked at for a given distance.
    pub fn look(&self, from: &GridPos, dir: &(i32, i32), dist: usize) -> Vec<(GridPos, T)> {
        let (from_x, from_y) = from;
        let (dir_x, dir_y) = dir;
    
        let mut results = Vec::new();
    
        for i in 1..=dist as i32 {
            let to_x = (*from_x as i32 + dir_x * i) as usize;
            let to_y = (*from_y as i32 + dir_y * i) as usize;

            if to_x < self.width && to_y < self.height {
                let to_idx = to_y * self.width + to_x;
                results.push(((to_x, to_y), self.entity[to_idx].clone()));
            }
    
        }
    
        results
    }

    /// Creates a list of all valid neighbours by type and coorrdinates in an
    /// orthogonal, or cardinal and ordinal pattern from a given position,
    /// selected by a generic direction provider (Ortho/Cando).
    pub fn neighbours<U: DirectionProvider>(&self, pos: &GridPos) -> Option<Vec<(GridPos, T)>> {
        let valid: Vec<(GridPos, T)> = U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;

                (new_x < self.width && new_y < self.height)
                    .then(|| (self.width * new_y) + new_x)
                    .and_then(|idx| self.entity.get(idx))
                    .map(|&tile| ((new_x, new_y), tile))
            })
            .collect();

        (!valid.is_empty()).then_some(valid)
    }

    /// Counts the number of neighbouring adjacent points by type in an orthogonal,
    /// or cardinal and ordinal pattern from a given position, selected by a
    /// generic direction provider (Ortho/Cando).
    ///
    /// # Example
    /// ```
    /// # use aoc_2019::prelude::*;
    /// let grid = Grid::new_fill(3, 3, '.');
    /// let cardinals = grid.adjacent_with_enum::<Ortho>(&(1, 1)).unwrap();
    /// 
    /// assert_eq!(grid.neighbours_count_by_type::<Ortho>(&(0, 0), '.'), 2);
    /// assert_eq!(grid.neighbours_count_by_type::<Ortho>(&(1, 0), '.'), 3);
    /// assert_eq!(grid.neighbours_count_by_type::<Ortho>(&(1, 1), '.'), 4);
    /// assert_eq!(grid.neighbours_count_by_type::<Ortho>(&(1, 1), '#'), 0);
    /// ```
    pub fn neighbours_count_by_type<U>(&self, pos: &GridPos, value: T) -> usize
    where 
        T: PartialEq,
        U: DirectionProvider,
    {
        U::get_directions()
            .filter_map(|(dx, dy)| {
                let new_x = pos.0.checked_add_signed(dx as isize)?;
                let new_y = pos.1.checked_add_signed(dy as isize)?;

                (new_x < self.width && new_y < self.height)
                    .then(|| (self.width * new_y) + new_x)
                    .and_then(|idx| self.entity.get(idx))
                    .and_then(|&tile| (tile == value).then_some(1))
            })
            .sum()
    }

    /// Returns the element in the adjacent square in the given direction.
    pub fn peek(&self, from: &GridPos, dir: &(i32, i32)) -> Result<T, GridError> {
        let (from_x, from_y) = from;
        let (dir_x, dir_y) = dir;

        let to_x = *from_x as i32 + dir_x;
        let to_y = *from_y as i32 + dir_y;

        if to_x < 0 || to_x >= self.width as i32 || to_y < 0 || to_y >= self.height as i32 {
            return Err(GridError::OutOfBounds);
        }

        let to_idx = (to_y as usize * self.width + to_x as usize) as usize;
        Ok(self.entity[to_idx])
    }

    /// Places an entity at positions [(x, y)]
    pub fn place_at<'a, I>(&mut self, points: I, value: T)
    where
        I: IntoIterator<Item = &'a GridPos>
    {
        for &(x, y) in points {
            let index = y * self.width + x;
            if index < self.entity.len() {
                self.entity[index] = value.clone();
            }
        }
    }

    /// Sets the entity at a specific index
    pub fn set(&mut self, idx: usize, value: T) {
        self.entity[idx] = value;
    }

    /// Returns the size of the grid as a tuple (width, height)
    pub fn size(&self) -> GridPos {
        (self.width, self.height)
    }

    /// Moves an entity from the start position to a direction.
    /// The 'ignore' option allows movement even if the position being moved to
    /// contains the element to be ignored.
    pub fn slide(&mut self, from: GridPos, dir: (i32, i32), ignore: Option<T>) -> Result<(), GridError> {
        let to_x = from.0 as i32 + dir.0;
        let to_y = from.1 as i32 + dir.1;

        if to_x < 0 || to_x >= self.width as i32 || to_y < 0 || to_y >= self.height as i32 {
            return Err(GridError::OutOfBounds);
        }

        let from_idx = (from.1 * self.width + from.0) as usize;
        let to_idx = (to_y as usize * self.width + to_x as usize) as usize;

        let from_tile = self.entity[from_idx];
        let to_tile = self.entity[to_idx];

        if from_tile == ignore.unwrap_or(to_tile) || to_tile == ignore.unwrap_or(from_tile) {
            self.entity.swap(from_idx, to_idx);
            return Ok(());
        } else {
            return Err(GridError::Collision);
        }
    }

    /// Returns the width of the grid
    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> Grid<T>
where
    T: std::fmt::Debug
{
    /// Draws a nice map, converting elements according to a given character
    /// map. Useful when elements contain enums.
    pub fn draw_enum_map(&self, char_map: &HashMap<T, char>)
    where
        T: Copy + Eq + Hash,
    {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                let ch = match char_map.get(&self.entity[idx]) {
                    Some(&character) => character,
                    None => '?', // Placeholder
                };
                print!("{}", ch);
            }
            println!();
        }
    }

    /// Draws a nice map, converting elements according to a given character
    /// map. Useful when elements contain enums. Also includes special node
    /// character map.
    pub fn draw_enum_node_map(&self, char_map: &HashMap<T, char>, nodes: &HashMap<GridPos, char>)
    where
        T: Copy + Eq + Hash,
    {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                let mut ch = match char_map.get(&self.entity[idx]) {
                    Some(&character) => character,
                    None => '?', // Placeholder
                };
                if nodes.contains_key(&(col, row)) {
                    ch = nodes[&(col, row)];
                }
                print!("{}", ch);
            }
            println!();
        }
    }

    /// Dumps a raw copy of the map, no matter what the elements contain.
    pub fn dump_raw(&self) {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            let start_idx = row * self.width;
            let end_idx = start_idx + self.width;
            let row_slice = &self.entity[start_idx..end_idx];
            println!("{:?}", row_slice);
        }
    }
}

impl<Char> Grid<Char>
where 
    Char: std::fmt::Debug + std::fmt::Display,
{
    /// Draws a map if the grid contains chars.
    pub fn draw_map(&self) {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                print!("{}", self.entity[idx]);
            }
            println!();
        }
    }
}

impl<T> Index<i32> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[idx].
    fn index(&self, idx: i32) -> &Self::Output {
        &self.entity[idx as usize]
    }
}

impl<T> Index<isize> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[idx].
    fn index(&self, idx: isize) -> &Self::Output {
        &self.entity[idx as usize]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[idx].
    fn index(&self, idx: usize) -> &Self::Output {
        &self.entity[idx]
    }
}

impl<T> Index<Range<i32>> for Grid<T> {
    type Output = [T];

    /// Returns a slice of elements for the given range on grid[idx].
    fn index(&self, range: Range<i32>) -> &Self::Output {
        &self.entity[range.start as usize..range.end as usize]
    }
}

impl<T> Index<Range<isize>> for Grid<T> {
    type Output = [T];

    /// Returns a slice of elements for the given range on grid[idx].
    fn index(&self, range: Range<isize>) -> &Self::Output {
        &self.entity[range.start as usize..range.end as usize]
    }
}

impl<T> Index<Range<usize>> for Grid<T> {
    type Output = [T];

    /// Returns a slice of elements for the given range on grid[idx].
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.entity[range]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[(x, y)].
    fn index(&self, (col, row): (i32, i32)) -> &Self::Output {
        let idx = (self.width * row as usize) + col as usize;
        &self.entity[idx]
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[(x, y)].
    fn index(&self, (col, row): (isize, isize)) -> &Self::Output {
        let idx = (self.width * row as usize) + col as usize;
        &self.entity[idx]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[(x, y)].
    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        let idx = (self.width * row) + col;
        &self.entity[idx]
    }
}

impl<T> IndexMut<i32> for Grid<T> {
    /// Changes the element at location on grid[idx].
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        &mut self.entity[idx as usize]
    }
}

impl<T> IndexMut<isize> for Grid<T> {
    /// Changes the element at location on grid[idx].
    fn index_mut(&mut self, idx: isize) -> &mut Self::Output {
        &mut self.entity[idx as usize]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    /// Changes the element at location on grid[idx].
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.entity[idx]
    }
}

impl<T> IndexMut<Range<i32>> for Grid<T> {
    /// Changes the elements for the given range on grid[idx].
    fn index_mut(&mut self, range: Range<i32>) -> &mut Self::Output {
        &mut self.entity[range.start as usize..range.end as usize]
    }
}

impl<T> IndexMut<Range<isize>> for Grid<T> {
    /// Changes the elements for the given range on grid[idx].
    fn index_mut(&mut self, range: Range<isize>) -> &mut Self::Output {
        &mut self.entity[range.start as usize..range.end as usize]
    }
}

impl<T> IndexMut<Range<usize>> for Grid<T> {
    /// Changes the elements for the given range on grid[idx].
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.entity[range]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    /// Changes the element at location on grid[(x, y)].
    fn index_mut(&mut self, (col, row): (i32, i32)) -> &mut Self::Output {
        let idx = (self.width * row as usize) + col as usize;
        &mut self.entity[idx]
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    /// Changes the element at location on grid[(x, y)].
    fn index_mut(&mut self, (col, row): (isize, isize)) -> &mut Self::Output {
        let idx = (self.width * row as usize) + col as usize;
        &mut self.entity[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    /// Changes the element at location on grid[(x, y)].
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        let idx = (self.width * row) + col;
        &mut self.entity[idx]
    }
}