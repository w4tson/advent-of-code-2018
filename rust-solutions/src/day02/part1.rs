use std::collections::HashMap;
use crate::day02::State;
use crate::day02::Checksum;

pub fn solve_part1(input : &str) -> i32{
    let check : HashMap<char, i32>= HashMap::new();
    
    let checksum : Checksum = input
        .lines()
        .map(to_checksum)
        .fold(Checksum{ two: 0, three: 0}, |acc, next|{
            Checksum { 
                two: acc.two + next.two, 
                three: acc.three + next.three 
            }
        });
        
    checksum.two * checksum.three
}

fn to_checksum(line : &str) -> Checksum {
    line
        .chars()
        .fold(State::new(), |acc, c| acc.next(c))
        .checksum()
}