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

struct Cart {
    location: (usize, usize),
    facing: Direction,
    intersections: usize,
    alive: bool,
}

impl Cart {
    pub fn new(facing: Direction, location: (usize, usize)) -> Cart {
        Cart { location, facing, intersections: 0, alive: true }
    }
    
    pub fn turn_clockwise(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South
        }
    }

    pub fn turn_counter_clockwise(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North
        }
    }
    
    pub fn advance(&mut self) {
        match self.facing {
            Direction::North => self.location.1 -= 1,
            Direction::East => self.location.0 += 1,
            Direction::South => self.location.1 += 1,
            Direction::West => self.location.0 -= 1,
        }    
    }
    
    pub fn maybe_turn(&mut self) {
        match self.intersections % 3 {
            0 => self.turn_counter_clockwise(),
            1 => { },
            2 => self.turn_clockwise(),
            _ => panic!("oops")
        }
        self.intersections +=1;
    }
}

impl FromStr for Track {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.lines().map(|l| l.len()).max().unwrap();
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
                //padding
                for n in 0..(width-len) {
                    row.push(TrackType::None);
                }
                row
            }).collect();
        
        Ok( Track { track, width, carts } )
    }
}

impl Track {
    
    
  pub fn println_it(&self) {
      for (y, row) in self.track.iter().enumerate() {
          row.iter().enumerate().for_each(|(x, t)| {
              let ch = match t {
                  TrackType::Vertical => "|",
                  TrackType::Horizontal => "-",
                  TrackType::Left => "\\",
                  TrackType::Right => "/",
                  TrackType::Intersection => "+",
                  TrackType::None => " ",
                  _ => panic!("adsf")
              };
              
              let s = self.alive_carts().iter().find(|&c| c.location == (x,y))
                  .map(|c| match c.facing {
                      Direction::North => "^",
                      Direction::South => "v",
                      Direction::West => "<",
                      Direction::East => ">"
                  })
                  .unwrap_or(ch);
              
              print!("{}", s);
              
          });
          println!();
      }
  }
    
  pub fn move_carts(&mut self) {
      self.carts.sort_by_key(|c| c.location);
      
      for i in 0..self.carts.len() {
          let mut cart = &mut self.carts[i];
          let (x,y) = cart.location;
          let track_type = &self.track[y][x];
          
          //skip if dead
          if !cart.alive { continue }
          
          //turn
          match (&cart.facing, track_type) {
              (Direction::North, TrackType::Left)|(Direction::South, TrackType::Left) => cart.turn_counter_clockwise(),
              (Direction::East, TrackType::Left)|(Direction::West, TrackType::Left) => cart.turn_clockwise(),
              (Direction::North, TrackType::Right)|(Direction::South, TrackType::Right) => cart.turn_clockwise(),
              (Direction::East, TrackType::Right)|(Direction::West, TrackType::Right) => cart.turn_counter_clockwise(),
              (_, TrackType::Intersection) => cart.maybe_turn(),
              _ => {}
              
          }
          
          //advance
          cart.advance();
          
          //check collision
          let collisions = self.collisions();
          
          for (&location, &count) in collisions.iter()  {
            self.carts.iter_mut()
                .filter(|c| c.location == location)
                .for_each(|c| {
                    println!("Removing cart at {},{}", c.location.0, c.location.1);
                    c.alive = false;
                })
              
          }
      }
  }

    pub fn has_mulitple_carts(&self) -> bool {
        let result = self.alive_carts().len() > 1;
        if result == false {
            println!("{:#?}", self.alive_carts()[0].location);
        }
        result
    }

    pub fn collisions(&self) -> HashMap<(usize, usize), usize> {
        let mut occurrences = HashMap::new();

        for value in &self.alive_carts() {
            *occurrences.entry(value.location).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .filter(|&(_, count)| count > 1)
            .collect()
    }
    
    pub fn alive_carts(&self) -> Vec<&Cart> {
        self.carts.iter()
            .filter(|&c| c.alive)
            .collect()
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
        let input = include_str!("test_data2");
        let input = read_puzzle_input("day13");
        let mut track : Track = input.parse()
                        .unwrap_or_else(|_| panic!("Couldn't parse {}", input));
       
            track.println_it();
        
            while track.has_mulitple_carts() {
                track.move_carts();
            }
    }
}