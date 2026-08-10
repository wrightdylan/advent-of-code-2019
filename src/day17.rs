use crate::prelude::*;
use crate::utils::{Machine, Ortho, Program};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Forward,
    Left,
    Right,
}

struct Robot {
    pos: (usize, usize),
    dir: Ortho,
}

impl Robot {
    fn new(pos: (usize, usize), dir: Ortho) -> Self {
        Self { pos, dir }
    }

    fn find_next_move(&self, grid: &Grid<char>) -> Option<(Move, Ortho)> {
        let check = |dir: Ortho| grid.peek(self.pos, dir).is_ok_and(|t| t == '#');

        if check(self.dir) {
            return Some((Move::Forward, self.dir));
        }

        let left = self.dir.turn_left();
        if check(left) {
            return Some((Move::Left, left));
        }

        let right = self.dir.turn_right();
        if check(right) {
            return Some((Move::Right, right));
        }

        None
    }

    fn make_move(&mut self, step: Move, dir: Ortho) {
        self.dir = dir;
        if step == Move::Forward {
            let (dx, dy) = dir.to_dir();
            
            self.pos.0 = self.pos.0.wrapping_add_signed(dx as isize);
            self.pos.1 = self.pos.1.wrapping_add_signed(dy as isize);
        }
    }
}

// Slight performance boost to grid function.
fn to_grid(vm: &mut Machine) -> Grid<char> {
    let slice = vm.inspect_output();

    let end_pos = slice.windows(2)
        .position(|w| w[0] == 10 && w[1] == 10)
        .map(|pos| pos + 1)
        .unwrap_or(slice.len());

    let raw_data: Vec<isize> = vm.drain_output(end_pos).collect();

    let width = raw_data.iter().position(|&c| c == 10).unwrap_or(raw_data.len());
    
    let entity: Vec<char> = raw_data.into_iter()
        .filter(|&c| c != 10)
        .map(|c| c as u8 as char)
        .collect();

    let num_rows = entity.len() / width;

    Grid::new(width, num_rows, entity)
}

fn alignment_parameter(grid: &Grid<char>) -> usize {
    let mut parameter = 0;
    
    for row in 1..grid.height() - 1 {
        for col in 1..grid.width() - 1 {
            if grid[(col, row)] == '#' && grid.count_neighbours_by_type::<Ortho>(&(col, row), '#') == 4 {
                parameter += col * row;
            }
        }
    }

    parameter
}

fn compress_path(path: Vec<Move>) -> Vec<String> {
    let mut new_path = Vec::new();
    let mut count = 0;

    for step in path {
        if let Move::Forward = step {
            count += 1;
            continue;
        }

        if count > 0 {
            new_path.push(count.to_string());
            count = 0;
        }

        let turn = match step {
            Move::Left => "L",
            Move::Right => "R",
            _ => unreachable!(),
        };
        new_path.push(turn.to_string());
    }

    if count > 0 {
        new_path.push(count.to_string());
    }

    new_path
}

fn solve(path: &[String], patterns: &mut Vec<Vec<String>>, main: &mut Vec<usize>) -> bool {
    if path.is_empty() {
        return !main.is_empty() && (main.len() * 2 - 1) <= 20;
    }

    for i in 0..patterns.len() {
        if path.starts_with(&patterns[i]) {
            main.push(i);
            if solve(&path[patterns[i].len()..], patterns, main) {
                return true;
            }
            main.pop();
        }
    }

    if patterns.len() < 3 {
        for len in (4..=10).step_by(2) {
            if len > path.len() { break; }
            
            let window = &path[..len];
            
            if window.join(",").len() > 20 { break; }

            patterns.push(window.to_vec());
            main.push(patterns.len() - 1);
            
            if solve(&path[len..], patterns, main) {
                return true;
            }
            
            main.pop();
            patterns.pop();
        }
    }

    false
}

fn prepare_input(sequence: Vec<String>) -> Vec<isize> {
    let formatted = sequence.join(",") + "\n";
    formatted.bytes().map(|b| b as isize).collect()
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut vm = Machine::new(input);

    vm.run();
    let grid = to_grid(&mut vm);

    alignment_parameter(&grid)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut vm = Machine::new(input);

    // Build the grid so the robot can work out a path on its own
    vm.inject(0, 2);
    vm.run();
    let grid = to_grid(&mut vm);
    // grid.draw_map();
    // vm.read_out_as_chars();

    let mut robot = Robot::new(grid.find_first(&'^').unwrap(), Ortho::North);
    let mut path = Vec::new();    

    while let Some((step, dir)) = robot.find_next_move(&grid) {
        path.push(step);
        robot.make_move(step, dir);
    }

    let new_path = compress_path(path);
    let mut main = Vec::new();
    let mut patterns = Vec::new();

    solve(&new_path, &mut patterns, &mut main);

    let main_strings: Vec<String> = main
        .into_iter()
        .map(|i| match i {
            0 => "A".to_string(),
            1 => "B".to_string(),
            2 => "C".to_string(),
            _ => unreachable!(),
        })
        .collect();

    vm.input_ext(&prepare_input(main_strings));
    vm.resume();
    for pattern in patterns {
        vm.input_ext(&prepare_input(pattern));
        vm.resume();
    }
    vm.input_push('n' as isize);
    vm.input_push(10);
    // vm.read_out_as_chars();
    vm.clear_output();
    vm.resume();
    // vm.status();
    vm.halt();

    vm.read_last() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

    const TEST2: &str = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......";

    fn test_result() -> Vec<String> {
        let result = vec!['R','8','R','8','R','4','R','4','R','8','L','6','L','2','R','4','R','4','R','8','R','8','R','8','L','6','L','2'];
        result
            .into_iter()
            .map(|c| c.to_string())
            .collect()
    }

    #[test]
    fn part1_test1() {
        let grid = Grid::<char>::new_from_block(TEST1);
        assert_eq!(alignment_parameter(&grid), 76);
    }

    #[test]
    fn part2_test1() {
        let grid = Grid::<char>::new_from_block(TEST2);
        let mut robot = Robot::new(grid.find_first(&'^').unwrap(), Ortho::North);
        let mut path = Vec::new();

        while let Some((step, dir)) = robot.find_next_move(&grid) {
            path.push(step);
            robot.make_move(step, dir);
        }

        let new_path = compress_path(path);

        assert_eq!(new_path, test_result());
    }

    #[test]
    fn part2_test2() {
        let result = test_result();

        let rmain = vec![65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10];
        // There are actually multiple possible solutions for the path that result in the same outcome
        // let rfunc_a = vec![82, 44, 56, 44, 82, 44, 56, 10];
        // let rfunc_b = vec![82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10];
        // let rfunc_c = vec![76, 44, 54, 44, 76, 44, 50, 10];

        let mut main = Vec::new();
        let mut patterns = Vec::new();

        solve(&result, &mut patterns, &mut main);

        let main_strings: Vec<String> = main
            .into_iter()
            .map(|i| match i {
                0 => "A".to_string(),
                1 => "B".to_string(),
                2 => "C".to_string(),
                _ => unreachable!(),
            })
            .collect();
        let main_input = prepare_input(main_strings);

        assert_eq!(main_input, rmain);
    }
}