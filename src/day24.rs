use binarray::*;
use crate::prelude::*;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Grid<u8> {
    let width = input.lines().next().unwrap_or("").len();
    let height = input.lines().count();
    let entity = input
        .bytes()
        .filter(|&b| b != b'\n')
        .collect();

    Grid::new(width, height, entity)
}

fn evolve(grid: Grid<u8>) -> Grid<u8> {
    let size = grid.width() * grid.height();
    let mut entity = Vec::with_capacity(size);

    for (tile, count) in grid.iter_with_neighbour_counts::<Ortho>(b'#') {
        let next_tile = match tile {
            b'#' => if count == 1 { b'#' } else { b'.' },
            b'.' => if count == 1 || count == 2 { b'#' } else { b'.' },
            _ => unreachable!(),
        };
        entity.push(next_tile);
    }

    Grid::new(grid.width(), grid.height(), entity)
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Grid<u8>) -> usize {
    let mut seen = HashSet::new();
    let mut grid = input.clone();

    loop {
        let bitmask = grid.try_to_bitmask(|&tile| tile == b'#').unwrap();
        let pattern = bitmask.try_extract::<u32>().unwrap();

        if !seen.insert(pattern) {
            let score: u32 = pattern.bit_indices()
                .map(|idx| 2u32.pow(idx as u32))
                .sum();
            return score as usize;
        }

        grid = evolve(grid);
    }
}

fn search_loop(mv_a: &mut Vec<Vec<u8>>, mv_b: &mut Vec<Vec<u8>>, lvl_idx: usize, time: &usize) {
    for idx in 0..25 {
        if idx == 12 { continue; }

        let x = idx % 5;
        let y = idx / 5;
        let current_tile = mv_a[lvl_idx][idx];

        let bug_count = count_recursive_neighbors(&mv_a, lvl_idx, x, y, time);

        mv_b[lvl_idx][idx] = match current_tile {
            b'#' => if bug_count == 1 { b'#' } else { b'.' },
            b'.' => if bug_count == 1 || bug_count == 2 { b'#' } else { b'.' },
            _ => current_tile,
        };
    }
}

fn count_recursive_neighbors(mv: &Vec<Vec<u8>>, lvl_idx: usize, x: usize, y: usize, time: &usize) -> usize {
    let mut count = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Moving outward to a shallower layer (level - 1)
        if nx < 0 {
            if lvl_idx > 0 && mv[lvl_idx - 1][11] == b'#' { count += 1; }
        } else if nx > 4 {
            if lvl_idx > 0 && mv[lvl_idx - 1][13] == b'#' { count += 1; }
        } else if ny < 0 {
            if lvl_idx > 0 && mv[lvl_idx - 1][7] == b'#' { count += 1; }
        } else if ny > 4 {
            if lvl_idx > 0 && mv[lvl_idx - 1][17] == b'#' { count += 1; }
        }
        // Moving inward to a deeper layer (level + 1)
        else if nx == 2 && ny == 2 {
            if lvl_idx < time * 2 {
                match (dx, dy) {
                    (1, 0) => {
                        for i in 0..5 { if mv[lvl_idx + 1][i * 5 + 0] == b'#' { count += 1; } }
                    }
                    (-1, 0) => {
                        for i in 0..5 { if mv[lvl_idx + 1][i * 5 + 4] == b'#' { count += 1; } }
                    }
                    (0, 1) => {
                        for i in 0..5 { if mv[lvl_idx + 1][0 * 5 + i] == b'#' { count += 1; } }
                    }
                    (0, -1) => {
                        for i in 0..5 { if mv[lvl_idx + 1][4 * 5 + i] == b'#' { count += 1; } }
                    }
                    _ => unreachable!(),
                }
            }
        }
        
        else {
            let target_idx = (ny * 5 + nx) as usize;
            if mv[lvl_idx][target_idx] == b'#' {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Grid<u8>) -> usize {
    // 401 levels total. Level 0 is located exactly at index 200.
    // Use a 'tick-tock' strategy, alternating between multiverses.
    let mut multiverse_a = vec![vec![b'.'; 25]; 401];
    let mut multiverse_b = vec![vec![b'.'; 25]; 401];
    for lvl in 0..401 {
        multiverse_a[lvl][12] = b'?';
        multiverse_b[lvl][12] = b'?';
    }
    let time = 200;

    for (idx, &tile) in input.entity().to_vec().iter().enumerate() {
        if idx != 12 {
            multiverse_a[200][idx] = tile;
        }
    }

    for minute in 1..=time {
        let min_lvl_idx = time - minute;
        let max_lvl_idx = time + minute;

        for lvl_idx in min_lvl_idx..=max_lvl_idx {
            multiverse_b[lvl_idx] = vec![b'.'; 25];
            multiverse_b[lvl_idx][12] = b'?';

            search_loop(&mut multiverse_a, &mut multiverse_b, lvl_idx, &time);
        }

        std::mem::swap(&mut multiverse_a, &mut multiverse_b);
    }

    multiverse_a.iter()
        .map(|level| level.iter().filter(|&&tile| tile == b'#').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "....#
#..#.
#..##
..#..
#....";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 2129920);
    }

    #[test]
    fn part2_test1() {
        let input = input_generator(TEST1);
        let mut multiverse_a = vec![vec![b'.'; 25]; 21];
        let mut multiverse_b = vec![vec![b'.'; 25]; 21];
        for lvl in 0..21 {
            multiverse_a[lvl][12] = b'?';
            multiverse_b[lvl][12] = b'?';
        }
        let time = 10;

        for (idx, &tile) in input.entity().to_vec().iter().enumerate() {
        if idx != 12 {
            multiverse_a[10][idx] = tile;
        }
    }

        for minute in 1..=time {
            let min_lvl_idx = time - minute;
            let max_lvl_idx = time + minute;

            for lvl_idx in min_lvl_idx..=max_lvl_idx {
                multiverse_b[lvl_idx] = vec![b'.'; 25];
                multiverse_b[lvl_idx][12] = b'?';

                search_loop(&mut multiverse_a, &mut multiverse_b, lvl_idx, &time);
            }

            std::mem::swap(&mut multiverse_a, &mut multiverse_b);
        }

        let bugs: usize = multiverse_a.iter()
            .map(|level| level.iter().filter(|&&tile| tile == b'#').count())
            .sum();

        assert_eq!(bugs, 99);
    }
}