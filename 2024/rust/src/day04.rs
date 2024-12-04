struct Point(isize, isize);

fn xmas_count(table: &[Vec<char>], row: isize, col: isize) -> i32 {
    let mut count = 0;

    let (rows, cols) = (table.len() as isize, table[0].len() as isize);

    if table[row as usize][col as usize] != 'X' {
        return 0;
    }

    let directions = vec![
        [Point(1, 0), Point(2, 0), Point(3, 0)],       // down
        [Point(-1, 0), Point(-2, 0), Point(-3, 0)],    // up
        [Point(0, 1), Point(0, 2), Point(0, 3)],       // right
        [Point(0, -1), Point(0, -2), Point(0, -3)],    // left
        [Point(-1, -1), Point(-2, -2), Point(-3, -3)], // up-left
        [Point(-1, 1), Point(-2, 2), Point(-3, 3)],    // up-right
        [Point(1, -1), Point(2, -2), Point(3, -3)],    // down-left
        [Point(1, 1), Point(2, 2), Point(3, 3)],       // down-right
    ];

    for [p1, p2, p3] in directions {
        let (nr1, nc1) = (row + p1.0, col + p1.1);
        let (nr2, nc2) = (row + p2.0, col + p2.1);
        let (nr3, nc3) = (row + p3.0, col + p3.1);

        if nr1 < 0
            || nr1 >= rows
            || nr2 < 0
            || nr2 >= rows
            || nr3 < 0
            || nr3 >= rows
            || nc1 < 0
            || nc1 >= cols
            || nc2 < 0
            || nc2 >= cols
            || nc3 < 0
            || nc3 >= cols
        {
            continue;
        }

        if table[nr1 as usize][nc1 as usize] == 'M'
            && table[nr2 as usize][nc2 as usize] == 'A'
            && table[nr3 as usize][nc3 as usize] == 'S'
        {
            count += 1;
        }
    }

    count
}

fn x_mas_count(table: &[Vec<char>], row: isize, col: isize) -> i32 {
    let mut count = 0;

    let (rows, cols) = (table.len() as isize, table[0].len() as isize);

    if table[row as usize][col as usize] != 'A' {
        return 0;
    }

    let (ulr, ulc) = (row - 1, col - 1); // up-left
    let (drr, drc) = (row + 1, col + 1); // down-right

    let (urr, urc) = (row - 1, col + 1); // up-right
    let (dlr, dlc) = (row + 1, col - 1); // down-left

    let is_good_row = |row: isize| row >= 0 && row < rows;
    let is_good_col = |col: isize| col >= 0 && col < cols;

    if is_good_row(ulr)
        && is_good_row(urr)
        && is_good_row(dlr)
        && is_good_row(drr)
        && is_good_col(ulc)
        && is_good_col(urc)
        && is_good_col(dlc)
        && is_good_col(drc)
        // up-left|down-right is 'SM' or 'MS'
        && ((table[ulr as usize][ulc as usize] == 'S' && table[drr as usize][drc as usize] == 'M')
            || (table[ulr as usize][ulc as usize] == 'M'
                && table[drr as usize][drc as usize] == 'S'))
        // up-right|down-left is 'SM' or 'MS'
        && ((table[urr as usize][urc as usize] == 'S' && table[dlr as usize][dlc as usize] == 'M')
            || (table[urr as usize][urc as usize] == 'M'
                && table[dlr as usize][dlc as usize] == 'S'))
    {
        count += 1;
    }

    count
}

pub fn part01(input: &str) -> i32 {
    let mut table: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        table.push(vec![]);
        let len = table.len();
        table[len - 1] = line.chars().collect();
    }

    let (rows, cols) = (table.len(), table[0].len());
    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            result += xmas_count(&table, row as isize, col as isize);
        }
    }

    result
}

pub fn part02(input: &str) -> i32 {
    let mut table: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        table.push(vec![]);
        let len = table.len();
        table[len - 1] = line.chars().collect();
    }

    let (rows, cols) = (table.len(), table[0].len());
    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            result += x_mas_count(&table, row as isize, col as isize);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!(18, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!(9, part02(input.trim()));
    }
}
