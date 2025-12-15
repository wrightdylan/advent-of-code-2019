use crate::prelude::*;

/// Specific grid errors
pub enum GridError {
    OutOfBounds,
    Collision,
}

/// 1D gridness
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub entity: Vec<T>,
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

    /// Places an entity at position (x, y)
    pub fn place_at<'a, I>(&mut self, points: I, value: T)
    where
        I: IntoIterator<Item = &'a (usize, usize)>
    {
        for &(x, y) in points {
            let index = y * self.width + x;
            if index < self.entity.len() {
                self.entity[index] = value.clone();
            }
        }
    }

    /// Returns a list of points that are within the given Manhattan distance
    /// of the start point.
    pub fn in_range(&self, pos: &(usize, usize), dist: usize) -> Vec<((usize, usize), usize)> {
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
    pub fn in_range_as<U: PartialEq>(&self, pos: &(usize, usize), dist: usize, ent_type: U) -> Vec<((usize, usize), usize)>
    where
        T: PartialEq<U>,
    {
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

    pub fn is_valid(&self, pos: &(usize, usize), dir: Ortho) -> bool {
        match dir {
            Ortho::North => if pos.1 == 0 { return false },
            Ortho::East => if pos.0 == self.width - 1 { return false },
            Ortho::South => if pos.1 == self.height - 1 { return false },
            Ortho::West => if pos.0 == 0 { return false },
        }

        true
    }

    /// Creates a list of all valid neighbouring adjacent points in a cardinal
    /// and orthogonal pattern from a given position.
    pub fn neighbours_cando(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for (dy, dx) in &CANDO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;
            if new_x < self.width && new_y < self.height {
                neighbours.push((new_x, new_y));
            }
        }

        neighbours
    }

    /// Creates a list of all valid neighbouring adjacent points in a cardinal
    /// and orthogonal pattern that match a given entity type from a given position.
    pub fn neighbours_cando_as<U: PartialEq>(&self, pos: &(usize, usize), ent_type: U) -> Vec<(usize, usize)>
    where
        T: PartialEq<U>,
    {
        let mut neighbours = Vec::new();

        for (dy, dx) in &CANDO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;

            if new_x < self.width && new_y < self.height {
                let idx = new_y * self.width + new_x;
                if let Some(entity) = self.entity.get(idx) {
                    if *entity == ent_type {
                        neighbours.push((new_x, new_y));
                    }
                }
            }
        }

        neighbours
    }

    /// Counts the number of neighbouring adjacent points in a cardinal and
    /// orthogonal pattern that match a given entity type from a given position.
    pub fn neighbours_cando_count<U: PartialEq>(&self, pos: &(usize, usize), ent_type: U) -> usize
    where
        T: PartialEq<U>,
    {
        let mut neighbours = 0;

        for (dy, dx) in &CANDO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;

            if new_x < self.width && new_y < self.height {
                let idx = new_y * self.width + new_x;
                if let Some(entity) = self.entity.get(idx) {
                    if *entity == ent_type {
                        neighbours += 1;
                    }
                }
            }
        }

        neighbours
    }

    /// Creates a list of all valid neighbouring adjacent points in a cardinal
    /// and orthogonal pattern from a given position and includes the appropriate
    /// enum.
    pub fn neighbours_cando_dir(&self, pos: &(usize, usize)) -> Vec<((usize, usize), Cando)> {
        let mut neighbours = Vec::new();

        for (dy, dx) in &CANDO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;
            let en = Cando::enumerate(dx, dy);
            if new_x < self.width && new_y < self.height {
                neighbours.push(((new_x, new_y), en));
            }
        }

        neighbours
    }

    /// Creates a list of all valid neighbouring adjacent points in an orthogonal
    /// pattern from a given position.
    pub fn neighbours_ortho(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for (dy, dx) in &ORTHO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;
            if new_x < self.width && new_y < self.height {
                neighbours.push((new_x, new_y));
            }
        }

        neighbours
    }

    /// Creates a list of all valid neighbouring adjacent points in an orthogonal
    /// pattern that match a given entity type from a given position.
    pub fn neighbours_ortho_as<U: PartialEq>(&self, pos: &(usize, usize), ent_type: U) -> Vec<(usize, usize)>
    where
        T: PartialEq<U>,
    {
        let mut neighbours = Vec::new();

        for (dy, dx) in &ORTHO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;

            if new_x < self.width && new_y < self.height {
                let idx = new_y * self.width + new_x;
                if let Some(entity) = self.entity.get(idx) {
                    if *entity == ent_type {
                        neighbours.push((new_x, new_y));
                    }
                }
            }
        }

        neighbours
    }

    /// Counts the number of neighbouring adjacent points in an orthogonal
    /// pattern that match a given entity type from a given position.
    pub fn neighbours_ortho_count<U: PartialEq>(&self, pos: &(usize, usize), ent_type: U) -> usize
    where
        T: PartialEq<U>,
    {
        let mut neighbours = 0;

        for (dy, dx) in &ORTHO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;

            if new_x < self.width && new_y < self.height {
                let idx = new_y * self.width + new_x;
                if let Some(entity) = self.entity.get(idx) {
                    if *entity == ent_type {
                        neighbours += 1;
                    }
                }
            }
        }

        neighbours
    }

    /// Creates a list of all valid neighbouring adjacent points in an orthogonal
    /// pattern from a given position and includes the orthogonal enum.
    pub fn neighbours_ortho_dir(&self, pos: &(usize, usize)) -> Vec<((usize, usize), Ortho)> {
        let mut neighbours = Vec::new();

        for (dy, dx) in &ORTHO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;
            let en = Ortho::enumerate(dx, dy);
            if new_x < self.width && new_y < self.height {
                neighbours.push(((new_x, new_y), en));
            }
        }

        neighbours
    }

    /// Returns the element in the adjacent square in the given direction.
    pub fn peek(&self, from: &(usize, usize), dir: &(i32, i32)) -> Result<T, GridError> {
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

    /// Returns a list of elements in order from the start position in the direction
    /// looked at for a given distance.
    pub fn look(&self, from: &(usize, usize), dir: &(i32, i32), dist: usize) -> Vec<((usize, usize), T)> {
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

    /// Moves an entity from the start position to a direction.
    /// The 'ignore' option allows movement even if the position being moved to
    /// contains the element to be ignored.
    pub fn slide(&mut self, from: (usize, usize), dir: (i32, i32), ignore: Option<T>) -> Result<(), GridError> {    
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
}

impl<T> Grid<T>
where T: std::fmt::Debug {
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
    pub fn draw_enum_node_map(&self, char_map: &HashMap<T, char>, nodes: &HashMap<(usize, usize), char>)
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
    Char: std::fmt::Debug,
{
    /// Draws a map.
    pub fn draw_map(&self) {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                print!("{:?}", self.entity[idx]);
            }
            println!();
        }
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

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    /// Returns the element at location on grid[(x, y)].
    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        let idx = (self.width * row) + col;
        &self.entity[idx]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    /// Changes the element at location on grid[(x, y)].
    fn index_mut(&mut self, (col, row): (i32, i32)) -> &mut T {
        let idx = (self.width * row as usize) + col as usize;
        &mut self.entity[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    /// Changes the element at location on grid[(x, y)].
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut T {
        let idx = (self.width * row) + col;
        &mut self.entity[idx]
    }
}