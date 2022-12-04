use std::fs;

pub fn read_input(input: String) -> String {
    fs::read_to_string(input).unwrap().trim().to_string()
}

pub fn get_code(ch: &char) -> u32 {
    let a_value = 'a' as u32;
    let big_a_value = 'A' as u32;
    let c_value = *ch as u32;

    match ch {
        'a'..='z' => c_value - a_value + 1,
        'A'..='Z' => c_value - big_a_value + 27,
        _ => panic!("unexpected char {}", ch),
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
