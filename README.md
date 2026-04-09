# :gift::christmas_tree: Advent of Code 2019 :christmas_tree::sparkles:

These are my solutions to 2019's famous [Advent of Code](https://adventofcode.com/2019/). After hearing about 2019's intcode computer, having done Day 17 from 2024, I had to give this one a look.

Solutions make use of `cargo-aoc` code helper ([here](https://github.com/gobanos/cargo-aoc)).

IntCode puzzles denoted with an asterisk.

## Solutions

All solutions linked below:
| Day | | Title | 1 :star: | 2 :star: | Solution | Rating |
|:-|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2019/day/1)  |   | The Tyranny of the Rocket Equation | 210ns  | 1.26µs | [day01.rs](./src/day01.rs) | :sunglasses: |
| [02](https://adventofcode.com/2019/day/2)  | * | 1202 Program Alarm                 | 832ns  | 513µs  | [day02.rs](./src/day02.rs) | :satisfied: |
| [03](https://adventofcode.com/2019/day/3)  |   | Crossed Wires                      | 89.0µs | 89.0µs | [day03.rs](./src/day03.rs) | :relaxed: |
| [04](https://adventofcode.com/2019/day/4)  |   | Secure Container                   | 3.92µs | 10.9µs | [day04.rs](./src/day04.rs) | :pensive: |
| [05](https://adventofcode.com/2019/day/5)  | * | Sunny with a Chance of Asteroids   | 4.07µs | 4.07µs | [day05.rs](./src/day05.rs) | :yum: |
| [06](https://adventofcode.com/2019/day/6)  |   | Universal Orbit Map                | 49.6µs | 30.5µs | [day06.rs](./src/day06.rs) | :frowning: |
| [07](https://adventofcode.com/2019/day/7)  | * | Amplification Circuit              | 86.9µs | 527µs  | [day07.rs](./src/day07.rs) | :grimacing: |
| [08](https://adventofcode.com/2019/day/8)  |   | Space Image Format                 | 280ns  | 27.9µs | [day08.rs](./src/day08.rs) | :relaxed: |
| [09](https://adventofcode.com/2019/day/9)  | * | Sensor Boost                       | 11.4µs | 3.20ms | [day09.rs](./src/day09.rs) | :confounded: |
| [10](https://adventofcode.com/2019/day/10) |   | Monitoring Station                 | 4.92ms | 5.10ms | [day10.rs](./src/day10.rs) | :scream: |
| [11](https://adventofcode.com/2019/day/11) | * | Space Police                       | 1.19ms | 160µs  | [day11.rs](./src/day11.rs) | :smiley: |
| [12](https://adventofcode.com/2019/day/12) |   | The N-Body Problem                 | 728µs  | 45.1ms | [day12.rs](./src/day12.rs) | :sweat: |
| [13](https://adventofcode.com/2019/day/13) | * | Care Package                       | 201µs  | 5.98ms | [day13.rs](./src/day13.rs) | :joy: |
| [14](https://adventofcode.com/2019/day/14) |   | Space Stoichiometry                | 45.7µs | 275µs  | [day14.rs](./src/day14.rs) | :open_mouth: |
| [15](https://adventofcode.com/2019/day/15) | * | Oxygen System                      | 2.74ms | 2.58ms | [day15.rs](./src/day15.rs) | :cowboy_hat_face: |
<!-- | [16](https://adventofcode.com/2019/day/16) |   | Flawed Frequency Transmission      |  |  | [day16.rs](./src/day16.rs) | :: | -->

## Notes
1. I thought having an array as a parameter cache in the VM would speed things up, but apparently simply returning an array is consistently marginally faster.
2. A simple typo on day 9 caused hours of searching.
3. Sometimes day 10 part 2 runs faster than part 1, even though it _is_ part 1 with extra steps. I had also expected part 2 to run much slower in general, but perhaps BTreeMap is just that efficient.
4. Day 15 - AI in intcode!