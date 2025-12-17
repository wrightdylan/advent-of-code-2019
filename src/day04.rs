trait Meter {
    fn inc(&mut self);
    fn fix(&mut self);
    fn test_groups(&self, password: &[u8; 6], group_size: fn(usize) -> bool) -> bool;
}

impl Meter for [u8; 6] {
    fn inc(&mut self) {
        self[5] += 1;

        for idx in (0..5).rev() {
            if self[idx + 1] == 10 {
                self[idx + 1] = 0;
                self[idx] += 1;
            } else {
                break;
            }
        }
    }

    fn fix(&mut self) {
        for idx in 0..5 {
            if self[idx] > self[idx + 1] {
                self[idx + 1] = self[idx];
            }
        }
    }

    fn test_groups(&self, password: &[u8; 6], group_size: fn(usize) -> bool) -> bool {
        let groups: Vec<usize> = password.iter().map(|&a| password.iter().filter(|&&b| b == a).count()).collect();
        
        groups.iter().any(|&group| group_size(group))
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> ([u8; 6], [u8; 6]) {
    let (left, right) = input.split_once('-').unwrap();
    let mut start = [0; 6];
    let mut end = [0; 6];

    left
        .chars()
        .zip(&mut start)
        .for_each(|(digit, idx)| {
            *idx = digit as u8 - b'0';
        });
    right
        .chars()
        .zip(&mut end)
        .for_each(|(digit, idx)| {
            *idx = digit as u8 - b'0';
        });

    (start, end)
}

// seq input1 input2 | grep -P '^(?=1*2*3*4*5*6*7*8*9*$).*(\d)\1' | wc -l ~20ms
#[aoc(day4, part1)]
pub fn solve_part1((start, end): &([u8; 6], [u8; 6])) -> usize {
    let mut sum = 0;
    let mut password = start.clone();
    password.fix();
    
    while password < *end {
        if password.windows(2).any(|pair| pair[0] > pair[1]) {
            password.fix();
        } else {
            sum += password.windows(2).any(|pair| pair[0] == pair[1]) as usize;
            // sum += password.test_groups(&password, |g| g >= 2) as usize;  // this works too, but is so much slower
            password.inc();
        }
    }

    sum
}

// seq input1 input2 | grep -P '^(?=1*2*3*4*5*6*7*8*9*$).*(\d)(?<!(?=\1)..)\1(?!\1)' | wc -l ~20ms
#[aoc(day4, part2)]
pub fn solve_part2((start, end): &([u8; 6], [u8; 6])) -> usize {
    let mut sum = 0;
    let mut password = start.clone();
    password.fix();

    while password < *end {
        if password.windows(2).any(|pair| pair[0] > pair[1]) {
            password.fix();
        } else {
            sum += password.test_groups(&password, |g| g == 2) as usize;
            password.inc();
        }
    }

    sum
}
