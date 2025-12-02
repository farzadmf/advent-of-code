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
            let delta = match rotation {
                Rotation::Left(count) => -count,
                Rotation::Right(count) => count,
            };
            let new_value = (value + delta).rem_euclid(100);
            (new_value, count + (new_value == 0) as i32)
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
