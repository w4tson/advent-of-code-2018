use std::collections::HashMap;
use std::hash::Hash;

pub mod file;

pub fn to_uint(s : char) -> u32 {
    s.to_digit(10).unwrap()
}

pub fn dupes<T : Eq + Hash + Copy>(numbers: &Vec<T>) -> Vec<T> {
    let mut occurrences:HashMap<T, i32> = HashMap::new();

    for value in numbers {
        *occurrences.entry(*value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .filter(|(_, v)| v>&1) 
        .map(|(val, _)| val)
        .collect()
}