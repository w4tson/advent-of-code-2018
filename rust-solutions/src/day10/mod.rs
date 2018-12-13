use itertools::Itertools;
use crate::day10::parse::board_size;
use bmp::Image;
use bmp::Pixel;
use image::ImageBuffer;
use std::collections::HashMap;


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

    pub fn update_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

fn solve(points : &mut Vec<Point>) {
    let mut sizes : HashMap<usize, u64> = HashMap::new();
    for i in 0..11346 {
//        println!("After {} seconds ", i);
        //
        match i {
            10316 ... 10376 => print(points, i),
            _ => ()
        };
        points.iter_mut().for_each(|p| p.update_position());
        let (width, height, _, _) = board_size(&points);
        sizes.insert(i,(width as i64 * height as i64) as u64);
    }

    let (iter_min, size) = sizes.iter()
        .min_by_key(|&(_, &v)| v)
        .unwrap();

    println!("\n {:#?}\n iter min {}  size {}", sizes, iter_min, size );

}


fn print(points : &Vec<Point>, iteration: usize) {
    let (bx, by, ox, oy) = board_size(&points);

    let mut img = ImageBuffer::new(bx as u32, by as u32);

    println!("producing {} ({},{})",iteration, bx, by);
    let o = (ox,oy);

    for y in 0..by {
        for x in 0..bx {
            let pixel = match  points.iter()
                .find(|&p| p.translate(o).0 == x as i32 && p.translate(o).1 == y as i32) {
                Some(_) =>    image::Luma([255u8]),
                _ => image::Luma([0u8])
            };


            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    img.save(format!("./target/day10/img{}.png", iteration));
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
        let input = include_str!("test_data2");
        let mut points : Vec<Point> = input.lines()
            .map(|s| s.parse().unwrap())
            .collect();
        
        println!("{:#?}", points);
        
        solve(&mut points);
//        solve(&vec![Point{ position: (15, 0), velocity: (0,0)},
//                    Point{ position: (5, -2), velocity: (0,0)}]);
    }
}