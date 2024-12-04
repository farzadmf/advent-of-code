use rust::day04::part01;
use std::fs;

fn main() {
    let file = fs::read_to_string("../input/day04").unwrap();

    println!("+++++++++++++++++++++++ PART 01 ++++++++++++++++++++++++++++");
    println!("{}", part01(&file));
    println!("----------------------- PART 01 ----------------------------");
}
