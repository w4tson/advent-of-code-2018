use std::collections::HashMap;

#[cfg(test)]
mod tests;
mod part1;

#[derive(Debug)]
struct State {
    seen: HashMap<char, i32>,
    
}

struct Checksum {
    two : i32,
    three : i32
}

impl State {
    pub fn new() -> State {
        let mut s = HashMap::new();
        State {
            seen: s
        }
    }

    
    pub fn next(&self, c: char) -> State {
        let mut seen = self.seen.clone(); 
        *seen.entry(c).or_insert(0) += 1;
        State { seen }
    }
    
    pub fn checksum(&self) -> Checksum {
        let two = self.seen
            .values()
            .find(|&x | *x == 2)
            .map(|_| 1)
            .unwrap_or(0);

        let three = self.seen
            .values()
            .find(|&x | *x == 3)
            .map(|_| 1)
            .unwrap_or(0);
        
        Checksum { two, three }
    }
}