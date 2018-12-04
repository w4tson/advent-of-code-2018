use crate::day04::to_observations;
use std::collections::HashMap;
use crate::day04::Wake;
use crate::day04::State;


pub fn solve_part1(result : State) -> i32 {
    
    println!("sleep = {:#?}", result.sleep);

    let (guard, sleep_minutes) = result.sleep
        .iter()
        .max_by_key(|&(_, mins)| mins.len())
        .unwrap_or_else(|| panic!("Couldn't find a max"));
    let popular_minute = mode(sleep_minutes);
    println!("Guard {} = slept for {} minutes total. {}", guard, sleep_minutes.len(), popular_minute);

        guard * mode(sleep_minutes)
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

