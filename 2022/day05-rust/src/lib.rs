use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Crate {
    letter: char,
}

impl Crate {
    // fn from_input(input: &str) -> Vec<Crate> {
    //     input
    //         .lines()
    //         .take_while(|line| line.to_string() != "")
    //         .map(|line| {
    //             // let parts = line.split(" ").collect::<Vec<&str>>();
    //             line.split(" ")
    //         })
    // }
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Crate {
            letter: s.chars().nth(1).unwrap(),
        })
    }
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap().to_string()
}
