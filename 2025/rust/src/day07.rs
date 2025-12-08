use std::collections::{HashMap, HashSet};

pub fn part01(input: &str) -> i32 {
    let mut manifold: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = (1, manifold[0].iter().position(|&ch| ch == 'S').unwrap());

    manifold[start.0][start.1] = '|';

    let mut total = 0;
    for r in 0..manifold.len() - 1 {
        let cur = manifold[r].clone();
        let next = &mut manifold[r + 1];

        let beams: HashSet<usize> = cur
            .clone()
            .iter()
            .enumerate()
            .filter_map(|(idx, &cell)| if cell == '|' { Some(idx) } else { None })
            .collect();

        beams.iter().for_each(|&idx| {
            if next[idx] == '^' {
                total += 1;
                if idx > 0 {
                    next[idx - 1] = '|';
                }
                if idx + 1 < cur.len() {
                    next[idx + 1] = '|';
                }
            } else {
                next[idx] = '|';
            }
        });
    }

    total
}

pub fn part02(input: &str) -> i64 {
    fn travel(
        manifold: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if row == manifold.len() {
            return 1;
        }

        if let Some(&result) = cache.get(&(row, col)) {
            return result;
        }

        let cell = manifold[row][col];
        let result = if cell == '^' {
            let left = if col > 0 {
                travel(manifold, row + 1, col - 1, cache)
            } else {
                0
            };
            let right = if col < manifold[row].len() - 1 {
                travel(manifold, row + 1, col + 1, cache)
            } else {
                0
            };
            left + right
        } else {
            travel(manifold, row + 1, col, cache)
        };

        cache.insert((row, col), result);
        result
    }

    let manifold: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = manifold[0].iter().position(|&ch| ch == 'S').unwrap();

    let mut cache = HashMap::new();
    travel(&manifold, 1, start, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
    }

    #[test]
    fn test_part01() {
        assert_eq!(21, part01(input().trim()));
    }

    #[test]
    fn test_part02() {
        assert_eq!(40, part02(input().trim()));
    }
}
