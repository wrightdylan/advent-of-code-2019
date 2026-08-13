use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Shuffle {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

impl FromStr for Shuffle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(Shuffle::DealIntoNewStack)
        } else if let Some(suffix) = s.strip_prefix("cut ") {
            let n = suffix.parse::<i128>().map_err(|e| e.to_string())?;
            Ok(Shuffle::Cut(n))
        } else if let Some(suffix) = s.strip_prefix("deal with increment ") {
            let n = suffix.parse::<i128>().map_err(|e| e.to_string())?;
            Ok(Shuffle::DealWithIncrement(n))
        } else {
            Err(format!("Invalid instruction: {}", s))
        }
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Shuffle> {
    input
        .lines()
        .map(|line| line.parse::<Shuffle>().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Linear {
    a: i128,
    b: i128,
    m: i128,
}

impl Linear {
    // Initialise the transformation matrix for a specific deck size
    fn new(m: i128) -> Self {
        Self {
            a: 1, // Start with multiplier 1
            b: 0, // Start with offset 0
            m,
        }
    }

    // Main builder step that dispatches the instruction types
    fn apply(self, instruction: Shuffle) -> Self {
        match instruction {
            Shuffle::DealIntoNewStack => self.compose(-1, -1),
            Shuffle::Cut(n) => self.compose(1, -n),
            Shuffle::DealWithIncrement(n) => self.compose(n, 0),
        }
    }

    // Builder pattern to compose a new step onto the existing transformation
    // Deal into new stack:   self.compose(-1, -1);
    // Cut N cards:           self.compose(1, -n);
    // Deal with increment N: self.compose(n, 0);
    fn compose(self, step_a: i128, step_b: i128) -> Self {
        Self {
            a: self.rem_euclid(step_a * self.a),
            b: self.rem_euclid(step_a * self.b + step_b),
            m: self.m,
        }
    }

    // Evaluate the final position of a given card index
    fn evaluate(&self, x: i128) -> i128 {
        self.rem_euclid(self.a * x + self.b)
    }

    // Repeatedly applies this transformation 'k' times
    fn pow(self, mut k: i128) -> Self {
        let mut result = Linear::new(self.m);
        // This holds the squared base
        let mut base = self;

        while k > 0 {
            // If the lowest bit is 1, accumulate the current base into our result
            if k & 1 == 1 {
                result = result.compose(base.a, base.b);
            }
            // Square the base to prepare for the next power of 2
            base = base.compose(base.a, base.b);
            // Shift the exponent down by 1 bit
            k >>= 1;
        }

        result
    }

    // Helper to handle Rust's truncated division remainder
    fn rem_euclid(&self, val: i128) -> i128 {
        val.rem_euclid(self.m)
    }
}

// The naive solution is to use a vector, but the fact that we are tracking the
// position of one card is a clue to the actual solution. Instead we can use the
// linear formula: f(x) = ax + b (mod M), where M is the number of cards.
// The techniques can be written mathematically:
// Deal into new stack:   f(x) = -x - 1 (mod M)
// Cut N cards:           f(x) = x - N (mod M)
// Deal with increment N: f(x) = N * x (mod M)
#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<Shuffle>) -> i128 {
    let deck_size = 10007;
    let target_card = 2019;

    let pipeline = input
        .iter()
        .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

    pipeline.evaluate(target_card)
}

// Returns (gcd, x, y) such that a*x + b*y = gcd
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

// Returns the modular multiplicative inverse of A modulo M
fn mod_inverse(a: i128, m: i128) -> i128 {
    let (_, x, _) = extended_gcd(a, m);
    x.rem_euclid(m)
}

// Instead of looping through 101,741,582,076,661 times, we can use exponentiation
// by squaring. And because we're lookig for a card at a certain position, rather
// than the position of a certain card, we can isolate the position x from the composer:
// x = (Y-B)*A^(-1) (mod M)
// where A^(-1) is the modular multiplicative inverse of A.
#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<Shuffle>) -> i128 {
    let deck_size = 119315717514047;
    let iterations = 101741582076661;
    let target_position = 2020;

    let pipeline = input
        .iter()
        .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

    // Exponentiation by squaring
    let total_pipeline = pipeline.pow(iterations);

    let final_a = total_pipeline.a;
    let final_b = total_pipeline.b;

    let a_inv = mod_inverse(final_a, deck_size);

    // Solve for x: x = (target_position - final_b) * a_inv % m
    let intermediate = (target_position - final_b).rem_euclid(deck_size);
    
    (intermediate * a_inv).rem_euclid(deck_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "deal with increment 7
deal into new stack
deal into new stack";

    const TEST2: &str = "cut 6
deal with increment 7
deal into new stack";

    const TEST3: &str = "deal with increment 7
deal with increment 9
cut -2";

    const TEST4: &str = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

    #[test]
    fn part1_test1() {
        let input = input_generator(TEST1);
        let deck_size = 10;
        let pipeline = input
            .iter()
            .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

        assert_eq!(pipeline.evaluate(0), 0);
        assert_eq!(pipeline.evaluate(3), 1);
        assert_eq!(pipeline.evaluate(2), 4);
        assert_eq!(pipeline.evaluate(1), 7);
        // Result: 0 3 6 9 2 5 8 1 4 7
    }

    #[test]
    fn part1_test2() {
        let input = input_generator(TEST2);
        let deck_size = 10;
        let pipeline = input
            .iter()
            .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

        assert_eq!(pipeline.evaluate(3), 0);
        assert_eq!(pipeline.evaluate(4), 3);
        assert_eq!(pipeline.evaluate(5), 6);
        assert_eq!(pipeline.evaluate(9), 8);
        // Result: 3 0 7 4 1 8 5 2 9 6
    }

    #[test]
    fn part1_test3() {
        let input = input_generator(TEST3);
        let deck_size = 10;
        let pipeline = input
            .iter()
            .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

        assert_eq!(pipeline.evaluate(0), 2);
        assert_eq!(pipeline.evaluate(4), 4);
        assert_eq!(pipeline.evaluate(5), 7);
        assert_eq!(pipeline.evaluate(9), 9);
        // Result: 6 3 0 7 4 1 8 5 2 9
    }

    #[test]
    fn part1_test4() {
        let input = input_generator(TEST4);
        let deck_size = 10;
        let pipeline = input
            .iter()
            .fold(Linear::new(deck_size), |trans, &ins| trans.apply(ins));

        assert_eq!(pipeline.evaluate(2), 1);
        assert_eq!(pipeline.evaluate(1), 4);
        assert_eq!(pipeline.evaluate(4), 5);
        assert_eq!(pipeline.evaluate(0), 7);
        // Result: 9 2 5 8 1 4 7 0 3 6
    }
}