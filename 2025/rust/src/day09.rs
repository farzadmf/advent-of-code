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

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
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
        assert_eq!(1, part02(input().trim()));
    }
}
