use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;
use std::prelude::v1::Vec;
use std::collections::HashMap;

pub mod parse;

type Result<T> = result::Result<T, Box<Error>>;

const WIDTH : usize = 6;

struct Track {
    track : Vec<Vec<TrackType>>,
    width : usize,
    carts : Vec<Cart>
}

enum TrackType {
    Vertical,
    Horizontal,
    Left,
    Right,
    Intersection,
    None
}

enum Direction {
    North,
    South,
    East,
    West
}

fn mode(numbers: &Vec<(usize, usize)>) -> ((usize, usize), usize) {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .expect("Cannot compute the mode of zero numbers")
}

struct Cart {
    location : (usize, usize),
    facing: Direction,
    intersections: usize
}

impl Cart {
    pub fn new(facing: Direction, location: (usize, usize)) -> Cart {
        Cart { location, facing, intersections: 0 }
    }

    pub fn to_char(&self) -> char {
        match self.facing {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
            _ => '!'
        }
    }

    pub fn advance_north(&mut self) {
        self.location.1 -= 1;
    }

    pub fn advance_south(&mut self) {
        self.location.1 += 1;
    }

    pub fn advance_east(&mut self) {
        self.location.0 += 1;
    }

    pub fn advance_west(&mut self) {
        self.location.0 -= 1;
    }

    pub fn advance(&mut self) {
        match &self.facing {
            Direction::North =>  self.location.1 -= 1,
            Direction::South =>  self.location.1 += 1,
            Direction::East =>  self.location.0 += 1,
            Direction::West =>  self.location.0 -= 1
        }
    }

    pub fn turn_clockwise(&mut self) {
//        println!("turn clockwise");
        match self.facing {
            Direction::North => self.facing = Direction::East,
            Direction::South => self.facing = Direction::West,
            Direction::East => self.facing = Direction::South,
            Direction::West => self.facing = Direction::North
        }
    }

    pub fn turn_counter_clockwise(&mut self) {
//        println!("turn counter clockwise");
        match self.facing {
            Direction::North => self.facing = Direction::West,
            Direction::East => self.facing = Direction::North,
            Direction::South => self.facing = Direction::East,
            Direction::West => self.facing = Direction::South
        }
    }

}

impl FromStr for Track {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.lines().map(|l| l.len()).max().unwrap();
        println!("width {}", width);
        let mut carts = vec![];
        let track : Vec<Vec<TrackType>> = s.lines().enumerate()
            .map(|(y,line)|{
                let mut row : Vec<TrackType> = line.chars().enumerate().map(|(x, ch)|{
                    match ch {
                        '|' => TrackType::Vertical,
                        '-' => TrackType::Horizontal,
                        '+' => TrackType::Intersection,
                        '/' => TrackType::Right,
                        '\\' => TrackType::Left,
                        'v' => { carts.push(Cart::new(Direction::South,(x,y))); TrackType::Vertical },
                        '^' => { carts.push(Cart::new(Direction::North,(x,y))); TrackType::Vertical },
                        '<' => { carts.push(Cart::new(Direction::West,(x,y))); TrackType::Horizontal },
                        '>' => { carts.push(Cart::new(Direction::East,(x,y))); TrackType::Horizontal },
                        ' ' => TrackType::None,
                        _ => panic!("{}", ch)
                }}).collect();
                let len = row.len();

                for n in 0..(width-len) {
                    row.push(TrackType::None);
                }
                row
            }).collect();
        
        
//        let t : Vec<TrackType> = t.iter().flat_map(|x| x).collect();
        

        Ok(
            Track {
                track,
                width,
                carts
            }
        )
    }
}

impl Track {
    pub fn print_it(&mut self) {
        for y in 0..self.track.len() {
            let row = &self.track[y];
            for x in 0..row.len() {
                let ch = match self.track[y][x] {
                    TrackType::Horizontal => '-',
                    TrackType::Vertical => '|',
                    TrackType::Left => '\\',
                    TrackType::Right => '/',
                    TrackType::Intersection => '+',
                    TrackType::None => '.',
                    _ => '.'
                };

                let ch = self.carts.iter().find(|cart| cart.location == (x,y))
                    .map(|cart| cart.to_char())
                    .unwrap_or_else(|| ch);
                print!("{}", ch);
            }
            println!();
        }
    }

  pub fn move_carts(&mut self) -> Option<(usize, usize)> {
     self.carts.sort_by_key(|cart| cart.location);
     for mut cart in &mut self.carts {
         
         let (x,y) = cart.location;
         let track = &self.track[y][x];
         

         //     |
         //---  /---
//              |
         match (track, &cart.facing) {
             (TrackType::Left, Direction::North) | (TrackType::Left, Direction::South) => cart.turn_counter_clockwise(),
             (TrackType::Left, Direction::East) | (TrackType::Left, Direction::West)=> cart.turn_clockwise(),
             (TrackType::Right, Direction::North) | (TrackType::Right, Direction::South) => cart.turn_clockwise(),
             (TrackType::Right, Direction::East) | (TrackType::Right, Direction::West)=> cart.turn_counter_clockwise(),
             (TrackType::Intersection,_) => {
                match cart.intersections % 3 {
                    0 => cart.turn_counter_clockwise(),
                    1 => { },
                    2 => cart.turn_clockwise(),
                    _ => panic!("doh")
                }
                cart.intersections += 1;
             },
             _ => {
//                 println!("no op");
                 //no op
             }
         }

         cart.advance();




     }

      let deduped_total = &self.carts.iter()
          .map(|cart| cart.location)
          .dedup()
          .count();

      let ((x,y), count) = mode(&self.carts.iter().map(|c| c.location).collect());
//      if self.carts.len() != *deduped_total {
//          println!("CRASH!!!! ");
//          self.carts.iter().for_each(|c| {
//              println!("{},{}", c.location.0, c.location.1);
//          })
//
//      }
//     self.print_it();
      match count > 1 {
          true => Some((x,y)),
          _ => None
      }
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
        let mut track : Track = input.parse()
                        .unwrap_or_else(|_| panic!("Couldn't parse {}", input));

        track.print_it();
        for i in 0..15 {
            println!("i = {}", i);
            track.move_carts();
        }
//        assert_that!(foos[1].bar()).is_equal_to(2);


    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day13");
        let mut track : Track = input.parse()
            .unwrap_or_else(|_| panic!("Couldn't parse {}", input));

        track.print_it();
        let mut found = false;

        while !found {
            match track.move_carts() {
                Some(crashed) => { println!("{:#?}" , crashed); break; },
                _ => {}
            }
        }
//        let foo : Foo = input.parse().unwrap();
//        println!("part1 {}", node.bar());
    }
}