mod part1;
mod part2;
#[cfg(test)]
mod tests;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

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

pub fn to_observations(input : &str) -> Vec<Observation> {
    let mut guard = -1;
    
    let mut observations : Vec<Observation> = vec![];
    
    input.lines().for_each(|line| {
        let tokens = line.split_at(17);
        let time_str = tokens.0.to_string();
        let info = tokens.1.to_string();

        let time = NaiveDateTime::parse_from_str(&time_str[1..17], "%Y-%m-%d %H:%M")
            .unwrap_or_else(|e| panic!("Couldn't parse date for line {}", line));
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
                
                observations.push(Observation{
                    guard,
                    time,
                    wake
                });
            }
        }
    });
    
    println!("{:#?}", &observations);
    
    observations
}