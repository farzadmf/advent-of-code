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
