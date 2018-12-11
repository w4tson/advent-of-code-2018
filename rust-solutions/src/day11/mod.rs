use itertools::Itertools;
use std::collections::HashMap;

struct Grid {
    cells : Vec<i32>,
    serial : i32
}

impl Grid {
    pub fn new(serial: i32) -> Grid {
        
        let mut cells = vec![0; 90000];
        
        for y in 1..=300 {
            for x in 1..=300 {

                if x == 3 && y ==5 {
                    println!("rack_id = {}", x + 10);
                }
                let rack_id = x + 10;
                let mut power = (rack_id * y) + serial;

                
                
                power *= rack_id;
                
                power = Grid::hundredth(power);
                
                power -= 5;
//                println!("{}", power);
                if x == 3 && y ==5 {
                    println!("power = {}", power);
                }
                cells[(x-1 + ((y-1) * 300)) as usize ] = power;
                
               
            }
        }
        
        Grid { cells, serial }
    }
    
    pub fn hundredth(n :i32) -> i32 {
        if n < 100 {
            return 0;
        } else {
            let mut s = format!("{}", n);
            let ch = s.remove(s.len() - 3);
            //TODO could the hundreds digit be negative? nah
            ch.to_digit(10).unwrap_or_else(|| panic!("problem converting char to num")) as i32
            
        }
    }
    
    pub fn power_at(&self, cell: (usize, usize)) -> i32 {
        self.cells[cell.0-1 + ((cell.1-1) * 300)]
    }
    
    pub fn print_it(&self) {
        for y in 1..=300 {
            for x in 1..=300 {
                print!("{}", self.cells[x-1 + ((y-1) * 300)]);
                if x == 300 { println!("\t\t\t\t\t"); }
            }
        }
    }
    
    pub fn largest_total_power(&self) -> (usize, usize) {

        let conv_size = 278*278;
        let mut conv = vec![0; conv_size];
        let mut powers : HashMap<(usize, usize), i32> = HashMap::new();
        
        for y in 1..=278 {
            for x in 1..=278 {
                let mut i1 = self.cells.windows(3).skip(y * 301);
                let mut i2 = self.cells.windows(3).skip((y + 1) * 301);
                let mut i3 = self.cells.windows(3).skip((y + 2) * 301);
                
                let top = i1.next().unwrap_or_else(|| panic!("nope1"));
                let middle = i2.next().unwrap_or_else(|| panic!("nope2"));
                let bottom = i3.next().unwrap_or_else(|| panic!("nope3"));
                
                let max_vec = [&top[..], &middle[..], &bottom[..]].concat().clone();
                
                if x ==1 && y ==1 {
                    eprintln!("max_vec = {:?}", max_vec);
                }
                
                let max = max_vec.iter().max().unwrap_or_else(|| panic!("no local max"));
//                    
                powers.insert((x,y), *max);
            }
        }
        
        let (max_cell, m) = 
            powers.iter()
            .max_by_key(|&(_, &v)| v)
            .unwrap_or_else(|| panic!("couldn't find max"));

        println!("{:#?} {}", max_cell, m);
        *max_cell
        
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

        let coord = Grid::new(18).largest_total_power();
        println!("coord = {:#?}", coord);

    }
}