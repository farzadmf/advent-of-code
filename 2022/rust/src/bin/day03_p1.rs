use rust::day03::{get_code, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input/day03".to_string());

    // From here: https://bit.ly/3F3fv3R
    let result = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(first, second)| first.chars().find(|&c| second.contains(c)))
        .flat_map(get_code)
        .sum::<u32>();

    println!("result: {}", result);

    println!("------------------- PART 1 -------------------");
}
