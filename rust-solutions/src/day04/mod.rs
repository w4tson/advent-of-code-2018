mod part1;
mod part2;
#[cfg(test)]
mod tests;

use chrono::NaiveDateTime;
use std::collections::HashMap;
use chrono::Timelike;

//[1518-11-01 23:58] Guard #99 begins shift
//[1518-04-18 00:32] falls asleep
#[derive(Debug)]
pub struct Observation {
    time : NaiveDateTime,
    guard: i32,
    wake : Wake
}

#[derive(PartialEq, Clone, Debug)]
enum Wake {
    Asleep,
    Awake
}

#[derive(Debug)]
pub struct State {
    sleep: HashMap<i32, Vec<i32>>
}

pub fn to_state(input : &str) -> State {
    to_observations(input)
        .windows(2)
        .fold(State{ sleep: HashMap::new() }, |acc, obv| {
            let mut mut_sleep = acc;
            let previous = &obv[0];
            let current = &obv[1];

            if previous.guard == current.guard {
                println!("guards equal");
                if current.wake == Wake::Awake {
                    for minute in previous.time.minute()..current.time.minute()  {
                        mut_sleep.sleep
                            .entry(previous.guard)
                            .or_default()
                            .push(minute as i32);
                    }

                }
            }

            mut_sleep
        })
}

pub fn to_observations(input : &str) -> Vec<Observation> {
    let mut guard = -1;
    
    let mut observations : Vec<Observation> = vec![];
    
    input.lines().for_each(|line| {
        let tokens = line.split_at(17);
        let time_str = tokens.0.to_string();
        let info = tokens.1.to_string();

        let time = NaiveDateTime::parse_from_str(&time_str[1..17], "%Y-%m-%d %H:%M")
            .unwrap_or_else(|_| panic!("Couldn't parse date for line {}", line));
//    println!("{}", time);
//    println!("{}", &info);


        match &info[0..9] {
            "] Guard #" => {
                guard = *&info[9..].split(" ").next().map(|m| m.parse().unwrap_or(0))
                    .unwrap_or_else(|| panic!("Problem parsing guard {}", line));
                println!("Guard changed {}", guard);
            }
            _ => {
                //[1518-04-18 00:32] falls asleep
                let wake = match &info[2..7] {
                    "wakes" => Wake::Awake,
                    _ => Wake::Asleep
                };

                println!("push {} {}", guard, &info[0..9]);
                
                observations.push(Observation{
                    guard,
                    time,
                    wake
                });
            }
        }
    });
    
    observations.sort_by_key(|x| x.time.timestamp_millis());
    println!("{:#?}", &observations);

    observations
}