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

impl Position {
    pub fn left(&mut self) -> &mut Self {
        if self.x > 0 {
            self.x -= 1;
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.x += 1;
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
                self.position.left();
            } else {
                self.position.right();
            }
        } else if y_diff >= 2 && x_diff == 0 {
            if head_is_up {
                self.position.up();
            } else {
                self.position.down();
            }
        } else {
            if head_is_left && head_is_up {
                // left up
                self.position.up().left();
            } else if head_is_up {
                // right up
                self.position.up().right();
            } else if head_is_left {
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

pub fn display_grid(head: &Head, tail: &Tail) {
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
