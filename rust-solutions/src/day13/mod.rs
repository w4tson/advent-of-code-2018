use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;
use std::prelude::v1::Vec;

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



struct Cart {
    location : (u32, u32),
    facing: Direction
}

impl Cart {
    pub fn new(facing: Direction, location: (u32, u32)) -> Cart {
        Cart { location, facing }
    }

}

impl FromStr for Track {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.lines().map(|l| l.len()).max().unwrap();
        let mut carts = vec![];
        let track : Vec<Vec<TrackType>> = s.lines().enumerate()
            .map(|(y,line)|{
                let y = y as u32;
                let mut row : Vec<TrackType> = line.chars().enumerate().map(|(x, ch)|{
                    let x = x as u32;
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
                for n in len..=(width-len) {
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
  pub fn move_carts() {
      
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
        let track : Track = input.parse()
                        .unwrap_or_else(|_| panic!("Couldn't parse {}", input));
       
            
//        assert_that!(foos[1].bar()).is_equal_to(2);
    }

    #[test]
    fn part1() {
//        let input = read_puzzle_input("day11");
//        let foo : Foo = input.parse().unwrap();
//        println!("part1 {}", node.bar());
    }
}