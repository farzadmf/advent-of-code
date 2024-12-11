#[derive(Debug, Copy, Clone)]
struct Value(i64);

pub fn part01(input: &str) -> i64 {
    let mut converted = "".to_string();
    let mut id = 0;

    let mut converted2 = vec![];

    input.chars().enumerate().for_each(|(pos, c)| {
        let value = c.to_string().parse().unwrap();

        let mut to_add = ".".to_string();
        let mut to_add2 = -1;
        if pos % 2 == 0 {
            to_add2 = id;
            to_add = id.to_string();
            id += 1;
        }

        converted.push_str(to_add.repeat(value).as_str());
        for _ in 0..value {
            converted2.push(Value(to_add2));
        }
    });

    let mut my = converted2;
    let (mut l, mut r) = (0, my.len() - 1);
    while l < r {
        while my[l].0 != -1 {
            l += 1;
        }
        while my[r].0 == -1 {
            r -= 1;
        }

        if l < r && my[l].0 == -1 && my[r].0 != -1 {
            (my[l], my[r]) = (my[r], my[l]);
            (l, r) = (l + 1, r - 1);
        }
    }

    let result = my.iter().enumerate().fold(0, |acc, (pos, c)| {
        if c.0 == -1 {
            return acc;
        }

        acc + (pos as i64) * (c.0)
    });

    result
}

pub fn part02(input: &str) -> i64 {
    input.lines().count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "
2333133121414131402
";
        assert_eq!(1928, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
