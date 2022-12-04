use std::collections::HashSet;

use day03_rust::{get_code, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input".to_string());
    let result: u32 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            let first_chars: HashSet<char> = first.chars().collect();
            let second_chars: HashSet<char> = second.chars().collect();

            let shared = first_chars.intersection(&second_chars).nth(0).unwrap();
            get_code(*shared).unwrap()
        })
        .sum();

    println!("result: {}", result);

    println!("------------------- PART 1 -------------------");
}
