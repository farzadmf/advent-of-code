use std::collections::HashSet;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_start(line: &str, win_size: usize) -> usize {
    // This is mainly for my 'input_small' file because it contains the wanted numbers.
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let code = parts[0];
    let chars = code.chars().collect::<Vec<_>>();

    let mut result = 0;
    for (idx, window) in chars.windows(win_size).enumerate() {
        let set: HashSet<_> = window.iter().collect();
        if set.len() == win_size {
            result = idx + win_size;
            break;
        }
    }

    result
}
