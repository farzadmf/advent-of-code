use rust::day02::part2;
use std::fs;

fn main() {
    // let input = fs::read_to_string("input").expect("error reading file");
    let input = fs::read_to_string("input/day02").expect("error reading file");
    part2(&input);
}
