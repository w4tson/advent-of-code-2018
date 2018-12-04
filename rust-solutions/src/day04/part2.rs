use crate::day04::State;
use std::collections::HashMap;

pub fn solve_part2(state : State) -> i32 {
    let (g, m, _) = state.sleep.iter()
        .map(|(k, mins)| {
            let (m, c) = mode(mins);
            (k, m, c)
        })
        .max_by_key(|&(_, _, c)| c)
        .unwrap_or_else(|| panic!("adsf"));

    println!("{} x {}= {}", g, m, g*m);

    g * m

}



fn mode(numbers: &[i32]) -> (i32, i32) {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .expect("Cannot compute the mode of zero numbers")
}
