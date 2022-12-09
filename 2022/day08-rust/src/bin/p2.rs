use day08_rust::read_input;

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");
    let input = read_input("input");

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
    let mut result = u32::MIN;

    // Collecting columns: https://bit.ly/3uB4hi9
    let mut columns: Vec<Vec<u32>> = Vec::new();
    for c in 0..cols {
        columns.push(jungle.iter().map(|r| r[c]).collect());
    }

    for r in 1..rows - 1 {
        let row = &jungle[r];

        for c in 1..cols - 1 {
            let tree = &row[c];

            let left = row[..c].iter().rev().collect::<Vec<_>>();
            let right = &row[c + 1..];
            let up = columns[c][..r].iter().rev().collect::<Vec<_>>();
            let down = &columns[c][r + 1..];

            let left_blocker = left
                .iter()
                .position(|t| t >= &tree)
                .unwrap_or(left.len() - 1) as u32
                + 1;
            let right_blocker = right
                .iter()
                .position(|t| t >= tree)
                .unwrap_or(right.len() - 1) as u32
                + 1;
            let up_blocker = up.iter().position(|t| t >= &tree).unwrap_or(up.len() - 1) as u32 + 1;
            let down_blocker: u32 = down
                .iter()
                .position(|t| t >= tree)
                .unwrap_or(down.len() - 1) as u32
                + 1;

            let value: u32 = left_blocker * right_blocker * up_blocker * down_blocker;

            // println!("for {:?} ({}, {})", tree, r, c);
            // println!("- left ({:?}) blocker position: {:?}", left, left_blocker);
            // println!(
            //     "- right ({:?}) blocker position: {:?}",
            //     right, right_blocker
            // );
            // println!("- down ({:?}) blocker position: {:?}", down, down_blocker);
            // println!("- up ({:?}) blocker position: {:?}", up, up_blocker);
            // println!("- value: {}", value);

            if value > result {
                result = value;
            }

            // let left_max = left.max().unwrap();
            // let right_max = right.iter().max().unwrap();
            // let up_max = up.max().unwrap();
            // let down_max = down.iter().max().unwrap();

            // if left_max < tree || right_max < tree || up_max < tree || down_max < tree {
            //     result += 1;
            // }
        }
    }

    println!("result: {}", result);
    println!("------------------- PART 2 -------------------");
}
