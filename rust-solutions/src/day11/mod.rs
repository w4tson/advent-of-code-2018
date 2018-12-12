use itertools::Itertools;
use std::collections::HashMap;
use rayon::prelude::*;


const GRID_SIZE: usize = 300;

struct Grid {
    cells : Vec<i32>,
    serial : i32
}

impl Grid {
    pub fn new(serial: i32) -> Grid {
        
        let mut cells: Vec<i32> = vec![0; (GRID_SIZE * GRID_SIZE)];
        
        for y in 1..=GRID_SIZE {
            for x in 1..=GRID_SIZE {
                let rack_id = x as i32 + 10;
                let mut power = (rack_id * y as i32) + serial;
                power *= rack_id;
                power = (power / 100) % 10;
                power -= 5;
                cells[(x-1 + ((y-1) * GRID_SIZE)) as usize ] = power;
            }
        }
        
        Grid { cells, serial }
    }
    
    pub fn power_at(&self, cell: (usize, usize)) -> i32 {
        self.cells[cell.0-1 + ((cell.1-1) * GRID_SIZE)]
    }
    
    pub fn print_it(&self) {
        for y in 1usize..=GRID_SIZE as usize {
            for x in 1usize..=GRID_SIZE as usize{
                print!("{:3} ", self.cells[x-1 + ((y-1) * GRID_SIZE)]);
                if x == 300 { println!("\t\t\t\t\t"); }
            }
        }
    }
    
    pub fn largest_total_power(&self, square_size: usize) -> (usize, usize, i32) {
        
        let max_conv_limit = GRID_SIZE - square_size +1;
        let mut overall_max = (0,0, i32::min_value());
        
        for y in 1..=max_conv_limit {
            for x in 1..=max_conv_limit {
                
                let mut total: i32 = 0;
                
                for z in 0..square_size {
                    let start_row = x-1+ ((y-1+z)*GRID_SIZE);
                    let row = &self.cells[start_row..start_row+square_size];
                    total += row.iter().sum::<i32>();
                }
                
                if total > overall_max.2 {
                    overall_max = (x,y,total);
                }

            }
        }
        
        overall_max
    }
}


mod tests {
    use super::*;
    use spectral::assert_that;

    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test() {
        let grid = Grid::new(8);
        
        assert_that!(Grid::new(8).power_at((3,5))).is_equal_to(4);
        assert_that!(Grid::new(57).power_at((122,79))).is_equal_to(-5);
        assert_that!(Grid::new(39).power_at((217,196))).is_equal_to(0);
        assert_that!(Grid::new(71).power_at((101,153))).is_equal_to(4);
        
        let grid = Grid::new(2694);
        let sizes = (1..=GRID_SIZE).collect::<Vec<usize>>();
        let result : HashMap<(usize, usize), i32> = sizes.par_iter().map(|size| {
            print!("i = {:3} ", size);
            let (x,y, max) = grid.largest_total_power(*size);
            println!(" {},{},{} ", x,y,max);
            ((x,y), *size as i32)
        })
        .collect();
    }
}