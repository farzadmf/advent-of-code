use rust::day12::{get_dist, get_grid, get_start};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let grid = get_grid("input/day12");
    let start = get_start(&grid);

    let result = get_dist(grid, start);

    println!("result: {}", result);

    println!("------------------- PART 1 -------------------");
}
