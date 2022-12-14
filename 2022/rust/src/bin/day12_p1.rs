use rust::day12::{get_dist, get_grid};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let (grid, start) = get_grid("input/day12");

    let result = get_dist(grid, start);

    println!("result: {}", result);

    println!("------------------- PART 1 -------------------");
}
