use std::thread;
use std::time::Duration;
use aoc2018::utils::file::read_puzzle_input;
use aoc2018::day15::Cave;

fn main() {
    let input = read_puzzle_input("day15");
    let mut cave: Cave = input.parse().unwrap();

    loop {
//            println!("{:#?}", cave);
        if let Some(result) = cave.tick() {
            println!("res {}", result);
            break;
        }
        if cave.rounds() % 50 ==0 { println!("{} {} Elves {} Goblins", cave.rounds(), cave.elves(), cave.goblins()) };
    }
}
