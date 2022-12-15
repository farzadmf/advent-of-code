use crate::read_input;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

type Rocks = HashMap<(usize, usize), char>;

pub fn get_lines(path: &str) -> Vec<Line> {
    let input = read_input(path);

    let mut lines: Vec<Line> = Vec::new();

    input.lines().for_each(|line| {
        let points = line
            .split("->")
            .map(|part| part.trim())
            .flat_map(|points| points.split(",").flat_map(|p| p.parse::<usize>()))
            .collect::<Vec<_>>();

        for i in (0..points.len() - 2).step_by(2) {
            lines.push(Line {
                p1: Point {
                    x: points[i],
                    y: points[i + 1],
                },
                p2: Point {
                    x: points[i + 2],
                    y: points[i + 3],
                },
            });
        }
    });

    lines
}

pub fn get_rocks(lines: &Vec<Line>) -> Rocks {
    let mut rocks = HashMap::new();

    for line in lines {
        if line.p1.x == line.p2.x {
            let min_y = line.p1.y.min(line.p2.y);
            let max_y = line.p1.y.max(line.p2.y);

            for y in min_y..=max_y {
                rocks.insert((line.p1.x, y), '#');
            }
        } else {
            let min_x = line.p1.x.min(line.p2.x);
            let max_x = line.p1.x.max(line.p2.x);

            for x in min_x..=max_x {
                rocks.insert((x, line.p1.y), '#');
            }
        }
    }
    rocks
}

pub fn display_grid(rocks: &Rocks) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MIN;
    let mut max_y = usize::MIN;

    for (x, y) in rocks.keys() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if rocks.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
