use day02_rust::part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    part1(&file);
}
