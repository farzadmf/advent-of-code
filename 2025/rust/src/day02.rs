pub fn part01(input: &str) -> i64 {
    input
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|values| values.parse::<i64>().expect("no number?"))
                .collect::<Vec<_>>()
        })
        .map(|numbers| numbers[0]..=numbers[1])
        .flat_map(|range| range.collect::<Vec<_>>())
        .map(|number| number.to_string())
        .filter(|number| number.len() % 2 == 0)
        .filter(|number| {
            let half = number.len() / 2;
            number[..half] == number[half..]
        })
        .map(|number| number.parse::<i64>().expect("no number?"))
        .sum::<i64>()
}

pub fn part02(input: &str) -> i32 {
    input.lines().count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
