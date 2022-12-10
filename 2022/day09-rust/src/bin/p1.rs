use day09_rust::{read_input, Direction, Head, Instruction, Position, Tail};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");
    let result = 0;

    let input = read_input("input_small");

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

    let mut head = Head {
        position: Position { x: 0, y: 0 },
    };
    let mut tail = Tail {
        position: Position { x: 0, y: 0 },
    };

    for i in &instructions {
        println!("instruction: {:?}", i);

        for _ in 0..i.count {
            head.go(&i.direction);
            tail.follow(&head);

            println!("\thead {:?} | tail {:?}", head.position, tail.position);
        }

        println!();
    }

    println!("result: {}", result);
    println!("------------------- PART 1 -------------------");
}
