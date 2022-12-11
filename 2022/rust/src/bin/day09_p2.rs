use rust::day09::{get_instructions, Knot, Position};
use std::collections::HashSet;

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let instructions = get_instructions("input/day09");
    let mut tail_path: HashSet<(usize, usize)> = HashSet::new();

    let knot_count = 10;
    let start = (1000, 1000);
    let mut knots: Vec<Knot> = Vec::new();
    for _ in 0..knot_count {
        let knot = Knot {
            position: Position {
                x: start.0,
                y: start.1,
            },
        };
        knots.push(knot);
    }

    tail_path.insert(start);
    for i in &instructions {
        for _ in 0..i.count {
            knots[0].go(&i.direction);

            for i in 1..knot_count {
                // WTF Rust???
                // - If I do `let follower = knots[i]`, it creates a copy
                //   of that element and hence `follow` won't affect it!!!
                let followee = knots[i - 1];
                knots[i].follow(&followee);
            }
            tail_path.insert((
                knots[knot_count - 1].position.x,
                knots[knot_count - 1].position.y,
            ));
        }
    }

    println!("result: {}", tail_path.len());

    println!("------------------- PART 2 -------------------");
}
