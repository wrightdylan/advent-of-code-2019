fn as_string(signal: Vec<i32>) -> String {
    signal.iter()
        .take(8)
        .map(|d| d.to_string())
        .collect()
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.chars()
        .map(|c| c as i32 - 48)
        .collect()
}

// This takes the repeating pattern of 0, 1, 0, -1, sums all of the positive "1"
// blocks, subtracts the negative "-1" blocks, and ignores all null blocks. It
// also reuses the memory for prefix (cumulative) sum for speed.
#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<i32>) -> String {
    let mut signal = input.clone();
    let n = input.len();
    let mut prefix_sums = vec![0; n + 1];

    for _ in 0..100 {
        prefix_sums[0] = 0;
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + signal[i];
        }

        signal = (1..=n).map(|k| {
            let mut total = 0;
            let mut block_start = k - 1;

            while block_start < n {
                let end_pos = (block_start + k).min(n);
                total += prefix_sums[end_pos] - prefix_sums[block_start];
                
                block_start += 2 * k;
                if block_start >= n { break; }

                let end_neg = (block_start + k).min(n);
                total -= prefix_sums[end_neg] - prefix_sums[block_start];
                
                block_start += 2 * k;
            }
            total.abs() % 10
        }).collect();
    }

    as_string(signal)
}

// This relies on a quirk of the maths involved. Due to the expanding pattern,
// and the huge offset, most of the input will be multiplied by 0, and can
// thus be ignored. The remainder is multiplied by 1, so can simply be added.
// Performance can be improved by running a suffix sum on the input. Because
// the truncated signal starts at the offset, the answer is the first 8 digits.
#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<i32>) -> String {
    let n = input.len();
    let total_len = n * 10_000;
    let offset = input.iter()
        .take(7)
        .fold(0, |acc, &digit| acc * 10 + digit) as usize;
    let mut signal: Vec<i32> = (offset..total_len)
        .map(|i| input[i % n])
        .collect();

    for _ in 0..100 {
        let mut running_sum = 0;
        for i in (0..signal.len()).rev() {
            running_sum = (running_sum + signal[i]) % 10;
            signal[i] = running_sum;
        }
    }

    as_string(signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1(input: &str, output: &str) {
        let digits: Vec<_> = input.chars()
            .map(|c| c as i32 - 48)
            .collect();

        let result = solve_part1(&digits);
        assert_eq!(result, output);
    }

    fn test_part2(input: &str, output: &str) {
        let digits: Vec<_> = input.chars()
            .map(|c| c as i32 - 48)
            .collect();

        let result = solve_part2(&digits);
        assert_eq!(result, output);
    }

    #[test]
    fn part1_test1() {
        test_part1("80871224585914546619083218645595", "24176176");
    }

    #[test]
    fn part1_test2() {
        test_part1("19617804207202209144916044189917", "73745418");
    }

    #[test]
    fn part1_test3() {
        test_part1("69317163492948606335995924319873", "52432133");
    }

    #[test]
    fn part2_test1() {
        test_part2("03036732577212944063491565474664", "84462026");
    }

    #[test]
    fn part2_test2() {
        test_part2("02935109699940807407585447034323", "78725270");
    }

    #[test]
    fn part2_test3() {
        test_part2("03081770884921959731165446850517", "53553731");
    }
}