#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut sum = 0;

    for mass in input {
        sum += mass / 3 - 2;
    }

    sum
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut sum = 0;

    fn module_fuel(mass: usize) -> usize {
        let mut fuel = (mass / 3).saturating_sub(2);

        if fuel > 0 {
            fuel += module_fuel(fuel);
        }

        fuel
    }

    for mass in input {
        sum += module_fuel(*mass);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator("1969")), 966);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator("100756")), 50346);
    }
}