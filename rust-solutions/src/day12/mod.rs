use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;


type Result<T> = result::Result<T, Box<Error>>;

const TEST_INPUT : &str = "#..#.#..##......###...###";
const INPUT : &str = "##..#.#.#..##..#..##..##..#.#....#.....##.#########...#.#..#..#....#.###.###....#..........###.#.#..";

struct Plants {
    array : Vec<bool>,
    left_most : i32,
    rules : Vec<Rule>
}

fn str_to_vec(s: &str) -> Vec<bool> {
    s.chars().map(|ch| {
        match ch {
            '#' => true,
            _ => false
        }
    }).collect()
}

//...## => #
#[derive(Debug)]
struct Rule {
    rule: Vec<bool>,
    survive: bool,
}

impl Rule {
    pub fn new(s : &str) -> Rule {
        let mut split = s.split(" => ");
        let rule = str_to_vec(split.next().unwrap());
        let survive = match split.next().unwrap() {
            "#" => true,
            _ => false
        };
        Rule { rule, survive }
    }
}

impl Plants {
    pub fn new(s :&str, rules_str: &str) -> Plants {
        let array = s.chars().map(|ch| {
                match ch {
                    '#' => true,
                    _ => false
                }
            }).collect();
        
        let rules = rules_str
            .lines()
            .map(|line| Rule::new(line))
            .collect();
        

        eprintln!("rules = {:#?}", rules);
        
        Plants { array , rules, left_most: 0}
    }
    
    pub fn next(&mut self) {
        self.array.push(false);
        self.array.push(false);
        self.array.push(false);
        self.array.push(false);
        self.array.insert(0,false);
        self.array.insert(0,false);
        self.array.insert(0,false);
        self.array.insert(0,false);
        
        let mut next_gen = vec![false; (self.array.len())];
        
        for i in 0..self.array.len()-4 {
            let mut slice = &mut self.array[i..i+5];
            
            for r in &self.rules {
//                println!("slice = {:#?}   rule = {:#?}",slice,rule.rule.as_slice());
//                if (slice[0] == r.rule[0]) {
//                    println!("WTF {}{} ", slice[1], r.rule[1]);
//                }
                if slice == r.rule.as_slice() {
                    if slice[2] != r.survive {
//                        println!("flip {} {}", i+2,r.survive);
                        
                    }
                    next_gen[i+2] = r.survive;
                }
                
            }
            
        }
        
        self.left_most -=4;
        self.array = next_gen
    }
    
    pub fn first_occupied(&self) -> i32 {
        let index = self.array.iter().position(|p| *p).unwrap();
        self.left_most + index as i32
    }

    pub fn last_occupied(&self) -> i32 {
        let index = self.array.iter().rev().position(|p| *p).unwrap();
        self.array.len() as i32  - (index as i32) + self.left_most  -1
    }
    
    pub fn sum_plants(&self) -> i32 {
        self.array.iter()
            .enumerate().
            fold(0,|acc, (i, item)| {
                let mut acc = acc;
                if *item == true {
                    acc += (i as i32 + self.left_most) as i32
                }
                
                acc
            })
        
    }
    
    pub fn print_it(&self) {
//        println!("leftmost = {}", self.left_most);
        for b in &self.array {
            match b {
                true => print!("#"),
                _ => print!(".")
            }
        }
        println!();
    }
}


mod tests {
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::error::Error;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        let mut plants = Plants::new(TEST_INPUT, TEST_RULES);
        plants.print_it();
        
        for gen in 1..=20 {
            plants.next();
            plants.print_it();
        }


        eprintln!("plants.print_it() = {:#?}", plants.first_occupied());
        eprintln!("plants.print_it() = {:#?}", plants.last_occupied());
        eprintln!("plants.print_it() = {:#?}", plants.sum_plants());
        
//        plants.print_it();
//        plants.next();
//        plants.print_it();
//        assert_that!(foos[0].bar()).is_equal_to(1);
//        assert_that!(foos[1].bar()).is_equal_to(2);
    }

    #[test]
    fn part1() {
        let mut plants = Plants::new(INPUT, RULES);
//        plants.print_it();

        let mut last = 0;
        
//        for gen in 1..=50_000 {
//            plants.next();
//
//            let score = plants.sum_plants();
//            let diff = score - last;
//            eprintln!("{:4} {} {}", gen, plants.sum_plants(), diff);
//            
//            last = score;
////            plants.print_it();
//        }

        //1625 37733 23
//        eprintln!("remaining = {:#?}", remaining);

//        eprintln!("plants.print_it() = {:#?}", plants.first_occupied());
//        eprintln!("plants.print_it() = {:#?}", plants.last_occupied());
//        eprintln!("plants.print_it() = {:#?}", plants.sum_plants());
    }
    
    const TEST_RULES :&str = "...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    const RULES :&str = "..##. => .
..... => .
##..# => .
...#. => .
#.... => .
...## => #
.#.#. => .
#..#. => #
##.#. => .
#..## => .
..#.. => .
#.#.# => .
###.# => .
###.. => .
.#... => #
.##.# => .
##... => #
..### => .
####. => .
#...# => #
.#..# => #
##### => #
..#.# => #
.#.## => #
#.### => .
....# => .
.###. => .
.#### => #
.##.. => .
##.## => #
#.##. => #
#.#.. => #";
}