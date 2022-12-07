use day06_rust::{get_start, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let input = read_input("input");

    input.lines().for_each(|line| {
        let start = get_start(line, 14);
        println!("got {}", start);
    });

    println!("------------------- PART 2 -------------------");
}
