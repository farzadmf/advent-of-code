use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Crate {
    pub letter: char,
}

impl Crate {
    pub fn from_line(line: &str) -> Vec<Option<Crate>> {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|ch| ch.iter().collect::<String>())
            .map(|parts| match parts.trim().parse::<Crate>() {
                Ok(my_crate) => Some(my_crate),
                Err(_) => None,
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err("".to_string()),
            _ => Ok(Crate {
                letter: s.chars().nth(1).unwrap(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub count: u16,
    pub from: u8,
    pub to: u8,
}

impl Move {
    pub fn from_line(line: &str) -> Move {
        let parts = line.split(" ").collect::<Vec<_>>();

        Move {
            count: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        }
    }

    pub fn from_lines(move_lines: Vec<&str>) -> Vec<Move> {
        move_lines
            .into_iter()
            .map(|line| Move::from_line(line))
            .collect::<Vec<_>>()
    }
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_stack_count(input: &str) -> usize {
    input
        .lines()
        .find(|line| line.trim().starts_with("1"))
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .len()
}
