use crate::utils::file::read_puzzle_input;
use crate::day04::part1::solve_part1;
use crate::day04::part2::solve_part2;
use spectral::assert_that;


#[test]
fn test1() {
    let input = include_str!("test_data");
    assert_that!(solve_part1(&input)).is_equal_to(240);
}

#[test]
fn part1() {
    let input = read_puzzle_input("day04");
    let result = solve_part1(&input);
    println!("{}", result );
}









//#[test]
//fn test2() {
//    let input = include_str!("test_data");
//    assert_that!(solve_part2(&input)).is_equal_to(1);
//
//}
//
//
//#[test]
//fn part2() {
//    let input = read_puzzle_input("day04");
//    solve_part2(&input);
//}