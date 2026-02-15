use crate::prelude::*;
use crate::utils::{Machine, Program};

fn print_vgrid(data: HashMap<(i32, i32), isize>) {
    // All coordinates are positive
    let mut cleaned = Vec::new();
    let mut x_max = 0;
    let mut y_max = 0;

    for (&key, &value) in data.iter() {
        if value == 1 {
            cleaned.push((key.0 as usize, key.1 as usize));
            if key.0 > x_max {
                x_max = key.0;
            }
            if key.1 > y_max {
                y_max = key.1;
            }
        }
    }

    let mut id = Grid::new_fill(x_max as usize + 1, y_max as usize + 1, ' ');
    id.place_at(&cleaned, '#');
    id.draw_map();
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

// The robot will output two numbers at a time:
// first is paint colour (0 = black, 1 = white)
// second is turn direction (0 = left, 1 = right). Turn 90deg the step forward one
#[aoc(day11, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut pos = (0, 0);
    let mut colour;
    let mut dir = Ortho::North;
    let mut vgrid = HashMap::new();
    let mut vm = Machine::new(input);
    vm.run();
    
    while vm.is_running() {
        let current_colour = *vgrid.get(&pos).unwrap_or(&0);
        vm.input_ext(&vec![current_colour]);
        vm.resume();

        // Using a Vec for output requires popping the data in reverse order.
        let next_dir = vm.pop_back().unwrap();
        colour = vm.pop_back().unwrap();

        vgrid.entry(pos).and_modify(|panel| *panel = colour).or_insert(colour);

        dir = match next_dir {
            0 => dir.turn_left(),
            1 => dir.turn_right(),
            _ => unreachable!(),
        };
        let next_pos = dir.to_dir();
        pos.0 += next_pos.0;
        pos.1 += next_pos.1;        
    }

    vgrid.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut pos = (0, 0);
    let mut colour;
    let mut dir = Ortho::North;
    let mut vgrid = HashMap::new();
    vgrid.insert((0,0), 1);
    let mut vm = Machine::new(input);
    vm.run();
    
    while vm.is_running() {
        let current_colour = *vgrid.get(&pos).unwrap_or(&0);
        vm.input_ext(&vec![current_colour]);
        vm.resume();

        colour = vm.pop_front().unwrap();
        vgrid.entry(pos).and_modify(|panel| *panel = colour).or_insert(colour);

        dir = match vm.pop_front().unwrap() {
            0 => dir.turn_left(),
            1 => dir.turn_right(),
            _ => unreachable!(),
        };
        let next_pos = dir.to_dir();
        pos.0 += next_pos.0;
        pos.1 += next_pos.1;        
    }

    print_vgrid(vgrid);

    0 // Check the printout for the answer
}