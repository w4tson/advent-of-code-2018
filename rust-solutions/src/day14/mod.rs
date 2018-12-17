use crate::day14::digits::Digits;

pub mod digits;

fn ends_with_sequence(slice: &[usize], seq: &[usize]) -> Option<usize> {
    if seq.len() > slice.len() { return None }
    match &slice[slice.len() - seq.len()..] == seq {
        true => Some(slice.len() - seq.len()),
        _ => None
    }
}

fn to_digits(s : &str) -> Vec<usize>{
    s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

pub fn solve1(recipe: &mut Vec<usize>, improve_after: usize) -> Vec<usize> {
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipe.len() < improve_after + 10 {
        let (recipe1, recipe2) = (recipe[elf1], recipe[elf2]);
        let new_recipes =  to_digits(&format!("{}", recipe1 + recipe2));
        
        recipe.extend(new_recipes.iter());
        
        elf1 = (1 + recipe1 + elf1) % recipe.len();
        elf2 = (1 + recipe2 + elf2) % recipe.len();
    }
    recipe.iter().skip(improve_after).take(10).map(|i| *i).collect()
}

pub fn solve2(recipe: &mut Vec<usize>, improve_after: &str) -> usize {
    let mut elf1 = 0;
    let mut elf2 = 1;
    let improve_after = to_digits(improve_after);
    while ends_with_sequence(recipe, &improve_after).is_none() {
        let (recipe1, recipe2) = (recipe[elf1], recipe[elf2]);
        let new_recipes =  to_digits(&format!("{}", recipe1 + recipe2));

        for r in new_recipes {
            recipe.push(r);
            if ends_with_sequence(recipe, &improve_after).is_some() {
                break;
            }
        }

        elf1 = (1 + recipe1 + elf1) % recipe.len();
        elf2 = (1 + recipe2 + elf2) % recipe.len();
        
        if recipe.len() % 1000 == 0 { println!("{}", recipe.len()); }
    }
    ends_with_sequence(recipe, &improve_after)
        .unwrap_or_else(|| panic!("no subsequence"))
    
}

pub fn print_it(recipe: &Vec<usize>, e1: usize, e2 : usize) {
    for i in 0..recipe.len() {
        if e1 == i { print!("("); print!("{}", recipe[i]);}
        if e2 == i { print!("["); print!("{}", recipe[i]);}
        if e1 != i && e2 != i { print!("{} ", recipe[i]); }
        if e1 == i { print!(")") }
        if e2 == i { print!("]") }
    }
    println!();
}


mod tests {
    use spectral::*;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::error::Error;

    #[test]
    fn test() {
        assert_that(&solve1(&mut vec![3,7],9 )).is_equal_to(vec![5,1,5,8,9,1,6,7,7,9]);
        assert_that(&solve1(&mut vec![3,7],5 )).is_equal_to(vec![0,1,2,4,5,1,5,8,9,1]);
        assert_that(&solve1(&mut vec![3,7],18 )).is_equal_to(vec![9,2,5,1,0,7,1,0,8,5]);
        assert_that(&solve1(&mut vec![3,7],2018 )).is_equal_to(vec![5,9,4,1,4,2,9,8,8,2]);
    }
    
    #[test]
    fn part1() {
        let result = solve1(&mut vec![3,7],909441 );
        println!("result {:#?}", result );
    }
    
    #[test]
    fn test2() {
        assert_that(&solve2(&mut vec![3,7],"51589" )).is_equal_to(9);
        assert_that(&solve2(&mut vec![3,7],"01245" )).is_equal_to(5);
        assert_that(&solve2(&mut vec![3,7],"92510" )).is_equal_to(18);
        assert_that(&solve2(&mut vec![3,7],"59414" )).is_equal_to(2018);
    }
    
    #[test]
    fn part2() {
        let mut input  = vec![3,7];
        let result = solve2(&mut input,"909441" );
        println!("{}", result);
    }
}

