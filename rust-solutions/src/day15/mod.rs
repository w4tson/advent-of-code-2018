use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;

pub mod parse;

type MResult<T> = result::Result<T, Box<Error>>;
type Coord = (usize, usize); 
//TODO attacks can happen immediately or after a turn in range
const INITIAL_POWER : i32  = 200;
const ATTACK_POWER : i32 = 3;

enum PlayerType {
    Elf,
    Goblin
}

struct Player {
    p_type : PlayerType,
    hit_points: i32,
    location: Coord
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.p_type {
            PlayerType::Elf => write!(f, "E"),
            _ => write!(f, "G"),
        }   
    }
}

struct Cave {   
    players: Vec<Player>,
    rounds: usize,
    map: Vec<Vec<bool>>
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.map.len() {
            let row = &self.map[y];
            for x in 0..row.len() {
                if let Some(p) = self.players.iter().find(|p| p.is_at(x,y)){
                    write!(f, "{}", p);  
                } else {
                    match row[x] {
                        true => write!(f, "#"),
                        _ => write!(f, "."),
                    };   
                }
                
            }
            writeln!(f);
        }
        Ok(())
    }
}

impl FromStr for Cave {
    type Err = Box<Error>;

    fn from_str(s: &str) -> MResult<Self> {
        let mut map: Vec<Vec<bool>> = Vec::new();
        let mut players = Vec::new();
        
        s.lines().enumerate().for_each(|(y, line)|{
            let mut row = Vec::new();
            line.chars().enumerate().for_each(|(x, ch)|{
                match ch {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    'G' => {
                        let goblin = Player::new(PlayerType::Goblin, (x,y));
                        players.push(goblin);
                        row.push(false);
                    },
                    'E' => {
                        let elf = Player::new(PlayerType::Elf, (x,y));
                        players.push(elf);
                        row.push(false);
                    },
                    _ => panic!("Unknown cave item")
                }    
            });
            map.push(row);
        });
        
        Ok( Cave { players, map, rounds: 0 } )
    }
}



impl Player {
    pub fn new(p_type: PlayerType, location: Coord) -> Player {
        Player { hit_points : INITIAL_POWER, p_type, location }
    }
    
    pub fn is_at(&self, x: usize, y: usize) -> bool {
        self.location == (x,y)
    }

}

impl Cave {
    
    pub fn tick(&mut self) {
        
        
        
        
        //for each unit
          // if no enemy units remaining: abort
          // if 
          //   adjacent to enemy attack closest in string order
          // else 
          //   identify enemy units
          //   calc shortest path to each
          //   pick winner (closest + string order)
          //   calculate the sq to advance to
          //   attack if necessary
          // inc round counter
    }
    
    pub fn shortest_path(&self, from: &Coord, to: &Coord) {
        
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
        let cave : Cave = input.parse().unwrap();
        
        println!("{:#?}", cave);
            

//        assert_that!(foos[1].bar()).is_equal_to(2);
    }

    #[test]
    fn part1() {
//        let input = read_puzzle_input("day11");
//        let foo : Foo = input.parse().unwrap();
//        println!("part1 {}", node.bar());
    }
    
    #[test]
    fn sp() {
        let input = include_str!("test_data");
        let cave : Cave = input.parse().unwrap();

        cave.shortest_path(&(2,1), &(4,2));
    }
}