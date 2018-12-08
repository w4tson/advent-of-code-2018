use std::str::FromStr;
use crate::day08::Node;
use std::fmt::Error;
use std::prelude::v1::Vec;

pub fn create_node(data: &[i32]) -> Node {
    let (children_size, meta) = (data[0], data[1] as usize);
    let data = &data[2..];
        
    match children_size {
        0 => Node::new(vec![], data[0..meta].to_vec()),
        _ => { 
            let mut node_size = 0;
            let mut children = vec![];
            for i in 0..children_size {
                let n = create_node(&data[node_size..]);
                node_size += n.size();
                children.push(n);
            }
            let meta = data[node_size..node_size+meta].to_vec();


            Node::new(children, meta)
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