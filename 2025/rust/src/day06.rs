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

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
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
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
