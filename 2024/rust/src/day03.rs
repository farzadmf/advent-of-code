use regex::Regex;

pub fn part01(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex creation failed!");

    let mut result = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let left: i32 = left.parse().unwrap();
        let right: i32 = right.parse().unwrap();

        result += left * right;
    }

    result
}

pub fn part02(input: &str) -> i32 {
    let re =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").expect("regex creation failed!");

    // After wasting 2 hours for a complicated solution (to keep track of start of conditions,
    // matching against start of multiplies etc.), got the idea of "enabled" from:
    // https://www.perrotta.dev/2024/12/advent-of-code-day-3/
    let mut enabled = true;
    let mut result = 0;

    for c in re.captures_iter(input) {
        match c.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if !enabled {
                    continue;
                }

                let left: i32 = c.get(1).unwrap().as_str().parse().unwrap();
                let right: i32 = c.get(2).unwrap().as_str().parse().unwrap();

                result += left * right;
            }
        }
    }

    result
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
