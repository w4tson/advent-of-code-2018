pub mod parse;
use crate::day17::parse::*;

pub struct Slice {
    map : Vec<Vec<char>>
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Error;

    #[test]
    fn test() {
        let input = include_str!("test_data");
        let slice : Result<Slice, Error> = input.parse();
    }
}