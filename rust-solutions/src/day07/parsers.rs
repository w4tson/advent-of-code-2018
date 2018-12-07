    use nom::*;
    use std::str::FromStr;
    use std::collections::HashMap;
    use core::fmt::Error;
    use itertools::Itertools;
    use std::prelude::v1::Vec;

    type GraphMap = HashMap<char, Vec<char>>;
    
    #[derive(Debug)]
    pub struct Graph {
        data: GraphMap
    }
    
    impl Graph {

        pub fn roots(&self) -> Vec<char> {
           self.data
                .keys()
                .filter(|&step| Graph::has_incoming_connections(&self.data, *step))
                .map(|&x| x)
                .collect()
        }
        
        fn has_incoming_connections(graph: &GraphMap, vertex: char) -> bool {
            graph.values()
                .flat_map(|x| x)
                .find(|&v| *v == vertex)
                .is_some()
        }

//              -->A--->B--
//            /    \        \
//          C       -->D----->E
//            \             /
//              ---->F-----  }
        
        pub fn traverse(&self) -> String {
            let mut graph = self.data.clone();
            let mut L  : Vec<char> = vec![];       // contains the sorted result 
            let mut S  : Vec<char> = self.roots(); // no incoming edge
            
            println!("{:#?}", self.roots());
            
            while !S.is_empty() {
                println!("S {:#?}", &S);
                Graph::sort_rev(&mut S);

                let n = S.pop().unwrap();
                L.push(n);
//                println!("considering {}", n);
                let mut m_nodes = graph.remove(&n).unwrap_or(vec![]);
                Graph::sort_rev(&mut m_nodes);
//                println!("m_nodes = {:#?}", m_nodes);
                for m in m_nodes {
                    if !Graph::has_incoming_connections(&graph, m) {
                        S.push(m);
//                        println!("pushing {} to S", m);
                    }
                }
            }
            
            println!("Graph = {:#?} ", &graph);
            println!("L = {:#?} ", &L);
            
            L.iter().collect()
        }
        
        fn sort_rev(v : &mut Vec<char>) {
            v.sort_unstable();
            v.reverse();
        }
    }
    
    named!(to_pair(&str) -> (char, char),
    do_parse!(
        tag!("Step ") >>
        from: verify!(nom::anychar, |c: char| c.is_alphabetic()) >>
        tag!(" must be finished before step ") >>
        to: verify!(nom::anychar, |c: char| c.is_alphabetic()) >>
        tag!(" can begin.") >>
        
        ( (from, to) )
        )
    );

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl FromStr for Graph {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
            let mut data: GraphMap = HashMap::new();
            for (from, to) in s.lines().map(|line | to_pair(&line).unwrap().1) {
                data.entry(from)
                    .or_default()
                    .push(to);
            }
           let the_graph = Graph { data };
            
            println!("{:#?}", the_graph);
            Ok(the_graph)
            
        }
    }
