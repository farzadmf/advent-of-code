use rust::day14::{display_grid, get_lines, get_rocks};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let lines = get_lines("input/day14_small");
    let rocks = get_rocks(&lines);

    display_grid(&rocks);

    println!("------------------- PART 1 -------------------");
}
