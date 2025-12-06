use std::ops::RangeInclusive;

use itertools::Itertools;

fn combine_ranges(range_str: &str) -> Vec<RangeInclusive<i64>> {
    range_str
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            left.parse().unwrap()..=right.parse().unwrap()
        })
        .sorted_by_key(|r| *r.start())
        .fold(vec![], |mut acc, range| {
            let (mut start, mut end) = (*range.start(), *range.end());

            if let Some(last) = acc.pop() {
                let (last_start, last_end) = (*last.start(), *last.end());

                if start <= last_end {
                    start = last_start;
                    end = end.max(last_end);
                } else {
                    acc.push(last_start..=last_end)
                }
            }

            acc.push(start..=end);

            acc
        })
}

pub fn part01(input: &str) -> i64 {
    let (first, second) = input.split_once("\n\n").expect("cound't split on new line");

    let ranges = combine_ranges(first);
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

pub fn part02(input: &str) -> i64 {
    let (first, _) = input.split_once("\n\n").expect("cound't split on new line");

    let ranges = combine_ranges(first);

    ranges
        .iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum()
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
        assert_eq!(14, part02(input.trim()));
    }
}
