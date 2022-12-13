use std::collections::HashSet;
use std::io::stdin;

pub fn g2g(grid: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    if row >= rows || row < 0 || col >= cols || col < 0 {
        return false;
    }

    true
    // let to_go = grid[row][col];
    // if to_go == -10 {
    //     return
    // }
}

pub fn display_grid(
    grid: &Vec<Vec<i32>>,
    cur_location: (usize, usize, usize),
    visited: &HashSet<(i32, i32)>,
) {
    let (rows, cols) = (grid.len(), grid[0].len());

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == -1 {
                print!("{:>4}", "S");
            } else if grid[r][c] == -10 {
                print!("{:>4}", "E");
            } else {
                print!("{:>4}", grid[r][c]);
            }
        }

        println!();
    }
    println!("----------------------------------");

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == -1 {
                print!("{:>4}", "S");
            } else if grid[r][c] == -10 {
                print!("{:>4}", "E");
            } else if (r, c) == (cur_location.0, cur_location.1) {
                print!("{:>4}", cur_location.2);
            } else if visited.contains(&(r as i32, c as i32)) {
                print!("{:>4}", "-");
            } else {
                print!("{:>4}", ".");
            }
        }

        println!();
    }
    println!("==================================");

    let mut hello = String::new();
    stdin().read_line(&mut hello).ok();
}
