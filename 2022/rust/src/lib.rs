pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
// pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
