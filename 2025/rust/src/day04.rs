pub fn part01(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell != '@' {
                continue;
            }

            let mut neighs: Vec<char> = Vec::new();

            for (dr, dc) in dirs {
                let (nr, nc) = (row_idx as i32 + dr, col_idx as i32 + dc);
                if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < row.len() as i32 {
                    neighs.push(grid[nr as usize][nc as usize]);
                }
            }

            if neighs.iter().filter(|&c| *c == '@').count() < 4 {
                total += 1;
            }
        }
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(13, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
