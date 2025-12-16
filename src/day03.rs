use crate::prelude::*;
use std::{ops::RangeInclusive, usize};

const UP:    Point = Point { x: 0, y: 1 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN:  Point = Point { x: 0, y: -1 };
const LEFT:  Point = Point { x: -1, y: 0 };

pub struct Stage {
    start: Point,
    xr:    RangeInclusive<i32>,
    yr:    RangeInclusive<i32>,
    ort:   bool,    // true = vertical, false = horizontal
    clen:  usize,   // Cumulative length
}

impl Stage {
    fn new(start: Point, xr: RangeInclusive<i32>, yr: RangeInclusive<i32>, ort: bool, clen: usize) -> Self {
        Self { start, xr, yr, ort, clen }
    }

    fn get_length(&self, point: &Point) -> usize {
        if self.ort {
            return self.clen + (self.start.y - point.y).abs() as usize;
        } else {
            return self.clen + (self.start.x - point.x).abs() as usize;
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x.abs() + self.y.abs()).cmp(&(other.x.abs() + other.y.abs())).reverse()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Stage>, Vec<Stage>) {
    fn parse_line(line: &str) -> Vec<Stage> {
        let mut stages = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        let mut clen = 0;

        for stage in line.split(',') {
            let (dir, ort) = match stage.chars().next().unwrap() {
                'U' => (UP, true),
                'R' => (RIGHT, false),
                'D' => (DOWN, true),
                'L' => (LEFT, false),
                _   => unreachable!("Invalid direction.")
            };
            let len = stage[1..].parse::<i32>().unwrap();
            let end = Point { x: (start.x + dir.x * len), y: (start.y + dir.y * len) };
            stages.push(
                Stage::new(
                    start,
                    start.x.min(end.x)..=start.x.max(end.x),
                    start.y.min(end.y)..=start.y.max(end.y),
                    ort,
                    clen
                ));
            start = end;
            clen += len as usize;
        }

        stages
    }

    let (one, two) = input.split_once("\n").unwrap();

    (parse_line(one), parse_line(two))
}

fn test_bounds(stage1: &Stage, stage2: &Stage) -> Option<Point> {
    if (stage1.ort && !stage2.ort) || (!stage1.ort && stage2.ort) {
        if stage2.xr.contains(&stage1.start.x) && stage1.yr.contains(&stage2.start.y) {
            return Some(Point {x: stage1.start.x, y: stage2.start.y});
        } else if stage1.xr.contains(&stage2.start.x) && stage2.yr.contains(&stage1.start.y) {
            return Some(Point {x: stage2.start.x, y: stage1.start.y});
        }
    }

    None
}

// Iterate through both sets of wires checking for bounding box collisions andd
// return a list of intersections, ignoring parallel paths. This should be
// significantly faster than resorting to HashMap intersections.
#[aoc(day3, part1)]
pub fn solve_part1((left, right): &(Vec<Stage>, Vec<Stage>)) -> usize {
    let mut min_heap = BinaryHeap::new();

    for wire1 in left {
        for wire2 in right {
            if wire1.start != wire2.start {
                if let Some(point) = test_bounds(wire1, wire2) {
                    min_heap.push(point);
                }
            }
        }
    }

    let result = min_heap.pop().unwrap();


    (result.x.abs() + result.y.abs()) as usize
}

#[aoc(day3, part2)]
pub fn solve_part2((left, right): &(Vec<Stage>, Vec<Stage>)) -> usize {
    let mut min_steps = usize::MAX;

    for wire1 in left {
        for wire2 in right {
            if wire1.start != wire2.start {
                if let Some(point) = test_bounds(wire1, wire2) {
                    let path = wire1.get_length(&point) + wire2.get_length(&point);
                    if path < min_steps {
                        min_steps = path;
                    }
                }
            }
        }
    }

    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "R8,U5,L5,D3
U7,R6,D4,L4";
    const TEST2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const TEST3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 6);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 159);
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), 135);
    }

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator(TEST1)), 30);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 610);
    }

    #[test]
    fn part2_test3() {
        assert_eq!(solve_part2(&input_generator(TEST3)), 410);
    }
}