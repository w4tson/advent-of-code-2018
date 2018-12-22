pub mod parse;
use crate::day17::parse::*;
use std::ops::Range;
use std::fmt::Display;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::option::Option::Some;

const WATER : char = '|';
const EMPTY : char = '.';
const WALL  : char = '#';
const STILL_WATER  : char = '~';

pub type Coord = (usize, usize);
pub struct CrossSection {
    map : Vec<Vec<char>>,
    width: FromTo,
    height: FromTo,
    stream: VecDeque<Coord>,
    seen: HashMap<Coord, usize>
}

#[derive(Copy, Clone)]
pub struct FromTo {
    start: usize,
    end: usize,
}

pub enum Flow {
    Left,
    Right,
    LeftAndRight
}


impl CrossSection {
    pub fn add_water(&mut self, x: usize, y: usize) {
        self.map[y][x] = WATER;
        self.stream.push_back((x, y));
    }

    pub fn is_water(&self, c: Coord) -> bool {
        self.map[c.1][c.0] == WATER
    }

    pub fn char_at_rel(&self, x: usize, y: usize) -> char {
        self.map[y][x - self.width.start]
    }

    fn update(&mut self, c: Coord, ch: char) {
        self.map[c.1][c.0] = ch;
    }

    pub fn is_container(&self, c: Coord) -> bool {
        let ch = self.map[c.1][c.0];
        ch == WALL || ch == STILL_WATER
    }

    pub fn is_contained_down(&self, c: Coord) -> bool {
        c.1 < self.height.count() - 1 && self.is_container((c.0, c.1 + 1))
    }

    pub fn is_contained_left(&self, c: Coord) -> bool {
        let left = (c.0 - 1, c.1);
        c.0 > 0 && (self.is_container(left) || (self.is_water(left) && self.is_contained_left(left)))
    }

    pub fn is_contained_right(&self, c: Coord) -> bool {
        let right = (c.0 + 1, c.1);
        c.0 < self.width.count() - 1 && (self.is_container(right) || (self.is_water(right) && self.is_contained_right(right)))
    }

    fn is_contained(&self, c: Coord) -> bool {
        self.is_contained_down(c) && self.is_contained_left(c) && self.is_contained_right(c)
    }

    pub fn can_flow_down(&self, c: Coord) -> Option<Coord> {
        if c.1 < self.height.count() - 1 && self.map[c.1 + 1][c.0] == EMPTY {
            Some((c.0, c.1 + 1))
        } else {
            None
        }
    }

    pub fn can_flow_left(&self, c: Coord) -> Option<Coord> {
        if c.0 > 0 && self.map[c.1][c.0 - 1] == EMPTY
            && self.is_contained_down(c)
            && c.1 != self.height.count() - 1 {
            Some((c.0 - 1, c.1))
        } else {
            None
        }
    }

    pub fn can_flow_right(&self, c: Coord) -> Option<Coord> {
        if c.0 < self.width.count() - 1 &&
            self.map[c.1][c.0 + 1] == EMPTY
            && self.is_contained_down(c)
            && c.1 != self.height.count() - 1 {
            Some((c.0 + 1, c.1))
        } else {
            None
        }
    }

    pub fn can_flow(&self, c: Coord) -> bool {
        self.can_flow_right(c).is_some() || self.can_flow_left(c).is_some() || self.can_flow_down(c).is_some()
    }

    fn flow(&mut self, coord: Coord) -> Vec<Coord> {
        let mut new_droplets = vec![];
        if let Some(down) = self.can_flow_down(coord) {
            &self.seen.entry(down).or_insert_with(|| {
                new_droplets.push(down);
                1
            });
//                new_droplets.push(down);
            self.update(down, WATER);
        } else {
            if let Some(left) = self.can_flow_left(coord) {
                self.seen.entry(left).or_insert_with(|| {
                    new_droplets.push(left);
                    1
                });
//                    new_droplets.push(left);
                self.update(left, WATER);
            }

            if let Some(right) = self.can_flow_right(coord) {
                &self.seen.entry(right).or_insert_with(|| {
                    new_droplets.push(right);
                    1
                });
//                    new_droplets.push(right);
                self.update(right, WATER);
            }
        }
        new_droplets
    }

    pub fn stream(&mut self) {
        for i in 0.. {
            let mut new_droplets: Vec<Coord> = vec![];
            let mut flowed = false;

            for s in 0..self.stream.len() {
                //can it move
                //if it is occupied
                let coord = self.stream[s];
                if self.is_water(coord) {
                    if self.can_flow(coord) {
                        let new_drops = self.flow(coord);
                        new_droplets.extend(new_drops.iter());
                        flowed = true;
                    } else {
                        //turn water still
                        if self.is_contained(coord) {
                            flowed = true;
                            //turn this to still water
                            self.update(coord, STILL_WATER);
                        }
                    }
                }
            }


            if self.char_at_rel(500, 0) == EMPTY {
                self.add_water(500 - self.width.start, 0);
            } else if !flowed {
                println!("still {}", self.count_still_water());
                break;
            }
            self.stream.extend(new_droplets.iter());
//            println!("\n{}\nRound {}", &self, i);
        }
    }

    pub fn count_still_water(&self) -> usize {
        self.map.iter().map(|row| {
            row.iter().filter(|&ch| *ch == STILL_WATER).count()
        }).sum()
    }

    pub fn count_water(&self) -> usize {
        self.map.iter().map(|row| {
            row.iter().filter(|&ch| *ch == WATER || *ch == STILL_WATER).count()
        }).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Error;
    use crate::day17::parse::*;
    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        if let Ok(cross_section) = input.parse::<CrossSection>() {
            let mut cross_section = cross_section;
            println!("{}", cross_section);
            cross_section.stream();
            let count = cross_section.count_water();
            eprintln!("count = {:#?}", count);
            
        }
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day17");
        if let Ok(cross_section) = input.parse::<CrossSection>() {
            let mut cross_section = cross_section;
//            println!("{}", cross_section);
            cross_section.stream();
            let count = cross_section.count_water();
            eprintln!("count = {:#?}", count);

        }
    }
}