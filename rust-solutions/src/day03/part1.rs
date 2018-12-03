use crate::day03::Grid;


pub fn solve_part1(grid : Grid) -> u32 {
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