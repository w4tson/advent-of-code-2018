use std::collections::HashMap;
use std::prelude::v1::Vec;
use super::worker::Worker;

type GraphMap = HashMap<char, Vec<char>>;

#[derive(Debug)]
pub struct Graph {
    data: GraphMap
}

impl Graph {
    
    pub fn new(data: GraphMap) -> Graph {
        Graph { data }
    }

    pub fn entry_points(&self) -> Vec<char> {
       self.data
            .keys()
            .filter(|&step| !Graph::has_incoming_connections(&self.data, *step))
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
//              ---->F-----  
    
    pub fn traverse(&self) -> String {
        let mut graph = self.data.clone();
        let mut L  : Vec<char> = vec![];              // contains the sorted result 
        let mut S  : Vec<char> = self.entry_points(); // no incoming edge
        
        println!("{:#?}", self.entry_points());
        
        while !S.is_empty() {
            println!("S {:#?}", &S);
            Graph::sort_rev(&mut S);

            let n = S.pop().unwrap();
            L.push(n);
            let mut m_nodes = graph.remove(&n).unwrap_or(vec![]);
            Graph::sort_rev(&mut m_nodes);
            for m in m_nodes {
                if !Graph::has_incoming_connections(&graph, m) {
                    S.push(m);
                }
            }
        }
        
        L.iter().collect()
    }
    
    pub fn traverse_concurrent(&self, concurrency: usize, step: usize) -> String {
        let mut graph = self.data.clone();
        let mut L  : Vec<char> = vec![];       // contains the sorted result 
        let mut S  : Vec<char> = self.entry_points(); // no incoming edge
        let mut time : usize = 0;
        let mut workers = vec![Worker::new(step); concurrency];

        println!("Entry points = {:#?}", self.entry_points());
        
        while !S.is_empty() || workers.iter().find(|&w| !w.is_free()).is_some() {
            
            workers
                .iter_mut()
                .enumerate()
                .for_each(|(i, mut worker)| {
                    
                    if let Some(n) = worker.do_work() {
                        L.push(n);
                        let mut m_nodes = graph.remove(&n).unwrap_or(vec![]);
                        Graph::sort_rev(&mut m_nodes);
                        for m in m_nodes {
                            if !Graph::has_incoming_connections(&graph, m) {
                                S.push(m);
                            }
                        }
                    }
                });

            //Dont forget to sort again!
            Graph::sort_rev(&mut S);
            
            workers
                .iter_mut()
                .enumerate()
                .for_each(|(i, mut worker)| {
                    if worker.is_free() {
                        if let Some(n) = S.pop() {
                            worker.accept(n);
                        }
                    }
                });

            
            print!("t={:3}\t\t", time);
            for worker in workers.iter() {
                let item = match worker.current {
                    Some(ch) => ch.to_string(),
                    _ => ".".to_string()
                };
                print!("{}\t\t", item);
            }
            let intermediate : String = L.iter().collect();
            println!("{}", intermediate);
            
            time += 1;
        }

        L.iter().collect()
    }
    
    fn sort_rev(v : &mut Vec<char>) {
        v.sort_unstable();
        v.reverse();
    }
}

