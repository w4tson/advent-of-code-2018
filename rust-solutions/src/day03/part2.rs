use crate::day03::Grid;
use std::collections::HashMap;
use crate::day03::to_fabric_sq;
use crate::day03::add;

pub fn solve_part2(input : &str) -> u32 {
    let mut grid : Grid = HashMap::new();

    input
        .lines()
        .map(to_fabric_sq)
        .for_each(|sq| {
            add(&sq, &mut grid);
        });

    let overlapping : Vec<&u32> = grid.values()
        .filter(|&entries| entries.len() > 1)
        .flatten()
        .collect();

    input
        .lines()
        .map(to_fabric_sq)
        .find(|sq| !overlapping.contains(&&sq.id))
        .unwrap()
        .id
        
    
}