#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;

aoc_lib! { year = 2019 }

mod utils;

pub mod prelude {
    pub use crate::{utils::*, hashset};
    pub use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
    pub use std::cmp::{Ordering, min, max};
    pub use std::fmt;
    pub use std::hash::Hash;
    pub use std::ops::{Index, IndexMut};
}