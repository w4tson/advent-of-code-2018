#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Action {
    ShiftStart(usize),
    Asleep,
    Wake,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Event {
    date: Date,
    action: Action,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
}

mod parsers {
    use nom::*;
    use super::{Action, Date, Event};
    use std::str::FromStr;
    
    named!(event(&str) -> Event,
    do_parse!(
        date: date >>
        tag!(" ") >>
        action: alt!(
            value!(Action::Asleep, tag!("falls asleep")) |
            value!(Action::Wake, tag!("wakes up")) |
            map!(
                delimited!(tag!("Guard "), guard_id, tag!(" begins shift")),
                Action::ShiftStart
                )
            ) >>
        (Event { date, action })
        )
    );

    named!(date(&str) -> Date,
    do_parse!(
        tag!("[") >>
        year: map_res!(digit, usize::from_str) >>
        tag!("-") >>
        month: map_res!(digit, usize::from_str) >>
        tag!("-") >>
        day: map_res!(digit, usize::from_str) >>
        tag!(" ") >>
        hour: map_res!(digit, usize::from_str) >>
        tag!(":") >>
        min: map_res!(digit, usize::from_str) >>
        tag!("]") >>
        (Date { year, month, day, hour, min })
        )
    );

    named!(guard_id(&str) -> usize,
    do_parse!(
        tag!("#") >>
        id: map_res!(digit, usize::from_str) >>
        (id)
        )
    );

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl FromStr for Event {
        type Err = ParseError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match event(input) {
                Ok(("", event)) => Ok(event),
                _ => Err(ParseError),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::file::read_puzzle_input;
    use crate::day05::foo::Event;

    #[test]
    fn test() {
        
        let input = read_puzzle_input("day04");
        let events : Result<Vec<Event>, _> = input
            .lines()
            .map(|line| line.parse())
            .take(10)
            .collect();
        
        for e in events {
            println!("{:#?}", e);
        }
    }
}