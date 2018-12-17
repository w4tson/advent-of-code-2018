use std::str::FromStr;
use crate::day17::Slice;
use std::fmt::Error;
use std::ops::Range;
use itertools::Itertools;

//x=495, y=2..7
//y=7, x=495..501
//x=501, y=3..7
//x=498, y=2..4
//x=506, y=1..2
//x=498, y=10..13
//x=504, y=10..13
//y=13, x=498..504

struct ClayArea {
    x: Range<usize>,
    y: Range<usize>
}

impl FromStr for Slice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        s.lines().map(|line| to_clay_area(line)).collect_vec();
        
        Ok(Slice{ map: vec![] })
    }
}

fn to_clay_area(line: &str) -> ClayArea {
    let mut xy = line.split(", ");

    let (xory, range_str1)= xy.next().unwrap().split_at(1);
    let (_, range_str2)= xy.next().unwrap().split_at(1);
    let range_str1  = &range_str1[1..];
    let range_str2 = &range_str2[1..];

//    let range1 = to_range(range_str1);
//    let range2 = to_range(range_str2);

//
//    println!("{}={} ({:#?})", xory1, range_str1, range1);
//    for x in range1 { print!("{} ",x); }; println!();
//
//
//    println!("{}={}", xory2, range_str2);
//    for x in range2 { print!("{} ",x); }; println!();

    match xory {
        "x" => ClayArea { x: to_range(range_str1), y: to_range(range_str2) },
        _ => ClayArea { x: to_range(range_str2), y: to_range(range_str1) }
    }
}

fn to_range(s: &str) -> Range<usize> {
    if s.contains("..") {
        let mut from_to = s.split("..");
        let from : usize = from_to.next().unwrap().parse().unwrap_or_else(|_| panic!("Couldn't parse from {}", s));
        let to : usize = from_to.next().unwrap().parse().unwrap_or_else(|_| panic!("Couldn't parse to   {}", s));
        Range { start: from, end: to + 1 }

    } else {
        let num: usize = s.parse().unwrap_or_else(|_| panic!("Couldn't parse single reuslt  {}", s));
        Range { start: num, end: num +1 }
    }
}

