use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Add = 2,
    Noop = 1,
}

impl OpCode {
    pub fn to_int(self) -> u8 {
        *(&self) as u8
    }
}

#[derive(Debug)]
pub struct Operation {
    pub code: OpCode,
    // NOTE: too lazy! Even though `noop` doesn't have a value, I set it to `0`!!!
    pub value: i32,
}

pub fn get_operations(path: &str) -> Vec<Operation> {
    let input = read_input(path);

    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let code = match split.nth(0).unwrap() {
                "addx" => OpCode::Add,
                _ => OpCode::Noop,
            };
            // Didn't know: `nth(idx)` removes elements until `idx`!!!??!!!
            let value = split.nth(0).unwrap_or("0").parse::<i32>().unwrap();

            Operation { code, value }
        })
        .collect::<Vec<_>>()
}
