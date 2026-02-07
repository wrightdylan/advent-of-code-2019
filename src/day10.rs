use crate::prelude::*;
use num_integer::gcd; // Uses Stein's algorithm rather than the slower Euclidean algorithm
use std::{collections::BTreeMap, f64::consts::PI};


#[derive(Debug)]
pub struct Asteroid {
    x: isize,
    y: isize,
}

impl Asteroid {
    // Reduced distance showing directionality.
    pub fn redist(&self, other: &Asteroid) -> (isize, isize) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let gcd = gcd(dx, dy);

        (dx/gcd, dy/gcd)
    }

    // Calculate angle and Manhattan distance from asteroid
    pub fn calc_pos(&self, other: &Asteroid) -> (f64, isize) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        // Angle of the ray in positive radians, with 0 being 'up'. Since the coordinate system is 'upside down', dy must be negative to correct for this.
        let angle = ((dx as f64).atan2(-dy as f64) + 2.0 * PI) % (2.0 * PI);

        // We can save some cycles by using Manhattan distance for a given angle rather than calculating with trig
        let dist = dx.abs() + dy.abs();

        (angle, dist)
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Asteroid {}

fn round_robin(mut targets: BTreeMap<u64, Vec<(isize, &Asteroid)>>, num: usize) -> (isize, isize) {
    let mut result = &Asteroid { x: 0, y: 0 };
    let mut item = 0;
    let mut keys_to_remove = Vec::new();

    'outer: while !targets.is_empty() {
        let mut keys = Vec::new();

        for (angle, asteroids) in targets.iter_mut() {
            if let Some(target) = asteroids.pop() {
                result = target.1;
                if item == num {
                    break 'outer;
                } else {
                    item += 1;
                }
            }

            if asteroids.is_empty() {
                keys.push(*angle);
            }
        }

        for key in keys {
            keys_to_remove.push(key);
        }

        for key in keys_to_remove.clone() {
            targets.remove(&key);
        }
    }

    (result.x, result.y)
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Asteroid> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| line
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(col, _)| Asteroid { x: col as isize, y: row as isize }))
        .collect()
}

fn find_best(input: &Vec<Asteroid>) -> (&Asteroid, usize) {
    let mut best = None;
    let mut highest = 0;

    for station in input {
        let mut visited = HashSet::new();
        for asteroid in input {
            if station != asteroid {
                visited.insert(station.redist(asteroid));
            }
        }

        if visited.len() > highest {
            highest = visited.len();
            best = Some(station);
        }
    }

    (best.unwrap(), highest)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Asteroid>) -> usize {
    let (_, highest) = find_best(input);

    highest
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Asteroid>) -> isize {
    let (station, _) = find_best(input);
    let mut targets = BTreeMap::<u64, Vec<(isize, &Asteroid)>>::new();

    for asteroid in input {
        if station != asteroid {
            let (angle, dist) = station.calc_pos(asteroid);
            targets.entry(angle.to_bits()).or_default().push((dist, asteroid));
        }
    }

    for distances in targets.values_mut() {
        distances.sort_by(|a, b| b.0.cmp(&a.0));
    }


    let res = round_robin(targets, 199);
    
    res.0 * 100 + res.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = ".#..#
.....
#####
....#
...##";

    const TEST2: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    const TEST3: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

    const TEST4: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

    const TEST5: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 8); // (3,4)
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 33); // (5,8)
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), 35); // (1,2)
    }

    #[test]
    fn part1_test4() {
        assert_eq!(solve_part1(&input_generator(TEST4)), 41); // (6,3)
    }

    #[test]
    fn part1_test5() {
        assert_eq!(solve_part1(&input_generator(TEST5)), 210); // (11,13)
    }

    #[test]
    fn part2_test5() {
        assert_eq!(solve_part2(&input_generator(TEST5)), 802); // (11,13)
    }
}