use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Error;

type Grid = Vec<HashMap<usize, i32>>;
type Coord = (usize, usize);

struct GridData {
    grid: Grid,
    size: usize,
    coords: Vec<Coord>
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
                grid[coord.0 + (size * coord.1)].insert(i, 0);
            });
        
        GridData { grid, size, coords }
    }
    
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

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(GridData::new(input.lines().map(GridData::to_coord).collect()))
    }
}

fn solve_part2(input : &str, threshold: i32) -> Result<usize, Error> {
    let mut grid = GridData::from_str(input)?;
    let result = grid.calc_area(threshold);
//    grid.print();
    
    Ok(result)
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
        assert_that!(solve_part2(&input, 32)).is_equal_to(Ok(16));
    }

    #[test]
    fn part2() {
        let input = read_puzzle_input("day06");
        let result = solve_part2(&input, 10000);
        println!("{:#?}", result);
    }
}