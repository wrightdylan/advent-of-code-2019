use crate::utils::{Machine, Program};

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Program) -> isize {
    let mut vm = Machine::new(input);

    vm.run();
    // vm.read_out_as_chars();
    
    // Jump check: (NOT A OR NOT B OR NOT C) AND D
    let springscript = vec![
        "NOT A J", // J = !A
        "NOT B T", // T = !B
        "OR T J",  // J = !A || !B
        "NOT C T", // T = !C
        "OR T J",  // J = !A || !B || !C
        "AND D J", // J = (!A || !B || !C) && D
        "WALK",
    ];
    vm.push_ascii_inst(&springscript);
    vm.resume();
    // vm.read_out_as_chars();
    // vm.status();
    
    vm.pop_back().unwrap()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Program) -> isize {
    let mut vm = Machine::new(input);

    vm.run();
    
    // Jump check: (NOT A OR NOT B OR NOT C) AND D AND (E OR H)
    let springscript = vec![
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "NOT E T", // T = !E
        "NOT T T", // T = E
        "OR H T",  // T = E || H
        "AND T J", // J = J && (E || H)
        "RUN",
    ];
    vm.push_ascii_inst(&springscript);
    vm.resume();
    // vm.status();
    
    vm.pop_back().unwrap()
}