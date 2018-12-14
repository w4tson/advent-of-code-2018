use crate::day14::digits::Digits;

pub mod digits;



pub fn solve(recipe: &mut Vec<usize>, improve_after: usize) -> Vec<usize> {
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipe.len() < improve_after + 10 {
        
        
//        print_it(&recipe, elf1, elf2);
        
        
        let recipe1 = recipe[elf1];
        let recipe2 = recipe[elf2];
        
        
        let score =  recipe1 + recipe2;
        
        
//        print!(" score {}+{}={}", recipe1, recipe2, score);
        
        
        let new_recipes =  Digits::new(score).collect::<Vec<_>>();
        
        recipe.extend(new_recipes.iter());
        
//        print!("   elf1 {}   elf2 {}", elf1,elf2);
        elf1 = (1 + recipe1 + elf1) % recipe.len();
        elf2 = (1 + recipe2 + elf2) % recipe.len();
//        print!("  new elf1: 1+{} % {}={}   ", recipe1, recipe.len(), elf1);
//        println!("  new elf2: 1+{} % {}={}", recipe2, recipe.len(), elf2);
//        println!();
    }
    recipe.iter().skip(improve_after).take(10).map(|i| *i).collect()
}

pub fn print_it(recipe: &Vec<usize>, e1: usize, e2 : usize) {
    for i in 0..recipe.len() {
        if e1 == i { print!("("); print!("{}", recipe[i]);}
        if e2 == i { print!("["); print!("{}", recipe[i]);}
        if e1 != i && e2 != i { print!("{} ", recipe[i]); }
        if e1 == i { print!(")") }
        if e2 == i { print!("]") }
    }
}


mod tests {
    use spectral::*;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::error::Error;


    #[test]
    fn test() {
        println!("asdf");
        let mut input  = vec![3,7];
        let mut input2  = vec![3,7];
        let mut input3  = vec![3,7];
        let mut input4  = vec![3,7];
        assert_that(&solve(&mut input,9 )).is_equal_to(vec![5,1,5,8,9,1,6,7,7,9]);
        assert_that(&solve(&mut input2,5 )).is_equal_to(vec![0,1,2,4,5,1,5,8,9,1]);
        assert_that(&solve(&mut input3,18 )).is_equal_to(vec![9,2,5,1,0,7,1,0,8,5]);
        assert_that(&solve(&mut input4,2018 )).is_equal_to(vec![5,9,4,1,4,2,9,8,8,2]);
    }
    
    #[test]
    fn part1() {
        let mut input  = vec![3,7];

        let result = solve(&mut input,909441 );
        println!("result {:#?}", result );
    }
}

