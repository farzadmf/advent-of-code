use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part01(input: &str) -> i64 {
    let (first, second) = input.split_once("\n\n").expect("cound't split on new line");
    let _ranges: Vec<RangeInclusive<i64>> = first
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            left.parse().unwrap()..=right.parse().unwrap()
        })
        .sorted_by_key(|r| *r.start())
        .collect();

    let mut ranges: Vec<RangeInclusive<i64>> = vec![];
    let mut start = *_ranges[0].start();
    let mut end = *_ranges[0].end();
    for range in &_ranges {
        if *range.start() <= end {
            end = *range.end().max(&end);
        } else if *range.start() > end {
            ranges.push(start..=end);
            start = *range.start();
            end = *range.end();
        }
    }
    ranges.push(start..=end);

    let numbers: Vec<i64> = second.lines().map(|str| str.parse().unwrap()).collect();

    let mut spoiled = 0;
    for number in &numbers {
        for range in &ranges {
            if range.contains(number) {
                spoiled += 1;
                break;
            }
        }
    }

    spoiled
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(3, part01(input.trim()));
    }

    #[test]
    fn test_part02() {
        let input = "part02";
        assert_eq!(1, part02(input.trim()));
    }
}
