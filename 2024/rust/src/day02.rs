fn is_safe(report: &str) -> bool {
    let values: Vec<i32> = report
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let total = values.len();
    let is_increasing = values[0] < values[total - 1];
    let mut is_safe = true;

    for i in 0..values.len() - 1 {
        let (cur, nxt) = (values[i], values[i + 1]);

        if (nxt == cur)
            || (is_increasing && nxt < cur)
            || (!is_increasing && nxt > cur)
            || (nxt - cur).abs() > 3
        {
            is_safe = false;
            break;
        }
    }

    is_safe
}

pub fn part01(input: &str) -> i32 {
    input
        .lines()
        .map(is_safe)
        .filter(|value| *value)
        .count()
        .try_into()
        .unwrap()
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(2, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
