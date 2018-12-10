use std::str::FromStr;
use crate::day08::Node;
use std::fmt::Error;
use std::prelude::v1::Vec;

pub fn create_node(data: &[i32]) -> Node {
    let (children_size, metadata_size) = (data[0], data[1] as usize);
    let data = &data[2..];
        
    match children_size {
        0 => Node::new(vec![], data[0..metadata_size].to_vec()),
        _ => { 
            let (offset, children) = (0..children_size)
                .fold((0, vec![]),|(offset, children) , _ | {
                    let child = create_node(&data[offset..]);
                    (offset + child.size(), [&children[..], &[child]].concat())
                });
            
            let metadata = data[offset..offset+metadata_size].to_vec();

            Node::new(children, metadata)
        }
    }
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let data :Vec<i32> = s.to_string()
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
            
        Ok(create_node(&data))
    }
}