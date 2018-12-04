use crate::utils::file::read_puzzle_input;
use crate::day04::part1::solve_part1;
use crate::day04::part2::solve_part2;
use spectral::assert_that;
use crate::day04::to_state;


#[test]
fn test1() {
    let input = include_str!("test_data");
    assert_that!(solve_part1(to_state(&input))).is_equal_to(240);
}

#[test]
fn part1() {
    let input = read_puzzle_input("day04");
    let result = solve_part1(to_state(&input));
    println!("{}", result );
}









#[test]
fn test2() {
    let input = include_str!("test_data");
    assert_that!(solve_part2(to_state(&input))).is_equal_to(4455);

}


#[test]
fn part2() {
    let input = read_puzzle_input("day04");
    solve_part2(to_state(&input));
}