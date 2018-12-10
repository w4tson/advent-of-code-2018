use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub mod parse;

#[derive(Debug, Clone)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})\n", self.children.len(), self.metadata.len());
        for c in &self.children {
            write!(f, "    {}\n", c);
        }
        write!(f, " ")
    }
}

impl Node {
    pub fn new(children : Vec<Node>, metadata: Vec<i32>) -> Node {
        Node { children, metadata }
    }

    pub fn size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.size())
            .sum::<usize>() + self.metadata.len() as usize + 2
    }
    
    pub fn metadata_sum(&self) -> i32 {
        self.children
            .iter()
            .map(|child| child.metadata_sum())
            .sum::<i32>() + self.metadata.iter().sum::<i32>()
    }
    
    pub fn value(&self) -> i32 {
        match self.children.len() {
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
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let node : Node = input.parse().unwrap(); 
        assert_that!(node.metadata_sum()).is_equal_to(138);
        assert_that!(node.value()).is_equal_to(66);
    }

    #[test]
    fn part1and2() {
        let input = read_puzzle_input("day08");
        let node : Node = input.parse().unwrap();
        println!("part1 {}", node.metadata_sum());
        println!("part2 {}", node.value());
    }
}