use std::collections::HashSet;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_start(line: &str, win_size: usize) -> usize {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let code = parts[0];
    let chars = code.chars().collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..code.len() - (win_size - 1) {
        let window = chars[i..i + win_size].to_vec();
        let set: HashSet<char> = HashSet::from_iter(window);
        if set.len() == win_size {
            result = i + win_size;
            break;
        }
    }

    result
}
