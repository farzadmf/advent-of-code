use rust::day08::read_input;

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input/day08");

    let rows = input.lines().collect::<Vec<_>>().len();
    let cols = input.lines().nth(0).unwrap().len();

    let mut jungle: Vec<Vec<u32>> = Vec::new();
    for _ in 0..rows {
        jungle.push(vec![0; cols]);
    }

    for r in 0..rows {
        let row = input.lines().nth(r).unwrap();

        for c in 0..cols {
            let col = row.chars().nth(c).unwrap();

            jungle[r][c] = col.to_digit(10).unwrap();
        }
    }

    // All the edges.
    let mut result = (2 * rows) + 2 * (cols - 2);

    // Collecting columns: https://bit.ly/3uB4hi9
    let mut columns: Vec<Vec<u32>> = Vec::new();
    for c in 0..cols {
        columns.push(jungle.iter().map(|r| r[c]).collect());
    }

    for r in 1..rows - 1 {
        let row = &jungle[r];

        for c in 1..cols - 1 {
            let tree = &row[c];

            let left = &row[..c];
            let right = &row[c + 1..];
            let up = &columns[c][..r];
            let down = &columns[c][r + 1..];

            let left_max = left.iter().max().unwrap();
            let right_max = right.iter().max().unwrap();
            let up_max = up.iter().max().unwrap();
            let down_max = down.iter().max().unwrap();

            if left_max < tree || right_max < tree || up_max < tree || down_max < tree {
                result += 1;
            }
        }
    }

    println!("result: {}", result);
    println!("------------------- PART 1 -------------------");
}
