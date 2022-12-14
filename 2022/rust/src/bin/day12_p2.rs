use rust::day12::{get_dist, get_grid, get_starts};

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let grid = get_grid("input/day12");
    let starts = get_starts(&grid);

    let mut min_dist = usize::MAX;

    for start in starts {
        let dist = get_dist(&grid, start);
        min_dist = min_dist.min(dist);
    }

    println!("min dist: {}", min_dist);

    println!("------------------- PART 2 -------------------");
}
