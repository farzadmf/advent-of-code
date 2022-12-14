use crate::read_input;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

pub const START: isize = -1;
pub const END: isize = -2;
pub const HIGHEST: isize = 'z' as isize - 'a' as isize;

pub fn get_grid(path: &str) -> (Vec<Vec<isize>>, (isize, isize)) {
    let input = read_input(path);

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => START,
                    'E' => END,
                    _ => c as isize - 'a' as isize,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut start = (0 as isize, 0 as isize);

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == START {
                start = (r as isize, c as isize)
            }
        }
    }

    (grid, start)
}

pub fn get_dist(grid: Vec<Vec<isize>>, start: (isize, isize)) -> usize {
    let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited = HashSet::new();
    let mut visited_map = HashMap::new();
    let (rows, cols) = (grid.len(), grid[0].len());

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (r, c, dist) = queue.pop_back().unwrap();
            let cur = grid[r as usize][c as usize];

            if cur == END {
                return dist;
            }

            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            visited_map.insert((r, c), dist);

            // display_grid(&grid, (r as usize, c as usize, dist), &visited_map);

            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                let (nr, nc) = (r + dr, c + dc);

                if nr >= rows as isize || nr < 0 || nc >= cols as isize || nc < 0 {
                    continue;
                }

                let mut to_go = grid[nr as usize][nc as usize];
                if to_go == END {
                    to_go = HIGHEST;
                }
                if to_go > cur && to_go - cur >= 2 {
                    continue;
                }

                queue.push_front((nr, nc, dist + 1));
            }
        }
    }

    0
}

pub fn display_grid(
    grid: &Vec<Vec<isize>>,
    cur_location: (usize, usize, usize),
    visited_map: &HashMap<(isize, isize), usize>,
) {
    let (rows, cols) = (grid.len(), grid[0].len());

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == START {
                print!("{:>4}", "S");
            } else if grid[r][c] == END {
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
            if grid[r][c] == START {
                print!("{:>4}", "S");
            } else if grid[r][c] == END {
                print!("{:>4}", "E");
            } else if (r, c) == (cur_location.0, cur_location.1) {
                print!("{:>4}", cur_location.2);
            } else if visited_map.contains_key(&(r as isize, c as isize)) {
                print!("{:>4}", visited_map.get(&(r as isize, c as isize)).unwrap());
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
