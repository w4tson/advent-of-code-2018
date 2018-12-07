use nom::*;
use std::str::FromStr;
use std::collections::HashMap;
use core::fmt::Error;
use itertools::Itertools;
use std::prelude::v1::Vec;
use crate::day07::graph::Graph;

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
        let mut data: HashMap<char, Vec<char>> = HashMap::new();
        for (from, to) in s.lines().map(|line | to_pair(&line).unwrap().1) {
            data.entry(from)
                .or_default()
                .push(to);
        }
        let the_graph = Graph::new(data);

        println!("{:#?}", the_graph);
        Ok(the_graph)

    }
}