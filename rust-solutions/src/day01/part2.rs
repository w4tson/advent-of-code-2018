use crate::day01::to_freq;
use std::collections::HashSet;


pub fn solve_part2(input : &str) -> i32 {
    input.lines().map(to_freq)
        .cycle()
        .scan(State::new(), |state, change| Some(state.next(&change)))
        .filter(Option::is_some)
        .take(1)
        .inspect(|&x| println!("{:#?}", x))
        .last()
        .unwrap()
        .unwrap()
}

struct State {
    seen: HashSet<i32>,
    last: i32
}

impl State {
    pub fn new() -> State {
        let mut s = HashSet::new();
        s.insert(0);
        State {
            seen: s,
            last: 0
        }
    }
    
    ///
    /// Takes the change and updates the last to give the 
    /// most current frequency. Returns an Optional of the 
    /// duplicated frequencies, None if the frequency is unique
    pub fn next(&mut self, change: &i32) -> Option<i32> {
        self.last += change;
        match self.seen.insert(self.last) {
            false => Some(self.last),
            _ => None
        }
    }
}
