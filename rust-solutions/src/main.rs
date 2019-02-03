use std::thread;
use std::time::Duration;
use aoc2018::utils::file::read_puzzle_input;
use aoc2018::day19::machine::Instruction;
use aoc2018::day19::machine::Machine;

fn main() {
    let input = read_puzzle_input("day21");

    let instructions : Vec<Instruction> =
        input.lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .expect("should be good");
    
    let mut machine = Machine::new( &instructions, 2, 0);
    machine.run();
}
