use num_integer::lcm;

#[derive(Debug, Clone, PartialEq)]
pub struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn new(line: &str) -> Self {
        let s = &line[1..line.len() - 1];
        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();

        let pos = [
            parts[0][2..].parse().unwrap(),
            parts[1][2..].parse().unwrap(),
            parts[2][2..].parse().unwrap(),
        ];

        Self { pos, vel: [0; 3] }
    }

    fn apply_v(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn energy(&self) -> isize {
        (self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs()) *
        (self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs())
    }
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            for k in 0..3 {
                if moons[i].pos[k] < moons[j].pos[k] {
                    moons[i].vel[k] += 1;
                } else if moons[i].pos[k] > moons[j].pos[k] {
                    moons[i].vel[k] -= 1;
                }
            }
        }
    }

    for moon in moons {
        moon.apply_v();
    }
}

fn axis_step(moons: &mut Vec<Moon>, axis: usize) {
    for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                if moons[i].pos[axis] < moons[j].pos[axis] {
                    moons[i].vel[axis] += 1;
                } else if moons[i].pos[axis] > moons[j].pos[axis] {
                    moons[i].vel[axis] -= 1;
                }
            }
        }

        for moon in moons {
            moon.apply_v();
        }
}


fn find_axis_period(input: &Vec<Moon>, axis: usize) -> isize {
    let initial_states = input.clone();
    let mut moons = input.clone();

    let mut period = 1;
    loop {
        axis_step(&mut moons, axis);

        if moons == initial_states {
            return period;
        }

        period += 1;
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| Moon::new(line))
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Vec<Moon>) -> isize {
    let mut moons = input.clone();

    for _ in 0..1000 {
        step(&mut moons);
    }
    
    moons.iter().map(|moon| moon.energy()).sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<Moon>) -> isize {
    let x_period = find_axis_period(input, 0);
    let y_period = find_axis_period(input, 1);
    let z_period = find_axis_period(input, 2);
    
    lcm(lcm(x_period, y_period), z_period)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const TEST2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn part1_test1() {
        let mut moons = TEST1
            .lines()
            .map(|line| Moon::new(line))
            .collect::<Vec<Moon>>();

        for _ in 0..10 {
            step(&mut moons);
        }

        let sum = moons.iter().map(|moon| moon.energy()).sum::<isize>();

        assert_eq!(sum, 179);
    }

    #[test]
    fn part1_test2() {
        let mut moons = TEST2
            .lines()
            .map(|line| Moon::new(line))
            .collect::<Vec<Moon>>();

        for _ in 0..100 {
            step(&mut moons);
        }

        let sum = moons.iter().map(|moon| moon.energy()).sum::<isize>();

        assert_eq!(sum, 1940);
    }

    #[test]
    fn part2_test1() {
        let input = TEST1
            .lines()
            .map(|line| Moon::new(line))
            .collect::<Vec<Moon>>();

        let x_period = find_axis_period(&input, 0);
        let y_period = find_axis_period(&input, 1);
        let z_period = find_axis_period(&input, 2);
        
        let period = lcm(lcm(x_period, y_period), z_period);

        assert_eq!(period, 2772);
    }

    #[test]
    fn part2_test2() {
        let input = TEST2
            .lines()
            .map(|line| Moon::new(line))
            .collect::<Vec<Moon>>();

        let x_period = find_axis_period(&input, 0);
        let y_period = find_axis_period(&input, 1);
        let z_period = find_axis_period(&input, 2);
        
        let period = lcm(lcm(x_period, y_period), z_period);

        assert_eq!(period, 4686774924);
    }
}