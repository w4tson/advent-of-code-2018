use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub mod parse;

#[derive(Debug)]
pub struct Node {
    metadata_size: i32,
    children_size: i32,
    
    children: Vec<Node>,
    metadata: Vec<i32>
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})\n", self.children_size, self.metadata_size);
        for c in &self.children {
            write!(f, "    {}\n", c);
        }
        write!(f, " ")
    }
}

impl Node {
    pub fn new(children : Vec<Node>, metadata: Vec<i32>) -> Node {
        Node {
            metadata_size: metadata.len() as i32, children_size: children.len() as i32, 
            children, metadata 
        }
    }

    pub fn new_childless(metadata: Vec<i32>) -> Node {
        Node {
            metadata_size: metadata.len() as i32, children_size: 0,
            children: vec![], metadata
        }
    }
    
    pub fn size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.size())
            .sum::<usize>() + self.metadata_size as usize + 2
    }
    
    pub fn total_metadata(&self) -> i32 {
        self.children
            .iter()
            .map(|child| child.total_metadata())
            .sum::<i32>() + self.metadata.iter().sum::<i32>()
    }
    
    pub fn value(&self) -> i32 {
        match self.children_size {
            0 => self.metadata.iter().sum::<i32>(),
            _ => {
                self.metadata.iter()
                    .map(|index| {
                        self.children.get(*index as usize -1)
                            .map(|child: &Node| child.value())
                            .unwrap_or_default()
                        
                    })
                    .sum::<i32>()
            }
        }
    }
    
}


mod tests {
//    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let nodes : Result<Node, _> = input.parse(); 
//        assert_that!(solve_part1(&input )).is_equal_to(Ok("CABDFE".to_string()));
    }

    #[test]
    fn part2() {
        let input = read_puzzle_input("day08");
        let nodes : Result<Node, _> = input.parse();
    }
}