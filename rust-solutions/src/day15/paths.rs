use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::collections::VecDeque;

type Coord = (usize, usize);

pub struct Cell {
    y: usize,
    x: usize,
    dist: usize,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Cell ({},{}) [{}]", self.x, self.y, self.dist)
    }
}

impl Cell {
    pub fn new(x: usize, y: usize, dist: usize) -> Cell {
        Cell { y, x, dist }
    }

    pub fn manhatten_dist(coord1: &Coord, coord2: &Coord) -> usize {
        let x = coord1.0 as i32 - coord2.0 as i32;
        let y = coord1.1 as i32 - coord2.1 as i32;
        (x.abs() + y.abs()) as usize
    }

    pub fn starting_cell(from: &Coord) -> Cell {
        Cell { x: from.0, y: from.1, dist: 0 }
    }
}

// fun of Cave.grid to boolean grid

pub fn min_distance(grid: Vec<Vec<bool>>, from: &Coord, to: &Coord) -> Option<usize> {
    let N = grid.len();
    let M = grid[0].len();

    let mut visited = grid.clone();

    let mut q = VecDeque::new(); 
    q.push_back(Cell::starting_cell(from));
    visited[from.1][from.0] = true;
    

    while !q.is_empty() {
        let p = q.pop_front().unwrap();

//        println!("{}", p);

        // found
        if (p.x, p.y) == *to { return Some(p.dist); }

        // up 
        if p.y >= 1 && !visited[p.y - 1][p.x] {
//            println!("up");
            q.push_back(Cell::new(p.x, p.y - 1, p.dist + 1));
            visited[p.y - 1][p.x] = true;
        }

        // down 
        if p.y + 1 < N && !visited[p.y + 1][p.x] {
//            println!("down");
            q.push_back(Cell::new(p.x, p.y + 1, p.dist + 1));
            visited[p.y + 1][p.x] = true;
        }

        // left 
        if p.x >= 1 && !visited[p.y][p.x - 1] {
//            println!("left");
            q.push_back(Cell::new(p.x - 1, p.y, p.dist + 1));
            visited[p.y][p.x - 1] = true;
        }

        // right 
        if p.x + 1 < M && !visited[p.y][p.x + 1] {
//            println!("right");
            q.push_back(Cell::new(p.x + 1, p.y, p.dist + 1));
            visited[p.y][p.x + 1] = true;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<Vec<bool>> = vec![
            vec![true, false, true, false],
            vec![false, true, false, false],
            vec![true, false, false, false],
            vec![false, false, false, false]
        ];

        assert_eq!(Some(6), min_distance(input, &(3, 0), &(0, 3)));
    }
    
    #[test]
    fn test2() {
//        #######
//        #.E...#
//        #.....#
//        #...G.#
//        #######

        let input: Vec<Vec<bool>> = vec![
            vec![true, true,  true,  true,  true,  true,  true],
            vec![true, false, false, false, false, false, true],
            vec![true, false, false, false, false, false, true],
            vec![true, false, false, false, false, false, true],
            vec![true, true,  true,  true,  true,  true,  true]
        ];
        assert_eq!(Some(3), min_distance(input, &(2, 1), &(3, 3)));
    }
    
    #[test]
    fn another() {
        //#######
        //#...G.#
        //#..G.G#
        //#.#.#G#
        //#...#E#
        //#.....#
        //#######

        let input: Vec<Vec<bool>> = vec![
            vec![true, true,  true,  true,  true,  true,  true],
            vec![true, false, false, false, true, false, true],
            vec![true, false, false, true,  false, false, true],
            vec![true, false, true,  false, true, true, true],
            vec![true, false, false, false, true, false, true],
            vec![true, false, false, false, false, false, true],
            vec![true, true,  true,  true,  true,  true,  true]
        ];

        assert_eq!(None, min_distance(input, &(5, 2), &(5, 5)));


    }
}