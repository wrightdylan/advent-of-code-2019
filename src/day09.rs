use crate::utils::{Machine, Program};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut vm = Machine::new(&input);
    
    vm.input_ext(&vec![1]);
    vm.run();
    vm.read_last() as usize
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut vm = Machine::new(&input);
    
    vm.input_ext(&vec![2]);
    vm.run();
    vm.read_last() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let test_input: Program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut vm = Machine::new(&test_input);
        vm.run();
        assert_eq!(vm.dump_output(), &test_input)
    }

    #[test]
    fn part1_test2() {
        let test_input: Program = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut vm = Machine::new(&test_input);
        vm.run();
        assert_eq!(vm.dump_output(), &vec![(34915192_isize * 34915192_isize)])
    }

    #[test]
    fn part1_test3() {
        let test_input: Program = vec![104,1125899906842624,99];
        let mut vm = Machine::new(&test_input);
        vm.run();
        assert_eq!(vm.dump_output(), &vec![1125899906842624_isize])
    }
}