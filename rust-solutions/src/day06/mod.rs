use std::collections::HashMap;
use crate::utils::dupes;
use itertools::Itertools;

type Coord= (i32, i32);
type IdAndDist = (i32, i32);
type Grid = HashMap<Coord, IdAndDist>;

struct GridData {
    grid: HashMap<Coord, IdAndDist>,
    size: i32,
    coords: Vec<(i32, i32)>,
    ring: i32
}

impl GridData {

    fn size_of_grid(c: &Vec<(i32, i32)>) -> i32 {
        c.iter().flat_map(| (x,y)| vec![x,y])
            .max()
            .unwrap() + 1
    }
    
    pub fn new(coords: Vec<(i32, i32)>) -> GridData {
        let mut grid = HashMap::new();
        
        coords.iter()
            .enumerate()
            .for_each(|(i,coord)|{
                grid.insert(*coord, (i as i32, 0));
            });
        GridData { grid, size: GridData::size_of_grid(&coords), coords, ring: 1 }
    }
    
    pub fn manhatten_dist(coord1: &Coord, coord2: &Coord) -> i32 {
        let x = coord1.0 - coord2.0;
        let y = coord1.1 - coord2.1;
        x.abs() + y.abs()
    }
    
    pub fn round(&mut self) {

        let updates : Vec<(i32, Coord, i32)>= self.coords.to_owned().iter()
            .enumerate()
            .flat_map(|(id, coord)| self.new_growth(id as i32, coord))
            .collect();


        updates
            .iter()
            .for_each(|(id, coord, dist)| {

                let (existing_id, existing_dist) = self.grid
                    .entry(*coord)
                    .or_insert((*id, 9999999));
                
                if *dist < *existing_dist {
                        *existing_id = *id;
                        *existing_dist = *dist;
                    } else if *existing_dist == *dist {
                        *existing_id = -1;
                        *existing_dist = *dist;
                    }
            });
        self.ring += 1;
    }

    fn new_growth(&self, id: i32, coord: &Coord) -> Vec<(i32, Coord, i32)>{
        //calculate the ring

        let width = self.ring * 2 + 1;
        let mut ring_coords : Vec<Coord> = vec![];

        for y in coord.1 - self.ring..coord.1 + self.ring+1 {
            for x in coord.0 - self.ring..coord.0 + self.ring+1 {
                if x>=0 && x < self.size && y >=0 && y < self.size {
                    if y == coord.1 - self.ring || y == coord.1 + self.ring {
                        ring_coords.push((x, y));
                    } else if x == coord.0 - self.ring || x == coord.0 + self.ring {
                        ring_coords.push((x, y))
                    }
                }
            }
        }
        ring_coords
            .iter()
            .map(|c| (id, *c, GridData::manhatten_dist(coord, c)))
            .collect()
    }


    fn find_finite(&self) -> Vec<i32> {
        let infinite : Vec<_> = self.grid
            .iter()
            .filter_map(|((x,y) ,(id, _))| {
                match *y == 0 || *y == self.size-1 || *x == 0 || *x == self.size-1 {
                    true => Some(*id),
                    _    => None
                }
            })
            .unique()
            .collect();
        (0..self.coords.len())
            .map(|x| x as i32)
            .filter(|x| !infinite.contains(x))
            .collect()
    }

    fn biggest_finite(&self) -> i32 {
        *self.find_finite()
            .iter()
            .max_by_key(|&id| self.count_all_for_id(id))
            .unwrap()
    }

    fn count_all_for_id(&self, id_filter: &i32) -> i32 {
        self.grid
            .values()
            .filter_map(|(id, dist)| match id == id_filter {
                true => Some(1),
                false => None
            })
            .sum()
    }

    fn largest(&self) -> i32 {
        self.count_all_for_id(&self.biggest_finite())
    }

    
    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let item : String = self.grid.get(&(x,y))
                    .map(|(id, _)| {
                        if *id == -1 { ".".to_string() }
                        else { ((*id as u8 + 'a' as u8) as char).to_string() }
                    })
                    .unwrap_or(".".to_string());

                if x == self.size-1 {
                    println!("{}", item);
                } else {
                    print!("{}", item)
                }
            }
        }
        println!();
    }
}

fn solve_part1(input : &str) -> i32 {
    let coords : Vec<_> = input.lines().map(to_coord).collect();

    let mut grid = GridData::new(coords);

    (0..grid.size/2+1)
        .for_each(|i| {
            grid.round();
        });
    
    grid.print();

    let infinite = grid.find_finite();
    let biggest_finite = grid.largest();
    println!("{:#?}", biggest_finite);

    biggest_finite
}

mod tests {
    use crate::utils::file::read_puzzle_input;
    use spectral::assert_that;
    use super::*;

    #[test]
    fn test1() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        
        println!("{}", input);
        assert_that!(solve_part1(&input)).is_equal_to(17);
    }

    #[test]
    fn part1() {
        let input = read_puzzle_input("day06");
        let result = solve_part1(&input);
        println!("{}", result);
    }
}

fn to_coord(line: &str) -> (i32, i32) {
    let mut split = line.split(", ");
    let x = split.next().map(|m| m.parse().unwrap_or(0)).unwrap();
    let y = split.next().map(|m| m.parse().unwrap_or(0)).unwrap();
    (x,y)
}

