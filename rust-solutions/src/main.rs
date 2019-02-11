use aoc2018::utils::file::read_puzzle_input;
use aoc2018::day22::Cave;

fn main() {
    let mut cave = Cave::new( 11817, (9,751));
    println!("dist to target {:#?}", cave.dist_to_target());
}
    