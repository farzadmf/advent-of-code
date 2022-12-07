use day06_rust::{get_packet_start, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input");

    input.lines().for_each(|line| {
        let start = get_packet_start(line);
        println!("got {}", start);
    });

    println!("------------------- PART 1 -------------------");
}
