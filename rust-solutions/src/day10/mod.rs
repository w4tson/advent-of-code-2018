use itertools::Itertools;
use crate::day10::parse::board_size;


pub mod parse;

#[derive(Debug)]
pub struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}  

impl Point {
    pub fn translate(&self, o: (i32, i32)) -> (i32,i32) {
        (self.position.0 + o.0, self.position.1 - o.1)
    }
}

fn solve(points : &Vec<Point>) {
    let (bx, by, ox, oy) = board_size(&points);
    
    println!("max {} {} {} {}", bx,by, ox,oy);
    
    let o = (6, -4);
    let bx = 22;
    let by = 16;
    
    for y in 0..by {
        for x in 0..bx {
//            print!(".");
            let is_occupied = points.iter()
                .find(|&p| p.translate(o).0 == x && p.translate(o).1 == y)
                .is_some();
            if is_occupied { print!("#") } else { print!(".") }
//            print!(".");
            if x == bx-1 { println!() }
        }
    }
}


fn print() {
    
}



mod tests {
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;
    use std::prelude::v1::Vec;
    use crate::day10::Point;
    use super::parse::*;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        let points : Vec<Point> = input.lines()
            .map(|s| s.parse().unwrap())
            .collect();
        
        println!("{:#?}", points);
        
        solve(&points);
//        solve(&vec![Point{ position: (15, 0), velocity: (0,0)},
//                    Point{ position: (5, -2), velocity: (0,0)}]);
    }
}