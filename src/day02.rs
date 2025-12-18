use crate::utils::Machine;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<isize>) -> usize {
    let mut vm = Machine::new(input);

    // 1202 program alarm
    vm.inject(1, 12);
    vm.inject(2, 2);

    vm.run();

    vm.read(0) as usize
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<isize>) -> usize {
    let target = 19690720;
    let mut vm = Machine::new(input);

    for noun in 0..100 {
        for verb in 0..100 {
            vm.inject(1, noun);
            vm.inject(2, verb);
            vm.run();

            if vm.read(0) == target {
                return (100 * noun + verb) as usize;
            } else {
                vm.reboot(input);
            }
        }
    }

    panic!("No solution!")
}
