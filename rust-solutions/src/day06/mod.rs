use std::collections::HashMap;
use crate::utils::dupes;

type Grid = HashMap<(i32, i32), Vec<i32>>;
type Coord= (i32, i32);

struct GridData {
    grid: HashMap<(i32, i32), Vec<i32>>,
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
                grid.insert(*coord, vec![i as i32]);
            });
        GridData { grid, size: GridData::size_of_grid(&coords), coords, ring: 1 }
    }
    
    pub fn manhatten_dist(&self, coord1: Coord, coord2: Coord) -> i32 {
        let x = coord1.0 - coord2.0;
        let y = coord1.1 - coord2.1;
        x.abs() + y.abs()
    }
    
    pub fn round(&mut self) {
        let foo = &self.coords.to_owned();
        let new_coords: HashMap<i32, Vec<(i32, i32)>> = foo.iter()
            .enumerate()
            .map(|(id, coord)| (id as i32, self.new_growth(coord)))
            .collect();
        
        let all = new_coords.values()
            .into_iter()
            .flat_map(|x| x)
            .collect();
        
        let d : Vec<&(i32, i32)> = dupes(&all);
        
        d.iter().for_each(|&x| {
            println!("{:#?}", x);
            self.grid.insert( *x, vec![9]);
        });
    }
    
    fn new_growth(&self, coord: &(i32, i32)) -> Vec<(i32, i32)>{
        //calculate the ring
        
        let width = self.ring * 2 + 1;
        let mut ring_coords : Vec<(i32 ,i32)> = vec![];
        
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
//        println!("{:#?}", ring_coords);
            
        //5,5
        
        //4,4 5,4 6,4
        //4,5 5,5 6,5
        //4,6 5,6 6,6
        
    }

    
    
    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let item : String = self.grid.get(&(x,y))
                    .map(|x| {
                        if x.len() == 0 { ".".to_string() } else {
                            let num = x[0];
                            num.to_string()
                        }
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
    grid.print();
    grid.round();
    grid.print();
//    grid.grow_area(&(8,9));
    
    10
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
        assert_that!(solve_part1(&input)).is_equal_to(10);
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

