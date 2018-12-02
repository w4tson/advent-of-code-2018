use crate::day02::part1::solve_part1;
use crate::day02::part2::solve_part2;
use spectral::assert_that;
use crate::utils::file::read_puzzle_input;



#[test]
fn test1() {
    let input = include_str!("test_data");
    
    assert_that!(&solve_part1(&input)).is_equal_to(12);
}

#[test]
fn part1() {
    let input = read_puzzle_input("day02");

    println!("{}", solve_part1(&input));
}

#[test]
fn test2() {
    let input = include_str!("test_data2");

    assert_that!(&solve_part2(&input)).is_equal_to("fgij".to_string());
}

#[test]
fn part2() {
    let input = read_puzzle_input("day02");

    solve_part2(&input);

}
