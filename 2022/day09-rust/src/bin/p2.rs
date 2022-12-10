use day09_rust::{get_instructions, Knot, Position};
use std::collections::HashSet;

fn main() {
    println!("+++++++++++++++++++ PART 2 +++++++++++++++++++");

    let instructions = get_instructions("input_small_p2");
    let mut tail_path: HashSet<(usize, usize)> = HashSet::new();

    let knot_count = 3;
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
    for i in &instructions[..1] {
        println!("instruction: {:?}", i);

        for _ in 0..i.count {
            knots[0].go(&i.direction);
            println!("    {:?}", knots[0]);

            for i in 1..knot_count {
                let mut folower = knots[i];
                let followee = knots[i - 1];
                folower.follow(&followee);

                println!("        {:?} <-- {:?}", folower, followee);
            }
            tail_path.insert((
                knots[knot_count - 1].position.x,
                knots[knot_count - 1].position.y,
            ));
        }
    }

    println!("------------------- PART 2 -------------------");
}
