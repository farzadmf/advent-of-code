use rust::day03::{get_code, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let input = read_input("input/day03".to_string());

    let result = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|lines| {
            lines[0]
                .chars()
                .find(|&ch| lines[1].contains(ch) && lines[2].contains(ch))
        })
        .flat_map(get_code)
        .sum::<u32>();

    println!("result: {:?}", result);

    println!("------------------- PART 2 -------------------");
}
