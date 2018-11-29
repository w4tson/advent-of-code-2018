use std::string::ToString;
use crate::utils::file::read_puzzle_input;
use crate::day01::part1::solve_part1;
use spectral::assert_that;


#[test]
fn part1() {
    let input = read_puzzle_input("test");
    assert_that!(&solve_part1(&input)).is_equal_to(&"Hello, Rusty World! WAT".to_string());
}