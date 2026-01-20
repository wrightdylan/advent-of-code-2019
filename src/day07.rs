use crate::utils::Machine;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}

fn amp(prog: &Vec<isize>, phase: &isize, input: isize) -> isize {
    let mut vm = Machine::new(prog);
    vm.input_ext(&[*phase, input.clone()]);

    vm.run();
    
    vm.read_last()
}

fn amp_chain(prog: &Vec<isize>, seq: Vec<&isize>) -> isize {
    let out_a = amp(prog, seq[0], 0);
    let out_b = amp(prog, seq[1], out_a);
    let out_c = amp(prog, seq[2], out_b);
    let out_d = amp(prog, seq[3], out_c);
    amp(prog, seq[4], out_d)
}

fn amp_loop(prog: &Vec<isize>, seq: Vec<&isize>) -> isize {
    let mut vm_a = Machine::new(prog);
    let mut vm_b = Machine::new(prog);
    let mut vm_c = Machine::new(prog);
    let mut vm_d = Machine::new(prog);
    let mut vm_e = Machine::new(prog);

    vm_a.input_ext(&[*seq[0], 0]);
    vm_b.input_ext(&[*seq[1]]);
    vm_c.input_ext(&[*seq[2]]);
    vm_d.input_ext(&[*seq[3]]);
    vm_e.input_ext(&[*seq[4]]);

    while vm_e.is_running() {
        vm_a.input_from(&mut vm_e);
        vm_b.input_from(&mut vm_a);
        vm_c.input_from(&mut vm_b);
        vm_d.input_from(&mut vm_c);
        vm_e.input_from(&mut vm_d);

        vm_a.resume();
        vm_b.resume();
        vm_c.resume();
        vm_d.resume();
        vm_e.resume();
    }

    vm_e.read_last()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<isize>) -> usize {
    let mut max = 0;

    let combo: Vec<isize> = (0..=4).collect();

    for phases in combo.iter().permutations(combo.len()) {
        let output = amp_chain(input, phases);
        if output > max {
            max = output;
        }
    }

    max as usize
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<isize>) -> usize {
    let mut max = 0;

    let combo: Vec<isize> = (5..=9).collect();

    for phases in combo.iter().permutations(combo.len()) {
        let output = amp_loop(input, phases);
        if output > max {
            max = output;
        }
    }

    max as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let test_input: Vec<isize> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let phase_seq: Vec<&isize> = vec![&4,&3,&2,&1,&0];
        assert_eq!(amp_chain(&test_input, phase_seq), 43210);
    }

    #[test]
    fn part1_test2() {
        let test_input: Vec<isize> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let phase_seq: Vec<&isize> = vec![&0,&1,&2,&3,&4];
        assert_eq!(amp_chain(&test_input, phase_seq), 54321);
    }

    #[test]
    fn part1_test3() {
        let test_input: Vec<isize> = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let phase_seq: Vec<&isize> = vec![&1,&0,&4,&3,&2];
        assert_eq!(amp_chain(&test_input, phase_seq), 65210);
    }

    #[test]
    fn part2_test1() {
        let test_input: Vec<isize> = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let phase_seq: Vec<&isize> = vec![&9,&8,&7,&6,&5];
        assert_eq!(amp_loop(&test_input, phase_seq), 139629729);
    }

    #[test]
    fn part2_test2() {
        let test_input: Vec<isize> = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let phase_seq: Vec<&isize> = vec![&9,&7,&8,&5,&6];
        assert_eq!(amp_loop(&test_input, phase_seq), 18216);
    }
}