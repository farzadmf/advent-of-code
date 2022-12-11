use rust::day02::part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("input/day02").unwrap();
    part1(&file);
}
