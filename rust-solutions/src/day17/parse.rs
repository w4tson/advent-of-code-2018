use std::str::FromStr;
use crate::day17::CrossSection;
use std::fmt::Error;
use std::ops::Range;
use itertools::Itertools;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::day17::FromTo;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

//x=495, y=2..7
//y=7, x=495..501
//x=501, y=3..7
//x=498, y=2..4
//x=506, y=1..2
//x=498, y=10..13
//x=504, y=10..13
//y=13, x=498..504


impl FromTo {
    pub fn range(&self) -> Range<usize> {
        Range{ start: self.start, end: self.end+1 }
    }
    
    pub fn count(&self) -> usize {
        self.end + 1 - self.start
    }
}

struct ClayArea {
    x: FromTo,
    y: FromTo
}

impl FromStr for CrossSection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let areas = s.lines().map(|line| to_clay_area(line)).collect_vec();
        
        let minx = areas.iter().min_by_key(|area| area.x.start).unwrap().x.start -1;
        let maxx = areas.iter().max_by_key(|area| area.x.end).unwrap().x.end +1;
        let miny = areas.iter().min_by_key(|area| area.y.start).unwrap().y.start;
        let maxy = areas.iter().max_by_key(|area| area.y.end).unwrap().y.end;

        let width = FromTo{ start: minx, end: maxx  };
        let height = FromTo{ start: miny, end: maxy  };

        let w = width.count();
        let h = height.count();
        let map = vec![vec!['.'; w]; h];
        
        
        println!("{}..{} {}",minx, maxx, w);
        println!("{}..{} {}",miny, maxy, h);
        
        let mut map = areas.iter().fold(map, | mut m, area| {
            for y in area.y.range() {
                for x in area.x.range() {
                    m[y-height.start][x-width.start] = '#';
                }
            }
            m
        });
        
//        map[0][500-width.start] = '+';
        
        
        Ok(CrossSection { map, width, height, stream: VecDeque::new(), seen: HashMap::new() })
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

fn to_range(s: &str) -> FromTo {
    if s.contains("..") {
        let mut from_to = s.split("..");
        let from : usize = from_to.next().unwrap().parse().unwrap_or_else(|_| panic!("Couldn't parse from {}", s));
        let to : usize = from_to.next().unwrap().parse().unwrap_or_else(|_| panic!("Couldn't parse to   {}", s));
        FromTo { start: from, end: to  }

    } else {
        let num: usize = s.parse().unwrap_or_else(|_| panic!("Couldn't parse single reuslt  {}", s));
        FromTo { start: num, end: num }
    }
}

impl Display for CrossSection {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.height.count() {
            for x in 0..self.width.count() {
                write!(f, "{}", self.map[y][x]);
            }
            writeln!(f);
        }
        
        write!(f,"")
    }
}

