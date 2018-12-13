extern crate aoc2018;

use aoc2018::utils::file::read_puzzle_input;
use std::time;
use std::thread;
use aoc2018::day13::Track;


fn main() {

    let input = read_puzzle_input("day13");
    let mut track: Track = input.parse()
        .unwrap_or_else(|_| panic!("Couldn't parse {}", input));

    track.println_it();

    while track.has_mulitple_carts() {
        print!("{}[2J", 27 as char);
        track.move_carts();
        track.println_it();

        let ten_millis = time::Duration::from_millis(1000);

        thread::sleep(ten_millis);
    }
}
