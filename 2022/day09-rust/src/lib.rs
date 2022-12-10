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

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Head {
    pub position: Position,
}

impl Head {
    pub fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }
}

#[derive(Debug)]
pub struct Tail {
    pub position: Position,
}

impl Tail {
    pub fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    pub fn follow(&mut self, head: &Head) {
        let y_diff = self.position.y.abs_diff(head.position.y);
        let x_diff = self.position.x.abs_diff(head.position.x);

        let head_is_left = head.position.x < self.position.x;
        let head_is_up = head.position.y > self.position.y;

        if x_diff <= 1 && y_diff <= 1 {
            return;
        }

        if x_diff >= 2 && y_diff == 0 {
            if head_is_left {
                self.position.x -= 1
            } else {
                self.position.x += 1
            }
        } else if y_diff >= 2 && x_diff == 0 {
            if head_is_up {
                self.position.y += 1
            } else {
                self.position.y -= 1
            }
        } else {
            // Cannot find the logic here! How to forumate diagonal???
            todo!();
        }
    }
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
