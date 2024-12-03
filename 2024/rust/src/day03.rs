pub fn part01(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
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
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
