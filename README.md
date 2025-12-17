# :gift::christmas_tree: Advent of Code 2019 :christmas_tree::sparkles:

These are my solutions to 2019's famous [Advent of Code](https://adventofcode.com/2019/). After hearing about 2019's intcode computer, having done Day 17 from 2024, I had to give this one a look.

Solutions make use of `cargo-aoc` code helper ([here](https://github.com/gobanos/cargo-aoc)).

## Solutions

All solutions linked below:
| Day | Title | 1 :star: | 2 :star: | Solution | Rating |
|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2019/day/1)  | The Tyranny of the Rocket Equation | 210ns  | 1.26µs | [day01.rs](./src/day01.rs) | :sunglasses: |
| [02](https://adventofcode.com/2019/day/2)  | 1202 Program Alarm                 | 832ns  | 513µs  | [day02.rs](./src/day02.rs) | :satisfied: |
| [03](https://adventofcode.com/2019/day/3)  | Crossed Wires                      | 89.0µs | 89.0µs | [day03.rs](./src/day03.rs) | :relaxed: |
| [04](https://adventofcode.com/2019/day/4)  | Secure Container                   | 3.92µs | 10.9µs | [day04.rs](./src/day04.rs) | :pensive: |

## Notes
1. I thought having an array as a parameter cache in the VM would speed things up, but apparently simply returning an array is consistently marginally faster.