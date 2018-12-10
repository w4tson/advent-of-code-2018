


    use nom::*;
    use std::str::FromStr;
    use std::fmt::Error;
//    
//    named!(event(&str) -> Event,
//    do_parse!(
//        date: date >>
//        tag!(" ") >>
//        action: alt!(
//            value!(Action::Asleep, tag!("falls asleep")) |
//            value!(Action::Wake, tag!("wakes up")) |
//            map!(
//                delimited!(tag!("Guard "), guard_id, tag!(" begins shift")),
//                Action::ShiftStart
//                )
//            ) >>
//        (Event { date, action })
//        )
//    );
//
//    named!(date(&str) -> Date,
//    do_parse!(
//        tag!("[") >>
//        year: map_res!(digit, usize::from_str) >>
//        tag!("-") >>
//        month: map_res!(digit, usize::from_str) >>
//        tag!("-") >>
//        day: map_res!(digit, usize::from_str) >>
//        tag!(" ") >>
//        hour: map_res!(digit, usize::from_str) >>
//        tag!(":") >>
//        min: map_res!(digit, usize::from_str) >>
//        tag!("]") >>
//        (Date { year, month, day, hour, min })
//        )
//    );
//
//    named!(guard_id(&str) -> usize,
//    do_parse!(
//        tag!("#") >>
//        id: map_res!(digit, usize::from_str) >>
//        (id)
//        )
//    );
//
//    #[derive(Debug, Clone)]
//    pub struct ParseError;
//
//    impl FromStr for Event {
//        type Err = ParseError;
//
//        fn from_str(input: &str) -> Result<Self, Self::Err> {
//            match event(input) {
//                Ok(("", event)) => Ok(event),
//                _ => Err(ParseError),
//            }
//        }
//    }
//}
    //#4 @ 225,790: 27x10

named!(int32 <&str, i32>,
    map_res!(digit, FromStr::from_str)
);

named!(
    integer<i32>,
    map_res!(
      map_res!(nom::digit, std::str::from_utf8),
      |s: &str| s.parse::<i32>()
    )
);
    
named!(fabric(&str) -> Fabric,
    do_parse!(
        tag!("#") >>
        id: int32 >>
        tag!(" @ ") >>
        left: int32 >>
        tag!(",") >>
        top: int32 >>
        tag!(": ") >>
        width: int32 >>
        tag!("x") >>
        _height: complete!(int32) >>
        ( Fabric { id, top, left, width, height: 0 } )
    )
);

#[derive(Debug)]
struct Fabric {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32
}

impl FromStr for Fabric {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Fabric { id: 0, top:0, left: 0, width: 0, height: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "#4 @ 225,790: 27x10";
        let result = fabric(&input);
        println!("{:#?}", result);
//        let result = input.parse().unwrap();
        
        
    }
}