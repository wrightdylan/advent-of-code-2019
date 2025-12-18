use crate::utils::Machine;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<isize>) -> usize {
    let mut vm = Machine::new(input);
    vm.input_ext(&[1]);

    vm.run();

    vm.read_last()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<isize>) -> usize {
    let mut vm = Machine::new(input);
    vm.input_ext(&[5]);

    vm.run();

    vm.read_last()
}