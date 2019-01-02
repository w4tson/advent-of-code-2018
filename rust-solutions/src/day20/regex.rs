use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use itertools::Itertools;
use std::fmt;
use std::fmt::Debug;


pub trait Regex : fmt::Display + Debug {

    fn max(&self) -> usize;

    fn append(&mut self, c : Box<Regex>);

    fn new_group(&mut self);

    fn str_size(&self) -> usize;
    
    fn all_paths(&self) -> Vec<Vec<char>>;
}


#[derive(Debug)]
pub struct RegexChar {
    c: char
}

impl RegexChar {
    pub fn new(c: char) -> RegexChar {
        RegexChar { c }
    }
}

#[derive(Debug)]
pub struct RegexOr {
    r : Vec<RegexAnd>
}

impl RegexOr {
    pub fn new() -> RegexOr {
        RegexOr { r: vec![RegexAnd::new()] }
    }
}

#[derive(Debug)]
pub struct RegexAnd {
    r : Vec<Box<Regex>>
}

impl RegexAnd {
    pub fn new() -> RegexAnd {
        RegexAnd { r: vec![] }
    }
}

impl Regex for RegexAnd {
    fn max(&self) -> usize {
        let result = self.r.iter()
            .map(|regex| regex.max())
            .sum::<usize>();
////        eprintln!("resut = {:#?}", result);
        result
    }

    fn append(&mut self, c: Box<Regex>) {
//        let mut latest = self.r.last_mut().unwrap_or_else(|| panic!("oops"));
        self.r.push(c);
    }

    fn new_group(&mut self) {
//        unimplemented!()    
    }

    fn str_size(&self) -> usize {
        self.r.iter()
            .map(|regex| regex.str_size())
            .sum::<usize>() 
    }

//        //ENWWW(NEEE|SSE(EE|N))
    fn all_paths(&self) -> Vec<Vec<char>> {
       self.r.iter().fold(vec![vec![]], |mut acc, item| {
//           eprintln!("item = {:#?}", item);
            cross_product(&mut acc, item.all_paths());
//           eprintln!("acc = {:#?}", acc);
            acc
        })
    }

    
}



impl Regex for RegexOr {
    
    fn max(&self) -> usize {
        if let Some(_) = self.r.iter().find(|&group| group.r.len() == 0) {
           return 0; 
        }
        
        let result = self.r.iter()
            .map(|group| group.max())
            .max()
            .unwrap();
//        eprintln!("resut = {:#?}", result);
        result
    }
    
    fn append(&mut self, c: Box<Regex>) {
        let latest = self.r.last_mut().unwrap_or_else(|| panic!("oops2 {:#?}", c));
        latest.append(c)
    }

    fn new_group(&mut self) {
        self.r.push(RegexAnd::new());
    }

    fn str_size(&self) -> usize {
        self.r.iter()
            .map(|group| group.str_size() + 1)
            .sum::<usize>() + 1
    }

    fn all_paths(&self) -> Vec<Vec<char>> {
        if let Some(_) = self.r.iter().find(|&group| group.r.len() == 0) {
            return vec![];
        }
        
        self.r.iter().fold(vec![],|mut acc, alternative|{
            acc.extend(alternative.all_paths());
            acc
        })      
    }

//    }
    
    //ENWWW(NEEE|SSE(EE|N))$
    //E(N|W)S(N|W))$
}

impl Regex for RegexChar {
    fn max(&self) -> usize { 1 }

    fn append(&mut self, c: Box<Regex>) {
        unimplemented!()
    }

    fn new_group(&mut self) {
        unimplemented!()
    }

    fn str_size(&self) -> usize {
        1
    }

    fn all_paths(&self) -> Vec<Vec<char>> {
        vec![vec![self.c]]
    }
}

impl Display for RegexChar {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.c)
    }
}

impl Display for RegexOr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let g = self.r.iter().join("|");
        
        write!(f, "({})", g)
    }
}

impl Display for RegexAnd {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let g = self.r.iter().join("");

        write!(f, "{}", g)
    }
}

fn cross_product(a: &mut Vec<Vec<char>>, b: Vec<Vec<char>>) {
//    eprintln!("cross product {:#?}    {:#?}\n len {}", a, b, b.len());
    for i in 0..a.len() {
        let orig = a[i].clone();
        for (j, new_bit) in b.clone().iter().enumerate() {
            if j == 0 {
                a[i].extend(new_bit);    
            } else {
                let mut new_path = orig.clone();
                new_path.extend(new_bit);
                a.push(new_path)
            }
            
        }
    }

}


#[test]
fn test_cp1() {
    let mut a = vec![vec!['a'], vec!['b']];
    let b = vec![vec!['z']];
    cross_product(&mut a, b);
    assert_eq!(vec![vec!['a','z'], vec!['b','z']], a)
}

#[test]
fn test_cp2() {
    let mut a = vec![vec![]];
    let b = vec![vec!['z']];
    cross_product(&mut a, b);
    assert_eq!(vec![vec!['z']], a) 
}

#[test]
fn test_cp3() {
    let mut a = vec![vec!['E']];
    let b = vec![vec!['N'], vec!['S']];
    cross_product(&mut a, b);
    assert_eq!(vec![vec!['E','N'], vec!['E','S']], a)
}

#[test]
fn test_cp4() {
    let mut a = vec![vec!['E']];
    let b = vec![vec!['N'], vec!['S']];
    cross_product(&mut a, b);
    assert_eq!(vec![vec!['E','N'], vec!['E','S']], a)
}

