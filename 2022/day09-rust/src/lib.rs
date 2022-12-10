use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub count: usize,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'R' => Ok(Direction::Right),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'U' => Ok(Direction::Up),
            _ => Err("shouldn't happen!".to_string()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn left(&mut self) -> &mut Self {
        if self.x > 0 {
            self.x -= 1;
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        print!("in right first {:?}", self.x);
        self.x += 1;
        print!(" | in right second {:?}", self.x);
        println!();
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.y += 1;
        self
    }

    pub fn down(&mut self) -> &mut Self {
        if self.y > 0 {
            self.y -= 1;
        }
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Knot {
    pub position: Position,
}

impl Knot {
    pub fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    pub fn follow(&mut self, other: &Knot) {
        let y_diff = self.position.y.abs_diff(other.position.y);
        let x_diff = self.position.x.abs_diff(other.position.x);

        let other_is_left = other.position.x < self.position.x;
        let other_is_up = other.position.y > self.position.y;

        if x_diff <= 1 && y_diff <= 1 {
            return;
        }

        println!("x_diff: {:?}, ydiff: {:?}", x_diff, y_diff);

        if x_diff >= 2 && y_diff == 0 {
            if other_is_left {
                self.position.left();
            } else {
                self.position.right();
            }
        } else if y_diff >= 2 && x_diff == 0 {
            if other_is_up {
                self.position.up();
            } else {
                self.position.down();
            }
        } else {
            if other_is_left && other_is_up {
                // left up
                self.position.up().left();
            } else if other_is_up {
                // right up
                self.position.up().right();
            } else if other_is_left {
                // left down
                self.position.down().left();
            } else {
                // right down
                self.position.down().right();
            }
        }
    }
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_instructions(path: &str) -> Vec<Instruction> {
    let input = read_input(path);

    input
        .lines()
        .map(|line| Instruction {
            direction: line.parse::<Direction>().unwrap(),
            count: line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        })
        .collect::<Vec<_>>()
}

pub fn display_grid(head: &Knot, tail: &Knot) {
    let rows = 5;
    let cols = 6;

    for r in 0..rows {
        for c in 0..cols {
            if r == rows - 1 - head.position.y && c == head.position.x {
                print!(" H");
            } else if r == rows - 1 - tail.position.y && c == tail.position.x {
                print!(" T");
            } else {
                print!(" .");
            }
        }
        println!();
    }
    println!("----------------------------")
}
