use core::result;
use std::error::Error;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;
use itertools::Itertools;
use crate::day15::paths::min_distance;
use std::option::Option::Some;

pub mod parse;
pub mod paths;

type MResult<T> = result::Result<T, Box<Error>>;

type Coord = (usize, usize);

//TODO attacks can happen immediately or after a turn in range
const INITIAL_POWER: i32 = 200;
const ATTACK_POWER: i32 = 3;

#[derive(PartialEq)]
pub enum PlayerType {
    Elf,
    Goblin,
}

#[derive(PartialEq)]
pub struct Player {
    p_type: PlayerType,
    hit_points: i32,
    location: Coord,
    attack_power: i32,
}



/// walls == true, spaces == false
pub struct Cave {
    players: Vec<Player>,
    rounds: i32,
    map: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}


impl Player {
    pub fn new(p_type: PlayerType, location: Coord) -> Player {
        Player { hit_points: INITIAL_POWER, p_type, location, attack_power: 3 }
    }

    pub fn is_at(&self, x: usize, y: usize) -> bool {
        self.location == (x, y)
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
    
    pub fn attacked(&mut self, hit_points: i32) {
        self.hit_points -= hit_points;
//        if self.hit_points < 0 { println!("Dead! : {:#?}", self); }
    }
}

impl Cave {
    
    pub fn tick(&mut self) -> Option<i32> {
        self.players.sort_by_key(|p| (p.location.1, p.location.0));
        
        for u in 0..self.players.len() {
            let player = &self.players[u];
//            println!("Player is {:#?}", player);
            
            if !player.is_alive() { continue }

            if self.has_no_enemies(player) {
                for ally in self.allies_of(&player.p_type) {
                    println!("{:#?}", ally);
                }
                return Some(self.allies_of(&player.p_type).iter().map(|&p| p.hit_points).sum::<i32>() * self.rounds);
            }
            
            if self.can_attack(u) {
                self.attack(u);
            } else {
                self.move_player(u);
            }

           
        }

//        for ally in &self.players {
//            println!("{:#?}", ally);
//        }

        self.rounds += 1;
        None


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
    
    fn move_player(&mut self, player_idx: usize) {
        let player = &self.players[player_idx];
        let targets = self.enemies_of(player);
        let in_range = targets.iter().flat_map(|&t| self.squares_in_range(&t.location)).collect_vec();
//        println!("in range {:#?}", in_range);

        let reachable: Vec<(Coord, usize)> = in_range
            .iter()
            .filter_map(|coord| {
                self.shortest_path(&player.location, coord).map(|n| (*coord, n))
            }).collect_vec();

//        println!("reachable {:#?}", reachable);

        if let Some(nearest) = reachable.iter().min_by_key(|&(_, dist)| *dist) {
            let (_, nearest_distance) = nearest;
//            println!("Found some near targets {} hops away", nearest_distance);
            let mut nearest = reachable.iter()
                .filter_map(|&(c, d)| match d {
                    x if *nearest_distance == x => Some(c),
                    _ => None
                }).collect_vec();


//            println!("Candidates = {:#?}", nearest);

            nearest.sort_by_key(|&(x,y)|(y,x));

            let chosen: Coord = *nearest.get(0).unwrap();
//            println!("chose {:#?}", chosen);


            let squares = self.squares_in_range(&player.location);
//            println!("squares {:#?}", squares);
            let mut starting_squares: Vec<Coord> = squares
                .iter()
                .filter_map(|starting_sq| {
                    let sp = self.shortest_path(starting_sq, &chosen);
                    match sp {
                        Some(d) if d == *nearest_distance -1 => Some(*starting_sq),
                        _ => None
                    }
                })
                .collect_vec();

            starting_squares.sort_by_key(|&(x,y)|(y,x)); //sort y values first

            let next_square = starting_squares.first().unwrap_or_else(|| panic!("oops {}", starting_squares.len()));
//            println!("Next sq = {:#?}", next_square);
            let player = &mut self.players[player_idx];
            player.location = *next_square;

            if self.can_attack(player_idx) {
                self.attack(player_idx);
            }

        }
        
    }
    

    pub fn shortest_path(&self, from: &Coord, to: &Coord) -> Option<usize> {
        let mut grid = self.map.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                grid[y][x] = match (x, y) {
                    _ if *from == (x,y) => false,
                    _ if *to == (x,y)  => false,
                    (_, _) if self.has_player_at(&(x, y)) => true,
                    _ => self.map[y][x]
                }
            }
        }
//        println!("shortest path grid\n {:#?}", grid);
        min_distance(grid, from, to)
    }

