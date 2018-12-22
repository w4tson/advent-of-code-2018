use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

pub mod parse;

type Result<T> = result::Result<T, Box<Error>>;
type Coord = (usize, usize);
const TREE : char = '|';
const LUMBER_YARD: char = '#';
const OPEN: char = '.';

struct LumberArea {
    map : Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl LumberArea {

    pub fn iterate(&mut self) {
        let mut new_map = self.map.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                new_map[y][x] = self.calc_acre((x,y));
            }
        }
        self.map = new_map;
    }
    
    fn calc_acre(&self, coord: Coord) -> char {
        let surrounding = self.surrounding_sqs(&coord);
        let acre = self.map[coord.1][coord.0];
        match acre {
            OPEN if self.num_of_trees(&surrounding) >= 3 => TREE,
            TREE if self.num_of_lumberyards(&surrounding) >= 3 => LUMBER_YARD,
            LUMBER_YARD if (self.num_of_lumberyards(&surrounding) == 0 || self.num_of_trees(&surrounding) == 0) => OPEN,
            _ => acre
        }
    }
    
    fn num_of_trees(&self, vec: &Vec<char>) -> usize {
        vec.iter().filter(|&ch| *ch == TREE).count()
    }

    fn num_of_lumberyards(&self, vec: &Vec<char>) -> usize {
        vec.iter().filter(|&ch| *ch == LUMBER_YARD).count()
    }

    fn surrounding_sqs(&self, coord: &Coord) -> Vec<char> {
        let mut squares = vec![];
        // up 
        if coord.1 >= 1 { squares.push(self.map[ coord.1 - 1][ coord.0]); }

        // up left 
        if coord.1 >= 1 && coord.0 >= 1 { squares.push(self.map[coord.1 - 1][coord.0-1]); }

        // left 
        if coord.0 >= 1 { squares.push(self.map[coord.1][coord.0 - 1]); }

        // up right 
        if coord.1 >= 1 && coord.0 + 1 < self.width { squares.push(self.map[coord.1 - 1][coord.0+1]); }

        // right 
        if coord.0 + 1 < self.width  { squares.push(self.map[coord.1][coord.0 + 1]); }

        // down 
        if coord.1 + 1 < self.height { squares.push(self.map[coord.1 + 1][coord.0]); }

        // down right 
        if coord.1 + 1 < self.height && coord.0 + 1< self.width { squares.push(self.map[coord.1 + 1][coord.0+1]); }

        // down left 
        if coord.1 + 1 < self.height && coord.0 >= 1 { squares.push(self.map[coord.1 + 1][coord.0-1]); }

        squares
    }
    
    pub fn resources(&self) -> (usize, usize) {
        self.map.iter()
            .map(|row| (self.num_of_trees(row), self.num_of_lumberyards(row)))
                .fold((0,0), |mut acc, (t, ly)|{
                    acc.0 += t;   
                    acc.1 += ly;
                    acc
                })
            
       
    }

    pub fn resource_value(&self) -> usize {
        let (t, ly) = self.resources();
        t * ly
    }
 }



mod tests {
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::error::Error;
    use super::parse::*;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        let input = read_puzzle_input("day18");

        let mut lumberArea : LumberArea = input.parse().unwrap();
            
        println!("{}", lumberArea);
        for i in 1..=5000 {
            lumberArea.iterate();
            let resources = lumberArea.resources();
            println!("{:4},  {:4},  {:4}  {:4}", i, resources.0, resources.1, resources.0*resources.1);
        }
        println!("resouece value = {}", lumberArea.resource_value());
    }
    
    
    #[test]
    fn test2() {
        //pattern repeats every 4918 - 7974 (56)
        println!("{}", (1000000000 -4862) % 56);
        
    }
}

