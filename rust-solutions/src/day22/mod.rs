use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub mod parse;

type Result<T> = result::Result<T, Box<Error>>;
type Coord = (usize, usize);


pub struct Cave {
    depth: usize,
    target: Coord,
    erosion_levels: HashMap<(usize,usize), usize>
}

impl Cave {
    
    pub fn new(depth: usize, target: Coord) -> Cave {
        Cave { depth, target, erosion_levels: HashMap::new() }
    }
    
    pub fn risk_level(&mut self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk += self.risk_at(&(x,y)); 
            }    
        }
        risk
    }
    
    fn geo_index(&mut self, loc: &Coord) -> usize {
        let target = self.target;
        match loc {
            (0,0) => 0,
            (x,y) if *x == self.target.0 && *y == self.target.1 => 0,
            (x,y) if *y == 0 => x * 16807,
            (x,y) if *x == 0 => y * 48271,
            (x,y) => self.erosion_level(&(x-1, *y)) * self.erosion_level(&(*x, y-1))
        }
    }
    
    fn erosion_level(&mut self, loc: &Coord) -> usize {
        if let Some(level) = self.erosion_levels.get(loc) {
            *level    
        } else {
            
            let level = ( self.geo_index(loc) + self.depth ) % 20183;
            self.erosion_levels.insert(*loc, level);
            level
        }
        
    }

    fn risk_at(&mut self, loc: &Coord) -> usize {
        self.erosion_level(loc) % 3 
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
        let mut cave = Cave::new( 510, (10,10));
        assert_eq!(114, cave.risk_level())
    }
    
    #[test]
    fn part1() {
        let mut cave = Cave::new( 11817, (9,751));
        eprintln!("cave.risk_level() = {:?}", cave.risk_level());
    }

}