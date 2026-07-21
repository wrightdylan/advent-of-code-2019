use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Entrance,
    Floor,
    Wall,
    Key(char),
    Door(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    distance: usize,
    doors: u32,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    let width = input.lines().next().unwrap_or("").len();
    let height = input.lines().count();

    let entity = input.bytes()
        .filter(|&b| b != b'\n')
        .map(|b| match b {
            b'@' => Tile::Entrance,
            b'.' => Tile::Floor,
            b'#' => Tile::Wall,
            b'a'..=b'z' => Tile::Key(b as char),
            b'A'..=b'Z' => Tile::Door(b as char),
            _ => unreachable!(),
        })
        .collect();

    Grid::new(width, height, entity)
}

fn key_index(ch: char) -> usize {
    match ch {
        'a'..='z' => (ch as usize) - ('a' as usize),
        '1' | '@' => 26,
        '2' => 27,
        '3' => 28,
        '4' => 29,
        _ => unreachable!(),
    }
}

pub fn fill_matrix_row(
    start_char: char, 
    start_pos: (usize, usize), 
    grid: &Grid<Tile>, 
    matrix: &mut Grid<Option<Edge>>
) {
    let mut visited = vec![false; grid.width() * grid.height()];
    let mut queue = VecDeque::with_capacity(grid.width() * grid.height());

    let start_idx = start_pos.1 * grid.width() + start_pos.0;
    visited[start_idx] = true;
    queue.push_back((start_pos.0, start_pos.1, 0, 0u32));

    let row = key_index(start_char);

    while let Some((x, y, dist, doors_mask)) = queue.pop_front() {
        let tile = grid[(x, y)];

        if dist > 0 {
            if let Tile::Key(dest_char) = tile {
                let col = key_index(dest_char);

                if matrix[(col, row)].is_none() {
                    matrix[(col, row)] = Some(Edge {
                        distance: dist,
                        doors: doors_mask,
                    });
                }
            }
        }

        // Keep track of locked doors along the path
        let mut next_doors_mask = doors_mask;
        if let Tile::Door(ch) = tile {
            let door_bit = ch as u32 - b'A' as u32;
            next_doors_mask |= 1 << door_bit;
        }

        // Explore neighbors
        if let Some(valid_neighbors) = grid.neighbours::<Ortho>(&(x, y)) {
            for (next_pos, next_tile) in valid_neighbors {
                if let Tile::Wall = next_tile {
                    continue;
                }

                let idx = next_pos.1 * grid.width() + next_pos.0;
                if !visited[idx] {
                    visited[idx] = true;
                    queue.push_back((next_pos.0, next_pos.1, dist + 1, next_doors_mask));
                }
            }
        }
    }
}

fn find_shortest_path(distances: &Grid<Option<Edge>>, total_keys: usize) -> usize {
    let mut cache = HashMap::new();
    dfs(26, 0, distances, total_keys, &mut cache)
}

