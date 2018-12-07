use std::collections::HashMap;
use crate::day07::parsers::Graph;
use std::fmt::Error;
use std::str::FromStr;

pub mod parsers;

fn solve_part1(inupt: &str) -> Result<String, Error> { 
    let graph = Graph::from_str(inupt)?;
    let result = graph.traverse();
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
    fn part1() {
        let input = read_puzzle_input("day07");
        let result = solve_part1(&input);
        println!("{:#?}", result);
    }
}