use rust::day04::part02;
use std::fs;

fn main() {
    let file = fs::read_to_string("../input/day04").unwrap();

    println!("+++++++++++++++++++++++ PART 02 ++++++++++++++++++++++++++++");
    println!("{}", part02(file.trim()));
    println!("----------------------- PART 02 ----------------------------");
}
