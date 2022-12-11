use std::fs;

pub fn read_input(input: String) -> String {
    fs::read_to_string(input).unwrap().trim().to_string()
}

pub fn get_code(ch: char) -> Option<u32> {
    if !ch.is_alphabetic() {
        None
    } else if ch.is_lowercase() {
        Some((ch as u32) - ('a' as u32) + 1)
    } else {
        Some((ch as u32) - ('A' as u32) + 27)
    }
}

/*
 * a b c d e f g h i j  k  l  m  n  o  p
 * 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
*/

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
