use std::fs;

pub fn read_input(input: String) -> String {
    fs::read_to_string(input).unwrap().trim().to_string()
}

pub fn get_points(line: &str) -> (u32, u32, u32, u32) {
    let ranges = line.split(",").collect::<Vec<&str>>();
    let part1 = ranges[0]
        .split("-")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let part2 = ranges[1]
        .split("-")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // println!(
    //     "({} <-> {}) | ({} <-> {})",
    //     part1[0], part1[1], part2[0], part2[1]
    // );

    (part1[0], part1[1], part2[0], part2[1])
}

pub fn covers(coords: (u32, u32, u32, u32)) -> u32 {
    let (f_st, f_end, s_st, s_end) = coords;

    let res = if (f_st >= s_st && f_end <= s_end) || (s_st >= f_st && s_end <= f_end) {
        1
    } else {
        0
    };

    // dbg!(res);
    res
}

pub fn overlaps(coords: (u32, u32, u32, u32)) -> u32 {
    let (f_st, f_end, s_st, s_end) = coords;

    // Missed this one where they're totally "un-overlappable"
    let res = if f_end < s_st || s_end < f_st {
        0
    } else if f_end >= s_st || s_end >= f_st {
        1
    } else {
        0
    };

    // dbg!(res);
    res
}
