use crate::day03::Grid;
use itertools::Itertools;


pub fn solve_part2(grid : Grid) -> u32 {

    let overlapping : Vec<&u32> = grid.values()
        .filter(|&entries| entries.len() > 1)
        .flat_map(|x|x)
        .collect();

    *grid.values()
        .flat_map(|x| x) // should use flatten but https://github.com/rust-lang/rust/issues/48919 etc
        .dedup()
        .find(|id| !overlapping.contains(&&id))
        .unwrap_or_else(|| panic!("Couldn't find the answer"))
}