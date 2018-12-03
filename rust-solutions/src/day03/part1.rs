use crate::day03::to_fabric_sq;
use crate::day03::FabricSq;
use std::prelude::v1::Vec;
use std::collections::HashMap;
use crate::day03::Grid;
use crate::day03::add;


pub fn solve_part1(input : &str) -> u32 {
    let mut grid : Grid = HashMap::new();
    
    input
        .lines()
        .map(to_fabric_sq)
        .for_each(|sq| {
            add(&sq, &mut grid);
        });
    
    grid.values()
        .filter(|&entries| entries.len() > 1)
        .count() as u32
    
}




//........
//...2222.
//...2222.
//.11XX22.
//.11XX22.
//.111133.
//.111133.
//........