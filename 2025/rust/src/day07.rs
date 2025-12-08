use std::collections::HashSet;

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

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "
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
";
        assert_eq!(21, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
