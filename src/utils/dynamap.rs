use crate::prelude::*;
use std::{ops::Add, vec};

type Pos = (i32, i32);

#[derive(Debug, Clone)]
pub struct DynaMap<T> {
    map:        HashMap<Pos, T>,
    incomplete: HashSet<Pos>, // Positions which have not been fully explored
}

impl<T: Clone + Copy + PartialEq + Eq + Hash> DynaMap<T> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), incomplete: HashSet::new() }
    }

    pub fn new_with(pos: Pos, tile: T) -> Self {
        let mut dm = Self { map: HashMap::new(), incomplete: HashSet::new() };
        dm.map.insert(pos, tile);
        dm.incomplete.insert(pos);
        dm
    }

    pub fn count(&self) -> HashMap<T, usize> {
        let mut counts = HashMap::new();

        for value in self.map.values() {
            *counts.entry(*value).or_insert(0) += 1;
        }

        counts
    }

    pub fn dijkstra<U>(&self, start: Pos, goal: Pos, valid: T) -> Option<Vec<Pos>>
    where
        T: PartialEq,
        U: DirectionProvider,
    {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut priority_queue = BinaryHeap::new();
        let mut came_from: HashMap<Pos, Pos> = HashMap::new();

        distances.insert(start, 0);
        priority_queue.push(State { cost: 0, position: start });

        while let Some(State { cost, position }) = priority_queue.pop() {
            if position == goal {
                let mut path = Vec::new();
                let mut curr = goal;
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(curr);
                    curr = prev;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            if cost > *distances.get(&position).unwrap_or(&usize::MAX) {
                continue;
            }

            for (dx, dy) in U::get_directions() {
                let next_pos = (position.0 + dx, position.1 + dy);

                if let Some(tile) = self.map.get(&next_pos) {
                    if next_pos == goal || *tile == valid {
                        let next_cost = cost + 1;
                        if next_cost < *distances.get(&next_pos).unwrap_or(&usize::MAX) {
                            distances.insert(next_pos, next_cost);
                            came_from.insert(next_pos, position);
                            priority_queue.push(State { cost: next_cost, position: next_pos });
                        }
                    }
                }
            }
        }

        None
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        self.map.get(pos)
    }

    /// Gets the X, Y extents of all mapped areas
    pub fn get_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        let mut keys = self.map.keys();
        
        let first = keys.next()?;
        let mut min_x = first.0;
        let mut max_x = first.0;
        let mut min_y = first.1;
        let mut max_y = first.1;

        for pos in keys {
            if pos.0 < min_x { min_x = pos.0; }
            if pos.0 > max_x { max_x = pos.0; }
            if pos.1 < min_y { min_y = pos.1; }
            if pos.1 > max_y { max_y = pos.1; }
        }

        Some((min_x, max_x, min_y, max_y))
    }

    /// Creates a list of all valid neighbours by type in an orthogonal, or
    /// cardinal and ordinal pattern from a given position, selected by a
    /// generic direction provider (Ortho/Cando).
    pub fn get_neighbours_by_type<U>(&self, pos: Pos, value: T) -> Option<Vec<Pos>> 
    where
        T: PartialEq,
        U: DirectionProvider,
    {
        let matching: Vec<Pos> = U::get_directions()
            .filter_map(|dir| {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                self.map.get(&new_pos).filter(|&val| *val == value).map(|_| new_pos)
            })
            .collect();

        if matching.is_empty() {
            None
        } else {
            Some(matching)
        }
    }

    /// Returns a list of unexplored coordinated from a given position based on
    /// a generic direction provider (Ortho/Cando).
    pub fn get_unexplored<U: DirectionProvider>(&self, pos: Pos) -> Option<Vec<Pos>> {
        let unexplored: Vec<Pos> = U::get_directions()
            .filter(|dir| !self.map.contains_key(&(dir.0 + pos.0, dir.1 + pos.1)))
            .collect();

        (!unexplored.is_empty()).then_some(unexplored)
    }

    /// Checks if the 'incomplete' queue is empty, and thus all tiles are explored.
    pub fn has_unexplored(&self) -> bool {
        !self.incomplete.is_empty()
    }

    pub fn insert(&mut self, key: Pos, value: T) {
        self.map.insert(key, value);
    }

    pub fn is_explored(&self, pos: Pos) -> bool {
        self.map.contains_key(&pos)
    }

    pub fn list_coords_by_tile(&self, target: T) -> Vec<Pos> {
        let mut results = Vec::new();

        for (key, val) in self.map.iter() {
            if *val == target {
                results.push(key.clone());
            }
        }

        results
    }

    pub fn list_tiles(&self) -> Vec<&T> {
        let unique: HashSet<&T> = self.map.values().collect();
        unique.into_iter().collect()
    }

    pub fn list_unexplored(&self) -> Vec<Pos> {
        self.incomplete.iter().cloned().collect()
    }

    /// Nearest unexplored by Manhattan distance
    pub fn nearest_unexp_mh(&self, pos: &Pos) -> Pos {
        let mut best_pos= (0, 0);
        let mut shortest = i32::MAX;

        for candidate in self.incomplete.clone() {
            let distance = (pos.0 - candidate.0).abs() + (pos.1 - candidate.1).abs();
            if shortest > distance {
                shortest = distance;
                best_pos = candidate;
            }
        }

        best_pos
    }

    pub fn new_pos(&mut self, key: Pos, value: T) {
        self.map.entry(key).or_insert(value);
        self.incomplete.insert(key);
    }

    /// Path to nearest unexplored by Dijkstra (seems janky)
    pub fn path_unexp_dijk<U: DirectionProvider>(&self, pos: &Pos, valid: T) -> Vec<Pos> {
        let mut best_path = Vec::new();
        let mut shortest = usize::MAX;

        for candidate in self.incomplete.clone() {
            let path = self.dijkstra::<U>(*pos, candidate, valid).unwrap();
            let length = path.len();
            if shortest > length {
                shortest = length;
                best_path = path;
            }
        }

        best_path
    }

    /// Path to nearest unexplored by Manhattan distance
    pub fn path_unexp_mh(&self, pos: &Pos, valid: T) -> Vec<Pos> {
        let goal = self.nearest_unexp_mh(pos);

        self.dijkstra::<Ortho>(*pos, goal, valid).unwrap()
    }

    pub fn set_explored(&mut self, key: &Pos) {
        self.incomplete.remove(key);
    }

    pub fn show_neighbours<U>(&self, pos: Pos)
    where 
        T: std::fmt::Debug,
        U: DirectionProvider,
    {
        U::get_directions()
            .filter_map(|dir| {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                self.map.get(&new_pos).map(|val| (new_pos, val))
            })
            .for_each(|(pos, value)| {
                println!("Pos: {:?}, Value: {:?}", pos, value);
            });
    }
}

pub fn add_pos(pos1: Pos, pos2: Pos) -> Pos {
    (pos1.0 + pos2.0, pos1.1 + pos2.1)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
