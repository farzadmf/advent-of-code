use rust::day04::{get_points, overlaps, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let input = read_input("input/day04".to_string());

    let result = input
        .lines()
        .map(|line| get_points(line))
        .map(|coords| overlaps(coords))
        .sum::<u32>();

    println!("result {}", result);

    println!("------------------- PART 2 -------------------");
}
