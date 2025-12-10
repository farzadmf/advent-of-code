#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone)]
struct Tile(i64, i64);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Tile {
    fn area(self, other: Tile) -> i64 {
        ((self.0 - other.0).abs() + 1) * ((self.1 - other.1).abs() + 1)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i64> = s.split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Self(parts[0], parts[1]))
    }
}

pub fn part01(input: &str) -> i64 {
    let tiles: Vec<Tile> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut max_area = 0;
    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            max_area = tiles[i].area(tiles[j]).max(max_area);
        }
    }

    max_area
}

fn connect(grid: &mut [Vec<char>], t1: Tile, t2: Tile) {
    if t1.0 == t2.0 {
        (t1.1.min(t2.1) + 1..t1.1.max(t2.1)).for_each(|x| {
            grid[x as usize][t1.0 as usize] = 'X';
        });
    }
    if t1.1 == t2.1 {
        (t1.0.min(t2.0) + 1..t1.0.max(t2.0)).for_each(|y| {
            grid[t1.1 as usize][y as usize] = 'X';
        });
    }
}

fn mark_outside(grid: &mut [Vec<char>]) {
    let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut stack = vec![(0, 0)];
    while let Some((x, y)) = stack.pop() {
        grid[x][y] = 'O';

        for (dx, dy) in dirs {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            if nx >= 0
                && nx < grid.len() as i32
                && ny >= 0
                && ny < grid[0].len() as i32
                && grid[nx as usize][ny as usize] == '.'
            {
                stack.push((nx as usize, ny as usize));
            }
        }
    }
}

fn mark_greens(grid: &mut [Vec<char>]) {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                grid[x][y] = 'X';
            } else if grid[x][y] == 'O' {
                grid[x][y] = '.';
            }
        }
    }
}

pub fn part02(input: &str) -> i32 {
    let tiles: Vec<Tile> = input.lines().map(|l| l.parse().unwrap()).collect();

    let (max_x, max_y) = tiles
        .iter()
        .fold((0, 0), |acc, &t| (acc.0.max(t.0), acc.1.max(t.1)));

    let mut grid = vec![vec!['.'; max_x as usize + 3]; max_y as usize + 2];
    for i in 0..tiles.len() - 1 {
        let cur = tiles[i];
        let next = tiles[i + 1];

        grid[cur.1 as usize][cur.0 as usize] = '#';
        grid[next.1 as usize][next.0 as usize] = '#';

        connect(&mut grid, cur, next);
    }
    connect(&mut grid, tiles[tiles.len() - 1], tiles[0]);

    mark_outside(&mut grid);
    mark_greens(&mut grid);

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }

    dbg!(max_x, max_y);

    5
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
    }

    #[test]
    fn test_part01() {
        assert_eq!(50, part01(input().trim()));
    }

    #[test]
    fn test_part02() {
        assert_eq!(24, part02(input().trim()));
    }
}
