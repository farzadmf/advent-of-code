use std::collections::HashSet;

use day09_rust::{display_grid, read_input, Direction, Head, Instruction, Position, Tail};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input");
    let mut tail_path: HashSet<(usize, usize)> = HashSet::new();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction {
            direction: line.parse::<Direction>().unwrap(),
            count: line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        })
        .collect::<Vec<_>>();

    // NOTE: trick is to NOT start from (0, 0)!!!
    let mut head = Head {
        position: Position { x: 1000, y: 1000 },
    };
    let mut tail = Tail {
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
