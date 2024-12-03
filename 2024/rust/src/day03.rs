use core::str;

use regex::Regex;

pub fn part01(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex creation failed!");

    let mut result = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let left = left.parse::<i32>().unwrap();
        let right = right.parse::<i32>().unwrap();

        result += left * right;
    }

    result
}

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part02(input.trim()));
    }
}
