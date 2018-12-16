use core::result;
use std::option::NoneError;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use crate::day16::machine::Machine;
use crate::day16::machine::Instruction;
use itertools::Itertools;
use itertools::structs::Chunk;
use std::str::Lines;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::file::read_puzzle_input;

pub mod machine;

struct InstructionAssertion {
    before: Vec<i32>,
    instruction: Instruction,
    after: Vec<i32>
}

fn get_assertions() -> Vec<InstructionAssertion> {
    let input  = include_str!("test_data");
    let mut assertions = vec![];
    for chunk in &input.lines().into_iter().chunks(4) {
        assertions.push(chunk_to_assertion(chunk));
    }
    assertions
}


fn before_after_vec(s : &str) -> Vec<i32> {
    let (_, before_str) = s.split_at(9);
    let before_str = &before_str[0..before_str.len()-1];
    before_str.split(", ").map(|n| n.parse::<i32>().unwrap_or_else(|_| panic!("oops"))).collect_vec()
}

fn chunk_to_assertion(chunk: Chunk<Lines>) -> InstructionAssertion {
    let chunk = chunk.collect_vec();
    let before = before_after_vec(chunk[0]);
    let instruction = chunk[1].split(" ").map(|n| n.parse::<i32>().unwrap_or_else(|_| panic!("oops"))).collect_vec();
    let after = before_after_vec(chunk[2]);
    InstructionAssertion { before , instruction: Instruction::new(instruction), after }

}

fn count_matching_ops(assertion: &InstructionAssertion) -> usize {
    let mut machine = Machine::new(assertion.before.clone());
    let results = machine.exec_all(&assertion.instruction);
    println!("wtf {:#?}", results);
    let count = results.into_iter()
        .filter_map(|a| a.ok().filter(|registers| *registers == assertion.after))
        .count();
    
    println!("{} matching ", count);
    count
}

fn possibles(assertion: &InstructionAssertion) -> HashSet<i32> {
    let mut machine = Machine::new(assertion.before.clone());
    let results = machine.exec_all(&assertion.instruction);
    println!("wtf {:#?}", results);
    let possibles  = results.into_iter().enumerate()
        .filter_map(|(i, result)| {
            match result {
                Ok(r) if r == assertion.after => Some(i as i32),
                _ => None
            }
        })
        .collect();

//    println!("{:#?} matching ", counts);
    possibles
}

pub fn part1() {
    let assertions = get_assertions();
   
    let count = assertions.iter()
        .filter(|&assertion| count_matching_ops(assertion) >= 3)
        .count();
    
    println!("count = {}", count);
    
}


pub fn part2() {
    let assertions = get_assertions();

    let init : HashMap<i32, HashSet<i32>> = HashMap::new();
    let counts = assertions.iter().fold(init, |mut acc, assertion| {
        let new_possibles = possibles(assertion);
        
        acc.entry(assertion.instruction.get_id())
            .or_insert((1..16).collect::<HashSet<i32>>())
            .retain(|c| new_possibles.contains(c));
        
        acc
    });

    println!("counts {:#?}", counts);

    let input = read_puzzle_input("day16");
    let instructions = input.lines()
        .map(|line| Instruction::new(line.split(" ").map(|n| n.parse::<i32>().unwrap_or_else(|_| panic!("oops"))).collect_vec()))
        .collect_vec();
    
    let mut machine = Machine::new(vec![0,0,0,0]);
    
    for instruction in instructions {
        machine.exec(instruction);
    }
    
    println!("reg0 = {}", machine.get_reg0());
    
    
}


mod tests {
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::error::Error;
    use super::machine::*;
    
    #[test]
    fn test_part2() {
        part2();
    }

    #[test]
    fn test() {
        
        
//        Before: [3, 2, 1, 1]
//        9 2 1 2
//        After:  [3, 2, 2, 1]
        
        
        
        part1();
        let mut m = Machine::new(vec![3,2,1,1]);
        let result = m.mulr(2,1,2);
        let result = m.mulr(1,1,2);
        println!("{:#?}", &result);
        
            
    }

    #[test]
    fn testInstructions() {
        let mut m = Machine::new(vec![0,1,3,3]);
        assert_that(&m.addr(2,1,3) ).is_equal_to(Ok(vec![0,1,3,4]));

        let mut m = Machine::new(vec![0,5,3,3]);
        assert_that(&m.addi(2,2,3) ).is_equal_to(Ok(vec![0,5,3,5]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.muli(2,2,3) ).is_equal_to(Ok(vec![0,5,3,6]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.mulr(2,2,3) ).is_equal_to(Ok(vec![0,5,3,9]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.banr(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 5 & 3]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.bani(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 5 & 2]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.borr(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 5 | 3]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.bori(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 5 | 2]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.setr(1,2,0) ).is_equal_to(Ok(vec![5,5,3, 9]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.seti(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 1]));

        let mut m = Machine::new(vec![0,5,3,9]);
        assert_that(&m.gtir(1,2,3) ).is_equal_to(Ok(vec![0,5,3, 0]));
    }
 
}