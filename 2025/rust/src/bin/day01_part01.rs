use rust::day01::part01;
use std::fs;

fn main() {
    let file = fs::read_to_string("../input/day01").unwrap();

    println!("+++++++++++++++++++++++ PART 01 ++++++++++++++++++++++++++++");
    println!("{}", part01(file.trim()));
    println!("----------------------- PART 01 ----------------------------");
}