fn dfs(
    current_idx: usize,
    held_keys: u32,
    distances: &Grid<Option<Edge>>,
    total_keys: usize,
    cache: &mut HashMap<(usize, u32), usize>
) -> usize {
    if held_keys.count_ones() as usize == total_keys {
        return 0;
    }

    if let Some(&dist) = cache.get(&(current_idx, held_keys)) {
        return dist;
    }

    let mut min_distance = usize::MAX;

    for next_key_idx in 0..26 {
        if (held_keys & (1 << next_key_idx)) != 0 {
            continue;
        }

        if let Some(edge) = distances[(next_key_idx, current_idx)] {
            
            // Check if we hold all the required doors for this path
            if (edge.doors & held_keys) == edge.doors {
                
                // Actively collect the key and recurse deeper
                let next_held = held_keys | (1 << next_key_idx);
                let next_dist = dfs(next_key_idx, next_held, distances, total_keys, cache);

                if next_dist != usize::MAX {
                    min_distance = min_distance.min(edge.distance + next_dist);
                }
            }
        }
    }

    cache.insert((current_idx, held_keys), min_distance);
    min_distance
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Grid<Tile>) -> usize {
    let mut poi = HashMap::new();

    for row in 0..input.height() {
        for col in 0..input.width() {
            match input[(col, row)] {
                Tile::Entrance => { poi.insert('@', (col, row)); }
                Tile::Key(ch)  => { poi.insert(ch, (col, row)); }
                _ => {}
            }
        }
    }

    // Distance matrix
    let mut distances: Grid<Option<Edge>> = Grid::new(26, 27, vec![None; 26 * 27]);

    let total_keys = poi.len() - 1;

    // Iterate through the list of PoIs, and run a BFS to find all endpoint keys
    // (lower case). Paths do not need to stop at doors (upper case) as these are
    // attributes of each edge, and the entrance '@' is treated as either a start
    // point or a normal path.
    for (start_char, start_pos) in poi {
        fill_matrix_row(start_char, start_pos, input, &mut distances);
    }

    find_shortest_path(&distances, total_keys)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robots {
    location: [usize; 4],
    keys: u32,
}

type CacheMap = HashMap<Robots, usize>;

pub fn find_shortest_path2(distances: &Grid<Option<Edge>>, total_keys: usize) -> usize {
    let mut cache = CacheMap::new();
    
    // Seed the initial world state
    let initial_state = Robots {
        location: [26, 27, 28, 29],
        keys: 0,
    };
    
    dfs_part2(initial_state, distances, total_keys, &mut cache)
}

fn dfs_part2(
    state: Robots,
    matrix: &Grid<Option<Edge>>,
    total_keys: usize,
    cache: &mut CacheMap
) -> usize {
    if state.keys.count_ones() as usize == total_keys {
        return 0;
    }

    if let Some(&dist) = cache.get(&state) {
        return dist;
    }

    let mut min_distance = usize::MAX;

    for next_key_idx in 0..26 {
        if (state.keys & (1 << next_key_idx)) != 0 {
            continue;
        }

        for robot_id in 0..4 {
            let current_loc = state.location[robot_id];
            
            if let Some(edge) = matrix[(next_key_idx, current_loc)] {
                
                if (edge.doors & state.keys) == edge.doors {
                    
                    let mut next_state = state;
                    next_state.keys |= 1 << next_key_idx;
                    next_state.location[robot_id] = next_key_idx;

                    let next_dist = dfs_part2(next_state, matrix, total_keys, cache);

                    if next_dist != usize::MAX {
                        min_distance = min_distance.min(edge.distance + next_dist);
                    }
                }
            }
        }
    }

    cache.insert(state, min_distance);
    min_distance
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Grid<Tile>) -> usize {
    let mut grid = input.clone();
    let mut poi = HashMap::new();

    for row in 0..grid.height() {
        for col in 0..grid.width() {
            match grid[(col, row)] {
                Tile::Entrance => {
                    grid[(col, row)] = Tile::Wall;
                    grid[(col - 1, row)] = Tile::Wall;
                    grid[(col + 1, row)] = Tile::Wall;
                    grid[(col, row - 1)] = Tile::Wall;
                    grid[(col, row + 1)] = Tile::Wall;
                    poi.insert('1', (col + 1, row - 1));
                    poi.insert('2', (col + 1, row + 1));
                    poi.insert('3', (col - 1, row + 1));
                    poi.insert('4', (col - 1, row - 1));
                }
                Tile::Key(ch)  => { poi.insert(ch, (col, row)); }
                _ => {}
            }
        }
    }

    // Distance matrix
    let mut distances: Grid<Option<Edge>> = Grid::new(26, 30, vec![None; 26 * 30]);

    let total_keys = poi.len() - 4;

    for (start_char, start_pos) in poi {
        fill_matrix_row(start_char, start_pos, &grid, &mut distances);
    }

    drop(grid);

    find_shortest_path2(&distances, total_keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "#########
#b.A.@.a#
#########";

    const TEST2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const TEST3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const TEST4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const TEST5: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    const TEST6: &str = "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######";

    const TEST7: &str = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";

    const TEST8: &str = "#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";

    const TEST9: &str = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 8); // a, b
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 86); // a, b, c, d, e, f
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), 132); // b, a, c, d, f, e, g
    }

    #[test]
    fn part1_test4() {
        assert_eq!(solve_part1(&input_generator(TEST4)), 136); // a, f, b, j, g, n, h, d, l, o, e, p, c, i, k, m
    }

    #[test]
    fn part1_test5() {
        assert_eq!(solve_part1(&input_generator(TEST5)), 81); // a, c, f, i, d, g, b, e, h
    }

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator(TEST6)), 8); // a, b, c, d
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST7)), 24); // a, b, c, d
    }

    #[test]
    fn part2_test3() {
        assert_eq!(solve_part2(&input_generator(TEST8)), 32); // a, b, c, d, e, f, g, h, i, j, k, l
    }

    #[test]
    fn part2_test4() {
        assert_eq!(solve_part2(&input_generator(TEST9)), 72); // e, h, i, a, b, c, d, e, f, g, k, j, l, n, m, o
    }
}