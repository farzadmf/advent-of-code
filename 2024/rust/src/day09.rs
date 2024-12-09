pub fn part01(input: &str) -> i32 {
    let mut converted = "".to_string();
    let mut id = 0;

    println!("{:+^60}", " EVENS ");
    input.chars().enumerate().for_each(|(pos, c)| {
        let value = c.to_string().parse().unwrap();

        let mut to_add = ".".to_string();
        if pos % 2 == 0 {
            to_add = id.to_string();
            id += 1;
        }

        converted.push_str(to_add.repeat(value).as_str());
    });

    id = 0;
    let my = input
        .chars()
        .enumerate()
        .fold("".to_string(), |acc, (pos, c)| {
            let count = c.to_string().parse().unwrap();
            let mut to_add = ".".to_string();

            if pos % 2 == 0 {
                to_add = id.to_string();
                id += 1;
            }

            format!("{}{}", acc, to_add.repeat(count))
        });

    let mut my: Vec<_> = my.as_str().chars().collect();

    let (mut l, mut r) = (0, my.len() - 1);
    while l < r {
        while my[l] != '.' {
            l += 1;
        }
        while my[r] == '.' {
            r -= 1;
        }

        if l < r && my[l] == '.' && my[r] != '.' {
            (my[l], my[r]) = (my[r], my[l]);
            (l, r) = (l + 1, r - 1);
        }
    }

    let result = my.iter().enumerate().fold(0, |acc, (pos, c)| {
        acc + pos * (c.to_string().parse::<usize>().unwrap_or(0))
    });

    result as i32
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
