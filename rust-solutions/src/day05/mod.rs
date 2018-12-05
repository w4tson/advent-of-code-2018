use itertools::Itertools;

mod foo;

fn solve_part1(input : &str) -> u32 {
    let removed = remove_pairs(input);
    println!("{}", removed);
    let removed2 = remove_pairs(&removed);
    println!("{}", removed2);
    
    1
}

struct State {
    s :String,
    prev_removed: bool
}

fn remove_pairs(polymer : &str) -> String {
    
    let c = 'a';
    //c.is_lowercase()
//    let state : String = "".t
    let mut s = polymer
        .chars()
        .tuple_windows()
        .fold(State { s: String::new(), prev_removed: false}, |iacc, (c1, c2)| {
            let mut acc = iacc;
            println!("({},{})", c1, c2);
            if !acc.prev_removed {
                if !c1.eq_ignore_ascii_case(&c2) || (
                    c1.is_lowercase() == c2.is_lowercase() ||
                    c1.is_uppercase() == c2.is_uppercase()) {
                    acc.s.push(c1);
                    acc.prev_removed = false;
                } else {
                    acc.prev_removed = true;
                }
            } else {
                acc.prev_removed = false;
            }
            acc

        });
    
    // dabAcCaCBAcCcaDA
    // dabAc aCBA  caDA
    
    s.s
    
}










mod tests {
    use crate::utils::file::read_puzzle_input;
    use spectral::assert_that;
    use super::*;

    #[test]
    fn test1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_that!(solve_part1(&input)).is_equal_to(1);
        
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day04");
        assert_that!(solve_part1(&input)).is_equal_to(240);
    }
}

