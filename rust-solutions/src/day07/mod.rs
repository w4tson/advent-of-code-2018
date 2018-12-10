use crate::day07::graph::Graph;
use std::fmt::Error;
use std::str::FromStr;

pub mod graph;
pub mod parser;
pub mod worker;

fn solve_part1(inupt: &str) -> Result<String, Error> { 
    let graph = Graph::from_str(inupt)?;
    let result = graph.traverse();
    println!("{}", result);
    Ok(result)
}

fn solve_part2(inupt: &str, concurrency: usize, step: usize) -> Result<String, Error> {
    let graph = Graph::from_str(inupt)?;
    let result = graph.traverse_concurrent(concurrency, step);
    println!("{}", result);
    Ok(result)
}

mod tests {
    use crate::utils::file::read_puzzle_input;
    use spectral::assert_that;
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test_data");

        println!("{}", input);
        assert_that!(solve_part1(&input )).is_equal_to(Ok("CABDFE".to_string()));
    }

    #[test]
    fn test2() {
        let input = include_str!("test_data");

        println!("{}", input);
        assert_that!(solve_part2(&input, 2, 0)).is_equal_to(Ok("CABFDE".to_string()));
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day07");
        let result = solve_part1(&input);
        println!("{:#?}", result);
    }

    #[test]
    fn part2() {
        let input = read_puzzle_input("day07");
        let result = solve_part2(&input, 5, 60);
        println!("{:#?}", result);
    }
}