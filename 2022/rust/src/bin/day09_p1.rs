use std::collections::HashSet;

use rust::day09::{get_instructions, Knot, Position};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let mut tail_path: HashSet<(usize, usize)> = HashSet::new();

    let instructions = get_instructions("input/day09");

    // NOTE: trick is to NOT start from (0, 0)!!!
    let mut head = Knot {
        position: Position { x: 1000, y: 1000 },
    };
    let mut tail = Knot {
        position: Position { x: 1000, y: 1000 },
    };

    // display_grid(&head, &tail);
    tail_path.insert((tail.position.x, tail.position.y));
    for i in &instructions {
        // println!("instruction: {:?}", i);

        for _ in 0..i.count {
            head.go(&i.direction);
            tail.follow(&head);
            tail_path.insert((tail.position.x, tail.position.y));

            // display_grid(&head, &tail);
            // println!("\thead {:?} | tail {:?}", head.position, tail.position);
        }

        // println!();
    }

    println!("result: {:?}", tail_path.len());
    println!("------------------- PART 1 -------------------");
}
