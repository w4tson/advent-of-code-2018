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

pub mod parse;

//type Result<T> = result::Result<T, Box<Error>>;
type Coord = (usize, usize);


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
        match (cave_type, &current.equipment) {
            (c,k) if c == Rocky && *k == Kit::Nowt => usize::max_value() / 2,
            (c,k) if c == Wet && *k == Kit::Torch => usize::max_value() / 2,
            (c,k) if c == Narrow && *k == Kit::ClimbingGear => usize::max_value() /2,
            (c,k) if *k != neighbour.equipment => 7,
            _ => 0
        }
    }
    
    pub fn dist_to_target(&mut self) -> usize {
        let mut unvisited: HashSet<Node> = HashSet::new();
        let mut visited : Vec<Node> = vec![];
        let mut distances : HashMap<Node, usize> = HashMap::new();
        let mut prev : HashMap<Node, Node> = HashMap::new();
        let origin = Node { location: (0,0), equipment: Torch };
        unvisited.insert(origin.clone());
        visited.push(Node { location: (0,0), equipment: ClimbingGear });
        visited.push(Node { location: (0,0), equipment: Nowt });
        let mut current = origin.clone();
        distances.insert(origin, 0);
        let mut result = 0;
        
        loop {
            // get all unvisited neighbours
            // for each
            //   calc tentative distance
            //   insert into distances map if value is smaller
            //
            // remove current node from unvisited and push it onto visited
            // has the destination been visited ? break;
            // *may need to check all 3 distances if there is less than 7 in the differnece
            
            // find the unvisited with the smallest dist 
            // set that to be current
//            thread::sleep(time::Duration::from_secs(1));

            
            
            let neighbours = current.unvisited_neighbours(&visited);
//            eprintln!("current = {} neighbours {:?}", current, neighbours);
            
            unvisited.extend(neighbours.clone().into_iter());
            
            for i in 0..neighbours.len() {
                let neighbour = &neighbours[i];
                let curr_distance = distances.get(&current).unwrap();
                
                //todo calc switch penalty. This is broken
                //     also i don't know why its not returning. Maybe do an investigation
                let tentative_dist = curr_distance + 1 + self.switch_penalty(&current, &neighbour);
                
                let mut dist = distances.entry(neighbour.clone())
                    .or_insert(usize::max_value());

                if tentative_dist < *dist {
                    *dist = tentative_dist;
                    prev.insert(neighbour.clone(), current.clone());
                }
            }
            
//            let index = unvisited.iter().position(|n| *n == current).unwrap();
            unvisited.remove(&current);
            visited.push(current.clone());

            if visited.iter().any(|n| n.location == self.target && n.equipment == Kit::Torch) {
                println!("Reached target {:#?}", current);
                
                let mut a = &current;
                while let Some(node) = prev.get(&a) {
                    println!(" : {}", node );
                    a = &node;
                }
                
                result = *distances.get(&current).unwrap();
                break;
            }

//            eprintln!("\nunvistited = {:#?}\n", unvisited.iter().map(|n| format!("{}, {}", n, distances.get(n).unwrap_or(&usize::max_value()))).collect_vec());
//            eprintln!("\nvisited = {:#?}\n", visited.iter().map(|n| format!("{}, {}", n, distances.get(n).unwrap_or(&usize::max_value()))).collect_vec());
            
            
            let shortest_unvisited= unvisited.iter().min_by_key(|n| distances.get(*n).unwrap_or(&usize::max_value()))
                .unwrap();

            current = shortest_unvisited.clone();

//            eprintln!("unvisited = {:#?}", unvisited);
            
        }
        
        


        result
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Kit {
    Nowt,
    ClimbingGear,
    Torch
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Node {
    location: Coord,
    equipment: Kit,
}

impl Node {
    fn unvisited_neighbours(&self, visited: &Vec<Node>) -> Vec<Node> {
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
                Node{location: *item, equipment: Kit::Torch},
                Node{location: *item, equipment: Kit::Nowt},
                Node{location: *item, equipment: Kit::ClimbingGear}
            ]);
            acc
        }).into_iter().filter(|neighbour|{
            !visited.contains(&&neighbour)
        })
        .collect()
        
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{}) {:#?}",self.location.0,self.location.1,self.equipment)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{}) {:#?}",self.location.0,self.location.1,self.equipment)
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
    fn part2() {
        let mut cave = Cave::new( 510, (10,10));
        assert_eq!(45, cave.dist_to_target())
    }

}