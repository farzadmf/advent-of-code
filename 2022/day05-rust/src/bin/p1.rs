use std::collections::VecDeque;

use day05_rust::{get_stack_count, read_input, Crate, Move};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");
    let result = 0;

    // Let's do this!
    let input = read_input("input_small");

    let (stack_lines, move_lines): (Vec<_>, Vec<_>) =
        input.lines().partition(|l| !l.to_string().contains("move"));

    let stack_count = get_stack_count(&input[..]);
    let stacks: Vec<VecDeque<Crate>> = Vec::new();

    for line in stack_lines.as_slice()[..stack_lines.len() - 2].to_vec() {
        let crates = Crate::from_line(line);
        println!("{:?}", crates);
    }

    let moves = Move::from_lines(move_lines);
    // stacks.into_iter().for_each(|line| {
    //     let crates = Crate::from_line(line);

    //     for c in crates {
    //         dbg!(c);
    //     }
    // });

    // input
    //     .lines()
    //     .partition(|line| line.to_string() == "")
    //     .
    //     .take_while(|line| line.trim().chars().nth(0).unwrap() != '1')
    //     .map(|line| line.split(" ").collect::<Vec<&str>>())
    //     .for_each(|a| println!("{:?}", a));

    // for line in input.lines().take_while(|line| line.to_string() != "") {
    //     let parts = line.trim().split(" ").collect::<Vec<&str>>();
    //     let crates = parts
    //         .into_iter()
    //         .map(|p| p.parse::<Crate>().unwrap())
    //         .collect::<Vec<Crate>>();
    //     dbg!(crates);
    // }

    println!("result: {}", result);
    println!("------------------- PART 1 -------------------");
}
