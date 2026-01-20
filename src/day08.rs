use crate::utils::Grid;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

pub struct Layer {
    pixels : Vec<u8>,
    counts: (usize, usize, usize),
}

impl Layer {
    fn new() -> Self {
        Self { pixels: Vec::with_capacity(SIZE), counts: (0, 0, 0) }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut layer = Layer::new();

    for (idx, ch) in input.chars().enumerate() {
        match ch {
            '0' => layer.counts.0 += 1,
            '1' => layer.counts.1 += 1,
            '2' => layer.counts.2 += 1,
            _   => unimplemented!(),
        }
        layer.pixels.push(ch as u8 -  b'0');

        if idx % SIZE == SIZE - 1 {
            layers.push(layer);
            layer = Layer::new()
        }
    }

    layers
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Layer>) -> usize {
    let mut min = usize::MAX;
    let mut result = 0;

    for layer in input {
        if layer.counts.0 < min {
            min = layer.counts.0;
            result = layer.counts.1 * layer.counts.2;
        }
    }

    result
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Layer>) -> usize {
    let mut image: Grid<Option<char>> = Grid::new_fill(WIDTH, HEIGHT, None);

    for layer in input {
        for (idx, pixel) in layer.pixels.iter().enumerate() {
            if image.entity[idx] == None {
                match pixel {
                    0 => image.entity[idx] = Some(' '),
                    1 => image.entity[idx] = Some('#'),
                    _ => continue,
                }
            }
        }
    }

    for row in 0..image.height {
        let start_idx = row * image.width;
        let end_idx = start_idx + image.width;
        let row_slice = &image.entity[start_idx..end_idx];
        for pos in row_slice {
            print!("{}", pos.unwrap())
        }
        println!("");
    }

    // Read the ASCII image generated
    0
}