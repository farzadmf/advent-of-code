use std::str::FromStr;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let number = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        match chars[0] {
            'L' => Ok(Rotation::Left(number)),
            'R' => Ok(Rotation::Right(number)),
            _ => Err("fuck".to_string()),
        }
    }
}

pub fn part01(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<Rotation>().expect("invalid move"))
        .fold((50, 0), |(value, count), rotation| {
            let new_value = match rotation {
                Rotation::Left(count) => value - count,
                Rotation::Right(count) => value + count,
            };
            let new_value = if new_value < 0 {
                100 + new_value
            } else {
                new_value
            } % 100;
            let new_count = if new_value == 0 { count + 1 } else { count };
            (new_value, new_count)
        })
        .1
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ";
        assert_eq!(3, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
