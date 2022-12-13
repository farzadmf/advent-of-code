use std::collections::{HashSet, VecDeque};

use rust::day12::display_grid;
use rust::read_input;

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input/day12_small");

    let mapped = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_lowercase() {
                        c as i32 - 'a' as i32
                    } else if c == 'S' {
                        -1
                    } else {
                        -10
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (rows, cols) = (mapped.len(), mapped[0].len());
    let mut start = (0 as i32, 0 as i32);

    for r in 0..rows {
        for c in 0..cols {
            if mapped[r][c] == -1 {
                start = (r as i32, c as i32)
            }
        }
    }

    let mut queue: VecDeque<(i32, i32, usize)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited = HashSet::new();

    let mut found = false;

    while !queue.is_empty() {
        if found {
            break;
        }

        for _ in 0..queue.len() {
            let (r, c, dist) = queue.pop_back().unwrap();
            let cur = mapped[r as usize][c as usize];

            if cur == -10 {
                println!("cur {} dist {}", cur, dist);
                found = true;
                break;
            }

            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));

            display_grid(&mapped, (r as usize, c as usize, dist), &visited);

            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                let (nr, nc) = (r + dr, c + dc);

                if nr >= rows as i32 || nr < 0 || nc >= cols as i32 || nc < 0 {
                    continue;
                }

                let to_go = mapped[nr as usize][nc as usize];
                let dest_elevation: i32;
                if to_go == -10 {
                    dest_elevation = 'z' as i32 - 'a' as i32;
                } else {
                    dest_elevation = to_go;
                }

                println!("{}", 'z' as i32 - 'a' as i32);
                // println!("dest elev: {}", dest_elevation);

                if dest_elevation > cur && dest_elevation - cur > 2 {
                    continue;
                }

                // println!("cur {}, to go {}", cur, to_go);

                queue.push_front((nr, nc, dist + 1));
            }
        }
    }

    println!("result: {}", visited.len());

    println!("------------------- PART 1 -------------------");
}
