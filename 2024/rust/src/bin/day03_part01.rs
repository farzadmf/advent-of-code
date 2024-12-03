use rust::day03::part01;
use std::fs;

fn main() {
    let file = fs::read_to_string("../input/day03").unwrap();

    println!("+++++++++++++++++++++++ PART 01 ++++++++++++++++++++++++++++");
    println!("{}", part01(&file));
    println!("----------------------- PART 01 ----------------------------");
}
