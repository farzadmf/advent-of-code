use std::collections::HashSet;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_packet_start(line: &str) -> usize {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let code = parts[0];
    let chars = code.chars().collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..code.len() - 3 {
        let window = chars[i..i + 4].to_vec();
        let set: HashSet<char> = HashSet::from_iter(window);
        if set.len() == 4 {
            result = i + 4;
            break;
        }
    }

    result
}
