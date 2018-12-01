pub mod part1;
pub mod part2;
#[cfg(test)]
pub mod tests;
use regex::Regex;



fn to_freq(s: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\+-]+)(\d+)$").unwrap();
    }

    let cap = RE.captures_iter(s).next().unwrap();

    let _postive = "+".to_string();

    let _sign_str = cap.get(1).map_or("".to_string(), |m| m.as_str().to_string());
    let sign = match _sign_str.as_ref() {
        "+" => 1,
        _ => -1
    };
    let value: i32 = cap.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));

    sign * value
}
