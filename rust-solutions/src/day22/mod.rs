use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use crate::day22::Kit::Torch;
use crate::day22::CaveType::Rocky;
use crate::day22::CaveType::Narrow;
use crate::day22::CaveType::Wet;
use std::thread;
use std::time;
use itertools::Itertools;
use crate::day22::Kit::Nowt;
use crate::day22::Kit::ClimbingGear;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub mod parse;

type Coord = (usize, usize);
const INFINITY : usize = usize::max_value()/2;


#[derive(PartialEq)]
enum CaveType {
    Rocky,
    Wet,
    Narrow,
}

pub struct Cave {
    depth: usize,
    target: Coord,
    erosion_levels: HashMap<(usize,usize), usize>
}

impl Cave {
    
    pub fn new(depth: usize, target: Coord) -> Cave {
        Cave { depth, target, erosion_levels: HashMap::new() }
    }
    
    pub fn risk_level(&mut self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk += self.risk_at(&(x,y)); 
            }    
        }
        risk
    }
    
    fn geo_index(&mut self, loc: &Coord) -> usize {
        let target = self.target;
        match loc {
            (0,0) => 0,
            (x,y) if *x == self.target.0 && *y == self.target.1 => 0,
            (x,y) if *y == 0 => x * 16807,
            (x,y) if *x == 0 => y * 48271,
            (x,y) => self.erosion_level(&(x-1, *y)) * self.erosion_level(&(*x, y-1))
        }
    }
    
    fn erosion_level(&mut self, loc: &Coord) -> usize {
        if let Some(level) = self.erosion_levels.get(loc) {
            *level    
        } else {
            
            let level = ( self.geo_index(loc) + self.depth ) % 20183;
            self.erosion_levels.insert(*loc, level);
            level
        }
        
    }

    fn risk_at(&mut self, loc: &Coord) -> usize {
        self.erosion_level(loc) % 3 
    }
    
    fn cave_type_at(&mut self, loc: &Coord) -> CaveType {
        match self.erosion_level(loc) % 3 {
            0 => Rocky,
            1 => Wet,
            _ => Narrow,
        }
    }
    
    fn switch_penalty(&mut self, current: &Node, neighbour: &Node) -> usize {
        let cave_type = self.cave_type_at(&neighbour.location); 
        let cave_type_current = self.cave_type_at(&current.location); 
        match (cave_type_current, cave_type, &current.equipment) {
            (_, c,k) if c == Rocky && *k == Kit::Nowt => INFINITY,
            (_, c,k) if c == Wet && *k == Kit::Torch => INFINITY,
            (_, c,k) if c == Narrow && *k == Kit::ClimbingGear => INFINITY,
            (c, _,k) if c == Rocky && *k == Kit::Nowt => INFINITY,
            (c, _,k) if c == Wet && *k == Kit::Torch => INFINITY,
            (c, _,k) if c == Narrow && *k == Kit::ClimbingGear => INFINITY,
            (_, c,k) if *k != neighbour.equipment => 7,
            _ => 0
        }
    }
    
    pub fn dist_to_target(&mut self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances : HashMap<(Coord, Kit), usize> = HashMap::new();

        let mut prev : HashMap<Node, Node> = HashMap::new();

        heap.push(Node { location: (0,0), equipment: Torch, cost: 0 });


        distances.insert(((0,0), Kit::Torch), 0);
        
        while let Some( current) = heap.pop() {

            let location = &current.location;
            let equipment = &current.equipment;
            let cost = &current.cost;
           

            if *location == self.target && *equipment == Kit::Torch {
                let mut a = current.clone();
                while let Some(n) = prev.get(&a)  {
                    println!("{}", n);
                    a = n.clone();
                }
                return Some(*cost); 
            }

            if *cost > *distances.entry((location.clone(), equipment.clone())).or_insert(INFINITY) {
                continue; 
            }
            
            let neighbours : Vec<Node> = current.unvisited_neighbours();
            
            for neighbour in neighbours {

                let next = Node { location: neighbour.location, cost: cost + 1 + self.switch_penalty(&current, &neighbour), equipment: neighbour.equipment };


                let mut next_curr_dist = distances.entry((next.location.clone(), next.equipment.clone())).or_insert(INFINITY);

                if next.cost < *next_curr_dist {
                    *next_curr_dist = next.cost;
                    prev.insert(next.clone(), current.clone());
                    heap.push(next);
                }
            }
        }

        None
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
enum Kit {
    Nowt,
    ClimbingGear,
    Torch
}

#[derive(Eq, Hash, Clone)]
struct Node {
    location: Coord,
    equipment: Kit,
    cost: usize
}

impl Node {
    fn unvisited_neighbours(&self,) -> Vec<Node> {
        let (x,y) = self.location;
        let mut neighbours: Vec<Coord> = vec![(x+1, y), (x,y+1)];
        if x > 0 {
            neighbours.push((x-1, y));
        }

        if y > 0 {
            neighbours.push((x, y-1));
        }


        neighbours.iter().fold(vec![], |mut acc, item|{
            acc.extend_from_slice(&[
                Node{location: *item, equipment: Kit::Torch, cost: INFINITY},
                Node{location: *item, equipment: Kit::Nowt,  cost: INFINITY},
                Node{location: *item, equipment: Kit::ClimbingGear, cost: INFINITY  }
            ]);
            acc
        })
        
        
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
        .then(self.location.0.cmp(&other.location.0))
        .then(self.location.1.cmp(&other.location.1))
            .then(self.equipment.cmp(&other.equipment))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost == other.cost && self.location.0 == other.location.0 && self.location.1 == other.location.1 && self.equipment == other.equipment 
    }
}



impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let cost : String = if self.cost == INFINITY { '\u{221E}'.to_string() } else { format!("{}",self.cost) };

        write!(f, "({},{}) {:#?} [{}]",self.location.0,self.location.1,self.equipment, cost)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let cost : String = if self.cost == INFINITY { '\u{221E}'.to_string() } else { format!("{}",self.cost) };
        write!(f, "({},{}) {:#?} [{}]",self.location.0,self.location.1,self.equipment, cost)
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
        let mut cave = Cave::new( 510, (10,10));
        assert_eq!(114, cave.risk_level())
    }
    
    #[test]
    fn part1() {
        let mut cave = Cave::new( 11817, (9,751));
        eprintln!("cave.risk_level() = {:?}", cave.risk_level());
    }
    
    #[test]
    fn part2_example() {      
        let mut cave = Cave::new( 510, (10,10));
        assert_eq!(Some(45), cave.dist_to_target())
    }

    #[test]
    fn part2() {
        let mut cave = Cave::new( 11817, (9,751));
        println!("Result = {:#?}", cave.dist_to_target());
    }
}