use crate::utils::file::read_puzzle_input;
use crate::day01::part1::solve_part1;
use spectral::assert_that;
use crate::day01::part2::solve_part2;


#[test]
fn test1() {
    let input = "+1\n+1\n+1";
    assert_that!(&solve_part1(&input)).is_equal_to(3);
}

#[test]
fn test2() {
    let input = read_puzzle_input("day01");

    println!("answer {}", solve_part1(&input));
    assert_that!(3).is_equal_to(3);
}

#[test]
fn test3() {

    assert_that!(solve_part2(&"+1\n-2\n+3\n+1")).is_equal_to(2);
    assert_that!(solve_part2(&"+1\n-1")).is_equal_to(0);
    assert_that!(solve_part2(&"-6\n+3\n+8\n+5\n-6")).is_equal_to(5);
    assert_that!(solve_part2(&"-6\n+3\n+8\n+5\n-6")).is_equal_to(5);
    assert_that!(solve_part2(&"+7\n+7\n-2\n-7\n-4")).is_equal_to(14);

}

#[test]
fn solve_part_2() {
    let input = read_puzzle_input("day01");
    println!("{}" , solve_part2(&input));
}