use std::str::FromStr;
use std::error::Error;
use core::result;

use lazy_static::lazy_static;
use regex::Regex;
use crate::day18::LumberArea;
use itertools::Itertools;
use std::fmt::Display;
use std::fmt::Formatter;


macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type AResult<T> = result::Result<T, Box<Error>>;

impl FromStr for LumberArea {
    type Err = Box<Error>;

    fn from_str(s: &str) -> AResult<Self> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();

        let map = s.lines().map(|line| line.chars().collect_vec()).collect_vec();


        Ok(LumberArea { map, width, height })
    }
}

impl Display for LumberArea {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.map[y][x]);
            }
            writeln!(f, "");
        }
        write!(f,"")
        
    }
}

