use std::str::FromStr;
use crate::day08::Node;
use std::fmt::Error;
use std::prelude::v1::Vec;

// 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2


//
//
// 3
// 2



pub fn create_node(data: &[i32]) -> Node {
    
    let children_size: usize = *data.get(0).unwrap() as usize;
    let meta : usize = *data.get(1).unwrap() as usize;
    let data = &data[2..];
        
    match children_size {
        0 => {
            let meta = data[0..meta].to_vec();
            Node::new_childless(meta)   
        },
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
            .map(to_int)
            .collect();
        
        let tree = create_node(&data);
        
        
        
        println!("{}", tree.total_metadata());
        println!("{}", tree.value());
        println!("{}", tree);
            
        Ok(Node { metadata_size: 0, children_size:0, children: vec![], metadata: vec![] })
    }
}

fn to_int(s: &str) -> i32 {
    s.parse().unwrap_or(0)
}