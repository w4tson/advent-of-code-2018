use std::marker::Sized;
use crate::day20::regex::Regex;
use crate::day20::regex::RegexAnd;
use crate::day20::regex::RegexChar;
use crate::day20::regex::RegexOr;

pub mod regex;

pub fn str_to_Regex(s : &str) -> Box<dyn Regex> {
    let mut r: Box<dyn Regex> = match s.chars().next().unwrap_or_else(|| panic!("Empty group")) {
        '^' => Box::new(RegexAnd::new()),
        '(' => Box::new(RegexOr::new()),
        _ => panic!("Should start with ( or ^ {}", s)
    };
    let mut skip = 0;
    
    for (i, c) in s[1..].chars().enumerate() {
        if i < skip { continue }
        match c {
            'N' | 'E' | 'W' | 'S' =>  r.append(Box::new(RegexChar::new(c))),
            '(' => {
                let regex = str_to_Regex(&s[i+1..]);
                let size = regex.str_size();
                skip = i + size;
                r.append(regex);
            },
            '|' => r.new_group(),
            ')' | '$' => { break; },
            _ => panic!("unexpected char! {}", c)
        }
    }
    
    r
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use itertools::Itertools;

    #[test]
    fn test() {
        assert_eq!(10, str_to_Regex("^ENWWW(NEEE|SSE(EE|N))$").max());
        assert_eq!(18, str_to_Regex("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").max());
        assert_eq!(23, str_to_Regex("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").max());
        assert_eq!(31, str_to_Regex("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").max());
        assert_eq!(14, str_to_Regex("^EES(SE(SSESWENWNN|)NNNWNWN)NN$").max());
        assert_eq!(3,  str_to_Regex("^EES(SE|NEWS|)$").max());


        let input = read_puzzle_input("day20");

        let regex = str_to_Regex(&input);

        eprintln!("regex = {}", regex);

        eprintln!("r = {} \nmax {}", regex, regex.max());
        
        //3573 low 
        //3574 Gold star - Off by one error! :(
    }
    
    #[test]
    fn part2() {
        let input = read_puzzle_input("day20");

        let regex = str_to_Regex(&input);
        let all_paths = regex.all_paths();
        let max = all_paths.iter().max_by_key(|path| path.len()).unwrap();
        println!("MAX = {}", max.len());
        let count = all_paths.iter()
            .filter(|&path| path.len() >= 999)
//            .inspect(|path| println!("{}", path.iter().join("")))
            .count();

        eprintln!("count = {:#?}", count);
        
        //670 too low
        //671 too low
    }
    
    #[test]
    fn foo() {
        let paths = str_to_Regex("^E(N|S)$").all_paths();
        assert_eq!(2, paths.len());
    }
    
    #[test]
    fn foo3() {
        let input = include_str!("test");
        let paths = str_to_Regex(input).all_paths();
        assert_eq!(1, paths.len());
    }

    #[test]
    fn foo2() {
        let paths = str_to_Regex("^E(NEWS|)$").all_paths();
        assert_eq!(1, paths.len());
    }
    
    #[test]
    fn test2() {
        let paths = str_to_Regex("^ENWWW(NEEE|SSE(EE|N))$").all_paths();
        assert_eq!(3, paths.len());
    }
}