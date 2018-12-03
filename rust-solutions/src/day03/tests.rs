use crate::utils::file::read_puzzle_input;
use crate::day03::part1::solve_part1;
use spectral::assert_that;
use crate::day03::part2::solve_part2;
use crate::day03::to_grid;

#[test]
fn test1() {
    
    let input = include_str!("test_data");
    
    assert_that!(solve_part1(to_grid(&input))).is_equal_to(4);
}

#[test]
fn part1() {
    let input = read_puzzle_input("day03");

    let result  = solve_part1(to_grid(&input));
    println!("{}", result );
}

#[test]
fn test2() {
    let input = include_str!("test_data");

    let result  = solve_part2(to_grid(&input));
    println!("{}", result );
}

#[test]
fn part2() {
    let input = read_puzzle_input("day03");

    let result  = solve_part2(to_grid(&input));
    println!("{}", result );
}