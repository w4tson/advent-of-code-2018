use std::str::FromStr;
use std::error::Error;
use core::result;

use lazy_static::lazy_static;
use regex::Regex;
use crate::day15::Cave;
use crate::day15::Player;
use crate::day15::PlayerType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;


macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

//type Result<T> = result::Result<T, Box<Error>>;
type MResult<T> = result::Result<T, Box<Error>>;

impl FromStr for Cave {
    type Err = Box<Error>;

    fn from_str(s: &str) -> MResult<Self> {
        let mut map: Vec<Vec<bool>> = Vec::new();
        let mut players = Vec::new();

        s.lines().enumerate().for_each(|(y, line)| {
            let mut row = Vec::new();
            line.chars().enumerate().for_each(|(x, ch)| {
                match ch {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    'G' => {
                        let goblin = Player::new(PlayerType::Goblin, (x, y));
                        players.push(goblin);
                        row.push(false);
                    }
                    'E' => {
                        let elf = Player::new(PlayerType::Elf, (x, y));
                        players.push(elf);
                        row.push(false);
                    }
                    _ => panic!("Unknown cave item")
                }
            });
            map.push(row);
        });

        let height = map.len();
        let width = map[0].len();

        Ok(Cave { players, map, rounds: 0, width, height })
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.p_type {
            PlayerType::Elf => write!(f, "E"),
            _ => write!(f, "G"),
        }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.p_type == PlayerType::Elf {
            write!(f, "Elf    ");
        } else {
            write!(f, "Golbin ");
        }
        write!(f, " ({},{})  [{}] ", self.location.0, self.location.1, self.hit_points)
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.map.len() {
            let row = &self.map[y];
            for x in 0..row.len() {
                if let Some(p) = self.players.iter().find(|p| p.is_at(x, y) && p.is_alive()) {
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
