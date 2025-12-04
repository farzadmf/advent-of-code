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

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
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
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
