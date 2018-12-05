use crate::day04::to_observations;
use std::collections::HashMap;
use crate::day04::Wake;
use chrono::Timelike;

#[derive(Debug)]
struct State {
    curr_total: i32,
    sleep: HashMap<i32, i32>
    
}

pub fn solve_part1(input : &str) -> i32 {
    let observations = to_observations(input);
    let guard_sleep : HashMap<i32, i32> = HashMap::new();
    let result = observations.windows(2)
        .fold(State{ curr_total: 0, sleep: HashMap::new() }, |acc, obv| {
           let mut mut_sleep = acc;
           let previous = &obv[0]; 
           let current = &obv[1]; 
            
           if previous.guard == current.guard {
               if current.wake == Wake::Awake {
                   let nap = current.time.minute() - previous.time.minute();
                   println!("Guard {} naps for {} minutes", current.guard, nap);
                   mut_sleep.curr_total += nap as i32;

               }
           } else {
               if previous.wake == Wake::Asleep {
                   let nap = 60 - previous.time.minute();
                   mut_sleep.curr_total += nap as i32;
               }

               *mut_sleep.sleep.entry(previous.guard)
                   .or_default() += mut_sleep.curr_total;
               
           }

            mut_sleep
        });
    
    println!("{:#?}", &result.sleep);
    
    240
}