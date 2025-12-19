use crate::prelude::*;

type Orbits = HashMap<String, Vec<String>>;
type Parents = HashMap<String, String>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (Orbits, Parents) {
    let mut orbits: Orbits = HashMap::new();
    let mut parents: Parents = HashMap::new();
    

    for line in input.lines() {
        let (left, right) = line.split_once(')').unwrap();
        orbits.entry(left.to_string()).or_default().push(right.to_string());
        parents.entry(right.to_string()).or_insert(left.to_string());
    }
    
    (orbits, parents)
}

#[aoc(day6, part1)]
pub fn solve_part1((orbits, _): &(Orbits, Parents)) -> usize {
    let mut sum = 0;
    let mut queue = VecDeque::from([("COM".to_string(), 0)]);

    while let Some((node, depth)) = queue.pop_front() {
        sum += depth;

        if let Some(children) = orbits.get(&node) {
            for child in children {
                queue.push_back((child.to_string(), depth + 1));
            }
        }
    }


    sum
}

#[aoc(day6, part2)]
pub fn solve_part2((_, parents): &(Orbits, Parents)) -> usize {
    let mut visited = HashSet::new();
    let mut you_anc = "YOU";
    let mut san_anc = "SAN";
    let mut you_lineage = Vec::new();
    let mut san_lineage = Vec::new();
    let mut _lca = "";

    loop {
        you_anc = parents.get(you_anc).unwrap();
        you_lineage.push(you_anc);
        if visited.contains(you_anc) {
            _lca = you_anc;
            break;
        } else {
            visited.insert(you_anc);
        }
        san_anc = parents.get(san_anc).unwrap();
        san_lineage.push(san_anc);
        if visited.contains(san_anc) {
            _lca = san_anc;
            break;
        } else {
            visited.insert(san_anc);
        }
    }

    you_lineage.iter().position(|&pos| pos == _lca).unwrap() +
    san_lineage.iter().position(|&pos| pos == _lca).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

const TEST2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 42);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 4);
    }
}