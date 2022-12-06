use day05_rust::{read_input, Crate};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");
    let result = 0;

    // Let's do this!
    let input = read_input("input_small");

    let (stacks, moves): (Vec<_>, Vec<_>) =
        input.lines().partition(|l| !l.to_string().contains("move"));

    stacks
        .into_iter()
        .for_each(|line| {
            line.chars().into::<Vec<char>>().un
        })

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
