use itertools::Itertools;

mod foo;

fn solve_part1(input : &str) -> usize {                                    
    let mut polymer : String = input.to_string();
    let mut done = false; 
    let mut old_length : usize;

    while !done {
        old_length = polymer.len();
        polymer = remove_pairs(&polymer);

        done = old_length == polymer.len()
    }

    println!("{}", polymer);

    polymer.len()
}

pub fn solve_part2(input: &str) -> usize {
    let atoz = (0..26).map(|x| (x + 'a' as u8) as char).collect::<Vec<_>>();

    atoz.iter()
        .map(|char_to_remove| {
            let modified_input : String = input.chars()
                .filter(|c| !c.eq_ignore_ascii_case(char_to_remove))
                .collect();
            solve_part1(&modified_input)
        })
        .min()
        .unwrap_or_default()
}

fn remove_pairs(polymer : &str) -> String {
    let mut skip = false;
    let mut reduced_polymer = String::new();
    
    for (c1, c2) in [polymer, "_"].concat().chars().tuple_windows() {
        if !skip {
            if !is_reaction(c1, c2) {
                reduced_polymer.push(c1);
                skip = false;
            } else {
                skip = true;
            }
        } else {
            skip = false;
        }
    }

    reduced_polymer
}

fn is_reaction(c1: char, c2: char) -> bool {
    c1.eq_ignore_ascii_case(&c2) &&
    c1.is_lowercase() != c2.is_lowercase() &&
    c1.is_uppercase() != c2.is_uppercase()
}

mod tests {
    use crate::utils::file::read_puzzle_input;
    use spectral::assert_that;
    use super::*;

    #[test]
    fn test1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_that!(solve_part1(&input)).is_equal_to(10);
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day05");
        let result = solve_part1(&input);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let input = "dabAcCaCBAcCcaDA";
        assert_that!(solve_part2(&input)).is_equal_to(4);
    }

    #[test]
    fn part2() {
        let input = read_puzzle_input("day05");
        let result = solve_part2(&input);
        println!("{}", result);
    }
}

