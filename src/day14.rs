use crate::{hashmap, prelude::*};

// Use signed int as some reactions may be required more than once, and this allows 'leftovers' from one reaction to be used in another.
type Products = HashMap<String, (isize, Vec<(String, isize)>)>;

fn process_reactions(reactions: &mut HashMap<String, isize>, products: &Products) {
    while let Some((precursor, quantity)) = reactions
        .iter_mut()
        .filter(|(_, required)| **required > 0)
        .find(|(reaction, _)| **reaction != "ORE".to_string()) {
            let (batch_size, precursors) = products.get(precursor).unwrap();
            let batches = *quantity / batch_size + (if *quantity % batch_size == 0 {0} else {1});
            *quantity -= batches * batch_size;

            for (intermediate, needed) in precursors {
                *reactions.entry(intermediate.to_string()).or_insert(0) += batches * needed;
            }
        }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Products {
    let mut products = HashMap::new();

    fn process_part(part: &str) -> (String, isize) {
        let (left, right) = part.split_once(' ').unwrap();

        (right.to_string(), left.parse().unwrap())
    }

    for line in input.lines() {
        let mut precursors = Vec::new();

        let (left, right) = line.trim().split_once(" => ").unwrap();
        let parts = left.split(", ").collect::<Vec<&str>>();
        for part in parts {
            precursors.push(process_part(part));
        }
        let (product, prod_qty) = process_part(right);
        products.insert(product, (prod_qty, precursors));
    }

    products
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Products) -> isize {
    let mut reactions = hashmap!("FUEL".to_string() => 1);
    process_reactions(&mut reactions, input);

    *reactions.get(&"ORE".to_string()).unwrap()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Products) -> isize {
    let mut fuel = 1;
    let mut reactions = hashmap!("FUEL".to_string() => fuel);
    process_reactions(&mut reactions, input);
    let ore_cost = reactions.get(&"ORE".to_string()).unwrap().clone(); // For 1 FUEL
    let mut adjustment = 1_000_000_000_000 / ore_cost; // A starting point since we can expect economies of scale

    loop {
        reactions.entry("FUEL".to_string()).and_modify(|x| *x += adjustment);
        process_reactions(&mut reactions, input);
        let new_cost = reactions.get(&"ORE".to_string()).unwrap().clone();
        if new_cost > 1_000_000_000_000 {
            break;
        } else {
            fuel += adjustment;
            adjustment = ((1_000_000_000_000 - new_cost) / ore_cost).max(1);
        }
    }
    
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const TEST2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const TEST3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const TEST4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const TEST5: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 31);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 165);
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), 13312);
    }

    #[test]
    fn part1_test4() {
        assert_eq!(solve_part1(&input_generator(TEST4)), 180697 );
    }

    #[test]
    fn part1_test5() {
        assert_eq!(solve_part1(&input_generator(TEST5)), 2210736);
    }
}