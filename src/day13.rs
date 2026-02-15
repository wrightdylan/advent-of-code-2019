use crate::prelude::*;
use crate::utils::{Machine, Program};

pub struct Cabinet {
    screen: Grid<isize>,
    score: isize,
    ball: isize,
    paddle: isize,
}

impl Cabinet {
    pub fn new(vm: &Machine) -> Self {
        // [x_min, y_min, _, x_max, y_max, _]
        let min_max = vm.prescan_min_max(3);

        Self {
            screen: Grid::new_fill(min_max[3] as usize + 1, min_max[4] as usize + 1, 0_isize),
            score: 0,
            ball: 0,
            paddle: 0,
        }
    }

    pub fn count_type(&self, target: isize) -> usize {
        self.screen.count_type(&target)
    }

    pub fn show_screen(&self) {
        self.screen.dump_raw();
    }

    pub fn state(&self) {
        println!("Score: {}", self.score);
        println!("Ball: pos: {}", self.ball);
        println!("Paddle pos: {}", self.paddle);
    }

    // Output:
    // <x position, y position, tile id>
    // If x position = -1, then the tile id is the score.
    //
    // Tile reference
    // 0 is an empty tile. No game object appears in this tile.
    // 1 is a wall tile. Walls are indestructible barriers.
    // 2 is a block tile. Blocks can be broken by the ball.
    // 3 is a horizontal paddle tile. The paddle is indestructible.
    // 4 is a ball tile. The ball moves diagonally and bounces off objects.
    fn update(&mut self, vm: &mut Machine) {
        while !vm.oq_is_empty() {
            let x_pos = vm.pop_front().unwrap();
            let y_pos = vm.pop_front().unwrap();
            let tile = vm.pop_front().unwrap();
            if x_pos == -1 {
                self.score = tile;
            } else {
                self.screen[(x_pos, y_pos)] = tile;
                match tile {
                    3 => self.paddle = x_pos,
                    4 => self.ball = x_pos,
                    _ => {},
                }
            }
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<isize>) -> usize {
    let mut vm = Machine::new(input);
    vm.run();

    let mut cabinet = Cabinet::new(&vm);
    cabinet.update(&mut vm);

    cabinet.count_type(2)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<isize>) -> isize {
    let mut vm = Machine::new(input);
    vm.inject(0, 2);
    vm.run();

    let mut cabinet = Cabinet::new(&vm);
    cabinet.update(&mut vm);

    // Build game logic
    while vm.is_running() {
        match cabinet.ball.cmp(&cabinet.paddle) {
            Ordering::Less    => vm.input_ext(&[-1]),
            Ordering::Equal   => vm.input_ext(&[0]),
            Ordering::Greater => vm.input_ext(&[1]),
        }
        vm.resume();
        cabinet.update(&mut vm);
    }

    cabinet.score
}