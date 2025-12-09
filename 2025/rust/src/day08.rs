#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Point(i64, i64, i64);

impl Point {
    fn distance(self, other: Point) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i64> = s
            .split(',')
            .map(|val_str| val_str.parse().unwrap())
            .collect();
        Ok(Self(parts[0], parts[1], parts[2]))
    }
}

fn find(parents: &mut Vec<usize>, val: usize) -> usize {
    if parents[val] != val {
        parents[val] = find(parents, parents[val]);
    }

    parents[val]
}

fn union(parents: &mut Vec<usize>, val1: usize, val2: usize) {
    let root1 = find(parents, val1);
    let root2 = find(parents, val2);

    if root1 != root2 {
        parents[root1] = root2;
    }
}

pub fn part01(input: &str, count: usize) -> i64 {
    let points: Vec<Point> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut distances: Vec<(usize, usize, i64)> = vec![];
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            distances.push((i, j, points[i].distance(points[j])));
        }
    }

    distances.sort_by_key(|dst| dst.2);

    let mut parents: Vec<usize> = { 0..points.len() }.collect();

    for (i, j, _) in distances.into_iter().take(count) {
        union(&mut parents, i, j)
    }

    let mut circuits: HashMap<usize, usize> = HashMap::new();
    for i in 0..parents.len() {
        let root = find(&mut parents, i);
        *circuits.entry(root).or_insert(0) += 1;
    }

    circuits.values().sorted().rev().take(3).product::<usize>() as i64
}

pub fn part02(input: &str) -> i64 {
    let points: Vec<Point> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut distances: Vec<(usize, usize, Point, Point, i64)> = vec![];
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            distances.push((i, j, points[i], points[j], points[i].distance(points[j])));
        }
    }

    distances.sort_by_key(|dst| dst.4);

    let mut parents: Vec<usize> = { 0..points.len() }.collect();

    for (i, j, p1, p2, _) in distances {
        union(&mut parents, i, j);

        let set: HashSet<usize> = { 0..parents.len() - 1 }
            .map(|p| find(&mut parents, p))
            .collect();

        if set.len() == 1 {
            return p1.0 * p2.0;
        }
    }

    5 // Whatever!
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
    }

    #[test]
    fn test_part01() {
        assert_eq!(40, part01(input().trim(), 10));
    }

    #[test]
    fn test_part02() {
        assert_eq!(25272, part02(input().trim()));
    }
}
