use std::str::FromStr;
use crate::day19::machine::Instruction;

pub mod machine;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::day19::machine::Machine;
    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        //seti 5 0 1
        
        let instructions : Vec<Instruction> = 
            input.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()
                .expect("should be good");
        let mut m = Machine::new(instructions, 0);
        m.run();
                
    }
    
    #[test]
    fn part1() {
        let input = read_puzzle_input("day19");
        //seti 5 0 1

        let instructions : Vec<Instruction> =
            input.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()
                .expect("should be good");
        let mut m = Machine::new( instructions, 3);
        m.run();
    }

    #[test]
    fn sum_all() {
        //sum of all factors of 10551282
        let result : i128 = 1+ 2+ 3+ 6+ 7+ 14+ 21+ 42+ 251221+ 502442+ 753663+ 1507326+ 1758547+ 3517094+ 5275641+ 10551282;
        eprintln!("result = {:#?}", result);

    }
}


