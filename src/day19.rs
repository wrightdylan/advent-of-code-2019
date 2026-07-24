use crate::utils::{Machine, Program};

fn deploy_drone(x: isize, y: isize, input: &Program, vm: &mut Machine) -> isize {
    vm.input_push(x);
    vm.input_push(y);
    vm.resume();
    
    let res = vm.read_last();
    vm.reboot(input);

    res
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut vm = Machine::new(input);

    let boundary = 50;
    let mut score = 0;

    vm.run();

    // for y in 0..boundary {
    //     for x in 0..boundary {
    //         vm.input_push(x);
    //         vm.input_push(y);
    //         vm.resume();
    //         score += vm.read_last();
    //         vm.reboot(input);
    //     }
    // }

    // This is my first optimisation, and makes the solution just over 7x faster
    let delimiter = 5;
    let mut start_x = 0;

    for y in 0..delimiter {
        let mut beam_active = 0;
        let mut row_started = false;

        for x in start_x..boundary {
            let res = deploy_drone(x, y, input, &mut vm);
            score += res;

            let state_changed = res ^ beam_active;
            
            if state_changed == 1 {
                beam_active ^= 1; 
                
                if !row_started {
                    start_x = x;
                    row_started = true;
                } else {
                    break;
                }
            }
        }
    }

    // This is my second optimisation, but it doesn't work for early rows, so the
    // first optimisation is used for those. This shaves off a couple hundred µs.
    let mut left_x = 0;
    let mut right_x = 0;

    for y in delimiter..boundary {
        loop {
            if deploy_drone(left_x, y, input, &mut vm) == 1 {
                break;
            }
            left_x += 1;
        }
        
        if right_x < left_x {
            right_x = left_x;
        }
        
        loop {
            if deploy_drone(right_x, y, input, &mut vm) == 0 {
                break;
            }
            right_x += 1;
        }
        
        score += right_x - left_x;
    }
    
    score as usize
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut vm = Machine::new(input);
    let mut x = 0;
    let mut y = 99;

    vm.run();

    loop {
        // Slide along the bottom beam boundary just like in part 1
        while deploy_drone(x, y, input, &mut vm) == 0 {
            x += 1;
        }

        // Check the top-right corner of the candidate 100x100 square
        if deploy_drone(x + 99, y - 99, input, &mut vm) == 1 {
            break;
        }

        y += 1;
    }
    
    (x * 10000 + y - 99) as usize
}