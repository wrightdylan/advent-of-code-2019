use crate::{hashset, utils::{DynaMap, Machine, Ortho, Program, add_pos}};

// Input commands:
// north (1), south (2), west (3), and east (4)
// 
// Status codes (in output):
// 0: The repair droid hit a wall. Its position has not changed.
// 1: The repair droid has moved one step in the requested direction.
// 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
//
// Map symbols:
// Floor: .
// Wall:  #
// Goal:  O

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

fn dir_cmd(dir: (i32, i32)) -> isize {
    match dir {
        (0, -1) => 1,
        (1, 0)  => 4,
        (0, 1)  => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    }
}

// This solution will completely explore the map in order to find the most optimal path between the start and goal.
#[aoc(day15, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut vm = Machine::new(input);
    let mut current_pos = (0, 0);
    let mut dynamap = DynaMap::<char>::new_with(current_pos, '.');
    let mut last_move: isize;
    let mut goal = (0, 0);

    vm.run();

    // This requires a boundary fill algorithm
    while dynamap.has_unexplored() {
        if let Some(explore) = dynamap.get_unexplored::<Ortho>(current_pos) {
            let mut explored_count = 0;
            let old_pos = current_pos;
            for &dir in explore.iter() {
                explored_count += 1;
                last_move = dir_cmd(dir);
                vm.input_ext(&[last_move]);
                vm.resume();
                let result = vm.pop_back().unwrap();
                match result {
                    0 => dynamap.insert(add_pos(current_pos, dir), '#'),
                    1 => {
                        current_pos = add_pos(old_pos, dir);
                        dynamap.new_pos(current_pos, '.');
                        break;
                    },
                    2 => {
                        current_pos = add_pos(old_pos, dir);
                        dynamap.new_pos(current_pos, 'O');
                        goal = current_pos;
                        break;
                    },
                    _ => unreachable!(),
                };
            }
            if explore.len() == explored_count {
                dynamap.set_explored(&old_pos);
            }
        } else {
            // Move to the next closest unexplored location
            dynamap.set_explored(&current_pos);
            let mut path = dynamap.path_unexp_mh(&current_pos, '.');
            path.reverse();
            path.pop();

            while let Some(next_pos) = path.pop() {
                let next_move = dir_cmd((next_pos.0 - current_pos.0, next_pos.1 - current_pos.1));
                vm.input_ext(&[next_move]);
                vm.resume();
                current_pos = next_pos;
            }
        }
    }
    vm.halt();

    dynamap.dijkstra::<Ortho>((0, 0), goal, '.').unwrap().len() - 1
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut vm = Machine::new(input);
    let mut current_pos = (0, 0);
    let mut dynamap = DynaMap::<char>::new_with(current_pos, '.');
    let mut last_move: isize;
    let mut goal = (0, 0);

    vm.run();

    // This requires a boundary fill algorithm
    while dynamap.has_unexplored() {
        if let Some(explore) = dynamap.get_unexplored::<Ortho>(current_pos) {
            let mut explored_count = 0;
            let old_pos = current_pos;
            for &dir in explore.iter() {
                explored_count += 1;
                last_move = dir_cmd(dir);
                vm.input_ext(&[last_move]);
                vm.resume();
                let result = vm.pop_back().unwrap();
                match result {
                    0 => dynamap.insert(add_pos(current_pos, dir), '#'),
                    1 => {
                        current_pos = add_pos(old_pos, dir);
                        dynamap.new_pos(current_pos, '.');
                        break;
                    },
                    2 => {
                        current_pos = add_pos(old_pos, dir);
                        dynamap.new_pos(current_pos, 'O');
                        goal = current_pos;
                        break;
                    },
                    _ => unreachable!(),
                };
            }
            if explore.len() == explored_count {
                dynamap.set_explored(&old_pos);
            }
        } else {
            // Move to the next closest unexplored location
            dynamap.set_explored(&current_pos);
            let mut path = dynamap.path_unexp_mh(&current_pos, '.');
            path.reverse();
            path.pop();

            while let Some(next_pos) = path.pop() {
                let next_move = dir_cmd((next_pos.0 - current_pos.0, next_pos.1 - current_pos.1));
                vm.input_ext(&[next_move]);
                vm.resume();
                current_pos = next_pos;
            }
        }
    }
    vm.halt();

    let mut vacant = dynamap.list_coords_by_tile('.').len();
    let mut frontier = hashset!(goal);
    let mut layer: Vec<(i32, i32)>;
    let mut time = 0;

    while vacant > 0 {
        time += 1;
        layer = frontier.drain().collect();
        vacant -= layer.len();

        for pos in layer {
            if let Some(new_neighbors) = dynamap.get_neighbour_by_type::<Ortho>(pos, '.') {
                for n in new_neighbors {
                    dynamap.insert(n, 'O');
                    frontier.insert(n);
                }
            }
        }
    }

    time
}