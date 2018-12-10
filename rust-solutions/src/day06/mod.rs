use std::collections::HashMap;
<<<<<<< HEAD
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
=======
use std::str::FromStr;
use std::fmt::Error;

type Grid = Vec<HashMap<usize, i32>>;
type Coord = (usize, usize);

struct GridData {
    grid: Grid,
    size: usize,
    coords: Vec<Coord>
>>>>>>> 21a04d5a2e77374e3f44639a7b6805ed3a56a6cc
}

impl GridData {

    fn to_coord(line: &str) -> Coord {
        let mut split = line.split(", ");
        let x = split.next().map(|m| m.parse().unwrap_or(0)).unwrap();
        let y = split.next().map(|m| m.parse().unwrap_or(0)).unwrap();
        (x,y)
    }

    fn size_of_grid(c: &Vec<Coord>) -> usize {
        c.iter().flat_map(| (x,y)| vec![x,y])
            .max()
            .unwrap() + 1
    }
    
    pub fn new(coords: Vec<Coord>) -> GridData {
        let size= GridData::size_of_grid(&coords);
        let mut grid = vec![HashMap::new(); size * size];
        
        coords.iter()
            .enumerate()
            .for_each(|(i,coord)|{
<<<<<<< HEAD
                grid.insert(*coord, (i as i32, 0));
=======
                grid[coord.0 + (size * coord.1)].insert(i, 0);
>>>>>>> 21a04d5a2e77374e3f44639a7b6805ed3a56a6cc
            });
        
        GridData { grid, size, coords }
    }
    
<<<<<<< HEAD
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
=======
    pub fn manhatten_dist(&self, coord1: &Coord, coord2: &Coord) -> i32 {
        let x: i32 = coord1.0 as i32 - coord2.0 as i32;
        let y: i32 = coord1.1 as i32 - coord2.1 as i32;
        x.abs() + y.abs()
    }
    
    pub fn calc_area(&mut self, threshold: i32) -> usize {
        
        for y in 0..self.size {
            for x in 0..self.size {
                
                let distances: HashMap<usize, i32> = self.coords
                    .iter()
                    .enumerate()
                    .map(|(id, coord)| (id, self.manhatten_dist(coord, &(x,y))))
                    .collect();
                
                self.grid[x+(y*self.size)] = distances;
            }
        }

        self.grid
            .iter()
            .map(|distances: &HashMap<usize, i32>| {
                let sum : i32 = distances.values().sum();
                sum
            })
            .filter(|total| *total < threshold)
            .count() 

    }

    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let item : String = self.grid.get(x+(y*self.size))
                    .map(|distances: &HashMap<usize, i32>| {
                        distances.iter()
                            .min_by_key(|(_, &v)| v)
                            .map(|(k,_ )| format!("{}", k))
                            .unwrap_or(".".to_string())
>>>>>>> 21a04d5a2e77374e3f44639a7b6805ed3a56a6cc
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

impl FromStr for GridData {
    type Err = Error;

<<<<<<< HEAD
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
=======
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(GridData::new(input.lines().map(GridData::to_coord).collect()))
    }
}

fn solve_part2(input : &str, threshold: i32) -> Result<usize, Error> {
    let mut grid = GridData::from_str(input)?;
    let result = grid.calc_area(threshold);
//    grid.print();
    
    Ok(result)
>>>>>>> 21a04d5a2e77374e3f44639a7b6805ed3a56a6cc
}

mod tests {
    use crate::utils::file::read_puzzle_input;
    use spectral::assert_that;
    use super::*;

    #[test]
    fn test2() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        
        println!("{}", input);
<<<<<<< HEAD
        assert_that!(solve_part1(&input)).is_equal_to(17);
=======
        assert_that!(solve_part2(&input, 32)).is_equal_to(Ok(16));
>>>>>>> 21a04d5a2e77374e3f44639a7b6805ed3a56a6cc
    }

    #[test]
    fn part2() {
        let input = read_puzzle_input("day06");
        let result = solve_part2(&input, 10000);
        println!("{:#?}", result);
    }
}