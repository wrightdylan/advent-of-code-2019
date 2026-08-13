use crate::utils::{Machine, Program};

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Program) -> isize {
    // Network initialisation
    let mut network: Vec<Machine> = (0..50)
        .map(|id| {
            let mut vm = Machine::new(input);
            vm.input_push(id);
            vm.run();

            vm
        })
        .collect();

    // Network orchestration loop
    loop {
        for id in 0..50 {
            while network[id].oq_len() >= 3 {
                let dest = network[id].pop_front().unwrap();
                let x = network[id].pop_front().unwrap();
                let y = network[id].pop_front().unwrap();

                if dest == 255 {
                    return y;
                } else {
                    let dest_idx = dest as usize;
                    network[dest_idx].input_push(x);
                    network[dest_idx].input_push(y);
                }
            }

            if network[id].iq_is_empty() {
                network[id].input_push(-1);
            }

            network[id].resume();
        }
    }
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Program) -> isize {
    let mut nat: Option<(isize, isize)> = None;
    let mut last_nat_y: Option<isize> = None;

    // Network initialisation
    let mut network: Vec<Machine> = (0..50)
        .map(|id| {
            let mut vm = Machine::new(input);
            vm.input_push(id);
            vm.run();

            vm
        })
        .collect();

    // Network orchestration loop
    loop {
        let mut network_activity = false;

        for id in 0..50 {
            while network[id].oq_len() >= 3 {
                let dest = network[id].pop_front().unwrap();
                let x = network[id].pop_front().unwrap();
                let y = network[id].pop_front().unwrap();

                network_activity = true;

                if dest == 255 {
                    nat = Some((x, y));
                } else {
                    let dest_idx = dest as usize;
                    network[dest_idx].input_push(x);
                    network[dest_idx].input_push(y);
                }
            }

            if network[id].iq_is_empty() {
                network[id].input_push(-1);
            } else {
                network_activity = true;
            }

            network[id].resume();
        }

        if !network_activity {
            if let Some((nat_x, nat_y)) = nat {
                if Some(nat_y) == last_nat_y {
                    return nat_y;
                }

                last_nat_y = Some(nat_y);

                // Wake up machine 0 by feeding it the NAT packet
                network[0].input_push(nat_x);
                network[0].input_push(nat_y);
            }
        }
    }
}