    fn has_player_at(&self, coord: &Coord) -> bool {
        self.players.iter().any(|player| player.location == *coord && player.is_alive())
    }

    fn has_no_enemies(&self, p: &Player) -> bool {
        self.enemies_of(p).len() == 0
    }

    fn enemies_of(&self, p: &Player) -> Vec<&Player> {
        self.players.iter().filter(|&o| o.p_type != p.p_type && o.is_alive()).collect()
    }

    fn allies_of(&self, pt: &PlayerType) -> Vec<&Player> {
        self.players.iter().filter(|&o| o.p_type == *pt && o.is_alive()).collect()
    }

    fn is_occupied(&self, coord: Coord) -> bool {
        match coord {
            (_, _) if self.has_player_at(&coord) => true,
            _ => self.map[coord.1][coord.0]
        }
    }

    fn squares_in_range(&self, coord: &Coord) -> Vec<Coord> {
        self.squares_nearby(coord).iter()
            .filter(|&sq| !self.is_occupied(*sq))
            .map(|&a|a)
            .collect()
    }
    
    fn squares_nearby(&self, coord: &Coord) -> Vec<Coord> {
        let mut squares = vec![];
        // up 
        if coord.1 >= 1 { squares.push((coord.0, coord.1 - 1)); }

        // left 
        if coord.0 >= 1 { squares.push((coord.0 - 1, coord.1)); }

        // right 
        if coord.0 < self.width  { squares.push((coord.0 + 1, coord.1)); }
        
        // down 
        if coord.1 + 1 < self.height { squares.push((coord.0, coord.1 + 1)); }

        squares
    }

    fn can_attack(&self, player_idx: usize) -> bool {
        let player = &self.players[player_idx];
        self.squares_nearby(&player.location)
            .iter()
            .any(|sq| self.enemies_of(player).iter().any(|&enemy| enemy.location == *sq))
    }
    
    fn targets_in_range(&self, player: &Player) -> Vec<&Player> {
        self.squares_nearby(&player.location)
            .iter()
            .filter_map(|sq| {
                self.enemies_of(player).iter().find(|&enemy| enemy.location == *sq).map(|&a|a)
            }).collect()
    }
    
    fn attack(&mut self, attacker_idx: usize) {
        let attacker = &self.players[attacker_idx];
        let power = attacker.attack_power;

        let enemy = *self.targets_in_range(attacker).iter().min_by_key(|&&enemy| enemy.hit_points)
            .expect("Should be someone to attack");
        
        let (idx, _) = self.players.iter().find_position(|&p| p == enemy).unwrap();
        let enemy = &mut self.players[idx];
        enemy.attacked(power);
    }
    
    pub fn rounds(&self) -> i32 {
        self.rounds
    }
    
    pub fn elves(&self) -> usize {
        self.allies_of(&PlayerType::Elf).len()
    }

    pub fn goblins(&self) -> usize {
        self.allies_of(&PlayerType::Goblin).len()
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
        let mut cave: Cave = input.parse().unwrap();

        println!("{:#?}", cave);

//        assert_that!(foos[1].bar()).is_equal_to(2);
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day15");
        let mut cave: Cave = input.parse().unwrap();

        loop {
//            println!("{:#?}", cave);
            if let Some(result) = cave.tick() {
                println!("res {}", result);
                break;
            }
            if cave.rounds() % 50 ==0 { println!("{} {} Elves {} Goblins", cave.rounds(), cave.elves(), cave.goblins()) };
        }
    }
    
    #[test]
    fn more() {
        let input = include_str!("test");
        let mut cave: Cave = input.parse().unwrap();

        for i in 0..50 {
            println!("{:#?}", cave);
            if let Some(result) = cave.tick() {
                println!("res {}", result);    
                break;
            }
            println!("{}", i);
        }
    }

    #[test]
    fn sp() {
        let input = include_str!("test_data");
        let cave: Cave = input.parse().unwrap();

        cave.shortest_path(&(2, 1), &(4, 2));
    }
    
    #[test]
    fn asdf() {
        let mut input = vec![(3,1),(2,2),(2,3),(3,4)];
        let mut input = [(4,1),(4,2),(5,2),(3,2),(5,3),(5,4)];  
        input.sort_by_key(|&(a,b)|(b,a));
        
        println!("{:#?}", input);
        
        for x in &input {
            println!("{},{}", x.0, x.1);
        }
    }
}