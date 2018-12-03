mod part1;
mod part2;
#[cfg(test)]
mod tests;

use regex::Regex;
use std::collections::HashMap;


#[derive(Debug)]
pub struct FabricSq {
    id: u32,
    top: u32,
    left: u32, 
    width: u32,
    height: u32
}

impl FabricSq {
    pub fn coords(&self) -> Vec<(u32, u32)> {
        let mut coords : Vec<(u32, u32)> = vec![];
        
        println!("x in {}..{}",self.left, self.left + self.width );
        println!("y in {}..{}",self.top, self.top + self.height );
        
        for x in self.left..(self.left + self.width ) {
            for y in self.top..(self.top + self.height ) {
                coords.push((x,y));
            }
        }
        
        coords
    }
}

pub fn to_fabric_sq(line : &str) -> FabricSq {
    //#123 @ 3,2: 5x4
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }

    let cap = RE.captures_iter(line).next().unwrap_or_else(|| panic!("Problem parsing {}", line));
    
    let id: u32 = cap.get(1).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let left: u32 = cap.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let top: u32 = cap.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let width: u32 = cap.get(4).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let height: u32 = cap.get(5).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    
    FabricSq { id, top, left, width, height }

}

pub type Grid = HashMap<(u32, u32), Vec<u32>>;


pub fn add(square: &FabricSq, grid : &mut Grid) {
    for coord in square.coords() {
        grid.entry(coord)
            .or_insert(vec![])
            .push(square.id);
    }
}

pub fn to_grid(input: &str) -> Grid {
    let mut grid : Grid = HashMap::new();

    input
        .lines()
        .map(to_fabric_sq)
        .for_each(|sq| {
            add(&sq, &mut grid);
        });
    
    grid
}