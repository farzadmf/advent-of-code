pub fn part01(input: &str) -> i64 {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut number_lines: Vec<Vec<&str>> = vec![];

    let signs: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();

    for line in lines {
        number_lines.push(line.split_whitespace().collect());
    }

    number_lines
        .iter()
        .fold(
            (0..number_lines[0].len())
                .map(|i| if signs[i] == "+" { 0 } else { 1 })
                .collect(),
            |mut acc: Vec<i64>, number_line| {
                number_line.iter().enumerate().for_each(|(idx2, number)| {
                    let sign = signs[idx2];

                    if sign == "+" {
                        acc[idx2] += (**number).parse::<i64>().unwrap();
                    } else {
                        acc[idx2] *= (**number).parse::<i64>().unwrap();
                    }
                });
                acc
            },
        )
        .iter()
        .sum()
}

pub fn part02(input: &str) -> i64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (ops_row, num_rows) = lines.split_last().unwrap();

    let ops: Vec<char> = ops_row.iter().filter(|c| **c != ' ').cloned().collect();

    let mut idx = 0;
    let mut numbers: Vec<Vec<i64>> = vec![vec![]; ops.len()];
    (0..num_rows[0].len()).for_each(|col| {
        let digits_str: String = num_rows
            .iter()
            .map(|l| l[col])
            .collect::<String>()
            .trim()
            .to_string();

        if digits_str.is_empty() {
            idx += 1;
        } else {
            numbers[idx].push(digits_str.parse().unwrap());
        }
    });

    numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, nums)| {
            if ops[ops.len() - 1 - idx] == '+' {
                return nums.iter().sum::<i64>();
            }

            nums.iter().product::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";
        assert_eq!(4277556, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "
123 328  51 64  
 45 64  387 23  
  6 98  215 314 
*   +   *   +   
";
        assert_eq!(3263827, part02(input.trim_matches('\n')));
    }
}
