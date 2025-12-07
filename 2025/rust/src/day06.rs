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
    let mut lines: Vec<&str> = input.lines().collect();
    let mut number_lines: Vec<Vec<&str>> = vec![];

    let signs: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();

    let mut numbers: Vec<Vec<char>> = vec![vec![]; signs.len()];
    lines.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, number)| {
            numbers[idx].push(number);
            dbg!(idx, number, &numbers);
        });

        // number_lines.push(line.split_whitespace().collect());
    });

    dbg!(numbers);

    // number_lines
    //     .iter()
    //     .enumerate()
    //     .for_each(|(idx, number_line)| {});

    5
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
        assert_eq!(3263827, part02(input.trim()));
    }
}
