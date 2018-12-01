use crate::day01::to_freq;

pub fn solve_part1(input : &str) -> i32 {
    input.lines().map(to_freq).sum()
}


