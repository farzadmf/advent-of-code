use itertools::Itertools;

pub fn part01(input: &str) -> i32 {
    input
        .lines()
        .map(|bank| {
            bank.chars()
                .combinations(2)
                .unique()
                .map(|pairs| {
                    pairs
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .expect("no number?")
                })
                .max()
                .expect("why can't do max")
        })
        .sum::<i32>()
}

pub fn part02(input: &str) -> i64 {
    input
        .lines()
        .map(|value| {
            let count = 12;
            let mut chars = value.chars().collect::<Vec<_>>();

            let mut joltage = String::new();
            while joltage.len() < count && !chars.is_empty() {
                let remaining = count - joltage.len();
                let end = chars.len() - remaining;
                let part = &chars[..=end];

                // Tried itertools position_max and max_by_key but both give the _last_
                //      index if there are repeating elements.
                let max = part.iter().max().expect("no max?");
                let max_idx = part
                    .iter()
                    .position(|x| x == max)
                    .expect("position failed?");
                joltage.push(chars[max_idx]);
                chars.drain(..=max_idx);
            }
            joltage.parse::<i64>().expect("cannot parse to i64?")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "
987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(357, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "
987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(3121910778619, part02(input.trim()));
    }
}
